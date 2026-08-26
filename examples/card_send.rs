use larksuite_oapi_sdk_rs::card::v1::{Card, CardDocument, Div, Element, Header, Text};
use larksuite_oapi_sdk_rs::service::im::v1::CreateMessageReqBody;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let chat_id = std::env::var("CHAT_ID").expect("CHAT_ID env var required");

    let card = CardDocument::new(Card::new().header(Header::new("Release health")).element(
        Element::Div(Div::new(Text::lark_md("**All systems ready**"))),
    ))?;
    let body = CreateMessageReqBody::interactive_card(chat_id, &card)?;

    let client = LarkClient::builder(app_id, app_secret).build()?;
    let response = client
        .im()
        .message
        .create("chat_id", &body, &RequestOption::default())
        .await?;

    println!(
        "sent: message_id={}",
        response
            .data
            .as_ref()
            .and_then(|data| data.message_id.as_deref())
            .unwrap_or("")
    );
    Ok(())
}
