use super::prelude::*;

// ── ACS ──

#[tokio::test]
async fn acs_rule_external_by_query_smoke() {
    let rule_body = r#"{"code":0,"msg":"ok","data":{"rule":{"rule_id":"rule-1"}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let bind_body = r#"{"code":0,"msg":"ok","data":{"device_id":"dev-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, rule_body),
        http_response(200, empty_body),
        http_response(200, bind_body),
        http_response(200, rule_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = serde_json::json!({"device_id":"dev-1","user_ids":["ou-1"]});
    let bind_body = serde_json::json!({"device_ids":["dev-1"],"rule_id":"rule-1"});
    let create_resp = client
        .acs()
        .rule_external
        .create_by_query(
            &CreateAcsRuleExternalQuery::new(&create_body)
                .rule_id("rule-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_resp = client
        .acs()
        .rule_external
        .delete_by_query(
            &DeleteAcsRuleExternalQuery::new("rule-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let bind_resp = client
        .acs()
        .rule_external
        .device_bind_by_query(
            &DeviceBindRuleExternalQuery::new(&bind_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let get_resp = client
        .acs()
        .rule_external
        .get_by_query(
            &GetRuleExternalQuery::new()
                .device_id("dev-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(create_resp.success());
    assert!(delete_resp.success());
    assert!(bind_resp.success());
    assert!(get_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/acs/v1/rule_external?"));
    assert!(request.contains("DELETE /open-apis/acs/v1/rule_external?"));
    assert!(request.contains("POST /open-apis/acs/v1/rule_external/device_bind"));
    assert!(request.contains("GET /open-apis/acs/v1/rule_external?"));
    assert!(request.contains("rule_id=rule-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("device_id=dev-1"));
    assert!(request.contains(r#""user_ids":["ou-1"]"#));
    assert!(request.contains(r#""device_ids":["dev-1"]"#));
}
