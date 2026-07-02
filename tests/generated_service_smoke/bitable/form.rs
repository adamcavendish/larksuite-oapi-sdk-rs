use super::prelude::*;

// ── Bitable ──

#[tokio::test]
async fn bitable_form_field_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"field_id":"fld-1","title":"Name"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFormFieldQuery::new("app-token-1", "tbl-1", "form-1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .bitable()
        .form_field
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].title.as_deref(), Some("Name"));
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains(
            "GET /open-apis/bitable/v1/apps/app-token-1/tables/tbl-1/forms/form-1/fields?"
        )
    );
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
