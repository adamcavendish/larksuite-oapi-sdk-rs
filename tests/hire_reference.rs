mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    ListSubjectQuery, ListTerminationReasonQuery, ListTodoQuery,
};

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
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

fn user_option() -> RequestOption {
    RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..Default::default()
    }
}

#[tokio::test]
async fn hire_reference_subject_list_deserializes_typed_items() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"subject-1","name":{"zh_cn":"校招","en_us":"Campus"},"create_time":"1710000000","active_status":1,"application_limit":20,"creator":{"id":"ou_user_1","name":{"zh_cn":"张三"}}}],"page_token":"next-1","has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let subject_ids = ["subject-1", "subject-2"];
    let query = ListSubjectQuery::new()
        .page(PageQuery::new().page_size(20).page_token("seed-token"))
        .user_id_type("open_id")
        .subject_ids(Some(subject_ids.as_slice()));
    let resp = client
        .hire()
        .subject
        .list_by_query(&query, &tenant_option())
        .await
        .unwrap();

    let data = resp.data.unwrap();
    let item = &data.items[0];
    assert_eq!(item.id.as_deref(), Some("subject-1"));
    assert_eq!(
        item.name.as_ref().and_then(|name| name.en_us.as_deref()),
        Some("Campus")
    );
    assert_eq!(item.application_limit, Some(20));
    assert_eq!(data.page_token.as_deref(), Some("next-1"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/subjects?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("subject_ids=subject-1"));
    assert!(request.contains("subject_ids=subject-2"));
}

#[tokio::test]
async fn hire_role_permissions_deserialize_typed_scopes() {
    let role = r#"{"code":0,"msg":"ok","data":{"role":{"id":"role-1","has_business_management_scope":true,"socail_permission_collection":{"feature_permissions":[{"id":"feature-1","name":"Feature"}],"management_permissions":[{"id":"management-1"}],"data_permissions":[{"id":"data-1","name":{"en_us":"Data"},"select_status":1}],"business_management_scopes":[{"entity":{"code":"application","name":{"en_us":"Application"}},"permission_groups":[{"permission_ids":["permission-1"],"scope_rule":{"rule_type":1}}]}]},"campus_permission_collection":{"business_management_scopes":[{"entity":{"code":"campus"},"permission_groups":[{"permission_ids":["permission-2"]}]}]}}}}"#;
    let user_roles = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou_user_1","role_id":"role-1","business_management_scopes":[{"entity":{"code":"application","name":{"en_us":"Application"}},"scope_rule":{"rule_type":2}}]}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, role),
        http_response(200, user_roles),
    ])
    .await;

    let client = client_for(addr);
    let role = client
        .hire()
        .role
        .get("role-1", &tenant_option())
        .await
        .unwrap()
        .data
        .unwrap()
        .role
        .unwrap();
    let user_role = client
        .hire()
        .user_role
        .list_by_query(
            &larksuite_oapi_sdk_rs::service::hire::v1::ListUserRoleQuery::new().page_size(Some(20)),
            &tenant_option(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);

    assert_eq!(
        role.socail_permission_collection
            .as_ref()
            .unwrap()
            .data_permissions
            .as_ref()
            .unwrap()[0]
            .select_status,
        Some(1)
    );
    assert_eq!(
        role.campus_permission_collection
            .as_ref()
            .unwrap()
            .business_management_scopes
            .as_ref()
            .unwrap()[0]
            .permission_groups
            .as_ref()
            .unwrap()[0]
            .permission_ids
            .as_ref()
            .unwrap()[0],
        "permission-2"
    );
    assert_eq!(
        user_role.business_management_scopes.as_ref().unwrap()[0]
            .scope_rule
            .as_ref()
            .unwrap()
            .rule_type,
        Some(2)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/roles/role-1 "));
    assert!(request.contains("GET /open-apis/hire/v1/user_roles?"));
}

#[tokio::test]
async fn hire_reference_todo_list_deserializes_typed_items() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"evaluation":{"talent_id":"talent-1","job_id":"job-1","application_id":"app-1","id":"eval-1"},"offer":{"id":"offer-1"}}],"page_token":"next-1","has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListTodoQuery::new()
        .page_size("20")
        .page_token("seed-token")
        .user_id("ou_user_1")
        .user_id_type("open_id")
        .type_("evaluation");
    let resp = client
        .hire()
        .todo
        .list_by_query(&query, &user_option())
        .await
        .unwrap();

    let data = resp.data.unwrap();
    let item = &data.items[0];
    assert_eq!(
        item.evaluation.as_ref().and_then(|todo| todo.id.as_deref()),
        Some("eval-1")
    );
    assert_eq!(
        item.offer.as_ref().and_then(|todo| todo.id.as_deref()),
        Some("offer-1")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/todos?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains("user_id=ou_user_1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("type=evaluation"));
}

#[tokio::test]
async fn hire_reference_iterator_pages_and_limits() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"folder_id":"folder-1"},{"folder_id":"folder-2"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"folder_id":"folder-3"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let mut iter = hire
        .talent_folder
        .list_by_iterator(Some(2), Some("open_id"))
        .limit(2);

    let first = iter.next(&tenant_option()).await.unwrap().unwrap();
    let second = iter.next(&tenant_option()).await.unwrap().unwrap();
    let third = iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first.folder_id.as_deref(), Some("folder-1"));
    assert_eq!(second.folder_id.as_deref(), Some("folder-2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("next-1"));

    let reqs = requests.lock().unwrap();
    assert_eq!(reqs.len(), 1);
    assert!(reqs[0].contains("GET /open-apis/hire/v1/talent_folders?"));
    assert!(reqs[0].contains("page_size=2"));
    assert!(reqs[0].contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_reference_iterator_sends_resume_token_and_filters() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou_user_1","role_id":"role-1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou_user_2","role_id":"role-1"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let mut iter = hire
        .user_role
        .list_by_iterator(
            Some(1),
            Some("ou_user_1"),
            Some("role-1"),
            Some("1710000000"),
            Some("1710009999"),
            Some("open_id"),
        )
        .page_token("seed-token");

    let _ = iter.next(&tenant_option()).await.unwrap();
    let _ = iter.next(&tenant_option()).await.unwrap();

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("GET /open-apis/hire/v1/user_roles?"));
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[0].contains("user_id=ou_user_1"));
    assert!(reqs[0].contains("role_id=role-1"));
    assert!(reqs[0].contains("update_start_time=1710000000"));
    assert!(reqs[0].contains("update_end_time=1710009999"));
    assert!(reqs[0].contains("user_id_type=open_id"));
    assert!(reqs[1].contains("page_token=next-1"));
}

#[tokio::test]
async fn hire_reference_iterator_limit_zero_is_unlimited() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"reason-1"}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"reason-2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let mut iter = hire
        .termination_reason
        .list_iterator_by_query(&ListTerminationReasonQuery::new().page_size(Some(1)))
        .limit(0);

    let first = iter.next(&tenant_option()).await.unwrap().unwrap();
    let second = iter.next(&tenant_option()).await.unwrap().unwrap();
    let third = iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("reason-1"));
    assert_eq!(second.id.as_deref(), Some("reason-2"));
    assert!(third.is_none());
    assert_eq!(requests.lock().unwrap().len(), 2);
}
