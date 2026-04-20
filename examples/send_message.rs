use larksuite_oapi_sdk_rs::service::im::v1::CreateMessageReqBody;
use larksuite_oapi_sdk_rs::{Client, RequestOption};

#[tokio::main]
async fn main() {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let chat_id = std::env::var("CHAT_ID").expect("CHAT_ID env var required");

    let client = Client::builder(&app_id, &app_secret).build();

    let option = RequestOption::default();

    let body = CreateMessageReqBody {
        receive_id: Some(chat_id),
        msg_type: Some("text".to_string()),
        content: Some(r#"{"text":"Hello from larksuite-oapi-sdk-rs!"}"#.to_string()),
        uuid: None,
    };

    let resp = client
        .im()
        .message
        .create("chat_id", &body, &option)
        .await
        .expect("send message failed");

    if resp.success() {
        println!(
            "sent: message_id={}",
            resp.data
                .as_ref()
                .and_then(|d| d.message_id.as_deref())
                .unwrap_or("")
        );
    } else {
        println!("error: {:?}", resp.code_error);
    }
}
