mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    ListEvaluationTaskQuery, ListExamMarkingTaskQuery, ListInterviewTaskQuery,
};

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

fn tenant_option() -> RequestOption {
    RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    }
}

#[tokio::test]
async fn hire_task_lists_deserialize_typed_items() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"evaluation-task-1","job_id":"job-1","talent_id":"talent-1","application_id":"app-1","activity_status":1}],"page_token":"next-1","has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .evaluation_task
        .list_by_query(
            &ListEvaluationTaskQuery::new()
                .page(PageQuery::new().page_size(20).page_token("seed-token"))
                .user_id("ou_user_1")
                .activity_status(1)
                .user_id_type("open_id"),
            &tenant_option(),
        )
        .await
        .unwrap();

    let data = resp.data.unwrap();
    let item = &data.items[0];
    assert_eq!(item.id.as_deref(), Some("evaluation-task-1"));
    assert_eq!(item.job_id.as_deref(), Some("job-1"));
    assert_eq!(item.talent_id.as_deref(), Some("talent-1"));
    assert_eq!(item.application_id.as_deref(), Some("app-1"));
    assert_eq!(item.activity_status, Some(1));
    assert_eq!(data.page_token.as_deref(), Some("next-1"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/evaluation_tasks?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains("user_id=ou_user_1"));
    assert!(request.contains("activity_status=1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_task_iterator_pages_and_limits() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"exam-task-1"},{"id":"exam-task-2"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"exam-task-3"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let mut iter = hire
        .exam_marking_task
        .list_by_iterator(Some(2), Some("ou_user_1"), Some(1), Some("open_id"))
        .limit(2);

    let first = iter.next(&tenant_option()).await.unwrap().unwrap();
    let second = iter.next(&tenant_option()).await.unwrap().unwrap();
    let third = iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("exam-task-1"));
    assert_eq!(second.id.as_deref(), Some("exam-task-2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("next-1"));

    let reqs = requests.lock().unwrap();
    assert_eq!(reqs.len(), 1);
    assert!(reqs[0].contains("GET /open-apis/hire/v1/exam_marking_tasks?"));
    assert!(reqs[0].contains("page_size=2"));
    assert!(reqs[0].contains("user_id=ou_user_1"));
    assert!(reqs[0].contains("activity_status=1"));
    assert!(reqs[0].contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_task_iterator_sends_resume_token_and_filters() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"interview-task-1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"interview-task-2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let query = ListInterviewTaskQuery::new()
        .page_size(Some(1))
        .page_token(Some("seed-token"))
        .user_id(Some("ou_user_1"))
        .activity_status(Some(1))
        .user_id_type(Some("open_id"));
    let hire = client.hire();
    let mut iter = hire.interview_task.list_iterator_by_query(&query);

    let _ = iter.next(&tenant_option()).await.unwrap();
    let _ = iter.next(&tenant_option()).await.unwrap();

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("GET /open-apis/hire/v1/interview_tasks?"));
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[0].contains("user_id=ou_user_1"));
    assert!(reqs[0].contains("activity_status=1"));
    assert!(reqs[0].contains("user_id_type=open_id"));
    assert!(reqs[1].contains("page_token=next-1"));
}

#[tokio::test]
async fn hire_task_iterator_limit_zero_is_unlimited() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"exam-task-1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"exam-task-2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let mut iter = hire
        .exam_marking_task
        .list_iterator_by_query(&ListExamMarkingTaskQuery::new().page_size(Some(1)))
        .limit(0);

    let first = iter.next(&tenant_option()).await.unwrap().unwrap();
    let second = iter.next(&tenant_option()).await.unwrap().unwrap();
    let third = iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("exam-task-1"));
    assert_eq!(second.id.as_deref(), Some("exam-task-2"));
    assert!(third.is_none());
    assert_eq!(requests.lock().unwrap().len(), 2);
}
