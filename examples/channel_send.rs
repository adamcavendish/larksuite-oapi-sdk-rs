use larksuite_oapi_sdk_rs::channel::{Channel, SendInput};
use larksuite_oapi_sdk_rs::{EventDispatcher, LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let chat_id = std::env::var("CHAT_ID").expect("CHAT_ID env var required");

    let client = LarkClient::builder(app_id, app_secret).build()?;
    let dispatcher = EventDispatcher::new("", "");
    let channel = Channel::builder(&client, dispatcher).build();

    let sent = Box::pin(channel.send(
        &SendInput {
            chat_id: Some(chat_id),
            markdown: Some("**Hello from channel send**".into()),
            title: Some("Rust SDK".into()),
            ..Default::default()
        },
        &RequestOption::default(),
    ))
    .await?;

    println!("sent message: {}", sent.message_id);
    Ok(())
}
