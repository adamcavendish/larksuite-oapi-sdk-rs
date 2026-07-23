package main

import (
	"encoding/json"
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
	generated, err := generate("go_service_catalog.json", "..")
	if err != nil {
		t.Fatal(err)
	}

	var report parityReport
	if err := json.Unmarshal(generated, &report); err != nil {
		t.Fatal(err)
	}
	if report.Summary.MetadataMismatches != 0 || report.Summary.MissingContracts != 0 || report.Summary.UnparsedRustRequests != 0 {
		t.Fatalf("parity summary = %#v", report.Summary)
	}
}
