use std::io;

use larksuite_oapi_sdk_rs::card::cardkit::{CardDocument, CardEntityMessage, IdempotencyKey};
use larksuite_oapi_sdk_rs::card::v2::{Body, Card, Config, Element, Markdown};
use larksuite_oapi_sdk_rs::service::im::v1::CreateMessageReqBody;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let chat_id = std::env::var("CHAT_ID").expect("CHAT_ID env var required");

    let document = CardDocument::new(
        Card::new()
            .config(Config::new().update_multi().streaming_mode(true))
            .body(Body::new().element(Element::Markdown(
                Markdown::new("Preparing").element_id("stream_content"),
            ))),
    )?;
    let client = LarkClient::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();
    let created = client.cardkit_cards().create(&document, &option).await?;
    let card_id = created
        .data
        .and_then(|data| data.card_id)
        .ok_or_else(|| io::Error::other("CardKit create response did not include card_id"))?;
    let entity = CardEntityMessage::new(card_id.clone())?;
    let message = CreateMessageReqBody::interactive_card(chat_id, &entity)?;
    client
        .im()
        .message
        .create("chat_id", &message, &option)
        .await?;

    let mut updates = client.cardkit_cards().update_session(card_id)?;
    let mut stream = updates.content_stream("stream_content")?;
    stream
        .replace(
            "Preparing complete",
            &IdempotencyKey::new("release-health-content-1")?,
            &option,
        )
        .await?;
    Ok(())
}
