use super::prelude::*;

// ── Hire ──

#[tokio::test]
async fn hire_v2_by_query_smoke() {
    let record_body = r#"{"code":0,"msg":"ok","data":{"interview_record":{"id":"record-1","interviewer":{"id":"ou-1","name":{"en_us":"Interviewer"}},"record_score":{"score":8.5,"total_score":10.0},"module_assessments":[{"module_name":{"en_us":"Technical"},"dimension_assessments":[{"dimension_name":{"en_us":"System design"},"dimension_score":5}]}]}}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"record-1","commit_status":1}],"has_more":false,"page_token":"next-page"}}"#;
    let talent_body = r#"{"code":0,"msg":"ok","data":{"talent":{"talent_id":"talent-v2-1","basic_info":{"name":"Taylor","email":"taylor@example.com"},"customized_data_list":[{"module_id":"module-1","name":{"en_us":"Profile"},"children":[{"object_id":"field-1","name":{"en_us":"Level"},"value":{"option":{"key":"senior","name":{"en_us":"Senior"}}}}]}],"tag_list":[{"id":"tag-1","name":{"en_us":"Priority"}}],"talent_pool_ref_list_v2":[{"id":"pool-1","name":{"en_us":"Engineering"}}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, record_body),
        http_response(200, list_body),
        http_response(200, talent_body),
    ])
    .await;

    let client = client_for(addr);

    let record = client
        .hire_v2()
        .interview_record
        .get_by_query(
            &GetHireV2InterviewRecordQuery::new("record-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .interview_record
        .unwrap();
    let records = client
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
        .unwrap()
        .data
        .unwrap();
    let talent = client
        .hire_v2()
        .talent
        .get_by_query(
            &GetHireV2TalentQuery::new("talent-v2-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .talent
        .unwrap();

    assert_eq!(record.id.as_deref(), Some("record-1"));
    assert_eq!(
        record
            .interviewer
            .as_ref()
            .unwrap()
            .name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Interviewer")
    );
    assert_eq!(
        record.module_assessments.as_ref().unwrap()[0]
            .dimension_assessments
            .as_ref()
            .unwrap()[0]
            .dimension_score,
        Some(5)
    );
    assert_eq!(records.items[0].commit_status, Some(1));
    assert_eq!(records.page_token.as_deref(), Some("next-page"));
    assert_eq!(talent.talent_id.as_deref(), Some("talent-v2-1"));
    assert_eq!(
        talent.basic_info.as_ref().unwrap().email.as_deref(),
        Some("taylor@example.com")
    );
    assert_eq!(
        talent.customized_data_list.as_ref().unwrap()[0]
            .children
            .as_ref()
            .unwrap()[0]
            .value
            .as_ref()
            .unwrap()
            .option
            .as_ref()
            .unwrap()
            .name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Senior")
    );
    assert_eq!(
        talent.talent_pool_ref_list_v2.as_ref().unwrap()[0]
            .name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Engineering")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v2/interview_records/record-1?"));
    assert!(request.contains("GET /open-apis/hire/v2/interview_records?"));
    assert!(request.contains("GET /open-apis/hire/v2/talents/talent-v2-1?"));
    assert!(request.contains("interview_id=interview-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}
