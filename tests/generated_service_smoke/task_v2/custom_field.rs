use super::prelude::*;

// Task v2 custom field smoke tests

#[tokio::test]
async fn task_v2_custom_field_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"custom_field":{"guid":"field-1","name":"Priority","number_setting":{"format":"number","decimal_count":2},"member_setting":{"multi":true},"single_select_setting":{"options":[{"guid":"option-1","color_index":3}]}}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetCustomFieldV2Query::new("field-1").user_id_type("open_id");
    let resp = client
        .task_v2()
        .custom_field
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let custom_field = resp.data.unwrap().custom_field.unwrap();
    assert_eq!(custom_field.guid.as_deref(), Some("field-1"));
    assert_eq!(custom_field.name.as_deref(), Some("Priority"));
    assert_eq!(custom_field.number_setting.unwrap().decimal_count, Some(2));
    assert_eq!(custom_field.member_setting.unwrap().multi, Some(true));
    assert_eq!(
        custom_field.single_select_setting.unwrap().options.unwrap()[0]
            .guid
            .as_deref(),
        Some("option-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/custom_fields/field-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_custom_field_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"field-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .task_v2()
        .custom_field
        .list(
            Some("tasklist"),
            Some("tasklist-guid-1"),
            Some(20),
            Some("next-page"),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/custom_fields?"));
    assert!(request.contains("resource_type=tasklist"));
    assert!(request.contains("resource_id=tasklist-guid-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn task_v2_custom_field_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"guid":"field-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCustomFieldV2Query::new()
        .resource_type("tasklist")
        .resource_id("tasklist-guid-1")
        .update_msec("1700000000000")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .task_v2()
        .custom_field
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].guid.as_deref(), Some("field-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/task/v2/custom_fields?"));
    assert!(request.contains("resource_type=tasklist"));
    assert!(request.contains("resource_id=tasklist-guid-1"));
    assert!(request.contains("update_msec=1700000000000"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn task_v2_custom_field_write_by_query_smoke() {
    let field_body =
        r#"{"code":0,"msg":"ok","data":{"custom_field":{"guid":"field-1","name":"Priority"}}}"#;
    let option_body = r#"{"code":0,"msg":"ok","data":{"option":{"guid":"option-1"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, field_body),
        http_response(200, field_body),
        http_response(200, field_body),
        http_response(200, empty_body),
        http_response(200, option_body),
        http_response(200, option_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"name":"Priority"});
    let patch_body = serde_json::json!({"name":"Priority updated"});
    let bind_body = serde_json::json!({"resource_type":"tasklist","resource_id":"tasklist-1"});
    let option_body = serde_json::json!({"name":"High"});
    let option_patch_body = serde_json::json!({"name":"Urgent"});

    client
        .task_v2()
        .custom_field
        .create_by_query(
            &CreateCustomFieldV2Query::new(&create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .patch_by_query(
            &PatchCustomFieldV2Query::new("field-1", &patch_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    client
        .task_v2()
        .custom_field
        .add_by_query(
            &AddCustomFieldV2Query::new("field-1", &bind_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .task_v2()
        .custom_field
        .remove_by_query(
            &RemoveCustomFieldV2Query::new("field-1", &bind_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let create_option_resp = client
        .task_v2()
        .custom_field
        .create_option_by_query(
            &CreateCustomFieldOptionV2Query::new("field-1", &option_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let patch_option_resp = client
        .task_v2()
        .custom_field
        .patch_option_by_query(
            &PatchCustomFieldOptionV2Query::new("field-1", "option-1", &option_patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        create_option_resp
            .data
            .unwrap()
            .option
            .unwrap()
            .guid
            .as_deref(),
        Some("option-1")
    );
    assert_eq!(
        patch_option_resp
            .data
            .unwrap()
            .option
            .unwrap()
            .guid
            .as_deref(),
        Some("option-1")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/task/v2/custom_fields?"));
    assert!(request.contains("PATCH /open-apis/task/v2/custom_fields/field-1?"));
    assert!(request.contains("POST /open-apis/task/v2/custom_fields/field-1/add "));
    assert!(request.contains("POST /open-apis/task/v2/custom_fields/field-1/remove "));
    assert!(request.contains("POST /open-apis/task/v2/custom_fields/field-1/options "));
    assert!(request.contains("PATCH /open-apis/task/v2/custom_fields/field-1/options/option-1 "));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""name":"Priority""#));
    assert!(request.contains(r#""name":"Priority updated""#));
    assert!(request.contains(r#""resource_type":"tasklist""#));
    assert!(request.contains(r#""name":"High""#));
    assert!(request.contains(r#""name":"Urgent""#));
}
