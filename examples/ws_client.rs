use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::event::EventDispatcher;
use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");

    let client = LarkClient::builder(&app_id, &app_secret).build()?;

    let dispatcher = EventDispatcher::new("", "").on_p2_im_message_receive_v1(
        |event: P2MessageReceiveV1| async move {
            println!(
                "[ws] message: id={} content={}",
                event.message.message_id, event.message.content
            );
            Ok(())
        },
    );

    let mut ws_client = client.ws_client(dispatcher);
    if let Ok(channel_tag) = std::env::var("WS_CHANNEL_TAG") {
        ws_client = ws_client.channel_tag(channel_tag);
    }
    let user_access_token = std::env::var("USER_ACCESS_TOKEN").ok();
    let control = user_access_token.as_ref().map(|_| ws_client.control());

    println!("starting ws client (Ctrl+C to stop)...");
    let ws_task = tokio::spawn(ws_client.start());

    if let (Some(control), Some(user_access_token)) = (control, user_access_token) {
        while control.connection_id().is_none() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        control.attach_user(&user_access_token).await?;
        println!("trusted user channel attached");
    }

    ws_task.await??;

    Ok(())
}
