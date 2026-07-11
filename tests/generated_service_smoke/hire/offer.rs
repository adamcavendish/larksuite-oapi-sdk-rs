use super::prelude::*;

// ── Hire ──

#[tokio::test]
async fn hire_offer_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"offer":{"id":"offer-1","salary_plan":{"currency":"CNY","customize_info_list":[{"object_id":"salary-field-1","customize_value":"value-1"}]}}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireOfferQuery::new("offer-1")
        .user_id_type("open_id")
        .department_id_type("open_department_id")
        .job_level_id_type("job_level_id")
        .job_family_id_type("job_family_id")
        .employee_type_id_type("employee_type_id");
    let resp = client
        .hire()
        .offer
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let offer = resp.data.unwrap().offer.unwrap();
    assert_eq!(offer.id.as_deref(), Some("offer-1"));
    assert_eq!(
        offer
            .salary_plan
            .as_ref()
            .unwrap()
            .customize_info_list
            .as_ref()
            .unwrap()[0]
            .customize_value
            .as_deref(),
        Some("value-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/offers/offer-1?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
}

#[tokio::test]
async fn hire_offer_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"offer-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireOfferQuery::new()
        .application_id("app-1")
        .talent_id("talent-1")
        .user_id_type("open_id")
        .employee_type_id_type("employee_type_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .offer
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("offer-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/offers?"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_offer_intern_status_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"offer_id":"offer-1","operation":"offboard","offboarding_info":{"actual_offboarding_date":"2022-03-02"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .offer
        .intern_offer_status(
            "offer-1",
            serde_json::json!({"operation": "offboard"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(resp.data.unwrap().operation.as_deref(), Some("offboard"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/offers/offer-1/intern_offer_status "));
}
