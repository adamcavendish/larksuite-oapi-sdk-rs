mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    GetInterviewRecordAttachmentQuery, GetMinutesQuery,
};
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn hire_auxiliary_read_responses_deserialize_and_send_filters() {
    let diversity = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"di-1","application_id":"app-1","talent_id":"talent-1","source_type":1,"di_data":[{"value":"1","object_attribute":{"title":{"zh_cn":"字段"},"data_type":1,"tags":[1,2],"is_di_data":true}}]}]}}"#;
    let attachment = r#"{"code":0,"msg":"ok","data":{"attachment":{"id":"attachment-1","name":"resume.pdf","mime":"application/pdf","create_time":"1710000000000"}}}"#;
    let minutes = r#"{"code":0,"msg":"ok","data":{"minutes":{"sentences":[{"content":"hello","speak_time":"1710000000001","user_type":1,"speaker_name":{"en_us":"Interviewer"}}]},"page_token":"minutes-next","has_more":false}}"#;
    let delivery = r#"{"code":0,"msg":"ok","data":{"status":3,"delivery":{"application_id":"app-2","id":"delivery-1","job_id":"job-1","job_post_id":"post-1","portal_resume_id":"resume-1","user_id":"user-1","talent_id":"talent-2"},"status_msg":"duplicate","extra_info":"app-old"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, diversity),
        http_response(200, attachment),
        http_response(200, minutes),
        http_response(200, delivery),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    let diversity_item = hire
        .diversity_inclusion
        .search(
            json!({"application_ids":["app-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let attachment = hire
        .interview_record_attachment
        .get_by_query(
            &GetInterviewRecordAttachmentQuery::new()
                .application_id("app-1")
                .interview_record_id("record-1")
                .language(1),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .attachment
        .unwrap();
    let minutes = hire
        .minutes
        .get_by_query(
            &GetMinutesQuery::new()
                .interview_id("interview-1")
                .page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap();
    let delivery = hire
        .website_delivery_task
        .get("website-1", "task-1", &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(diversity_item.application_id.as_deref(), Some("app-1"));
    assert_eq!(
        diversity_item.di_data.as_ref().unwrap()[0]
            .object_attribute
            .as_ref()
            .and_then(|attr| attr.title.as_ref())
            .and_then(|title| title.zh_cn.as_deref()),
        Some("字段")
    );
    assert_eq!(
        diversity_item.di_data.as_ref().unwrap()[0]
            .object_attribute
            .as_ref()
            .and_then(|attr| attr.tags.as_ref())
            .unwrap(),
        &[1, 2]
    );
    assert_eq!(attachment.name.as_deref(), Some("resume.pdf"));
    assert_eq!(
        minutes
            .minutes
            .as_ref()
            .and_then(|minutes| minutes.sentences.as_ref())
            .unwrap()[0]
            .speaker_name
            .as_ref()
            .and_then(|name| name.en_us.as_deref()),
        Some("Interviewer")
    );
    assert_eq!(minutes.page_token.as_deref(), Some("minutes-next"));
    assert_eq!(
        delivery
            .delivery
            .as_ref()
            .and_then(|delivery| delivery.job_post_id.as_deref()),
        Some("post-1")
    );
    assert_eq!(delivery.status, Some(3));
    assert_eq!(delivery.extra_info.as_deref(), Some("app-old"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/applications/diversity_inclusions/search"));
    assert!(request.contains("GET /open-apis/hire/v1/interview_records/attachments?"));
    assert!(request.contains("GET /open-apis/hire/v1/minutes?"));
    assert!(request.contains("GET /open-apis/hire/v1/websites/website-1/delivery_tasks/task-1"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("interview_record_id=record-1"));
    assert!(request.contains("language=1"));
    assert!(request.contains("interview_id=interview-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains(r#""application_ids":["app-1"]"#));
}
