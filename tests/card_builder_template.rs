use larksuite_oapi_sdk_rs::LarkError;
use larksuite_oapi_sdk_rs::card::template::TemplateMessage;
use larksuite_oapi_sdk_rs::service::im::v1::{CreateMessageReqBody, ReplyMessageReqBody};

const PUBLISHED_TEMPLATE_FIXTURE: &str =
    include_str!("fixtures/card_protocol/card_builder/published_template_message.json");

#[test]
fn published_template_message_matches_the_im_template_contract() {
    let template = TemplateMessage::new("AAqi6xJ8rabcd")
        .unwrap()
        .template_version_name("1.0.0")
        .unwrap()
        .template_variables(serde_json::json!({
            "looping": [
                {"title": "和风陶韵", "image": {"img_key": "img_v3_xxx"}},
                {"title": "匠心之作", "image": {"img_key": "img_v3_yyy"}}
            ]
        }))
        .unwrap()
        .template_variable("title", serde_json::json!("Products"))
        .unwrap();

    let expected: serde_json::Value = serde_json::from_str(PUBLISHED_TEMPLATE_FIXTURE).unwrap();
    let content: serde_json::Value = serde_json::from_str(&template.to_content().unwrap()).unwrap();
    assert_eq!(content, expected);

    let create = CreateMessageReqBody::interactive_card("oc_card", &template).unwrap();
    let reply = ReplyMessageReqBody::interactive_card(&template).unwrap();
    assert_eq!(create.msg_type.as_deref(), Some("interactive"));
    assert_eq!(reply.msg_type.as_deref(), Some("interactive"));
    assert_eq!(create.content, reply.content);
}

#[test]
fn published_template_message_rejects_invalid_bindings() {
    assert!(matches!(
        TemplateMessage::new(""),
        Err(LarkError::IllegalParam(message)) if message.contains("template_id")
    ));
    assert!(matches!(
        TemplateMessage::new("tpl")
            .unwrap()
            .template_version_name(""),
        Err(LarkError::IllegalParam(message)) if message.contains("template_version_name")
    ));
    assert!(matches!(
        TemplateMessage::new("tpl")
            .unwrap()
            .template_variables(serde_json::json!(["not", "an", "object"])),
        Err(LarkError::IllegalParam(message)) if message.contains("variables")
    ));
    assert!(matches!(
        TemplateMessage::new("tpl")
            .unwrap()
            .template_variable("", serde_json::json!("value")),
        Err(LarkError::IllegalParam(message)) if message.contains("variable name")
    ));
}
