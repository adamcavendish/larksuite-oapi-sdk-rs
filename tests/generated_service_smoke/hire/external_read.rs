use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_external_read_query_smoke() {
    let applications =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"external-app-1"}],"has_more":false}}"#;
    let background_checks =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"bg-1"}],"has_more":false}}"#;
    let interviews =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"interview-1"}],"has_more":false}}"#;
    let offers = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"offer-1"}],"has_more":false}}"#;
    let talent_interviews =
        r#"{"code":0,"msg":"ok","data":{"items":[{"application_id":"app-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, applications),
        http_response(200, background_checks),
        http_response(200, interviews),
        http_response(200, offers),
        http_response(200, talent_interviews),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    hire.external_application
        .list_by_query(
            &ListHireExternalApplicationQuery::new()
                .page(page)
                .talent_id("talent-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.external_background_check
        .batch_query_by_query(
            &BatchQueryHireExternalBackgroundCheckQuery::new()
                .page(page)
                .external_application_id("external-app-1"),
            json!({"external_background_check_id_list":["bg-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.external_interview
        .batch_query_by_query(
            &BatchQueryHireExternalInterviewQuery::new()
                .page(page)
                .external_application_id("external-app-1"),
            json!({"external_interview_id_list":["interview-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.external_offer
        .batch_query_by_query(
            &BatchQueryHireExternalOfferQuery::new()
                .page(page)
                .external_application_id("external-app-1"),
            json!({"external_offer_id_list":["offer-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.interview
        .get_by_talent_query(
            &GetHireByTalentInterviewQuery::new()
                .talent_id("talent-1")
                .user_id_type("open_id")
                .job_level_id_type("job_level_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/external_applications?"));
    assert!(request.contains("POST /open-apis/hire/v1/external_background_checks/batch_query?"));
    assert!(request.contains("POST /open-apis/hire/v1/external_interviews/batch_query?"));
    assert!(request.contains("POST /open-apis/hire/v1/external_offers/batch_query?"));
    assert!(request.contains("GET /open-apis/hire/v1/interviews/get_by_talent?"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("external_application_id=external-app-1"));
    assert!(request.contains(r#""external_offer_id_list":["offer-1"]"#));
}
