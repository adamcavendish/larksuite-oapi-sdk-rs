use super::prelude::*;
use larksuite_oapi_sdk_rs::service::sheets::v3::{
    AppendValueReqBody, BatchSetRangeValueReqBody, FindCondition, FindSheetReqBody,
    MoveDimensionReqBody, PrependValueReqBody, ReplaceSheetReqBody, SetRangeValueReqBody,
    SheetCellValue, SheetDimension, SheetValueRange,
};

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
    let find_body = FindSheetReqBody {
        find_condition: Some(FindCondition {
            range: Some("sheet-1!A1:B10".into()),
            ..Default::default()
        }),
        find: Some("draft".into()),
    };
    let move_body = MoveDimensionReqBody {
        source: Some(SheetDimension {
            major_dimension: Some("ROWS".into()),
            start_index: Some(0),
            end_index: Some(1),
        }),
        destination_index: Some(4),
    };
    let replace_body = ReplaceSheetReqBody {
        find_condition: Some(FindCondition {
            range: Some("sheet-1!A1:B10".into()),
            ..Default::default()
        }),
        find: Some("draft".into()),
        replacement: Some("published".into()),
    };
    let find = client
        .sheets()
        .sheet
        .find("sht-1", "sheet-1", &find_body, &RequestOption::default())
        .await
        .unwrap();
    let move_dimension = client
        .sheets()
        .sheet
        .move_dimension("sht-1", "sheet-1", &move_body, &RequestOption::default())
        .await
        .unwrap();
    let replace = client
        .sheets()
        .sheet
        .replace("sht-1", "sheet-1", &replace_body, &RequestOption::default())
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

#[tokio::test]
async fn sheets_range_value_resources_use_typed_data() {
    let single_body = r#"{"code":0,"msg":"ok","data":{"revision":2,"spreadsheetToken":"sht-1","valueRange":{"range":"sheet-1!A1:B1","values":[["Ada",3]]}}}"#;
    let batch_body = r#"{"code":0,"msg":"ok","data":{"revision":3,"valueRanges":[{"range":"sheet-1!A1","values":[["Ada"]]}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, single_body),
        http_response(200, single_body),
        http_response(200, batch_body),
        http_response(200, single_body),
        http_response(200, single_body),
        http_response(200, single_body),
    ])
    .await;

    let client = client_for(addr);
    let range = SheetValueRange {
        range: Some("sheet-1!A1:B1".into()),
        values: Some(vec![vec![
            Some(SheetCellValue::Text("Ada".into())),
            Some(SheetCellValue::Number(3.0)),
        ]]),
        ..Default::default()
    };
    let option = RequestOption::default();
    let get = client
        .sheets()
        .range
        .get("sht-1", "sheet-1!A1:B1", None, None, None, &option)
        .await
        .unwrap();
    let set = client
        .sheets()
        .range
        .set(
            "sht-1",
            "sheet-1!A1:B1",
            &SetRangeValueReqBody {
                value_range: Some(range.clone()),
            },
            &option,
        )
        .await
        .unwrap();
    let batch_get = client
        .sheets()
        .range
        .batch_get("sht-1", &["sheet-1!A1"], None, None, None, &option)
        .await
        .unwrap();
    let batch_set = client
        .sheets()
        .range
        .batch_set(
            "sht-1",
            &BatchSetRangeValueReqBody {
                value_ranges: Some(vec![range.clone()]),
            },
            &option,
        )
        .await
        .unwrap();
    let append = client
        .sheets()
        .range
        .append(
            "sht-1",
            "sheet-1!A1",
            &AppendValueReqBody {
                value_range: Some(range.clone()),
            },
            Some("INSERT_ROWS"),
            &option,
        )
        .await
        .unwrap();
    let prepend = client
        .sheets()
        .range
        .prepend(
            "sht-1",
            "sheet-1!A1",
            &PrependValueReqBody {
                value_range: Some(range),
            },
            &option,
        )
        .await
        .unwrap();

    match get
        .data
        .as_ref()
        .and_then(|data| data.value_range.as_ref())
        .and_then(|range| range.values.as_ref())
        .and_then(|rows| rows.first())
        .and_then(|row| row.get(1))
        .and_then(Option::as_ref)
    {
        Some(SheetCellValue::Number(value)) => assert_eq!(*value, 3.0),
        _ => panic!("expected a numeric typed Sheets cell"),
    }
    assert_eq!(set.data.as_ref().and_then(|data| data.revision), Some(2));
    assert_eq!(
        batch_get
            .data
            .as_ref()
            .and_then(|data| data.value_ranges.as_ref())
            .map(Vec::len),
        Some(1)
    );
    assert_eq!(
        batch_set.data.as_ref().and_then(|data| data.revision),
        Some(2)
    );
    assert_eq!(append.data.as_ref().and_then(|data| data.revision), Some(2));
    assert_eq!(
        prepend.data.as_ref().and_then(|data| data.revision),
        Some(2)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/sheets/v3/spreadsheets/sht-1/values/sheet-1!A1:B1"));
    assert!(request.contains("PUT /open-apis/sheets/v3/spreadsheets/sht-1/values/sheet-1!A1:B1"));
    assert!(request.contains("GET /open-apis/sheets/v3/spreadsheets/sht-1/values_batch_get"));
    assert!(request.contains("POST /open-apis/sheets/v3/spreadsheets/sht-1/values_batch_update"));
    assert!(request.contains("POST /open-apis/sheets/v3/spreadsheets/sht-1/values_append"));
    assert!(request.contains("POST /open-apis/sheets/v3/spreadsheets/sht-1/values_prepend"));
    assert!(request.contains("\"value_range\":{\"range\":\"sheet-1!A1:B1\""));
}
