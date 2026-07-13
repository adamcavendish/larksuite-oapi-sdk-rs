use super::prelude::*;

// ── Cardkit ──

#[tokio::test]
async fn cardkit_card_instance_by_query_smoke() {
    let create_body = r#"{"code":0,"msg":"ok","data":{"instance_id":"instance-1"}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, create_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_request = CreateCardInstanceReqBody {
        template_id: Some("template-1".into()),
        template_variable: Some(serde_json::json!({"title":"Card"})),
    };
    let update_request = UpdateCardInstanceReqBody {
        template_variable: Some(serde_json::json!({"title":"Updated"})),
    };

    let create_resp = client
        .cardkit()
        .card_instance
        .create_by_query(
            &CreateCardkitInstanceQuery::new(&create_request),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .cardkit()
        .card_instance
        .update_by_query(
            &UpdateCardkitInstanceQuery::new("instance-1", &update_request),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(update_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/cardkit/v1/card_instances "));
    assert!(request.contains("PUT /open-apis/cardkit/v1/card_instances/instance-1 "));
    assert!(request.contains(r#""template_id":"template-1""#));
    assert!(request.contains(r#""title":"Card""#));
    assert!(request.contains(r#""title":"Updated""#));
}

#[tokio::test]
async fn cardkit_card_by_query_smoke() {
    let value_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, value_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, value_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"type":"template","data":{"title":"Card"}});
    let update_body = serde_json::json!({"data":{"title":"Updated"}});
    let batch_body = serde_json::json!({"operations":[{"op":"replace","path":"/title"}]});
    let convert_body = serde_json::json!({"card_id":"card-1"});
    let settings_body = serde_json::json!({"config":{"wide":true}});

    let create_resp = client
        .cardkit()
        .card
        .create_by_query(
            &CreateCardkitCardQuery::new(&create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .cardkit()
        .card
        .update_by_query(
            &UpdateCardkitCardQuery::new("card-1", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let batch_resp = client
        .cardkit()
        .card
        .batch_update_by_query(
            &BatchUpdateCardkitCardQuery::new("card-1", &batch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let convert_resp = client
        .cardkit()
        .card
        .id_convert_by_query(
            &IdConvertCardkitCardQuery::new(&convert_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let settings_resp = client
        .cardkit()
        .card
        .settings_by_query(
            &SettingsCardkitCardQuery::new("card-1", &settings_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(update_resp.success());
    assert!(batch_resp.success());
    assert!(convert_resp.success());
    assert!(settings_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/cardkit/v1/cards "));
    assert!(request.contains("PUT /open-apis/cardkit/v1/cards/card-1 "));
    assert!(request.contains("POST /open-apis/cardkit/v1/cards/card-1/batch_update "));
    assert!(request.contains("POST /open-apis/cardkit/v1/cards/id_convert "));
    assert!(request.contains("PATCH /open-apis/cardkit/v1/cards/card-1/settings "));
    assert!(request.contains(r#""title":"Card""#));
    assert!(request.contains(r#""title":"Updated""#));
    assert!(request.contains(r#""operations":[{"op":"replace","path":"/title"}]"#));
    assert!(request.contains(r#""card_id":"card-1""#));
    assert!(request.contains(r#""wide":true"#));
}

#[tokio::test]
async fn cardkit_card_element_by_query_smoke() {
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"tag":"div","text":"create"});
    let update_body = serde_json::json!({"tag":"div","text":"update"});
    let patch_body = serde_json::json!({"text":"patch"});
    let content_body = serde_json::json!({"content":"content update"});

    let create_resp = client
        .cardkit()
        .card_element
        .create_by_query(
            &CreateCardkitElementQuery::new("card-1", &create_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_resp = client
        .cardkit()
        .card_element
        .update_by_query(
            &UpdateCardkitElementQuery::new("card-1", "element-1", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_resp = client
        .cardkit()
        .card_element
        .delete_by_query(
            &DeleteCardkitElementQuery::new("card-1", "element-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let patch_resp = client
        .cardkit()
        .card_element
        .patch_by_query(
            &PatchCardkitElementQuery::new("card-1", "element-1", &patch_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let content_resp = client
        .cardkit()
        .card_element
        .content_by_query(
            &ContentCardkitElementQuery::new("card-1", "element-1", &content_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(update_resp.success());
    assert!(delete_resp.success());
    assert!(patch_resp.success());
    assert!(content_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/cardkit/v1/cards/card-1/elements "));
    assert!(request.contains("PUT /open-apis/cardkit/v1/cards/card-1/elements/element-1 "));
    assert!(request.contains("DELETE /open-apis/cardkit/v1/cards/card-1/elements/element-1 "));
    assert!(request.contains("PATCH /open-apis/cardkit/v1/cards/card-1/elements/element-1 "));
    assert!(request.contains("PUT /open-apis/cardkit/v1/cards/card-1/elements/element-1/content "));
    assert!(request.contains(r#""text":"create""#));
    assert!(request.contains(r#""text":"update""#));
    assert!(request.contains(r#""text":"patch""#));
    assert!(request.contains(r#""content":"content update""#));
}
