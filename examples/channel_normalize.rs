#![recursion_limit = "256"]

use larksuite_oapi_sdk_rs::channel::{Channel, ChannelPolicy};
use larksuite_oapi_sdk_rs::{Client, EventDispatcher, LarkError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let verification_token = std::env::var("VERIFICATION_TOKEN").unwrap_or_default();
    let encrypt_key = std::env::var("ENCRYPT_KEY").unwrap_or_default();

    let client = Client::builder(app_id, app_secret).build()?;
    let dispatcher = EventDispatcher::new(verification_token, encrypt_key);

    Channel::builder(&client, dispatcher)
        .policy(ChannelPolicy::default().require_mention(false))
        .on_message(|message| async move {
            println!("message_id: {}", message.message_id);
            println!("raw content: {}", message.content);
            println!("normalized text: {}", message.text.unwrap_or_default());
            for resource in message.resources {
                println!("resource: {} {}", resource.resource_type, resource.file_key);
            }
            Ok::<(), LarkError>(())
        })
        .build()
        .start()
        .await?;

    Ok(())
}
