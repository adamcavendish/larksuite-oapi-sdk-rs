use larksuite_oapi_sdk_rs::service::im::v1::ListMessageQuery;
use larksuite_oapi_sdk_rs::{Client, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let chat_id = std::env::var("CHAT_ID").expect("CHAT_ID env var required");
    let start_time = std::env::var("START_TIME").ok();
    let end_time = std::env::var("END_TIME").ok();
    let page_token = std::env::var("PAGE_TOKEN").ok();

    let client = Client::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    let query = ListMessageQuery::new("chat", &chat_id)
        .start_time(start_time.as_deref())
        .end_time(end_time.as_deref())
        .sort_type("ByCreateTimeDesc")
        .page_size(20)
        .page_token(page_token.as_deref());

    let resp = client.im().message.list_by_query(&query, &option).await?;

    if resp.success() {
        let items = resp
            .data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .map(Vec::len)
            .unwrap_or(0);
        println!("messages: {items}");
        println!(
            "next_page_token: {:?}",
            resp.data
                .as_ref()
                .and_then(|data| data.page_token.as_deref())
        );
    } else {
        println!("error: {}", resp.code_error);
    }

    Ok(())
}
