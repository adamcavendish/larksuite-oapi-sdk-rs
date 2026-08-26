mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::JsonValue;
use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::card::cardkit::{
    CardBatchAction, CardElement, CardSettings, CardTemplate, ElementInsertion,
    ElementInsertionPosition, IdempotencyKey, PartialCardElement, TemplateVariables,
    UpdateSequence,
};
use larksuite_oapi_sdk_rs::card::v2::{Config, Element, Markdown};
use larksuite_oapi_sdk_rs::req::RequestOption;

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

fn request_body(request: &str) -> serde_json::Value {
    serde_json::from_str(request.split("\r\n\r\n").nth(1).unwrap()).unwrap()
}

#[tokio::test]
async fn cardkit_session_orders_settings_element_and_batch_mutations() {
    let empty = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
    ])
    .await;
    let client = client_for(addr);
    let option = RequestOption::default();
    let mut session = client.cardkit_cards().update_session("card-1").unwrap();
    let settings = CardSettings::new().config(Config::new().update_multi());
    let element = CardElement::new(Element::Markdown(
        Markdown::new("Preparing").element_id("stream_content"),
    ));
    let insertion =
        ElementInsertion::new(ElementInsertionPosition::Append, [element.clone()]).unwrap();
    let patch = PartialCardElement::new(serde_json::json!({"content": "Updated"}).into()).unwrap();

    session
        .update_settings(
            &settings,
            &IdempotencyKey::new("settings").unwrap(),
            &option,
        )
        .await
        .unwrap();
    session
        .insert_elements(&insertion, &IdempotencyKey::new("insert").unwrap(), &option)
        .await
        .unwrap();
    session
        .replace_element(
            "stream_content",
            &element,
            &IdempotencyKey::new("replace").unwrap(),
            &option,
        )
        .await
        .unwrap();
    session
        .patch_element(
            "stream_content",
            &patch,
            &IdempotencyKey::new("patch").unwrap(),
            &option,
        )
        .await
        .unwrap();
    session
        .delete_element(
            "stream_content",
            &IdempotencyKey::new("delete").unwrap(),
            &option,
        )
        .await
        .unwrap();
    session
        .batch_update(
            [
                CardBatchAction::update_settings(settings.clone()).unwrap(),
                CardBatchAction::add_elements(insertion).unwrap(),
                CardBatchAction::delete_elements(["stream_content".to_string()]).unwrap(),
                CardBatchAction::patch_element("stream_content", patch).unwrap(),
                CardBatchAction::replace_element("stream_content", element).unwrap(),
            ],
            &IdempotencyKey::new("batch").unwrap(),
            &option,
        )
        .await
        .unwrap();
    assert_eq!(
        session.next_sequence(),
        Some(UpdateSequence::new(7).unwrap())
    );

    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 6);
    assert!(requests[0].starts_with("PATCH /open-apis/cardkit/v1/cards/card-1/settings "));
    assert!(requests[1].starts_with("POST /open-apis/cardkit/v1/cards/card-1/elements "));
    assert!(
        requests[2].starts_with("PUT /open-apis/cardkit/v1/cards/card-1/elements/stream_content ")
    );
    assert!(
        requests[3]
            .starts_with("PATCH /open-apis/cardkit/v1/cards/card-1/elements/stream_content ")
    );
    assert!(
        requests[4]
            .starts_with("DELETE /open-apis/cardkit/v1/cards/card-1/elements/stream_content ")
    );
    assert!(requests[5].starts_with("POST /open-apis/cardkit/v1/cards/card-1/batch_update "));

    let bodies: Vec<_> = requests
        .iter()
        .map(|request| request_body(request))
        .collect();
    for (index, body) in bodies.iter().enumerate() {
        assert_eq!(body["sequence"], (index + 1) as i64);
    }
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(bodies[0]["settings"].as_str().unwrap()).unwrap()
            ["config"]["update_multi"],
        true
    );
    let inserted: Vec<serde_json::Value> =
        serde_json::from_str(bodies[1]["elements"].as_str().unwrap()).unwrap();
    assert_eq!(inserted[0]["tag"], "markdown");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(bodies[3]["partial_element"].as_str().unwrap())
            .unwrap()["content"],
        "Updated"
    );
    let actions: Vec<serde_json::Value> =
        serde_json::from_str(bodies[5]["actions"].as_str().unwrap()).unwrap();
    assert_eq!(actions.len(), 5);
    assert_eq!(actions[0]["action"], "partial_update_setting");
    assert_eq!(actions[1]["action"], "add_elements");
    assert_eq!(actions[2]["action"], "delete_elements");
    assert_eq!(actions[3]["action"], "partial_update_element");
    assert_eq!(actions[4]["action"], "update_element");
    assert_eq!(
        actions[0]["params"]["settings"]["config"]["update_multi"],
        true
    );
    assert_eq!(actions[1]["params"]["elements"][0]["tag"], "markdown");
    assert_eq!(
        actions[3]["params"]["partial_element"]["content"],
        "Updated"
    );
    assert_eq!(actions[4]["params"]["element"]["tag"], "markdown");
}

#[tokio::test]
async fn cardkit_template_instances_and_message_conversion_are_typed() {
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(
            200,
            r#"{"code":0,"msg":"ok","data":{"instance_id":"instance-1"}}"#,
        ),
        http_response(200, r#"{"code":0,"msg":"ok"}"#),
    ])
    .await;
    let client = client_for(addr);
    let option = RequestOption::default();
    let template = CardTemplate::new("tpl-1").unwrap().variables(
        TemplateVariables::new(serde_json::json!({"title": "Deployment"}).into()).unwrap(),
    );

    let created = client
        .cardkit_cards()
        .create_instance(&template, &option)
        .await
        .unwrap();
    assert_eq!(
        created.data.unwrap().instance_id.as_deref(),
        Some("instance-1")
    );
    client
        .cardkit_cards()
        .update_instance(
            "instance-1",
            TemplateVariables::new(JsonValue::from(serde_json::json!({"title": "Complete"})))
                .unwrap(),
            &option,
        )
        .await
        .unwrap();
    let requests = requests.lock().unwrap();
    assert!(requests[0].starts_with("POST /open-apis/cardkit/v1/card_instances "));
    assert!(requests[1].starts_with("PUT /open-apis/cardkit/v1/card_instances/instance-1 "));
    assert_eq!(request_body(&requests[0])["template_id"], "tpl-1");
    assert_eq!(
        request_body(&requests[1])["template_variable"]["title"],
        "Complete"
    );
}

#[test]
fn cardkit_mutation_inputs_reject_invalid_shapes_before_a_request() {
    assert!(CardSettings::new().validate().is_err());
    assert!(
        CardSettings::new()
            .config(Config {
                update_multi: Some(false),
                ..Config::new()
            })
            .validate()
            .is_err()
    );
    assert!(PartialCardElement::new(serde_json::json!({}).into()).is_err());
    assert!(PartialCardElement::new(serde_json::json!({"tag": "markdown"}).into()).is_err());
    assert!(ElementInsertion::new(ElementInsertionPosition::Append, []).is_err());
    assert!(CardBatchAction::delete_elements(Vec::new()).is_err());
    assert!(CardBatchAction::update_settings(CardSettings::new()).is_err());
    assert!(CardTemplate::new("").is_err());
    assert!(TemplateVariables::new(serde_json::json!(["not", "an", "object"]).into()).is_err());

    let insertion = ElementInsertion::new(
        ElementInsertionPosition::InsertBefore,
        [CardElement::new(Element::Markdown(Markdown::new("text")))],
    )
    .unwrap();
    assert!(insertion.validate().is_err());
    assert!(CardBatchAction::add_elements(insertion).is_err());

    for invalid_id in ["1bad", "bad-id", "x/y", "too_long_element_id_1"] {
        assert!(CardBatchAction::delete_elements([invalid_id.to_string()]).is_err());
    }
    assert!(
        ElementInsertion::new(
            ElementInsertionPosition::Append,
            [CardElement::new(Element::Markdown(Markdown::new("text")))]
        )
        .unwrap()
        .target_element_id("bad-id")
        .is_err()
    );
    assert!(
        CardBatchAction::patch_element(
            "x/y",
            PartialCardElement::new(serde_json::json!({"content": "text"}).into()).unwrap()
        )
        .is_err()
    );
    assert!(
        CardBatchAction::replace_element(
            "1bad",
            CardElement::new(Element::Markdown(Markdown::new("text")))
        )
        .is_err()
    );
}

#[tokio::test]
async fn cardkit_direct_mutations_reject_invalid_element_ids_before_dispatch() {
    let (addr, _handle, requests) = mock_server_with_requests(Vec::new()).await;
    let client = client_for(addr);
    let option = RequestOption::default();
    let key = IdempotencyKey::new("key").unwrap();
    let element = CardElement::new(Element::Markdown(Markdown::new("text")));
    let patch = PartialCardElement::new(serde_json::json!({"content": "text"}).into()).unwrap();
    let mut session = client.cardkit_cards().update_session("card-1").unwrap();

    assert!(
        session
            .replace_element("x/y", &element, &key, &option)
            .await
            .is_err()
    );
    assert!(
        session
            .patch_element("bad-id", &patch, &key, &option)
            .await
            .is_err()
    );
    assert!(session.delete_element("1bad", &key, &option).await.is_err());
    assert!(session.content_stream("too_long_element_id_1").is_err());
    assert_eq!(session.next_sequence(), Some(UpdateSequence::FIRST));
    assert!(requests.lock().unwrap().is_empty());
}
