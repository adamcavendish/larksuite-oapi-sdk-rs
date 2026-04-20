use larksuite_oapi_sdk_rs::config::Config;
use larksuite_oapi_sdk_rs::event::EventDispatcher;
use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;
use larksuite_oapi_sdk_rs::ws::WsClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");

    let config = Config::new(&app_id, &app_secret);

    let dispatcher = EventDispatcher::new("", "").on_p2_im_message_receive_v1(
        |event: P2MessageReceiveV1| async move {
            println!(
                "[ws] message: id={} content={}",
                event.message.message_id, event.message.content
            );
            Ok(())
        },
    );

    let client = WsClient::new(config, dispatcher);
    println!("starting ws client (Ctrl+C to stop)...");
    client.start().await.expect("ws client error");
}
