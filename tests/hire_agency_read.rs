mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    BatchQueryAgencyQuery, GetAgencyAccountQuery, GetAgencyQuery, QueryAgencyQuery,
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
async fn hire_agency_read_responses_deserialize_and_send_filters() {
    let batch_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"supplier-1","name":"North","label_list":[{"id":"label-1","name":{"zh_cn":"重点"}}],"admin_list":[{"user_id":"ou_1","name":{"en_us":"Admin"},"email":"admin@example.com"}],"agency_protect_time":{"day":180,"use_default":true},"cooperation_status":1,"talent_protect_time":{"day":365,"forever":false}}],"has_more":false,"page_token":"batch-next"}}"#;
    let get_body = r#"{"code":0,"msg":"ok","data":{"agency":{"id":"agency-1","name":"North","contactor_id":"ou_contact","contactor_name":{"en_us":"Contact"}}}}"#;
    let account_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"account-1","status":1,"role":2,"user_info":{"user_id":"ou_account","name":{"zh_cn":"顾问"},"email":"agent@example.com","mobile":"138"}}],"has_more":false,"page_token":"account-next"}}"#;
    let protect_body = r#"{"code":0,"msg":"ok","data":{"is_onboarded":false,"onboarded_in_protection":true,"onboarded_protection":{"protection_type":1,"application_id":"app-1"},"protection_list":[{"agency_supplier_id":"supplier-1","agency_supplier_user_name":{"en_us":"Consultant"}}]}}"#;
    let query_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"agency-2","name":"South"}]}}"#;
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

    let supplier = agency
        .batch_query_by_query(
            &BatchQueryAgencyQuery::new()
                .page(page)
                .user_id_type("open_id"),
            json!({"keyword":"North"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let agency_detail = agency
        .get_by_query(
            "agency-1",
            &GetAgencyQuery::new().user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .agency
        .unwrap();
    let account = agency
        .get_agency_account_by_query(
            &GetAgencyAccountQuery::new()
                .page(page)
                .user_id_type("open_id"),
            json!({"supplier_id":"supplier-1","status":1}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let protection = agency
        .protect_search(json!({"talent_id":"talent-1"}), &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap();
    let queried_agency = agency
        .query_by_query(
            &QueryAgencyQuery::new()
                .name("North")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);

    assert_eq!(supplier.name.as_deref(), Some("North"));
    assert_eq!(
        supplier.label_list.as_ref().unwrap()[0]
            .name
            .as_ref()
            .and_then(|name| name.zh_cn.as_deref()),
        Some("重点")
    );
    assert_eq!(
        supplier.admin_list.as_ref().unwrap()[0]
            .name
            .as_ref()
            .and_then(|name| name.en_us.as_deref()),
        Some("Admin")
    );
    assert_eq!(
        supplier
            .agency_protect_time
            .as_ref()
            .and_then(|time| time.day),
        Some(180)
    );
    assert_eq!(
        supplier
            .talent_protect_time
            .as_ref()
            .and_then(|time| time.day),
        Some(365)
    );
    assert_eq!(
        agency_detail.contactor_name.and_then(|name| name.en_us),
        Some("Contact".to_string())
    );
    assert_eq!(
        account
            .user_info
            .as_ref()
            .and_then(|user| user.name.as_ref())
            .and_then(|name| name.zh_cn.as_deref()),
        Some("顾问")
    );
    assert_eq!(protection.is_onboarded, Some(false));
    assert_eq!(protection.onboarded_in_protection, Some(true));
    assert_eq!(
        protection
            .onboarded_protection
            .as_ref()
            .and_then(|item| item.application_id.as_deref()),
        Some("app-1")
    );
    assert_eq!(
        protection.protection_list[0]
            .agency_supplier_user_name
            .as_ref()
            .and_then(|name| name.en_us.as_deref()),
        Some("Consultant")
    );
    assert_eq!(queried_agency.name.as_deref(), Some("South"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/agencies/batch_query?"));
    assert!(request.contains("GET /open-apis/hire/v1/agencies/agency-1?"));
    assert!(request.contains("POST /open-apis/hire/v1/agencies/get_agency_account?"));
    assert!(request.contains("POST /open-apis/hire/v1/agencies/protection_period/search"));
    assert!(request.contains("GET /open-apis/hire/v1/agencies/query?"));
    assert_eq!(request.matches("page_token=seed-token").count(), 2);
    assert_eq!(request.matches("page_size=20").count(), 2);
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("name=North"));
    assert!(request.contains(r#""keyword":"North""#));
    assert!(request.contains(r#""supplier_id":"supplier-1""#));
    assert!(request.contains(r#""status":1"#));
    assert!(request.contains(r#""talent_id":"talent-1""#));
}
