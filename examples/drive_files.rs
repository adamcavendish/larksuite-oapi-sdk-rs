use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::drive::v1::ListFileQuery;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let folder_token = std::env::var("FOLDER_TOKEN").ok();
    let drive_file_token = std::env::var("DRIVE_FILE_TOKEN").ok();
    let export_file_token = std::env::var("EXPORT_FILE_TOKEN").ok();
    let export_ticket = std::env::var("EXPORT_TICKET").ok();

    let client = LarkClient::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    let query = ListFileQuery::new()
        .folder_token(folder_token.as_deref())
        .order_by("EditedTime")
        .direction("DESC")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20));

    let files = client.drive().file.list_by_query(&query, &option).await?;
    if files.success() {
        let count = files
            .data
            .as_ref()
            .and_then(|data| data.files.as_ref())
            .map(Vec::len)
            .unwrap_or(0);
        println!("files: {count}");
        println!(
            "next_page_token: {:?}",
            files
                .data
                .as_ref()
                .and_then(|data| data.next_page_token.as_deref())
        );
    } else {
        println!("list files error: {}", files.code_error);
    }

    if let Some(file_token) = drive_file_token {
        let download = client.drive().file.download(&file_token, &option).await?;
        println!(
            "downloaded drive file: file_name={:?}, bytes={}",
            download.file_name,
            download.data.len()
        );
    }

    if let (Some(ticket), Some(file_token)) =
        (export_ticket.as_deref(), export_file_token.as_deref())
    {
        let task = client
            .drive()
            .export_task
            .get(ticket, file_token, &option)
            .await?;
        if task.success() {
            println!(
                "export status: {:?}",
                task.data
                    .as_ref()
                    .and_then(|data| data.result.as_ref())
                    .and_then(|result| result.job_status)
            );
        } else {
            println!("export task error: {}", task.code_error);
        }

        let download = client
            .drive()
            .export_task
            .download(file_token, &option)
            .await?;
        println!(
            "downloaded export: file_name={:?}, bytes={}",
            download.file_name,
            download.data.len()
        );
    }

    Ok(())
}
