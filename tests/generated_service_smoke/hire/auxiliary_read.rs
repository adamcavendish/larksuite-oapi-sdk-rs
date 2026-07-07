use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_auxiliary_read_query_smoke() {
    let diversity = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"di-1"}]}}"#;
    let attachment = r#"{"code":0,"msg":"ok","data":{"attachment":{"id":"attachment-1"}}}"#;
    let minutes = r#"{"code":0,"msg":"ok","data":{"minutes":{"sentences":[]},"has_more":false}}"#;
    let delivery = r#"{"code":0,"msg":"ok","data":{"status":3,"delivery":{"id":"delivery-1"},"status_msg":"ok"}}"#;
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

    hire.diversity_inclusion
        .search(
            json!({"application_ids":["app-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.interview_record_attachment
        .get_by_query(
            &GetHireInterviewRecordAttachmentQuery::new()
                .application_id("app-1")
                .interview_record_id("record-1")
                .language(1),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.minutes
        .get_by_query(
            &GetHireMinutesQuery::new()
                .interview_id("interview-1")
                .page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.website_delivery_task
        .get("website-1", "task-1", &RequestOption::default())
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/applications/diversity_inclusions/search"));
    assert!(request.contains("GET /open-apis/hire/v1/interview_records/attachments?"));
    assert!(request.contains("GET /open-apis/hire/v1/minutes?"));
    assert!(request.contains("GET /open-apis/hire/v1/websites/website-1/delivery_tasks/task-1"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("interview_record_id=record-1"));
    assert!(request.contains("language=1"));
    assert!(request.contains("interview_id=interview-1"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains(r#""application_ids":["app-1"]"#));
}
