use super::prelude::*;
use larksuite_oapi_sdk_rs::service::base::v3::{DeleteFormQuestionsReqBody, ListFormsQuery};

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
