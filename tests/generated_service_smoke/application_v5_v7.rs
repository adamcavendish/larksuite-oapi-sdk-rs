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
        resp.data.as_ref().and_then(|data| data.url.as_deref()),
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
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let ability_body = PatchApplicationAbilityReqBody::new()
        .web_app(AppAbilityWeb {
            pc_url: Some("https://example.com/app".into()),
            ..Default::default()
        })
        .bot(AppAbilityBot {
            message_card_callback_url: Some("https://example.com/card".into()),
            ..Default::default()
        });
    let base_body = PatchApplicationBaseReqBody::new()
        .i18ns(vec![AppI18nInfo {
            i18n_key: Some("zh_cn".into()),
            name: Some("Application".into()),
            ..Default::default()
        }])
        .avatar_url("https://example.com/avatar.png")
        .homepage_url("https://example.com/home");
    let config_body = PatchApplicationConfigReqBody::new()
        .scope(AppConfigScope {
            add_scopes: vec![AppConfigScopeItem {
                scope_name: Some("im:message".into()),
                token_type: Some("tenant".into()),
            }],
            ..Default::default()
        })
        .callback(AppConfigCallback {
            request_url: Some("https://example.com/callback".into()),
            ..Default::default()
        });

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
            .and_then(|data| data.version_id.as_deref()),
        Some("ver-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/application/v7/applications/cli_a/publish "));
    assert!(request.contains(r#""mobile_default_ability":"gadget""#));
    assert!(request.contains(r#""pc_default_ability":"web_app""#));
    assert!(request.contains(r#""version":"1.1.1""#));
}

#[tokio::test]
async fn application_v7_app_slash_command_lifecycle_smoke() {
    let create = r#"{"code":0,"msg":"ok","data":{"command_id":"cmd-1"}}"#;
    let list = r#"{"code":0,"msg":"ok","data":{"items":[{"command_id":"cmd-1","command":"greet","description":{"default_value":"Send a greeting","i18n":{"en_us":"Send a greeting","zh_cn":"发送一句问候"}},"icon":{"icon_key":"skill_outlined"},"update_time":"1716963953"}]}}"#;
    let empty = r#"{"code":0,"msg":"ok","data":{}}"#;
    let duplicate = r#"{"code":40000000,"msg":"command already exists"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, create),
        http_response(200, list),
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, duplicate),
    ])
    .await;

    let client = client_for(addr);
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };
    let description = AppSlashCommandDescription::new("Send a greeting")
        .i18n(
            AppSlashCommandI18n::new()
                .insert("en_us", "Send a greeting")
                .insert("zh_cn", "发送一句问候"),
        )
        .icon(AppSlashCommandIcon::new("skill_outlined"));
    let create_body = CreateAppSlashCommandReqBody::new("greet", description.clone());

    let created = client
        .application_v7()
        .app_slash_command
        .create(&create_body, &option)
        .await
        .unwrap();
    let listed = client
        .application_v7()
        .app_slash_command
        .list(&option)
        .await
        .unwrap();
    let patch_body = PatchAppSlashCommandReqBody::new().description(description);
    let patched = client
        .application_v7()
        .app_slash_command
        .patch_by_query(
            &PatchAppSlashCommandQuery::new("cmd/a?b", &patch_body),
            &option,
        )
        .await
        .unwrap();
    let deleted = client
        .application_v7()
        .app_slash_command
        .delete_by_query(&DeleteAppSlashCommandQuery::new("cmd/a?b"), &option)
        .await
        .unwrap();
    let duplicate = client
        .application_v7()
        .app_slash_command
        .create(&create_body, &option)
        .await
        .unwrap_err();

    assert!(created.success());
    assert_eq!(
        created
            .data
            .as_ref()
            .and_then(|data| data.command_id.as_deref()),
        Some("cmd-1")
    );
    assert!(listed.success());
    let command = listed.data.as_ref().unwrap().items.first().unwrap();
    assert_eq!(command.command.as_deref(), Some("greet"));
    assert_eq!(
        command
            .description
            .as_ref()
            .and_then(|description| description.i18n.as_ref())
            .and_then(|i18n| i18n.values.get("zh_cn"))
            .map(String::as_str),
        Some("发送一句问候")
    );
    assert_eq!(
        command
            .icon
            .as_ref()
            .and_then(|icon| icon.icon_key.as_deref()),
        Some("skill_outlined")
    );
    assert!(patched.success());
    assert!(deleted.success());
    assert!(matches!(
        duplicate,
        larksuite_oapi_sdk_rs::LarkError::Api(_)
    ));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/application/v7/app_slash_commands "));
    assert!(request.contains("GET /open-apis/application/v7/app_slash_commands "));
    assert!(request.contains("PATCH /open-apis/application/v7/app_slash_commands/cmd%2Fa%3Fb "));
    assert!(request.contains("DELETE /open-apis/application/v7/app_slash_commands/cmd%2Fa%3Fb "));
    assert!(
        request.contains("authorization: Bearer tenant-token"),
        "{request}"
    );
    assert!(request.contains("content-type: application/json"));
    assert!(request.contains(r#""command":"greet""#));
    assert!(!request.contains(r#""command":"/greet""#));
    assert!(request.contains(r#""icon":{"icon_key":"skill_outlined"}"#));
}
