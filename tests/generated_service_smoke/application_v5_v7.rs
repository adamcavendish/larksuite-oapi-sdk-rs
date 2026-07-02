use super::prelude::*;

// ── Application v5 / v7 ──

#[tokio::test]
async fn application_v5_favourite_and_recommend_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"app_list":[{"app_id":"cli_a"}],"has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body), http_response(200, body)]).await;

    let client = client_for(addr);
    let option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };

    let favourite = client
        .application_v5()
        .application
        .favourite_by_query(
            &FavouriteApplicationQuery::new()
                .language("zh_cn")
                .page_size(20)
                .page_token("fav-page"),
            &option,
        )
        .await
        .unwrap();
    let recommend = client
        .application_v5()
        .application
        .recommend_by_query(
            &RecommendApplicationQuery::new()
                .language("zh_cn")
                .recommend_type("user_unremovable")
                .page_size(30)
                .page_token("rec-page"),
            &option,
        )
        .await
        .unwrap();

    assert!(favourite.success());
    assert!(recommend.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/application/v5/applications/favourite?"));
    assert!(request.contains("language=zh_cn"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=fav-page"));
    assert!(request.contains("GET /open-apis/application/v5/applications/recommend?"));
    assert!(request.contains("recommend_type=user_unremovable"));
    assert!(request.contains("page_size=30"));
    assert!(request.contains("page_token=rec-page"));
}

#[tokio::test]
async fn application_v7_app_avatar_upload_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"url":"https://example.com/avatar.png"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .application_v7()
        .app_avatar_upload
        .create_by_query(
            &CreateAppAvatarUploadQuery::avatar(
                "avatar.png",
                b"avatar-bytes".to_vec(),
                Some("image/png"),
            ),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("url"))
            .and_then(|url| url.as_str()),
        Some("https://example.com/avatar.png")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/application/v7/app_avatar/upload "));
    assert!(request.contains("name=\"avatar\""));
    assert!(request.contains("filename=\"avatar.png\""));
    assert!(request.contains("avatar-bytes"));
}

#[tokio::test]
async fn application_v7_patch_resources_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ok":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let ability_body = PatchApplicationAbilityReqBody::new()
        .web_app(serde_json::json!({"pc_url":"https://example.com/app"}))
        .bot(serde_json::json!({"card_request_url":"https://example.com/card"}));
    let base_body = PatchApplicationBaseReqBody::new()
        .i18ns(serde_json::json!([{"locale":"zh_cn","name":"应用"}]))
        .avatar_url("https://example.com/avatar.png")
        .homepage_url("https://example.com/home");
    let config_body = PatchApplicationConfigReqBody::new()
        .scope(serde_json::json!({"tenant":["im:message"]}))
        .callback(serde_json::json!({"request_url":"https://example.com/callback"}));

    let ability = client
        .application_v7()
        .application_ability
        .patch_by_query(
            &PatchApplicationAbilityQuery::new("cli_a", &ability_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let base = client
        .application_v7()
        .application_base
        .patch_by_query(
            &PatchApplicationBaseQuery::new("cli_a", &base_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let config = client
        .application_v7()
        .application_config
        .patch_by_query(
            &PatchApplicationConfigQuery::new("cli_a", &config_body)
                .department_id_type("open_department_id")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(ability.success());
    assert!(base.success());
    assert!(config.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("PATCH /open-apis/application/v7/applications/cli_a/ability "));
    assert!(request.contains(r#""web_app":{"pc_url":"https://example.com/app"}"#));
    assert!(request.contains("PATCH /open-apis/application/v7/applications/cli_a/base "));
    assert!(request.contains(r#""avatar_url":"https://example.com/avatar.png""#));
    assert!(request.contains("PATCH /open-apis/application/v7/applications/cli_a/config?"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""callback":{"request_url":"https://example.com/callback"}"#));
}

#[tokio::test]
async fn application_v7_publish_create_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"version_id":"ver-1","version":"1.1.1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let publish_body = CreateApplicationPublishReqBody::new()
        .mobile_default_ability("gadget")
        .pc_default_ability("web_app")
        .remark("ship")
        .changelog("updated")
        .version("1.1.1");
    let resp = client
        .application_v7()
        .application_publish
        .create_by_query(
            &CreateApplicationPublishQuery::new("cli_a", &publish_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.get("version_id"))
            .and_then(|version_id| version_id.as_str()),
        Some("ver-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/application/v7/applications/cli_a/publish "));
    assert!(request.contains(r#""mobile_default_ability":"gadget""#));
    assert!(request.contains(r#""pc_default_ability":"web_app""#));
    assert!(request.contains(r#""version":"1.1.1""#));
}
