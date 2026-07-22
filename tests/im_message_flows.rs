mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::LarkError;
use larksuite_oapi_sdk_rs::card::{Card, CardHeader, div};
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::im::v1::{
    CreateMessageReqBody, MessageType, PatchMessageReqBody, ReplyMessageReqBody,
    UpdateMessageReqBody,
};
use serde::{Serialize, Serializer};

struct FailingSerialize;

impl Serialize for FailingSerialize {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(serde::ser::Error::custom(
            "intentional serialization failure",
        ))
    }
}

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[test]
fn non_exhaustive_message_type_wrapper_serializes_as_string() {
    let known = CreateMessageReqBody::default().msg_type(MessageType::TEXT);
    let custom = CreateMessageReqBody::default().msg_type(MessageType::new("future_type"));

    assert_eq!(serde_json::to_value(known).unwrap()["msg_type"], "text");
    assert_eq!(
        serde_json::to_value(custom).unwrap()["msg_type"],
        "future_type"
    );
}

#[test]
fn reply_in_thread_body_serializes() {
    let body = ReplyMessageReqBody {
        msg_type: Some("text".to_string()),
        content: Some(r#"{"text":"hello"}"#.to_string()),
        reply_in_thread: Some(true),
        uuid: Some("uuid-1".to_string()),
    };

    let json = serde_json::to_value(&body).unwrap();
    assert_eq!(json["msg_type"], "text");
    assert_eq!(json["reply_in_thread"], true);
    assert_eq!(json["content"], r#"{"text":"hello"}"#);
}

#[test]
fn interactive_card_helpers_surface_sdk_json_errors() {
    let create = CreateMessageReqBody::interactive_card("oc_group", FailingSerialize).unwrap_err();
    let reply = ReplyMessageReqBody::interactive_card(FailingSerialize).unwrap_err();
    let patch = PatchMessageReqBody::interactive_card(FailingSerialize).unwrap_err();
    let update = UpdateMessageReqBody::interactive_card(FailingSerialize).unwrap_err();

    assert!(matches!(create, LarkError::Json(_)));
    assert!(matches!(reply, LarkError::Json(_)));
    assert!(matches!(patch, LarkError::Json(_)));
    assert!(matches!(update, LarkError::Json(_)));
}

#[test]
fn interactive_card_content_accepts_pre_serialized_card_json() {
    let content = r#"{"config":{"wide_screen_mode":true}}"#;
    let reply = ReplyMessageReqBody::interactive_card_content(content);
    let patch = PatchMessageReqBody::interactive_card_content(content);

    let reply_json = serde_json::to_value(&reply).unwrap();
    let patch_json = serde_json::to_value(&patch).unwrap();
    assert_eq!(reply_json["msg_type"], "interactive");
    assert_eq!(reply_json["content"], content);
    assert_eq!(patch_json["content"], content);
}

#[test]
fn card_can_be_sent_as_reply_and_patched_later() {
    let card = Card::new()
        .header(CardHeader::new("Working"))
        .element(div("Preparing answer"));

    let reply = ReplyMessageReqBody::interactive_card(&card)
        .unwrap()
        .reply_in_thread(true);
    let reply_json = serde_json::to_value(&reply).unwrap();
    assert_eq!(reply_json["msg_type"], "interactive");
    assert_eq!(reply_json["reply_in_thread"], true);

    let reply_content: serde_json::Value =
        serde_json::from_str(reply_json["content"].as_str().unwrap()).unwrap();
    assert_eq!(reply_content["header"]["title"]["content"], "Working");

    let updated = Card::new()
        .header(CardHeader::new("Done"))
        .element(div("Final answer"));
    let patch = PatchMessageReqBody::interactive_card(&updated).unwrap();
    let patch_json = serde_json::to_value(&patch).unwrap();
    let patch_content: serde_json::Value =
        serde_json::from_str(patch_json["content"].as_str().unwrap()).unwrap();
    assert_eq!(patch_content["header"]["title"]["content"], "Done");
}

#[test]
fn card_can_be_created_and_updated_as_interactive_message() {
    let card = Card::new()
        .header(CardHeader::new("Live"))
        .element(div("Initial"));
    let create = CreateMessageReqBody::interactive_card("oc_group", &card).unwrap();
    let create_json = serde_json::to_value(&create).unwrap();
    assert_eq!(create_json["receive_id"], "oc_group");
    assert_eq!(create_json["msg_type"], "interactive");

    let update = UpdateMessageReqBody::interactive_card(&card).unwrap();
    let update_json = serde_json::to_value(&update).unwrap();
    assert_eq!(update_json["msg_type"], "interactive");
    assert!(update_json["content"].as_str().unwrap().contains("Live"));
}

#[tokio::test]
async fn reply_card_then_patch_uses_typed_im_methods() {
    let reply_resp = r#"{"code":0,"msg":"ok","data":{"message_id":"om_live"}}"#;
    let patch_resp = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, reply_resp),
        http_response(200, patch_resp),
    ])
    .await;

    let client = client_for(addr);
    let card = Card::new()
        .header(CardHeader::new("Working"))
        .element(div("Preparing answer"));
    let body = ReplyMessageReqBody::interactive_card(&card)
        .unwrap()
        .reply_in_thread(true);

    let sent = client
        .im()
        .message
        .reply("om_root", &body, &RequestOption::default())
        .await
        .unwrap();
    assert_eq!(sent.data.unwrap().message_id.as_deref(), Some("om_live"));

    let final_card = Card::new()
        .header(CardHeader::new("Done"))
        .element(div("Final answer"));
    let patch = PatchMessageReqBody::interactive_card(&final_card).unwrap();
    let patched = client
        .im()
        .message
        .patch("om_live", &patch, &RequestOption::default())
        .await
        .unwrap();
    assert!(patched.success());

    let request_dump = requests.lock().unwrap().join("\n---\n");
    assert!(request_dump.contains("POST /open-apis/im/v1/messages/om_root/reply"));
    assert!(request_dump.contains("PATCH /open-apis/im/v1/messages/om_live"));
    assert!(request_dump.contains(r#""reply_in_thread":true"#));
    assert!(request_dump.contains(r#""msg_type":"interactive"#));
    assert!(request_dump.contains("Working"));
    assert!(request_dump.contains("Done"));
}
