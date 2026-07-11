mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::hire::v1::GetTalentQuery;

#[tokio::test]
async fn hire_talent_profile_deserializes_typed_nested_data() {
    let body = r#"{"code":0,"msg":"ok","data":{"talent":{"id":"talent-1","interview_registration_list":[{"id":"interview-registration-1","registration_time":1710000000,"download_url":"https://example.com/interview"}],"registration_list":[{"id":"registration-1","registration_time":1710000100,"scenario":5}],"customized_data_list":[{"object_id":"module-1","name":{"en_us":"Profile"},"children":[{"object_id":"field-1","value":{"option_list":[{"key":"option-1","name":{"en_us":"Engineering"}}],"time_range":{"start_time":"1710000000","end_time":"-"},"customized_attachment":[{"file_id":"file-1","name":"resume.pdf","file_size":42}]}}]}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;
    let client = Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap();

    let talent = client
        .hire()
        .talent
        .get_by_query(&GetTalentQuery::new("talent-1"), &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .talent
        .unwrap();

    assert_eq!(
        talent.interview_registration_list.as_ref().unwrap()[0]
            .download_url
            .as_deref(),
        Some("https://example.com/interview")
    );
    assert_eq!(
        talent.registration_list.as_ref().unwrap()[0].scenario,
        Some(5)
    );
    let value = &talent.customized_data_list.as_ref().unwrap()[0]
        .children
        .as_ref()
        .unwrap()[0]
        .value;
    assert_eq!(
        value.as_ref().unwrap().option_list.as_ref().unwrap()[0]
            .key
            .as_deref(),
        Some("option-1")
    );
    assert_eq!(
        value
            .as_ref()
            .unwrap()
            .customized_attachment
            .as_ref()
            .unwrap()[0]
            .file_size,
        Some(42)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/talents/talent-1 "));
}
