use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_write_utility_query_smoke() {
    let employee = r#"{"code":0,"msg":"ok","data":{"employee":{"id":"employee-1"}}}"#;
    let interviewer =
        r#"{"code":0,"msg":"ok","data":{"interviewer":{"user_id":"ou_interviewer"}}}"#;
    let created_note = r#"{"code":0,"msg":"ok","data":{"note":{"id":"note-1"}}}"#;
    let patched_note = r#"{"code":0,"msg":"ok","data":{"note":{"id":"note-1"}}}"#;
    let attachment = r#"{"code":0,"msg":"ok","data":{"id":"attachment-1"}}"#;
    let created_channel = r#"{"code":0,"msg":"ok","data":{"id":"channel-1"}}"#;
    let updated_channel = r#"{"code":0,"msg":"ok","data":{"id":"channel-1"}}"#;
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

    hire.employee
        .patch(
            "employee-1",
            json!({"onboard_status":2}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.interviewer
        .patch(
            "interviewer-1",
            json!({"verify_status":1}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.note
        .create(
            json!({"talent_id":"talent-1","content":"created"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.note
        .patch(
            "note-1",
            json!({"content":"patched"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.attachment
        .create(
            json!({"file_name":"resume.pdf","file_content":"base64"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.website_channel
        .create(
            "website-1",
            json!({"channel_name":"Campus"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.website_channel
        .update(
            "website-1",
            "channel-1",
            json!({"channel_name":"Campus Updated"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

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
    assert!(request.contains(r#""file_name":"resume.pdf""#));
    assert!(request.contains(r#""channel_name":"Campus""#));
    assert!(request.contains(r#""channel_name":"Campus Updated""#));
}
