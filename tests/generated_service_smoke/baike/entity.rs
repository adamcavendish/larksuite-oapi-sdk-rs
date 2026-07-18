use super::prelude::*;

// ── Baike ──

#[tokio::test]
async fn baike_entity_by_query_smoke() {
    let entity_body = r#"{"code":0,"msg":"ok","data":{"entity":{"id":"entity-1"}}}"#;
    let list_body =
        r#"{"code":0,"msg":"ok","data":{"entities":[{"id":"entity-1"}],"has_more":false}}"#;
    let search_body =
        r#"{"code":0,"msg":"ok","data":{"entities":[{"id":"entity-1"}],"has_more":false}}"#;
    let empty_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let value_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"entity-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, entity_body),
        http_response(200, entity_body),
        http_response(200, entity_body),
        http_response(200, list_body),
        http_response(200, search_body),
        http_response(200, empty_body),
        http_response(200, value_body),
        http_response(200, value_body),
    ])
    .await;

    let client = client_for(addr);
    let create_body = CreateBaikeEntityReqBody {
        main_keys: Some(vec![BaikeTerm {
            key: Some("Term".into()),
            ..Default::default()
        }]),
        description: Some("Description".into()),
        ..Default::default()
    };
    let update_body = CreateBaikeEntityReqBody {
        main_keys: Some(vec![BaikeTerm {
            key: Some("Term updated".into()),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let search_body = SearchBaikeEntityReqBody {
        query: Some("term".into()),
        sources: Some(vec![1]),
        ..Default::default()
    };
    let highlight_body = json_value!({"text":"Term highlight"});
    let extract_body = json_value!({"text":"Term extract"});
    let match_body = json_value!({"text":"Term match"});

    let create_resp = Box::pin(client.baike().entity.create_by_query(
        &CreateBaikeEntityQuery::new(&create_body).user_id_type("open_id"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    let update_resp = Box::pin(client.baike().entity.update_by_query(
        &UpdateBaikeEntityQuery::new("entity-1", &update_body).user_id_type("open_id"),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    let get_resp = Box::pin(
        client.baike().entity.get_by_query(
            &GetBaikeEntityQuery::new("entity-1")
                .provider("provider-1")
                .outer_id("outer-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        ),
    )
    .await
    .unwrap();
    let list_resp = Box::pin(
        client.baike().entity.list_by_query(
            &ListBaikeEntityQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .provider("provider-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        ),
    )
    .await
    .unwrap();
    let search_resp = Box::pin(
        client.baike().entity.search_by_query(
            &SearchBaikeEntityQuery::new(&search_body)
                .page(PageQuery::new().page_size(10).page_token("search-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        ),
    )
    .await
    .unwrap();
    let highlight_resp = Box::pin(client.baike().entity.highlight_by_query(
        &HighlightBaikeEntityQuery::new(&highlight_body),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    let extract_resp = Box::pin(client.baike().entity.extract_by_query(
        &ExtractBaikeEntityQuery::new(&extract_body),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    let match_resp = Box::pin(client.baike().entity.match_by_query(
        &MatchBaikeEntityQuery::new(&match_body),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    assert!(create_resp.success());
    assert!(update_resp.success());
    assert!(get_resp.success());
    assert!(list_resp.success());
    assert!(search_resp.success());
    assert!(highlight_resp.success());
    assert!(extract_resp.success());
    assert!(match_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/baike/v1/entities?"));
    assert!(request.contains("PUT /open-apis/baike/v1/entities/entity-1?"));
    assert!(request.contains("GET /open-apis/baike/v1/entities/entity-1?"));
    assert!(request.contains("GET /open-apis/baike/v1/entities?"));
    assert!(request.contains("POST /open-apis/baike/v1/entities/search?"));
    assert!(request.contains("POST /open-apis/baike/v1/entities/highlight"));
    assert!(request.contains("POST /open-apis/baike/v1/entities/extract"));
    assert!(request.contains("POST /open-apis/baike/v1/entities/match"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("provider=provider-1"));
    assert!(request.contains("outer_id=outer-1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("page_size=10"));
    assert!(request.contains("page_token=search-page"));
    assert!(request.contains(r#""key":"Term""#));
    assert!(request.contains(r#""key":"Term updated""#));
    assert!(request.contains(r#""query":"term""#));
    assert!(request.contains(r#""sources":[1]"#));
    assert!(request.contains(r#""text":"Term highlight""#));
    assert!(request.contains(r#""text":"Term extract""#));
    assert!(request.contains(r#""text":"Term match""#));
}
