use super::prelude::*;

// ── Sheets ──

#[tokio::test]
async fn sheets_get_spreadsheet_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"spreadsheet":{"spreadsheet_token":"sht-1","title":"Budget"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .sheets()
        .spreadsheet
        .get("sht-1", None, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let ss = resp.data.unwrap().spreadsheet.unwrap();
    assert_eq!(ss.spreadsheet_token.as_deref(), Some("sht-1"));
    assert_eq!(ss.title.as_deref(), Some("Budget"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/sheets/v3/spreadsheets/sht-1"));
}

#[tokio::test]
async fn sheets_sheet_find_replace_and_move_smoke() {
    let find_body = r#"{"code":0,"msg":"ok","data":{"find_result":{"matched_cells":["A1"],"matched_formula_cells":["B1"],"rows_count":1}}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let replace_body = r#"{"code":0,"msg":"ok","data":{"replace_result":{"matched_cells":["A1"],"rows_count":1}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, find_body),
        http_response(200, empty_body),
        http_response(200, replace_body),
    ])
    .await;

    let client = client_for(addr);
    let body = serde_json::json!({"find_condition": {"find": "draft"}});
    let find = client
        .sheets()
        .sheet
        .find("sht-1", "sheet-1", &body, &RequestOption::default())
        .await
        .unwrap();
    let move_dimension = client
        .sheets()
        .sheet
        .move_dimension("sht-1", "sheet-1", &body, &RequestOption::default())
        .await
        .unwrap();
    let replace = client
        .sheets()
        .sheet
        .replace("sht-1", "sheet-1", &body, &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(
        find.data
            .as_ref()
            .and_then(|data| data.find_result.as_ref())
            .and_then(|result| result.matched_formula_cells.as_ref())
            .and_then(|cells| cells.first())
            .map(String::as_str),
        Some("B1")
    );
    assert_eq!(
        replace
            .data
            .as_ref()
            .and_then(|data| data.replace_result.as_ref())
            .and_then(|result| result.rows_count),
        Some(1)
    );
    assert!(move_dimension.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/sheets/v3/spreadsheets/sht-1/sheets/sheet-1/find "));
    assert!(
        request.contains(
            "POST /open-apis/sheets/v3/spreadsheets/sht-1/sheets/sheet-1/move_dimension "
        )
    );
    assert!(
        request.contains("POST /open-apis/sheets/v3/spreadsheets/sht-1/sheets/sheet-1/replace ")
    );
}
