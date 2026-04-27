use larksuite_oapi_sdk_rs::card::{
    ActionComponent, ActionElement, ButtonComponent, Card, CardConfig, CardHeader, ChartMdElement,
    ColumnElement, ColumnSetElement, DatePickerComponent, DivElement, DivField, Element,
    FormElement, ImgElement, MarkdownElement, MultiSelectStaticComponent, NoteElement,
    OverflowComponent, SelectOption, SelectStaticComponent, TextObject, TimePickerComponent,
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

// ── New tests for untested components ──

#[test]
fn config_enable_forward() {
    let config = CardConfig::new().enable_forward(true);
    let json = serde_json::to_value(config).unwrap();
    assert_eq!(json["enable_forward"], true);
}

#[test]
fn text_object_plain() {
    let text = TextObject::plain("hello");
    assert_eq!(text.tag, "plain_text");
    assert_eq!(text.content, "hello");
    assert!(text.lines.is_none());
}

#[test]
fn text_object_lark_md() {
    let text = TextObject::lark_md("**bold**");
    assert_eq!(text.tag, "lark_md");
    assert_eq!(text.content, "**bold**");
}

#[test]
fn text_object_lines() {
    let text = TextObject::plain("multi").lines(3);
    assert_eq!(text.lines, Some(3));
}

#[test]
fn div_with_text_and_extra() {
    let div = DivElement::new().text(TextObject::plain("content"));
    let json = serde_json::to_value(Element::Div(div)).unwrap();
    assert_eq!(json["text"]["content"], "content");
}

#[test]
fn img_element() {
    let img = ImgElement::new("img_key_abc").alt(TextObject::plain("alt text"));
    let json = serde_json::to_value(Element::Img(img)).unwrap();
    assert_eq!(json["tag"], "img");
    assert_eq!(json["img_key"], "img_key_abc");
    assert_eq!(json["alt"]["content"], "alt text");
}

#[test]
fn note_element_text_and_img() {
    let note = NoteElement::new()
        .text(TextObject::plain("note text"))
        .img(ImgElement::new("note_img"));
    let json = serde_json::to_value(Element::Note(note)).unwrap();
    assert_eq!(json["tag"], "note");
    assert_eq!(json["elements"].as_array().unwrap().len(), 2);
}

#[test]
fn markdown_element_with_alignment() {
    let md = MarkdownElement::new("# Title").text_align("center");
    let json = serde_json::to_value(Element::Markdown(md)).unwrap();
    assert_eq!(json["content"], "# Title");
    assert_eq!(json["text_align"], "center");
}

#[test]
fn column_set_element() {
    let col1 = ColumnElement::new()
        .width("weighted")
        .element(larksuite_oapi_sdk_rs::card::md("Col 1"));
    let col2 = ColumnElement::new().element(larksuite_oapi_sdk_rs::card::md("Col 2"));
    let cs = ColumnSetElement::new()
        .flex_mode("bisect")
        .column(col1)
        .column(col2);
    let json = serde_json::to_value(Element::ColumnSet(cs)).unwrap();
    assert_eq!(json["tag"], "column_set");
    assert_eq!(json["flex_mode"], "bisect");
    assert_eq!(json["columns"].as_array().unwrap().len(), 2);
    assert_eq!(json["columns"][0]["width"], "weighted");
}

#[test]
fn form_element() {
    let form = FormElement::new("my_form").element(larksuite_oapi_sdk_rs::card::md("Form content"));
    let json = serde_json::to_value(Element::Form(form)).unwrap();
    assert_eq!(json["tag"], "form");
    assert_eq!(json["name"], "my_form");
    assert_eq!(json["elements"].as_array().unwrap().len(), 1);
}

#[test]
fn chart_md_element() {
    let chart = ChartMdElement::new(serde_json::json!({"type": "bar"}))
        .color_theme("dark")
        .height("300px");
    let json = serde_json::to_value(Element::ChartMd(chart)).unwrap();
    assert_eq!(json["tag"], "chart_md");
    assert_eq!(json["chart_spec"]["type"], "bar");
    assert_eq!(json["color_theme"], "dark");
    assert_eq!(json["height"], "300px");
}

#[test]
fn button_component_with_url_and_type() {
    let btn = ButtonComponent::new(TextObject::plain("Open"))
        .button_type("primary")
        .value(serde_json::json!({"k": "v"}))
        .url("https://example.com");
    let json = serde_json::to_value(ActionComponent::Button(btn)).unwrap();
    assert_eq!(json["text"]["content"], "Open");
    assert_eq!(json["type"], "primary");
    assert_eq!(json["url"], "https://example.com");
    assert_eq!(json["value"]["k"], "v");
}

#[test]
fn select_static_component() {
    let select = SelectStaticComponent {
        placeholder: TextObject::plain("Choose one"),
        options: vec![
            SelectOption {
                text: TextObject::plain("Option A"),
                value: "a".to_string(),
            },
            SelectOption {
                text: TextObject::plain("Option B"),
                value: "b".to_string(),
            },
        ],
        value: None,
    };
    let action = ActionElement::new().action(ActionComponent::SelectStatic(select));
    let json = serde_json::to_value(Element::Action(action)).unwrap();
    assert_eq!(json["actions"][0]["tag"], "select_static");
    assert_eq!(json["actions"][0]["options"].as_array().unwrap().len(), 2);
}

#[test]
fn multi_select_static_component() {
    let ms = MultiSelectStaticComponent::new(TextObject::plain("Select many"))
        .option(SelectOption {
            text: TextObject::plain("A"),
            value: "a".to_string(),
        })
        .option(SelectOption {
            text: TextObject::plain("B"),
            value: "b".to_string(),
        });
    let json = serde_json::to_value(ActionComponent::MultiSelectStatic(ms)).unwrap();
    assert_eq!(json["tag"], "multi_select_static");
    assert_eq!(json["options"].as_array().unwrap().len(), 2);
}

#[test]
fn date_picker_component() {
    let dp = DatePickerComponent {
        placeholder: Some(TextObject::plain("Pick date")),
        initial_date: Some("2026-01-01".to_string()),
        value: None,
    };
    let json = serde_json::to_value(ActionComponent::DatePicker(dp)).unwrap();
    assert_eq!(json["tag"], "date_picker");
    assert_eq!(json["initial_date"], "2026-01-01");
}

#[test]
fn time_picker_component() {
    let tp = TimePickerComponent::new()
        .placeholder(TextObject::plain("Pick time"))
        .initial_time("09:30");
    let json = serde_json::to_value(ActionComponent::TimePicker(tp)).unwrap();
    assert_eq!(json["tag"], "time_picker");
    assert_eq!(json["initial_time"], "09:30");
}

#[test]
fn overflow_component() {
    let overflow = OverflowComponent {
        options: vec![SelectOption {
            text: TextObject::plain("More"),
            value: "more".to_string(),
        }],
        value: None,
    };
    let json = serde_json::to_value(ActionComponent::Overflow(overflow)).unwrap();
    assert_eq!(json["tag"], "overflow");
    assert_eq!(json["options"].as_array().unwrap().len(), 1);
}

#[test]
fn action_element_layout() {
    let action = ActionElement {
        actions: vec![],
        layout: Some("flow".to_string()),
    };
    let json = serde_json::to_value(Element::Action(action)).unwrap();
    assert_eq!(json["layout"], "flow");
}

#[test]
fn card_serde_roundtrip() {
    let card = Card::new()
        .config(
            CardConfig::new()
                .wide_screen_mode(true)
                .enable_forward(false),
        )
        .header(CardHeader::new("Title").template("green"))
        .element(larksuite_oapi_sdk_rs::card::div("content"))
        .element(larksuite_oapi_sdk_rs::card::hr())
        .element(larksuite_oapi_sdk_rs::card::md("**bold**"));

    let json_str = serde_json::to_string(&card).unwrap();
    let deserialized: Card = serde_json::from_str(&json_str).unwrap();
    assert_eq!(deserialized.elements.len(), 3);
    assert!(deserialized.config.unwrap().wide_screen_mode.unwrap());
}

#[test]
fn test_text_object_i18n() {
    use larksuite_oapi_sdk_rs::card::TextI18n;

    let text = TextObject::plain("Hello").i18n(TextI18n {
        zh_cn: Some("你好".to_string()),
        en_us: Some("Hello".to_string()),
        ko_kr: Some("안녕하세요".to_string()),
        ..Default::default()
    });

    let json = serde_json::to_value(&text).unwrap();
    assert_eq!(json["content"], "Hello");
    assert_eq!(json["i18n"]["zh_cn"], "你好");
    assert_eq!(json["i18n"]["en_us"], "Hello");
    assert_eq!(json["i18n"]["ko_kr"], "안녕하세요");
    assert!(json["i18n"].get("ja_jp").is_none());
}

#[test]
fn test_template_card_in_callback_card() {
    use larksuite_oapi_sdk_rs::event::{CallbackCard, TemplateCard};

    let card = CallbackCard::template(TemplateCard {
        template_id: Some("tpl_abc123".to_string()),
        template_variable: Some(
            [("name".to_string(), serde_json::json!("Alice"))]
                .into_iter()
                .collect(),
        ),
        template_version_name: Some("1.0.0".to_string()),
    });

    let json = serde_json::to_value(&card).unwrap();
    assert_eq!(json["type"], "template");
    assert_eq!(json["data"]["template_id"], "tpl_abc123");
    assert_eq!(json["data"]["template_variable"]["name"], "Alice");
    assert_eq!(json["data"]["template_version_name"], "1.0.0");
}

#[test]
fn template_color_constants() {
    use larksuite_oapi_sdk_rs::card::*;
    assert_eq!(TEMPLATE_BLUE, "blue");
    assert_eq!(TEMPLATE_WATHET, "wathet");
    assert_eq!(TEMPLATE_TURQUOISE, "turquoise");
    assert_eq!(TEMPLATE_GREEN, "green");
    assert_eq!(TEMPLATE_YELLOW, "yellow");
    assert_eq!(TEMPLATE_ORANGE, "orange");
    assert_eq!(TEMPLATE_RED, "red");
    assert_eq!(TEMPLATE_CARMINE, "carmine");
    assert_eq!(TEMPLATE_VIOLET, "violet");
    assert_eq!(TEMPLATE_PURPLE, "purple");
    assert_eq!(TEMPLATE_INDIGO, "indigo");
    assert_eq!(TEMPLATE_GREY, "grey");
}

#[test]
fn message_card_builder() {
    use larksuite_oapi_sdk_rs::card::*;

    let card = MessageCard::new()
        .config(
            MessageCardConfig::new()
                .wide_screen_mode(true)
                .enable_forward(false),
        )
        .header(MessageCardHeader::new("Hello").template(TEMPLATE_BLUE))
        .element(
            serde_json::json!({"tag": "div", "text": {"tag": "plain_text", "content": "world"}}),
        )
        .card_link(MessageCardURL::new("https://example.com"));

    let json = card.to_json();
    assert_eq!(json["config"]["wide_screen_mode"], true);
    assert_eq!(json["config"]["enable_forward"], false);
    assert_eq!(json["header"]["template"], "blue");
    assert_eq!(json["header"]["title"]["tag"], "plain_text");
    assert_eq!(json["header"]["title"]["content"], "Hello");
    assert_eq!(json["elements"][0]["tag"], "div");
    assert_eq!(json["card_link"]["url"], "https://example.com");
}

#[test]
fn message_card_serde_roundtrip() {
    use larksuite_oapi_sdk_rs::card::*;

    let card = MessageCard::new()
        .header(MessageCardHeader::new("Test"))
        .config(MessageCardConfig::new().update_multi(true));

    let json_str = serde_json::to_string(&card).unwrap();
    let deserialized: MessageCard = serde_json::from_str(&json_str).unwrap();
    assert_eq!(deserialized.header.unwrap().title.unwrap().content, "Test");
    assert_eq!(deserialized.config.unwrap().update_multi, Some(true));
}

#[test]
fn message_card_action_select_menus() {
    use larksuite_oapi_sdk_rs::card::*;

    let option_a = MessageCardEmbedSelectOption::new()
        .value("a")
        .text(MessageCardPlainText::new("Option A"));
    let option_b = MessageCardEmbedSelectOption::new()
        .value("b")
        .text(MessageCardPlainText::new("Option B"))
        .url("https://example.com/b");

    let static_menu = MessageCardEmbedSelectMenuStatic::new().base(
        MessageCardEmbedSelectMenuBase::new()
            .placeholder(MessageCardPlainText::new("Choose one"))
            .initial_option("a")
            .option(option_a.clone())
            .option(option_b.clone())
            .value(serde_json::json!({"scope": "single"})),
    );

    let multi_menu = MessageCardEmbedSelectMenuMulti::new().base(
        MessageCardEmbedSelectMenuBase::new()
            .placeholder(MessageCardPlainText::new("Choose many"))
            .initial_options(vec!["a".to_string(), "b".to_string()])
            .options(vec![option_a, option_b]),
    );

    let person_menu = MessageCardEmbedSelectMenuPerson::new().base(
        MessageCardEmbedSelectMenuBase::new()
            .placeholder(MessageCardPlainText::new("Choose person"))
            .confirm(
                MessageCardActionConfirm::new()
                    .title(MessageCardPlainText::new("Confirm"))
                    .text(MessageCardPlainText::new("Continue?")),
            ),
    );

    let action = MessageCardAction::new()
        .layout(MessageCardActionLayout::Flow)
        .action(static_menu)
        .action(multi_menu)
        .action(person_menu);
    let card = MessageCard::new().element(action);
    let json = card.to_json();
    let element = &json["elements"][0];

    assert_eq!(element["tag"], "action");
    assert_eq!(element["layout"], "flow");
    assert_eq!(element["actions"][0]["tag"], "select_static");
    assert_eq!(
        element["actions"][0]["placeholder"]["content"],
        "Choose one"
    );
    assert_eq!(element["actions"][0]["initial_option"], "a");
    assert_eq!(
        element["actions"][0]["options"][1]["url"],
        "https://example.com/b"
    );
    assert_eq!(element["actions"][0]["value"]["scope"], "single");
    assert_eq!(element["actions"][1]["tag"], "multi_select_static");
    assert_eq!(element["actions"][1]["initial_options"][1], "b");
    assert_eq!(element["actions"][2]["tag"], "select_person");
    assert_eq!(
        element["actions"][2]["confirm"]["title"]["content"],
        "Confirm"
    );

    let decoded: MessageCardAction = serde_json::from_value(element.clone()).unwrap();
    assert_eq!(decoded.layout, Some(MessageCardActionLayout::Flow));
    assert_eq!(decoded.actions.len(), 3);
}
