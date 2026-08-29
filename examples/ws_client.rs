use larksuite_oapi_sdk_rs::event::EventDispatcher;
use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;
use larksuite_oapi_sdk_rs::service::im::v1::CreateMessageReqBody;
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");

    let client = LarkClient::builder(&app_id, &app_secret).build()?;

    // Keep the SDK event handler short: the WebSocket client can acknowledge
    // delivery as soon as the event is queued. A worker performs outbound I/O.
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<P2MessageReceiveV1>(128);
    let worker_client = client.clone();
    let worker = tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            if event.message.message_type != "text" {
                continue;
            }

            let message_id = event.message.message_id;
            let chat_id = event.message.chat_id;
            println!(
                "[worker] message: id={message_id} content={}",
                event.message.content
            );

            let body = CreateMessageReqBody {
                receive_id: Some(chat_id),
                msg_type: Some("text".to_string()),
                content: Some(r#"{"text":"Hello from the queued worker!"}"#.to_string()),
                // Keep retries for the same inbound message idempotent.
                uuid: Some(format!("reply-{message_id}")),
            };

            match worker_client
                .im()
                .message
                .create("chat_id", &body, &RequestOption::default())
                .await
            {
                Ok(response) if response.success() => {
                    println!("[worker] reply sent");
                }
                Ok(response) => {
                    eprintln!("[worker] reply rejected: {:?}", response.code_error);
                }
                Err(error) => {
                    eprintln!("[worker] reply failed: {error}");
                }
            }
        }
    });

    let dispatcher = EventDispatcher::new("", "").on_p2_im_message_receive_v1(
        move |event: P2MessageReceiveV1| {
            let event_tx = event_tx.clone();
            async move {
                event_tx.try_send(event).map_err(|error| {
                    LarkError::Event(format!("event queue is full or closed: {error}"))
                })
            }
        },
    );

    let mut ws_client = client
        .ws_client(dispatcher)
        .on_ready(|| println!("websocket connected"))
        .on_disconnected(|| eprintln!("websocket disconnected"))
        .on_error(|error| eprintln!("websocket error: {error}"));
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
    worker.await?;

    Ok(())
}
