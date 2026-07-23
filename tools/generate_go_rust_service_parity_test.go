package main

import "testing"

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

func TestExtractRustContractsReportsParameterizedMacroPath(t *testing.T) {
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
	if len(contracts) != 0 || len(unparsed) != 1 {
		t.Fatalf("contracts = %#v, unparsed = %#v", contracts, unparsed)
	}
	if unparsed[0].Reason != "non-literal request path" {
		t.Fatalf("unparsed reason = %q", unparsed[0].Reason)
	}
}

func TestMetadataMatchingNormalizesTokenOrder(t *testing.T) {
	contract := goEndpoint{TokenTypes: []string{"User", "Tenant"}, FileUpload: true}
	candidates := []rustEndpoint{{TokenTypes: []string{"Tenant", "User"}, FileUpload: true}}

	if !anyMetadataMatches(contract, candidates) {
		t.Fatal("expected metadata to match after token normalization")
	}
}
