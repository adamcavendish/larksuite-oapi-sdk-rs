use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_agency_read_query_smoke() {
    let batch_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"supplier-1"}],"has_more":false}}"#;
    let get_body = r#"{"code":0,"msg":"ok","data":{"agency":{"id":"agency-1","name":"North"}}}"#;
    let account_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"account-1"}],"has_more":false}}"#;
    let protect_body = r#"{"code":0,"msg":"ok","data":{"is_onboarded":false,"protection_list":[{"agency_supplier_id":"supplier-1"}]}}"#;
    let query_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"agency-2"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, batch_body),
        http_response(200, get_body),
        http_response(200, account_body),
        http_response(200, protect_body),
        http_response(200, query_body),
    ])
    .await;

    let client = client_for(addr);
    let agency = &client.hire().agency;
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    agency
        .batch_query_by_query(
            &BatchQueryHireAgencyQuery::new()
                .page(page)
                .user_id_type("open_id"),
            json!({"keyword":"North"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    agency
        .get_by_query(
            "agency-1",
            &GetHireAgencyQuery::new().user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    agency
        .get_agency_account_by_query(
            &GetHireAgencyAccountQuery::new()
                .page(page)
                .user_id_type("open_id"),
            json!({"supplier_id":"supplier-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    agency
        .protect_search(json!({"talent_id":"talent-1"}), &RequestOption::default())
        .await
        .unwrap();
    agency
        .query_by_query(
            &QueryHireAgencyQuery::new()
                .name("North")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/agencies/batch_query?"));
    assert!(request.contains("GET /open-apis/hire/v1/agencies/agency-1?"));
    assert!(request.contains("POST /open-apis/hire/v1/agencies/get_agency_account?"));
    assert!(request.contains("POST /open-apis/hire/v1/agencies/protection_period/search"));
    assert!(request.contains("GET /open-apis/hire/v1/agencies/query?"));
    assert_eq!(request.matches("page_token=seed-token").count(), 2);
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("name=North"));
    assert!(request.contains(r#""keyword":"North""#));
    assert!(request.contains(r#""supplier_id":"supplier-1""#));
    assert!(request.contains(r#""talent_id":"talent-1""#));
}
