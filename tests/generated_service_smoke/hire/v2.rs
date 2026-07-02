use super::prelude::*;

// ── Hire ──

#[tokio::test]
async fn hire_v2_by_query_smoke() {
    let record_body = r#"{"code":0,"msg":"ok","data":{"interview_record":{"id":"record-1"}}}"#;
    let list_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"record-1"}],"has_more":false}}"#;
    let talent_body = r#"{"code":0,"msg":"ok","data":{"talent":{"id":"talent-v2-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, record_body),
        http_response(200, list_body),
        http_response(200, talent_body),
    ])
    .await;

    let client = client_for(addr);

    client
        .hire_v2()
        .interview_record
        .get_by_query(
            &GetHireV2InterviewRecordQuery::new("record-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire_v2()
        .interview_record
        .list_by_query(
            &ListHireV2InterviewRecordQuery::new()
                .interview_id("interview-1")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire_v2()
        .talent
        .get_by_query(
            &GetHireV2TalentQuery::new("talent-v2-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v2/interview_records/record-1?"));
    assert!(request.contains("GET /open-apis/hire/v2/interview_records?"));
    assert!(request.contains("GET /open-apis/hire/v2/talents/talent-v2-1?"));
    assert!(request.contains("interview_id=interview-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}
