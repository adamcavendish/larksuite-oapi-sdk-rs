use larksuite_oapi_sdk_rs::channel::SendInput;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let chat_id = std::env::var("CHAT_ID").expect("CHAT_ID env var required");

    let client = LarkClient::builder(app_id, app_secret).build()?;
    let messaging = client.channel_messaging();

    let sent = Box::pin(messaging.send(
        &SendInput {
            chat_id: Some(chat_id),
            markdown: Some("**Hello from channel messaging**".into()),
            title: Some("Rust SDK".into()),
            uuid: std::env::var("MESSAGE_UUID").ok(),
            ..Default::default()
        },
        &RequestOption::default(),
    ))
    .await?;

    println!("sent message: {}", sent.message_id);
    Ok(())
}
