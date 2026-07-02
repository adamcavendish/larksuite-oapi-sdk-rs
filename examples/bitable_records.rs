use larksuite_oapi_sdk_rs::service::bitable::v1::{ListRecordQuery, ListTableQuery, ListViewQuery};
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::{Client, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let app_token = std::env::var("APP_TOKEN").expect("APP_TOKEN env var required");
    let table_id = std::env::var("TABLE_ID").expect("TABLE_ID env var required");
    let view_id = std::env::var("VIEW_ID").ok();

    let client = Client::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    let tables = client
        .bitable()
        .table
        .list_by_query(
            &ListTableQuery::new(&app_token).page(PageQuery::new().page_size(20)),
            &option,
        )
        .await?;
    if tables.success() {
        println!(
            "tables: {}",
            tables
                .data
                .as_ref()
                .map(|data| data.items.len())
                .unwrap_or(0)
        );
    } else {
        println!("table list error: {}", tables.code_error);
    }

    let views = client
        .bitable()
        .view
        .list_by_query(
            &ListViewQuery::new(&app_token, &table_id)
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20)),
            &option,
        )
        .await?;
    if views.success() {
        println!(
            "views: {}",
            views
                .data
                .as_ref()
                .map(|data| data.items.len())
                .unwrap_or(0)
        );
    } else {
        println!("view list error: {}", views.code_error);
    }

    let records = client
        .bitable()
        .record
        .list_by_query(
            &ListRecordQuery::new(&app_token, &table_id)
                .view_id(view_id.as_deref())
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20)),
            &option,
        )
        .await?;
    if records.success() {
        println!(
            "records: {}",
            records
                .data
                .as_ref()
                .map(|data| data.items.len())
                .unwrap_or(0)
        );
    } else {
        println!("record list error: {}", records.code_error);
    }

    Ok(())
}
