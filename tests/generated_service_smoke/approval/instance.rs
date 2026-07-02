use super::prelude::*;

// ── Approval ──

#[tokio::test]
async fn approval_instance_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"instance_code":"inst-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .instance
        .get_by_query(
            &GetInstanceQuery::new("inst-1")
                .locale("en-US")
                .user_id("ou-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.instance_code.as_deref()),
        Some("inst-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/instances/inst-1?"));
    assert!(request.contains("locale=en-US"));
    assert!(request.contains("user_id=ou-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn approval_instance_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"instance_code_list":["inst-1"],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .approval()
        .instance
        .list_by_query(
            &ListInstanceQuery::new("apv-1", "1700000000", "1700003600")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.instance_code_list.first())
            .map(String::as_str),
        Some("inst-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/approval/v4/instances?"));
    assert!(request.contains("approval_code=apv-1"));
    assert!(request.contains("start_time=1700000000"));
    assert!(request.contains("end_time=1700003600"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn approval_instance_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"instance_code":"inst-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = InstanceSearch {
        approval_code: Some("apv-1".to_string()),
        user_id: Some("ou-1".to_string()),
        ..Default::default()
    };
    let resp = client
        .approval()
        .instance
        .query_by_query(
            &QueryInstanceQuery::new(&body)
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/approval/v4/instances/query?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""approval_code":"apv-1""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
}

#[tokio::test]
async fn approval_instance_search_cc_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"cc_title":"Review"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = CcSearch {
        cc_title: Some("Review".to_string()),
        user_id: Some("ou-1".to_string()),
        ..Default::default()
    };
    let resp = client
        .approval()
        .instance
        .search_cc_by_query(
            &SearchCcInstanceQuery::new(&body)
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/approval/v4/instances/search_cc?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""cc_title":"Review""#));
    assert!(request.contains(r#""user_id":"ou-1""#));
}
