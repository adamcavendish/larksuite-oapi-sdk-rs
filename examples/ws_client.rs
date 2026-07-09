#![recursion_limit = "256"]

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::event::EventDispatcher;
use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");

    let client = Client::builder(&app_id, &app_secret).build()?;

    let dispatcher = EventDispatcher::new("", "").on_p2_im_message_receive_v1(
        |event: P2MessageReceiveV1| async move {
            println!(
                "[ws] message: id={} content={}",
                event.message.message_id, event.message.content
            );
            Ok(())
        },
    );

    let ws_client = client.ws_client(dispatcher);
    println!("starting ws client (Ctrl+C to stop)...");
    ws_client.start().await?;

    Ok(())
}
