use larksuite_oapi_sdk_rs::{Client, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let spreadsheet_token =
        std::env::var("SPREADSHEET_TOKEN").expect("SPREADSHEET_TOKEN env var required");
    let range = std::env::var("SHEET_RANGE").unwrap_or_else(|_| "Sheet1!A1:C10".to_string());

    let client = Client::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    let spreadsheet = client
        .sheets()
        .spreadsheet
        .get(&spreadsheet_token, Some("open_id"), &option)
        .await?;
    if spreadsheet.success() {
        println!(
            "spreadsheet title: {:?}",
            spreadsheet
                .data
                .as_ref()
                .and_then(|data| data.spreadsheet.as_ref())
                .and_then(|spreadsheet| spreadsheet.title.as_deref())
        );
    } else {
        println!("spreadsheet error: {}", spreadsheet.code_error);
    }

    let values = client
        .sheets()
        .range
        .get(
            &spreadsheet_token,
            &range,
            Some("ToString"),
            Some("FormattedString"),
            Some("open_id"),
            &option,
        )
        .await?;
    if values.success() {
        let row_count = values
            .data
            .as_ref()
            .and_then(|data| data.get("valueRange"))
            .and_then(|value_range| value_range.get("values"))
            .and_then(|values| values.as_array())
            .map(Vec::len)
            .unwrap_or(0);
        println!("range: {range}");
        println!("rows: {row_count}");
    } else {
        println!("range error: {}", values.code_error);
    }

    Ok(())
}
