use super::prelude::*;

// ── ACS ──

#[tokio::test]
async fn acs_rule_external_by_query_smoke() {
    let create_body = r#"{"code":0,"msg":"ok","data":{"rule_id":"rule-1"}}"#;
    let rule_body =
        r#"{"code":0,"msg":"ok","data":{"rules":[{"id":"rule-1","devices":[{"id":"dev-1"}]}]}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let bind_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, create_body),
        http_response(200, empty_body),
        http_response(200, bind_body),
        http_response(200, rule_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateRuleExternalReqBody {
        rule: Some(Rule {
            name: Some("Office access".into()),
            devices: vec![DeviceExternal {
                id: Some("dev-1".into()),
                ..Default::default()
            }],
            opening_time: Some(OpeningTimeExternal {
                day_times: vec![OpeningTimePeriodExternal {
                    start_hhmm: Some(900),
                    end_hhmm: Some(1800),
                }],
                ..Default::default()
            }),
            ..Default::default()
        }),
    };
    let bind_body = DeviceBindRuleExternalReqBody {
        device_id: Some("dev-1".into()),
        rule_ids: vec!["rule-1".into()],
    };
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
    assert_eq!(
        create_resp
            .data
            .as_ref()
            .and_then(|data| data.rule_id.as_deref()),
        Some("rule-1")
    );
    assert_eq!(
        get_resp.data.as_ref().unwrap().rules[0].devices[0]
            .id
            .as_deref(),
        Some("dev-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/acs/v1/rule_external?"));
    assert!(request.contains("DELETE /open-apis/acs/v1/rule_external?"));
    assert!(request.contains("POST /open-apis/acs/v1/rule_external/device_bind"));
    assert!(request.contains("GET /open-apis/acs/v1/rule_external?"));
    assert!(request.contains("rule_id=rule-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("device_id=dev-1"));
    assert!(request.contains(r#""name":"Office access""#));
    assert!(request.contains(r#""device_id":"dev-1""#));
    assert!(request.contains(r#""rule_ids":["rule-1"]"#));
}
