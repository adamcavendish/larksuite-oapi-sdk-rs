use larksuite_oapi_sdk_rs::card::{
    ActionElement, Card, CardConfig, CardHeader, DivElement, DivField, TextObject,
};

#[test]
fn test_card_to_json_basic() {
    let card = Card::new()
        .config(CardConfig::new().wide_screen_mode(true))
        .header(CardHeader::new("Test Title").template("blue"))
        .element(larksuite_oapi_sdk_rs::card::div("Hello world"))
        .element(larksuite_oapi_sdk_rs::card::hr());

    let json = card.to_json();
    assert_eq!(json["config"]["wide_screen_mode"], true);
    assert_eq!(json["header"]["title"]["content"], "Test Title");
    assert_eq!(json["header"]["template"], "blue");
    assert!(json["elements"].is_array());
    assert_eq!(json["elements"].as_array().unwrap().len(), 2);
}

#[test]
fn test_card_markdown_element() {
    let card = Card::new().element(larksuite_oapi_sdk_rs::card::md("**bold**"));
    let json = card.to_json();
    let elem = &json["elements"][0];
    assert_eq!(elem["tag"], "markdown");
    assert_eq!(elem["content"], "**bold**");
}

#[test]
fn test_card_action_button() {
    let action = ActionElement::new().action(larksuite_oapi_sdk_rs::card::button(
        "Click me",
        serde_json::json!({"action": "click"}),
    ));

    use larksuite_oapi_sdk_rs::card::Element;
    let card = Card::new().element(Element::Action(action));
    let json = card.to_json();
    let elem = &json["elements"][0];
    assert_eq!(elem["tag"], "action");
    let btn = &elem["actions"][0];
    assert_eq!(btn["tag"], "button");
    assert_eq!(btn["text"]["content"], "Click me");
    assert_eq!(btn["value"]["action"], "click");
}

#[test]
fn test_div_with_fields() {
    use larksuite_oapi_sdk_rs::card::Element;

    let div = DivElement::new()
        .field(DivField::new(TextObject::lark_md("**Key**")).short())
        .field(DivField::new(TextObject::plain("Value")));

    let card = Card::new().element(Element::Div(div));
    let json = card.to_json();
    let elem = &json["elements"][0];
    assert_eq!(elem["tag"], "div");
    assert_eq!(elem["fields"][0]["is_short"], true);
    assert_eq!(elem["fields"][1]["is_short"], false);
}

#[test]
fn test_empty_card_serializes() {
    let card = Card::new();
    let json = card.to_json();
    assert!(json.is_object());
    assert!(!json.as_object().unwrap().contains_key("elements"));
}
