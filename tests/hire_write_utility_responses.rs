mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn hire_write_utility_responses_deserialize_and_preserve_requests() {
    let employee = r#"{"code":0,"msg":"ok","data":{"employee":{"id":"employee-1","application_id":"app-1","onboard_status":2,"department":"dept-1"}}}"#;
    let interviewer = r#"{"code":0,"msg":"ok","data":{"interviewer":{"user_id":"ou_interviewer","verify_status":1}}}"#;
    let created_note = r#"{"code":0,"msg":"ok","data":{"note":{"id":"note-1","talent_id":"talent-1","content":"created","mention_entity_list":[{"id":"ou_mention","name":"Mentioned"}]}}}"#;
    let patched_note = r#"{"code":0,"msg":"ok","data":{"note":{"id":"note-1","application_id":"app-1","content":"patched","notify_mentioned_user":true}}}"#;
    let attachment = r#"{"code":0,"msg":"ok","data":{"id":"attachment-1"}}"#;
    let created_channel = r#"{"code":0,"msg":"ok","data":{"id":"channel-1","name":"Campus","link":"https://jobs.example.test/campus","code":"campus"}}"#;
    let updated_channel = r#"{"code":0,"msg":"ok","data":{"id":"channel-1","name":"Campus Updated","link":"https://jobs.example.test/campus-new","code":"campus-new"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, employee),
        http_response(200, interviewer),
        http_response(200, created_note),
        http_response(200, patched_note),
        http_response(200, attachment),
        http_response(200, created_channel),
        http_response(200, updated_channel),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let employee = Box::pin(hire.employee.patch(
        "employee-1",
        json!({"onboard_status":2}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .employee
    .unwrap();
    let interviewer = Box::pin(hire.interviewer.patch(
        "interviewer-1",
        json!({"verify_status":1}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .interviewer
    .unwrap();
    let created_note = Box::pin(hire.note.create(
        json!({"talent_id":"talent-1","content":"created"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .note
    .unwrap();
    let patched_note = Box::pin(hire.note.patch(
        "note-1",
        json!({"content":"patched"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .note
    .unwrap();
    let attachment = Box::pin(hire.attachment.create(&RequestOption::default()))
        .await
        .unwrap()
        .data
        .unwrap();
    let created_channel = Box::pin(hire.website_channel.create(
        "website-1",
        json!({"channel_name":"Campus"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();
    let updated_channel = Box::pin(hire.website_channel.update(
        "website-1",
        "channel-1",
        json!({"channel_name":"Campus Updated"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();

    assert_eq!(employee.id.as_deref(), Some("employee-1"));
    assert_eq!(employee.onboard_status, Some(2));
    assert_eq!(interviewer.user_id.as_deref(), Some("ou_interviewer"));
    assert_eq!(interviewer.verify_status, Some(1));
    assert_eq!(created_note.content.as_deref(), Some("created"));
    assert_eq!(
        created_note.mention_entity_list.as_ref().unwrap()[0]
            .name
            .as_deref(),
        Some("Mentioned")
    );
    assert_eq!(patched_note.content.as_deref(), Some("patched"));
    assert_eq!(patched_note.notify_mentioned_user, Some(true));
    assert_eq!(attachment.id.as_deref(), Some("attachment-1"));
    assert_eq!(created_channel.code.as_deref(), Some("campus"));
    assert_eq!(updated_channel.name.as_deref(), Some("Campus Updated"));
    assert_eq!(
        updated_channel.link.as_deref(),
        Some("https://jobs.example.test/campus-new")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/hire/v1/employees/employee-1 "));
    assert!(request.contains("PATCH /open-apis/hire/v1/interviewers/interviewer-1 "));
    assert!(request.contains("POST /open-apis/hire/v1/notes "));
    assert!(request.contains("PATCH /open-apis/hire/v1/notes/note-1 "));
    assert!(request.contains("POST /open-apis/hire/v1/attachments "));
    assert!(request.contains("POST /open-apis/hire/v1/websites/website-1/channels "));
    assert!(request.contains("PUT /open-apis/hire/v1/websites/website-1/channels/channel-1 "));
    assert!(request.contains(r#""onboard_status":2"#));
    assert!(request.contains(r#""verify_status":1"#));
    assert!(request.contains(r#""talent_id":"talent-1""#));
    assert!(request.contains(r#""content":"patched""#));
    assert!(request.contains(r#""channel_name":"Campus""#));
    assert!(request.contains(r#""channel_name":"Campus Updated""#));
}
