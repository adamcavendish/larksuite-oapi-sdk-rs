package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestExtractRustContractsParsesLiteralRequest(t *testing.T) {
	source := `
impl MessageResource<'_> {
    pub async fn upload(&self, option: &RequestOption) -> Result<(), LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/im/v1/messages/:message_id/files",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .file_upload()
        .send()
        .await
    }
}
`

	contracts, unparsed := extractRustContracts("src/service/im/v1.rs", source)
	if len(unparsed) != 0 || len(contracts) != 1 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
	contract := contracts[0]
	if contract.Method != "POST" || contract.Path != "/open-apis/im/v1/messages/{}/files" {
		t.Fatalf("contract route = %s %s", contract.Method, contract.Path)
	}
	if !contract.FileUpload || !equalTokenTypes(contract.TokenTypes, []string{"Tenant", "User"}) {
		t.Fatalf("contract metadata = %#v", contract)
	}
}

func TestExtractRustContractsRecognizesFormBodyAsFileUpload(t *testing.T) {
	source := `
impl FileResource<'_> {
    pub async fn upload(&self, fields: Vec<FormDataField>, option: &RequestOption) -> Result<(), LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/im/v1/files",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(fields)
        .send()
        .await
    }
}
`

	contracts, unparsed := extractRustContracts("src/service/im/v1.rs", source)
	if len(unparsed) != 0 || len(contracts) != 1 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
	if !contracts[0].FileUpload {
		t.Fatalf("contract metadata = %#v", contracts[0])
	}
}

func TestExtractRustContractsParsesFormattedPath(t *testing.T) {
	source := `
impl DataSourceResource<'_> {
    pub async fn get(&self, data_source_id: &str, option: &RequestOption) -> Result<(), LarkError> {
        let path = format!("/open-apis/search/v2/data_sources/{}", data_source_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send()
        .await
    }
}
`

	contracts, unparsed := extractRustContracts("src/service/search/v2.rs", source)
	if len(unparsed) != 0 || len(contracts) != 1 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
	if contracts[0].Path != "/open-apis/search/v2/data_sources/{}" {
		t.Fatalf("contract path = %q", contracts[0].Path)
	}
}

func TestExtractRustContractsParsesOkrV2RequestHelper(t *testing.T) {
	source := `
fn request<'a>(config: &'a Config, method: http::Method, path: impl Into<String>, option: &'a RequestOption) -> RestRequest<'a> {
    RestRequest::new(config, method, path, vec![AccessTokenType::User, AccessTokenType::Tenant], option)
}
impl OkrCategoryResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<(), LarkError> {
        request(self.config, http::Method::GET, "/open-apis/okr/v2/categories", option)
            .send()
            .await
    }
}
`

	contracts, unparsed := extractRustContracts("src/service/okr/v2.rs", source)
	if len(unparsed) != 0 || len(contracts) != 1 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
	contract := contracts[0]
	if contract.Function != "list" || contract.Method != "GET" || contract.Path != "/open-apis/okr/v2/categories" {
		t.Fatalf("contract = %#v", contract)
	}
	if !equalTokenTypes(contract.TokenTypes, []string{"Tenant", "User"}) {
		t.Fatalf("token types = %#v", contract.TokenTypes)
	}
}

func TestExtractRustContractsSkipsMacroDefinitions(t *testing.T) {
	source := `
macro_rules! external_resource {
    ($base_path:literal) => {
        pub async fn create(&self, option: &RequestOption) -> Result<(), LarkError> {
            RestRequest::new(
                self.config,
                http::Method::POST,
                $base_path,
                vec![AccessTokenType::Tenant],
                option,
            )
            .send()
            .await
        }
    };
}
`

	contracts, unparsed := extractRustContracts("src/service/hire/v1.rs", source)
	if len(contracts) != 0 || len(unparsed) != 0 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
}

func TestExtractRustContractsDoesNotInventRoutesForUnusedHelpers(t *testing.T) {
	source := `
async fn request(method: http::Method) {
    RestRequest::new(
        config,
        method,
        "/open-apis/base/v3/bases/:base_token/tables/:table_id/fields/:field_id/field_extensions",
        vec![AccessTokenType::User, AccessTokenType::Tenant],
        option,
    )
}
`

	contracts, unparsed := extractRustContracts("src/service/base/v3.rs", source)
	if len(contracts) != 0 || len(unparsed) != 0 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
}

func TestExtractRustContractsExpandsSupportedMacroInvocations(t *testing.T) {
	source := `
post_method!(search, SearchResp, (), "/open-apis/corehr/v2/banks/search");
get_method!(query_recent_change, RecentChangeResp, RecentChangeData, "/open-apis/corehr/v2/banks/query_recent_change");
post_query!(ActivityResource, ActivityQuery, query, query_by_query, ActivityData, ActivityResp, "/open-apis/performance/v2/activity/query");
external_crud_resource!(ExternalApplicationResource, "/open-apis/hire/v1/external_applications", CreateResp, CreateData, UpdateResp, UpdateData, DeleteResp, external_application_id);
`

	contracts, unparsed := extractRustContracts("src/service/example.rs", source)
	if len(unparsed) != 0 || len(contracts) != 6 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}

	want := map[string]string{
		"POST /open-apis/corehr/v2/banks/search":             "search",
		"GET /open-apis/corehr/v2/banks/query_recent_change": "query_recent_change",
		"POST /open-apis/performance/v2/activity/query":      "ActivityResource",
		"POST /open-apis/hire/v1/external_applications":      "create",
		"PUT /open-apis/hire/v1/external_applications/{}":    "update",
		"DELETE /open-apis/hire/v1/external_applications/{}": "delete",
	}
	for _, contract := range contracts {
		key := routeKey(contract.Method, contract.Path)
		if want[key] != contract.Function {
			t.Fatalf("unexpected contract %#v", contract)
		}
		if !equalTokenTypes(contract.TokenTypes, []string{"Tenant"}) || contract.FileUpload {
			t.Fatalf("contract metadata = %#v", contract)
		}
		delete(want, key)
	}
	if len(want) != 0 {
		t.Fatalf("missing contracts = %#v", want)
	}
}

func TestExtractRustContractsReportsUnsupportedMacroPath(t *testing.T) {
	source := `
post_method!(search, SearchResp, (), path);
`

	contracts, unparsed := extractRustContracts("src/service/corehr/v2.rs", source)
	if len(contracts) != 0 || len(unparsed) != 1 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
	if unparsed[0].Reason != "non-literal supported macro request path" {
		t.Fatalf("unparsed reason = %q", unparsed[0].Reason)
	}
}

func TestExtractRustContractsExpandsTokenAwarePostMacro(t *testing.T) {
	source := `
post_method_with_tokens!(parents, ParentsResp, ParentsData, "/open-apis/corehr/v2/departments/parents", vec![AccessTokenType::Tenant, AccessTokenType::User]);
`

	contracts, unparsed := extractRustContracts("src/service/corehr/v2.rs", source)
	if len(unparsed) != 0 || len(contracts) != 1 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
	contract := contracts[0]
	if contract.Function != "parents" || contract.Method != "POST" || contract.Path != "/open-apis/corehr/v2/departments/parents" {
		t.Fatalf("contract route = %#v", contract)
	}
	if !equalTokenTypes(contract.TokenTypes, []string{"Tenant", "User"}) {
		t.Fatalf("contract token types = %#v", contract.TokenTypes)
	}
}

func TestMetadataMatchingNormalizesTokenOrder(t *testing.T) {
	contract := goEndpoint{TokenTypes: []string{"User", "Tenant"}, FileUpload: true}
	candidates := []rustEndpoint{{TokenTypes: []string{"Tenant", "User"}, FileUpload: true}}

	if !anyMetadataMatches(contract, candidates) {
		t.Fatal("expected metadata to match after token normalization")
	}
}

func TestCurrentParityBaselineHasNoUnresolvedContracts(t *testing.T) {
	generated, err := generate("go_service_catalog.json", "go_rust_service_parity_omissions.json", "..")
	if err != nil {
		t.Fatal(err)
	}

	var report parityReport
	if err := json.Unmarshal(generated, &report); err != nil {
		t.Fatal(err)
	}
	if report.SchemaVersion != 3 {
		t.Fatalf("schema version = %d, want 3", report.SchemaVersion)
	}
	if report.Summary.MetadataMismatches != 0 || report.Summary.MissingContracts != 0 || report.Summary.UnparsedRustRequests != 0 {
		t.Fatalf("parity summary = %#v", report.Summary)
	}
	if report.Summary.IntentionalOmissions != 1 || len(report.IntentionalOmissions) != 1 {
		t.Fatalf("intentional omissions = %#v", report.IntentionalOmissions)
	}
}

func TestSparkSessionContractsAreParsed(t *testing.T) {
	sourceFile := "src/service/spark/v1/session.rs"
	source, err := os.ReadFile(filepath.Join("..", sourceFile))
	if err != nil {
		t.Fatal(err)
	}
	contracts, unparsed := extractRustContracts(sourceFile, string(source))
	if len(unparsed) != 0 || len(contracts) != 6 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
	want := map[string]string{
		"create":   "POST /open-apis/spark/v1/apps/{}/sessions",
		"list":     "GET /open-apis/spark/v1/apps/{}/sessions",
		"get":      "GET /open-apis/spark/v1/apps/{}/sessions/{}",
		"chat":     "POST /open-apis/spark/v1/apps/{}/sessions/{}/chat",
		"stop":     "POST /open-apis/spark/v1/apps/{}/sessions/{}/stop",
		"messages": "GET /open-apis/spark/v1/apps/{}/sessions/{}/turns/{}/reply_message",
	}
	for _, contract := range contracts {
		route, ok := want[contract.Function]
		if !ok || contract.Method+" "+contract.Path != route ||
			!equalTokenTypes(contract.TokenTypes, []string{"User"}) || contract.FileUpload {
			t.Fatalf("unexpected session contract: %#v", contract)
		}
		delete(want, contract.Function)
	}
	if len(want) != 0 {
		t.Fatalf("missing session contracts: %#v", want)
	}
}

func TestRequestHelperMetadataSurvivesRelocationAndRename(t *testing.T) {
	source := `
fn build_call<'a>(config: &'a Config, path: impl Into<String>, verb: http::Method, option: &'a RequestOption) -> RestRequest<'a> {
    RestRequest::new(config, verb, path, vec![AccessTokenType::User], option)
}
impl Resource<'_> {
    pub async fn get(&self, option: &RequestOption) {
        build_call(self.config, "/open-apis/example/v1/items", http::Method::GET, option).send().await
    }
}
`
	for _, path := range []string{"src/service/example/v1.rs", "src/service/example/v1/mod.rs", "src/service/example/v1/resources.rs"} {
		contracts, unparsed := extractRustContracts(path, source)
		if len(unparsed) != 0 || len(contracts) != 1 {
			t.Fatalf("%s: contracts=%#v unparsed=%#v", path, contracts, unparsed)
		}
		c := contracts[0]
		if c.SourceFile != path || c.Function != "get" || c.Method != "GET" || c.Path != "/open-apis/example/v1/items" || !equalTokenTypes(c.TokenTypes, []string{"User"}) {
			t.Fatalf("%s: wrong contract %#v", path, c)
		}
	}
}

func TestCurrentOkrContractsSurviveRelocation(t *testing.T) {
	source, err := os.ReadFile("../src/service/okr/v2.rs")
	if err != nil {
		t.Fatal(err)
	}
	for _, path := range []string{"src/service/okr/v2.rs", "src/service/okr/v2/resources.rs"} {
		contracts, unparsed := extractRustContracts(path, string(source))
		if len(contracts) != 32 || len(unparsed) != 0 {
			t.Fatalf("%s: contracts=%d unparsed=%#v", path, len(contracts), unparsed)
		}
	}
}

func TestRequestHelperTokenParameterIsNotHardcoded(t *testing.T) {
	source := `
fn dispatch(config: &Config, method: http::Method, path: &str, tokens: Vec<AccessTokenType>, option: &RequestOption) -> RestRequest<'_> {
    RestRequest::new(config, method, path, tokens, option)
}
async fn create() {
    dispatch(config, http::Method::POST, "/open-apis/example/v1/items", vec![AccessTokenType::Tenant], option).file_upload().send().await;
}
`
	contracts, unparsed := extractRustContracts("src/service/example.rs", source)
	if len(contracts) != 1 || len(unparsed) != 0 {
		t.Fatalf("contracts=%#v unparsed=%#v", contracts, unparsed)
	}
	if !equalTokenTypes(contracts[0].TokenTypes, []string{"Tenant"}) || !contracts[0].FileUpload {
		t.Fatalf("wrong metadata: %#v", contracts[0])
	}
}

func TestRequestHelperKeepsUnresolvedCallVisible(t *testing.T) {
	source := `
fn build_request(config: &Config, method: http::Method, path: &str, option: &RequestOption) -> RestRequest<'_> {
    RestRequest::new(config, method, path, vec![AccessTokenType::Tenant], option)
}
async fn create() {
    build_request(config, verb, path_from_runtime, option).send().await;
}
`
	contracts, unparsed := extractRustContracts("src/service/example.rs", source)
	if len(contracts) != 0 || len(unparsed) != 1 || unparsed[0].Function != "create" {
		t.Fatalf("contracts=%#v unparsed=%#v", contracts, unparsed)
	}
}

func TestRequestHelpersResolveAcrossSplitModules(t *testing.T) {
	for _, importLine := range []string{"use super::build_request;", "use super::{build_request, Resource};", "use super::build_request as dispatch;", "use super::*;"} {
		t.Run(importLine, func(t *testing.T) {
			root := t.TempDir()
			dir := filepath.Join(root, "src", "service", "example", "v1")
			if err := os.MkdirAll(dir, 0755); err != nil {
				t.Fatal(err)
			}
			helper := `pub(super) fn build_request(config: &Config, method: http::Method, path: &str, option: &RequestOption) -> RestRequest<'_> {
                RestRequest::new(config, method, path, vec![AccessTokenType::User], option)
            }`
			name := "build_request"
			if strings.Contains(importLine, " as ") {
				name = "dispatch"
			}
			caller := importLine + `
impl Resource<'_> {
                pub async fn get(&self, option: &RequestOption) {
                    ` + name + `(self.config, http::Method::GET, "/open-apis/example/v1/items", option).send().await
                }
            }`
			for name, source := range map[string]string{"mod.rs": helper, "resources.rs": caller} {
				if err := os.WriteFile(filepath.Join(dir, name), []byte(source), 0644); err != nil {
					t.Fatal(err)
				}
			}
			contracts, unparsed, err := rustServiceContracts(root)
			if err != nil {
				t.Fatal(err)
			}
			if len(contracts) != 1 || len(unparsed) != 0 {
				t.Fatalf("contracts=%#v unparsed=%#v", contracts, unparsed)
			}
			if contracts[0].Function != "get" || !equalTokenTypes(contracts[0].TokenTypes, []string{"User"}) {
				t.Fatalf("wrong contract: %#v", contracts[0])
			}
		})
	}
}

func TestMethodRequestHelpersKeepReceiverScopesSeparate(t *testing.T) {
	source := `
impl First {
    fn request(&self, method: http::Method) { RestRequest::new(config,method,"/first",vec![AccessTokenType::Tenant],option) }
    async fn get(&self) { self.request(http::Method::GET).send().await; }
}
impl Second {
    fn request(&self, method: http::Method) { RestRequest::new(config,method,"/second",vec![AccessTokenType::User],option) }
    async fn get(&self) { self.request(http::Method::GET).send().await; }
}`
	contracts, unparsed := extractRustContracts("src/service/example.rs", source)
	if len(contracts) != 2 || len(unparsed) != 0 {
		t.Fatalf("contracts=%#v unparsed=%#v", contracts, unparsed)
	}
	for i, want := range []string{"Tenant", "User"} {
		if !equalTokenTypes(contracts[i].TokenTypes, []string{want}) {
			t.Fatalf("wrong receiver metadata: %#v", contracts[i])
		}
	}
}

func TestHelperTokenMetadataMustBeLiteral(t *testing.T) {
	source := `
fn build_request(config: &Config, method: http::Method, path: &str, tokens: Vec<AccessTokenType>, option: &RequestOption) -> RestRequest<'_> {
    RestRequest::new(config, method, path, tokens, option)
}
async fn get() { build_request(config, http::Method::GET, "/items", tokens_from_runtime, option).send().await; }
`
	contracts, unparsed := extractRustContracts("src/service/example.rs", source)
	if len(contracts) != 0 || len(unparsed) != 1 || unparsed[0].Reason != "non-literal request helper access token types" {
		t.Fatalf("contracts=%#v unparsed=%#v", contracts, unparsed)
	}
}

func TestHelperRetainsNoAuthAndUploadMetadata(t *testing.T) {
	source := `
fn build_request(config: &Config, path: &str, option: &RequestOption) -> RestRequest<'_> {
    RestRequest::new(config, http::Method::POST, path, vec![AccessTokenType::None], option).file_upload()
}
async fn create() { build_request(config, "/items", option).send().await; }
`
	contracts, unparsed := extractRustContracts("src/service/example.rs", source)
	if len(contracts) != 1 || len(unparsed) != 0 {
		t.Fatalf("contracts=%#v unparsed=%#v", contracts, unparsed)
	}
	if len(contracts[0].TokenTypes) != 0 || !contracts[0].FileUpload || contracts[0].Method != "POST" {
		t.Fatalf("wrong metadata: %#v", contracts[0])
	}
}

func TestBaseFieldExtensionHelperCallsAreVisible(t *testing.T) {
	source, err := os.ReadFile("../src/service/base/v3.rs")
	if err != nil {
		t.Fatal(err)
	}
	contracts, unparsed := extractRustContracts("src/service/base/v3/resources.rs", string(source))
	if len(unparsed) != 0 {
		t.Fatalf("unparsed=%#v", unparsed)
	}
	want := map[string]bool{"GET": false, "PUT": false}
	for _, contract := range contracts {
		if contract.Path == "/open-apis/base/v3/bases/{}/tables/{}/fields/{}/field_extensions" {
			if _, ok := want[contract.Method]; !ok {
				t.Fatalf("unexpected contract: %#v", contract)
			}
			if !equalTokenTypes(contract.TokenTypes, []string{"Tenant", "User"}) {
				t.Fatalf("wrong tokens: %#v", contract)
			}
			want[contract.Method] = true
		}
	}
	for method, seen := range want {
		if !seen {
			t.Errorf("missing %s field-extension contract", method)
		}
	}
}

func TestDirectRequestMetadataDoesNotGuessDynamicExpressions(t *testing.T) {
	for _, source := range []string{
		`async fn get() { RestRequest::new(config, select_method(http::Method::GET), "/items", vec![AccessTokenType::User], option).send().await; }`,
		`async fn get() { RestRequest::new(config, http::Method::GET, "/items", tokens_from_runtime, option).send().await; }`,
	} {
		contracts, unparsed := extractRustContracts("src/service/example.rs", source)
		if len(contracts) != 0 || len(unparsed) != 1 {
			t.Fatalf("contracts=%#v unparsed=%#v", contracts, unparsed)
		}
	}
}

func TestTokenListHelpersComeFromTheirDefinitions(t *testing.T) {
	source := `
fn auth_modes() -> Vec<AccessTokenType> { vec![AccessTokenType::User] }
fn dispatch(config: &Config, method: http::Method, path: &str, option: &RequestOption) -> RestRequest<'_> {
    RestRequest::new(config,method,path,auth_modes(),option)
}
async fn direct() { RestRequest::new(config,http::Method::GET,"/direct",auth_modes(),option).send().await; }
async fn forwarded() { dispatch(config,http::Method::POST,"/forwarded",option).send().await; }
`
	contracts, unparsed := extractRustContracts("src/service/example/moved.rs", source)
	if len(contracts) != 2 || len(unparsed) != 0 {
		t.Fatalf("contracts=%#v unparsed=%#v", contracts, unparsed)
	}
	for _, contract := range contracts {
		if !equalTokenTypes(contract.TokenTypes, []string{"User"}) {
			t.Fatalf("wrong tokens: %#v", contract)
		}
	}
}
