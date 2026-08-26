mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::card::cardkit::{
    CardDocument, CardEntityMessage, CardUpdateMetadata, IdempotencyKey, UpdateSequence,
};
use larksuite_oapi_sdk_rs::card::v2::{Body, Card, Config, Element, Markdown};
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::im::v1::CreateMessageReqBody;

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

fn streaming_document() -> CardDocument {
    CardDocument::new(
        Card::new()
            .config(Config::new().update_multi().streaming_mode(true))
            .body(Body::new().element(Element::Markdown(
                Markdown::new("Preparing").element_id("stream_content"),
            ))),
    )
    .expect("valid shared Card JSON 2.0 document")
}

#[test]
fn cardkit_document_rejects_invalid_card_and_serializes_card_json_envelopes() {
    assert!(CardDocument::new(Card::new()).is_err());

    let document = streaming_document();
    let create = document.create_request().unwrap();
    assert_eq!(create.r#type.as_deref(), Some("card_json"));
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(create.data.as_deref().unwrap()).unwrap(),
        serde_json::to_value(document.card()).unwrap(),
    );

    let metadata = CardUpdateMetadata::new(
        IdempotencyKey::new("update-1").unwrap(),
        UpdateSequence::new(7).unwrap(),
    );
    let update = document.update_request(&metadata).unwrap();
    assert_eq!(update.uuid.as_deref(), Some("update-1"));
    assert_eq!(update.sequence, Some(7));
    assert_eq!(update.card.unwrap().r#type.as_deref(), Some("card_json"));

    assert!(IdempotencyKey::new("").is_err());
    assert!(UpdateSequence::new(0).is_err());
}

#[test]
fn cardkit_entity_message_serializes_as_the_documented_im_envelope() {
    let message = CardEntityMessage::new("card-1").unwrap();
    assert_eq!(message.card_id(), "card-1");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&message.to_content().unwrap()).unwrap(),
        serde_json::json!({"type": "card", "data": {"card_id": "card-1"}}),
    );

    let request = CreateMessageReqBody::interactive_card("oc_card", &message).unwrap();
    assert_eq!(request.msg_type.as_deref(), Some("interactive"));
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(request.content.as_deref().unwrap()).unwrap(),
        serde_json::json!({"type": "card", "data": {"card_id": "card-1"}}),
    );
    assert!(CardEntityMessage::new("").is_err());
}

#[tokio::test]
async fn cardkit_helpers_create_update_and_stream_in_sequence() {
    let create_body = r#"{"code":0,"msg":"ok","data":{"card_id":"card-1"}}"#;
    let send_body = r#"{"code":0,"msg":"ok","data":{"message_id":"om_card-1"}}"#;
    let update_body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, create_body),
        http_response(200, send_body),
        http_response(200, update_body),
        http_response(200, empty_body),
        http_response(200, empty_body),
    ])
    .await;

    let client = client_for(addr);
    let document = streaming_document();
    let option = RequestOption::default();
    let created = client
        .cardkit_cards()
        .create(&document, &option)
        .await
        .unwrap();
    let card_id = created.data.unwrap().card_id.unwrap();
    let entity = CardEntityMessage::new(card_id).unwrap();
    let send = CreateMessageReqBody::interactive_card("oc_card", &entity).unwrap();
    client
        .im()
        .message
        .create("chat_id", &send, &option)
        .await
        .unwrap();

    let metadata = CardUpdateMetadata::new(
        IdempotencyKey::new("full-update").unwrap(),
        UpdateSequence::new(5).unwrap(),
    );
    let mut updates = client
        .cardkit_cards()
        .resume_update_session("card-1", metadata.sequence)
        .unwrap();
    let update = updates
        .replace_document(&document, &metadata.idempotency_key, &option)
        .await
        .unwrap();
    assert!(update.success());
    assert!(update.data.is_some());
    assert_eq!(
        updates.next_sequence(),
        Some(UpdateSequence::new(6).unwrap())
    );

    let mut stream = updates.content_stream("stream_content").unwrap();
    stream
        .replace(
            "Preparing",
            &IdempotencyKey::new("content-1").unwrap(),
            &option,
        )
        .await
        .unwrap();
    assert_eq!(
        stream.next_sequence(),
        Some(UpdateSequence::new(7).unwrap())
    );
    stream
        .replace(
            "Preparing complete",
            &IdempotencyKey::new("content-2").unwrap(),
            &option,
        )
        .await
        .unwrap();
    assert_eq!(
        stream.next_sequence(),
        Some(UpdateSequence::new(8).unwrap())
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/cardkit/v1/cards "));
    assert!(request.contains("POST /open-apis/im/v1/messages?receive_id_type=chat_id "));
    assert!(request.contains("PUT /open-apis/cardkit/v1/cards/card-1 "));
    assert!(
        request.contains("PUT /open-apis/cardkit/v1/cards/card-1/elements/stream_content/content ")
    );
    assert!(request.contains(r#""type":"card_json""#));
    assert!(request.contains(r#"\"type\":\"card\",\"data\":{\"card_id\":\"card-1\"}"#));
    assert!(request.contains(r#""uuid":"full-update""#));
    assert!(request.contains(r#""sequence":5"#));
    assert!(request.contains(r#""uuid":"content-1""#));
    assert!(request.contains(r#""content":"Preparing""#));
    assert!(request.contains(r#""sequence":6"#));
    assert!(request.contains(r#""uuid":"content-2""#));
    assert!(request.contains(r#""content":"Preparing complete""#));
    assert!(request.contains(r#""sequence":7"#));
}

#[test]
fn cardkit_stream_rejects_empty_identifiers() {
    let client = LarkClient::builder("test_app_id", "test_secret")
        .build()
        .unwrap();
    assert!(client.cardkit_cards().update_session("").is_err());
    let mut updates = client.cardkit_cards().update_session("card").unwrap();
    assert!(updates.content_stream("").is_err());
}

#[tokio::test]
async fn failed_content_update_keeps_its_sequence_for_retry() {
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(500, r#"{"code":999,"msg":"retry"}"#),
        http_response(200, r#"{"code":0,"msg":"ok"}"#),
    ])
    .await;
    let client = client_for(addr);
    let option = RequestOption::default();
    let key = IdempotencyKey::new("retry-content").unwrap();
    let mut updates = client.cardkit_cards().update_session("card-1").unwrap();
    let mut stream = updates.content_stream("stream_content").unwrap();

    assert!(stream.replace("Preparing", &key, &option).await.is_err());
    assert_eq!(
        stream.next_sequence(),
        Some(UpdateSequence::new(1).unwrap())
    );
    stream.replace("Preparing", &key, &option).await.unwrap();
    assert_eq!(
        stream.next_sequence(),
        Some(UpdateSequence::new(2).unwrap())
    );

    let request = requests.lock().unwrap().join("\n");
    assert_eq!(request.matches(r#""sequence":1"#).count(), 2);
}
