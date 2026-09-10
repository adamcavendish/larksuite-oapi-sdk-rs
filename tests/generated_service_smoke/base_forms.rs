use super::prelude::*;
use larksuite_oapi_sdk_rs::service::base::v3::{DeleteFormQuestionsReqBody, ListFormsQuery};

#[tokio::test]
async fn base_visible_fields_contract() {
    use larksuite_oapi_sdk_rs::service::base::v3::SetViewVisibleFieldsReqBody;

    let ordered: Vec<String> = (1..=12).rev().map(|index| format!("fld-{index}")).collect();
    // CLI's Base adapter accepts array-valued response data as well as objects.
    let response_body = serde_json::json!({"code":0,"msg":"ok","data":ordered}).to_string();
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, &response_body); 5]).await;
    let client = client_for(addr);
    let body = SetViewVisibleFieldsReqBody::new(ordered.clone());
    for (user, token) in [(true, "user-token"), (false, "tenant-token")] {
        let option = if user {
            RequestOption {
                user_access_token: Some(token.into()),
                ..Default::default()
            }
        } else {
            RequestOption {
                tenant_access_token: Some(token.into()),
                ..Default::default()
            }
        };
        let response = client
            .base_v3()
            .view
            .get_visible_fields("base /one", "table /two", "view /three", &option)
            .await
            .unwrap();
        assert!(response.success());
        assert_eq!(response.data.unwrap(), json_value!(ordered));
        let response = client
            .base_v3()
            .view
            .set_visible_fields("base /one", "table /two", "view /three", &body, &option)
            .await
            .unwrap();
        assert!(response.success());
        assert_eq!(response.data.unwrap(), json_value!(ordered));
        // The shared app-ID helper must not mutate caller options.
        assert!(option.headers.is_none());
    }
    client
        .base_v3()
        .view
        .set_visible_fields(
            "base /one",
            "table /two",
            "view /three",
            &SetViewVisibleFieldsReqBody::new(std::iter::empty::<String>()),
            &RequestOption {
                tenant_access_token: Some("tenant-token".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 5);
    for (index, request) in requests.iter().enumerate() {
        let (headers, body) = request.split_once("\r\n\r\n").unwrap();
        let method = if index == 0 || index == 2 {
            "GET"
        } else {
            "PUT"
        };
        let target = format!(
            "{method} /open-apis/base/v3/bases/base%20%2Fone/tables/table%20%2Ftwo/views/view%20%2Fthree/visible_fields HTTP/1.1"
        );
        assert_eq!(headers.lines().next(), Some(target.as_str()));
        let token = if index < 2 {
            "user-token"
        } else {
            "tenant-token"
        };
        assert!(
            headers
                .lines()
                .any(|line| line == format!("authorization: Bearer {token}"))
        );
        assert!(headers.lines().any(|line| line == "x-app-id: test_app_id"));
        if method == "GET" {
            assert!(body.is_empty());
        } else {
            let actual: serde_json::Value = serde_json::from_str(body).unwrap();
            let fields = if index == 4 { vec![] } else { ordered.clone() };
            assert_eq!(actual, serde_json::json!({"visible_fields":fields}));
        }
    }
}

#[tokio::test]
async fn base_v3_form_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body); 9]).await;
    let client = client_for(addr);
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_owned()),
        ..RequestOption::default()
    };
    let question_delete = DeleteFormQuestionsReqBody::new(["fld-1", "fld-2"]);

    client
        .base_v3()
        .form
        .list(
            &ListFormsQuery::new("base token", "table id").page(PageQuery::new().page_size(20)),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .form
        .get("base token", "table id", "form id", &option)
        .await
        .unwrap();
    client
        .base_v3()
        .form
        .create(
            "base token",
            "table id",
            json_value!({"name":"Survey"}),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .form
        .update(
            "base token",
            "table id",
            "form id",
            json_value!({"description":"Updated"}),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .form
        .delete("base token", "table id", "form id", &option)
        .await
        .unwrap();
    client
        .base_v3()
        .form_question
        .list("base token", "table id", "form id", &option)
        .await
        .unwrap();
    client
        .base_v3()
        .form_question
        .create(
            "base token",
            "table id",
            "form id",
            json_value!({"questions":[{"title":"Name","type":"text"}]}),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .form_question
        .update(
            "base token",
            "table id",
            "form id",
            json_value!({"questions":[{"id":"fld-1","title":"Name"}]}),
            &option,
        )
        .await
        .unwrap();
    client
        .base_v3()
        .form_question
        .delete(
            "base token",
            "table id",
            "form id",
            &question_delete,
            &option,
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    for needle in [
        "GET /open-apis/base/v3/bases/base%20token/tables/table%20id/forms?page_size=20 ",
        "GET /open-apis/base/v3/bases/base%20token/tables/table%20id/forms/form%20id ",
        "POST /open-apis/base/v3/bases/base%20token/tables/table%20id/forms ",
        "PATCH /open-apis/base/v3/bases/base%20token/tables/table%20id/forms/form%20id ",
        "DELETE /open-apis/base/v3/bases/base%20token/tables/table%20id/forms/form%20id ",
        "GET /open-apis/base/v3/bases/base%20token/tables/table%20id/forms/form%20id/questions ",
        "POST /open-apis/base/v3/bases/base%20token/tables/table%20id/forms/form%20id/questions ",
        "PATCH /open-apis/base/v3/bases/base%20token/tables/table%20id/forms/form%20id/questions ",
        "DELETE /open-apis/base/v3/bases/base%20token/tables/table%20id/forms/form%20id/questions ",
        "authorization: Bearer tenant-token",
        "x-app-id: test_app_id",
        r#""question_ids":["fld-1","fld-2"]"#,
    ] {
        assert!(request.contains(needle), "missing {needle}:\n{request}");
    }
    assert!(!request.contains(r#""keep_field":false"#));
}
