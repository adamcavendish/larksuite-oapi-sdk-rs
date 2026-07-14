use super::prelude::*;

// ── Search ──

#[tokio::test]
async fn search_message_app_doc_wiki_by_query_smoke() {
    let message_body = r#"{"code":0,"msg":"ok","data":{"items":["om_1"],"has_more":false}}"#;
    let app_body = r#"{"code":0,"msg":"ok","data":{"items":["cli_a"],"has_more":false}}"#;
    let doc_wiki_body =
        r#"{"code":0,"msg":"ok","data":{"total":1,"has_more":false,"res_units":[{"id":"doc-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, message_body),
        http_response(200, app_body),
        http_response(200, doc_wiki_body),
    ])
    .await;

    let client = client_for(addr);
    let message_query_body = SearchMessageReqBody {
        query: Some("status update".into()),
        from_ids: Some(vec!["ou-1".into()]),
        chat_ids: Some(vec!["oc-1".into()]),
        with_url: Some(true),
        page_size: Some(20),
        page_token: Some("next-page".into()),
        ..Default::default()
    };
    let app_query_body = SearchAppReqBody {
        query: Some("approval".into()),
        page_size: Some(10),
        page_token: Some("app-page".into()),
    };
    let doc_wiki_query_body = SearchDocWikiReqBody {
        query: Some("handbook".into()),
        doc_filter: Some(DocFilter {
            creator_ids: vec!["ou-1".into()],
            ..Default::default()
        }),
        page_token: Some("doc-page".into()),
        page_size: Some(5),
        ..Default::default()
    };

    client
        .search()
        .message
        .create_by_query(
            &CreateMessageSearchQuery::new(&message_query_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .app
        .create_by_query(
            &CreateAppSearchQuery::new(&app_query_body).user_id_type("union_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .search()
        .doc_wiki
        .search_by_query(
            &SearchDocWikiQuery::new(&doc_wiki_query_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/search/v2/message?user_id_type=open_id "));
    assert!(request.contains("POST /open-apis/search/v2/app?user_id_type=union_id "));
    assert!(request.contains("POST /open-apis/search/v2/doc_wiki/search "));
    assert!(request.contains(r#""query":"status update""#));
    assert!(request.contains(r#""with_url":true"#));
    assert!(request.contains(r#""query":"approval""#));
    assert!(request.contains(r#""query":"handbook""#));
    assert!(request.contains(r#""creator_ids":["ou-1"]"#));
}
