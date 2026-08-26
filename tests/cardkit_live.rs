use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::card::cardkit::{CardDocument, IdempotencyKey, UpdateSequence};
use larksuite_oapi_sdk_rs::card::v2::{Body, Card, Config, Element, Markdown};
use larksuite_oapi_sdk_rs::req::RequestOption;

#[tokio::test]
#[ignore = "requires a staging Feishu app with cardkit:card:write"]
async fn feishu_cardkit_full_update_accepts_empty_object_data() {
    assert_eq!(
        required_env("FEISHU_CARDKIT_LIVE"),
        "1",
        "set FEISHU_CARDKIT_LIVE=1 only for the staging CardKit check"
    );

    let client = LarkClient::builder(
        required_env("FEISHU_APP_ID"),
        required_env("FEISHU_APP_SECRET"),
    )
    .build()
    .unwrap();
    let document = CardDocument::new(Card::new().config(Config::new().update_multi()).body(
        Body::new().element(Element::Markdown(
            Markdown::new("SDK CardKit empty-data check").element_id("status"),
        )),
    ))
    .unwrap();
    let option = RequestOption::default();

    let created = client
        .cardkit_cards()
        .create(&document, &option)
        .await
        .unwrap();
    let card_id = created.data.unwrap().card_id.unwrap();
    let mut updates = client
        .cardkit_cards()
        .resume_update_session(card_id, UpdateSequence::FIRST)
        .unwrap();
    let idempotency_key = IdempotencyKey::new(format!(
        "sdk-empty-data-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    ))
    .unwrap();

    let response = updates
        .replace_document(&document, &idempotency_key, &option)
        .await
        .expect("CardKit full update should decode data: {} successfully");
    assert!(response.success());
    assert!(response.data.is_some());
    let envelope: serde_json::Value = serde_json::from_slice(&response.api_resp.raw_body).unwrap();
    assert_eq!(envelope["data"], serde_json::json!({}));
    assert_eq!(
        updates.next_sequence(),
        Some(UpdateSequence::new(2).unwrap())
    );
}

fn required_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} must be set for the live CardKit test"))
}
