use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use larksuite_oapi_sdk_rs::card::v1::{
    Card as CardV1, CardStyle, Chart as ChartV1, ChartColorTheme, Config as ConfigV1, CustomColor,
    CustomTextSize, Div as DivV1, Element as ElementV1, Fallback, FallbackCondition,
    Header as HeaderV1, HeaderIcon, HeaderTag, HeaderTagColor, HeaderText, Image as ImageV1,
    ImageScale, Locale, Markdown as MarkdownV1, MultiUrl, StandardIcon, Text as TextV1, WidthMode,
};
use larksuite_oapi_sdk_rs::card::{
    ActionComponent, ActionElement, ButtonComponent, ButtonType, Card, CardConfig, CardHeader,
    Element, ImageMode, ImgElement, TemplateColor, TextObject,
};
use larksuite_oapi_sdk_rs::event::{CallbackCard, CardActionTriggerRequest};
use serde::Deserialize;
use serde_json::Value;

const MANIFEST: &str = include_str!("fixtures/card_protocol/card_json_v1.json");
const FIXTURE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/card_protocol");

const REQUIRED_SURFACES: &[&str] = &[
    "card",
    "config",
    "card_link",
    "header",
    "i18n_elements",
    "fallback",
    "div",
    "markdown",
    "img",
    "img_combination",
    "person",
    "person_list",
    "chart",
    "table",
    "hr",
    "note",
    "column_set",
    "column",
    "collapsible_panel",
    "interactive_container",
    "form",
    "action",
    "button",
    "overflow",
    "select_static",
    "multi_select_static",
    "select_person",
    "multi_select_person",
    "date_picker",
    "picker_time",
    "picker_datetime",
    "input",
    "select_img",
    "checker",
    "open_url",
    "callback",
    "card.action.trigger",
];

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    protocol: String,
    version: String,
    sources: Sources,
    surfaces: Vec<Surface>,
    fixtures: Vec<Fixture>,
}

#[derive(Debug, Deserialize)]
struct Sources {
    official_docs: OfficialDocs,
    go_sdk: ImplementationSource,
    lark_cli: ImplementationSource,
}

#[derive(Debug, Deserialize)]
struct OfficialDocs {
    role: String,
    accessed_on: String,
    urls: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ImplementationSource {
    role: String,
    repository: String,
    revision: String,
    artifacts: Vec<ReferenceArtifact>,
}

#[derive(Debug, Deserialize)]
struct ReferenceArtifact {
    path: String,
    required_snippets: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Surface {
    id: String,
    kind: String,
    wire_tag: Option<String>,
    status: String,
    sources: Vec<String>,
    remediation: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Fixture {
    id: String,
    path: String,
    kind: String,
    expected_tags: Vec<String>,
}

fn manifest() -> Manifest {
    serde_json::from_str(MANIFEST).expect("Card JSON 1.0 protocol manifest must be valid JSON")
}

fn fixture(path: &str) -> Value {
    let full_path = Path::new(FIXTURE_ROOT).join(path);
    serde_json::from_str(
        &fs::read_to_string(&full_path)
            .unwrap_or_else(|error| panic!("read fixture {}: {error}", full_path.display())),
    )
    .unwrap_or_else(|error| panic!("parse fixture {}: {error}", full_path.display()))
}

fn assert_reference_revision(environment: &str, expected_revision: &str) {
    let Ok(directory) = env::var(environment) else {
        return;
    };
    let output = Command::new("git")
        .args(["-C", &directory, "rev-parse", "HEAD"])
        .output()
        .unwrap_or_else(|error| panic!("run git for {environment}: {error}"));
    assert!(
        output.status.success(),
        "resolve {environment}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("git revision is UTF-8")
            .trim(),
        expected_revision,
        "{environment} must match the checked-in protocol inventory"
    );
}

fn assert_reference_artifacts(environment: &str, source: &ImplementationSource) {
    let Ok(directory) = env::var(environment) else {
        return;
    };
    for artifact in &source.artifacts {
        let relative_path = Path::new(&artifact.path);
        assert!(
            relative_path.is_relative()
                && !relative_path
                    .components()
                    .any(|component| matches!(component, std::path::Component::ParentDir)),
            "{environment} contains an unsafe reference artifact path {}",
            artifact.path
        );
        let full_path = Path::new(&directory).join(relative_path);
        let contents = fs::read_to_string(&full_path).unwrap_or_else(|error| {
            panic!("read reference artifact {}: {error}", full_path.display())
        });
        for snippet in &artifact.required_snippets {
            assert!(
                contents.contains(snippet),
                "{environment} artifact {} is missing required evidence {snippet:?}",
                artifact.path
            );
        }
    }
}

#[test]
fn card_json_v1_inventory_is_complete_and_traceable() {
    let manifest = manifest();
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.protocol, "card-json");
    assert_eq!(manifest.version, "1.0");

    assert_eq!(manifest.sources.official_docs.role, "normative");
    assert_eq!(manifest.sources.official_docs.accessed_on, "2026-08-24");
    assert!(manifest.sources.official_docs.urls.len() >= 4);
    assert!(
        manifest
            .sources
            .official_docs
            .urls
            .iter()
            .all(|url| url.starts_with("https://open.larksuite.com/"))
    );
    for source in [&manifest.sources.go_sdk, &manifest.sources.lark_cli] {
        assert_eq!(source.role, "implementation_cross_check");
        assert!(
            source
                .repository
                .starts_with("https://github.com/larksuite/")
        );
        assert_eq!(source.revision.len(), 40);
        assert!(source.revision.chars().all(|ch| ch.is_ascii_hexdigit()));
        assert!(!source.artifacts.is_empty());
        for artifact in &source.artifacts {
            assert!(!artifact.path.is_empty());
            assert!(!artifact.required_snippets.is_empty());
        }
    }
    assert_reference_revision(
        "CARD_PROTOCOL_GO_SDK_DIR",
        &manifest.sources.go_sdk.revision,
    );
    assert_reference_revision("CARD_PROTOCOL_CLI_DIR", &manifest.sources.lark_cli.revision);
    assert_reference_artifacts("CARD_PROTOCOL_GO_SDK_DIR", &manifest.sources.go_sdk);
    assert_reference_artifacts("CARD_PROTOCOL_CLI_DIR", &manifest.sources.lark_cli);

    let ids: BTreeSet<_> = manifest
        .surfaces
        .iter()
        .map(|surface| surface.id.as_str())
        .collect();
    let expected: BTreeSet<_> = REQUIRED_SURFACES.iter().copied().collect();
    assert_eq!(
        ids, expected,
        "inventory must classify every Card JSON 1.0 surface"
    );

    for surface in &manifest.surfaces {
        assert_eq!(
            surface.status, "implemented",
            "{} remains unimplemented in the Card JSON 1.0 inventory",
            surface.id
        );
        assert!(
            matches!(
                surface.status.as_str(),
                "implemented" | "partial" | "missing" | "known_divergence" | "under_investigation"
            ),
            "{} has an unknown status {}",
            surface.id,
            surface.status
        );
        assert!(!surface.kind.is_empty(), "{} is missing a kind", surface.id);
        assert!(!surface.sources.is_empty(), "{} has no sources", surface.id);
        assert!(
            surface.sources.iter().all(|source| {
                matches!(source.as_str(), "official_docs" | "go_sdk" | "lark_cli")
            })
        );
        if surface.status == "known_divergence" || surface.status == "under_investigation" {
            assert!(
                surface
                    .remediation
                    .as_deref()
                    .is_some_and(|remediation| !remediation.is_empty()),
                "{} needs an explicit remediation",
                surface.id
            );
        }
        if let Some(tag) = &surface.wire_tag {
            assert!(!tag.is_empty(), "{} has an empty wire tag", surface.id);
        }
    }
}

#[test]
fn card_json_v1_fixtures_are_complete_json_examples() {
    let manifest = manifest();
    let ids: BTreeSet<_> = manifest
        .fixtures
        .iter()
        .map(|fixture| fixture.id.as_str())
        .collect();
    assert_eq!(
        ids.len(),
        manifest.fixtures.len(),
        "fixture IDs must be unique"
    );

    for entry in &manifest.fixtures {
        assert!(entry.path.starts_with("card_json_v1/"));
        let value = fixture(&entry.path);
        assert!(value.is_object(), "{} must be a JSON object", entry.id);
        match entry.kind.as_str() {
            "outbound_card" => {
                let tags: Vec<_> = value["elements"]
                    .as_array()
                    .expect("outbound card fixture must have elements")
                    .iter()
                    .map(|element| element["tag"].as_str().expect("element tag"))
                    .collect();
                assert_eq!(tags, entry.expected_tags);
            }
            "outbound_element" => {
                assert_eq!(value["tag"], entry.expected_tags[0]);
                assert_eq!(value["actions"][0]["tag"], entry.expected_tags[1]);
            }
            "callback_payload" => assert!(entry.expected_tags.is_empty()),
            other => panic!("unknown fixture kind {other}"),
        }
    }
}

#[test]
fn current_card_builder_matches_baseline_protocol_fixtures() {
    let card = Card::new()
        .config(CardConfig::new().enable_forward(true).update_multi(true))
        .header(CardHeader::new("Protocol baseline").template(TemplateColor::Default))
        .element(Element::Div(
            larksuite_oapi_sdk_rs::card::DivElement::new()
                .text(TextObject::plain("A display block")),
        ))
        .element(Element::Markdown(
            larksuite_oapi_sdk_rs::card::MarkdownElement::new("**A markdown block**"),
        ))
        .element(larksuite_oapi_sdk_rs::card::hr())
        .element(Element::Img(
            ImgElement::new("img_protocol_baseline")
                .alt(TextObject::plain("Protocol image"))
                .mode(ImageMode::FitHorizontal),
        ));
    assert_eq!(
        card.to_json(),
        fixture("card_json_v1/basic_card.json").into()
    );

    let action = ActionElement::new().action(ActionComponent::Button(
        ButtonComponent::new(TextObject::plain("Approve"))
            .button_type(ButtonType::Primary)
            .value(serde_json::json!({"operation": "approve"}).into()),
    ));
    assert_eq!(
        serde_json::to_value(Element::Action(action)).expect("serialize button action"),
        fixture("card_json_v1/callback_button.json")
    );
}

#[test]
fn modern_v1_root_matches_chart_fixture() {
    let chart = ChartV1::new(
        serde_json::json!({
            "type": "line",
            "data": {"values": [{"time": "09:00", "value": 92}, {"time": "10:00", "value": 97}]},
            "xField": "time",
            "yField": "value"
        })
        .into(),
    )
    .aspect_ratio("16:9")
    .color_theme(ChartColorTheme::Brand)
    .preview(true)
    .height("320px");
    let card = CardV1::new()
        .config(
            ConfigV1::new()
                .enable_forward(true)
                .update_multi(true)
                .width_mode(WidthMode::Fill)
                .use_custom_translation(true)
                .enable_forward_interaction(true)
                .style(
                    CardStyle::new()
                        .text_size(
                            "cus-0",
                            CustomTextSize::new()
                                .default("medium")
                                .pc("medium")
                                .mobile("large"),
                        )
                        .color(
                            "cus-0",
                            CustomColor::new("rgba(5,157,178,0.52)", "rgba(78,23,108,0.49)"),
                        ),
                ),
        )
        .card_link(
            MultiUrl::new("https://example.com/card").pc_url("https://example.com/desktop-card"),
        )
        .header(
            HeaderV1::new("Release health")
                .title_text(HeaderText::plain("Release health").i18n(BTreeMap::from([
                    (Locale::EnUs, "Release health".to_string()),
                    (Locale::ZhCn, "发布健康度".to_string()),
                ])))
                .subtitle("Current deployment")
                .template(TemplateColor::Wathet)
                .icon(HeaderIcon::StandardIcon(
                    StandardIcon::new("approval_colorful").color(HeaderTagColor::Blue),
                ))
                .text_tag(HeaderTag::new("Stable", HeaderTagColor::Green)),
        )
        .element(ElementV1::Div(DivV1::new(
            TextV1::plain("All checks are passing.").i18n_content(BTreeMap::from([(
                Locale::ZhCn,
                "所有检查均已通过。".to_string(),
            )])),
        )))
        .element(ElementV1::Markdown(MarkdownV1::new(
            "**Traffic is stable**",
        )))
        .element(ElementV1::Hr)
        .element(ElementV1::Img(
            ImageV1::new("img_release_health", TextV1::plain("Release graph"))
                .i18n_img_key(BTreeMap::from([(
                    Locale::ZhCn,
                    "img_release_health_zh".to_string(),
                )]))
                .scale_type(ImageScale::CropTop),
        ))
        .element(ElementV1::Chart(chart))
        .i18n_elements(BTreeMap::from([(
            Locale::ZhCn,
            vec![ElementV1::Markdown(MarkdownV1::new("**流量稳定**"))],
        )]))
        .fallback(
            Fallback::new()
                .trigger_condition(FallbackCondition::MinClientVersion("v7.9".into()))
                .trigger_condition(FallbackCondition::ElementTags(vec![
                    "chart".into(),
                    "table".into(),
                ])),
        );

    assert_eq!(
        card.to_json(),
        fixture("card_json_v1/modern_v1_chart_card.json").into()
    );
}

#[test]
fn card_action_trigger_fixture_matches_typed_callback_input() {
    let payload = fixture("card_json_v1/card_action_trigger.json");
    let request: CardActionTriggerRequest =
        serde_json::from_value(payload).expect("deserialize card.action.trigger fixture");
    assert_eq!(request.token, "card_action_token");
    assert_eq!(request.operator.expect("operator").open_id, "ou_operator");
    let action = request.action.expect("action");
    assert_eq!(action.tag, "button");
    assert_eq!(action.value["operation"].as_str(), Some("approve"));
    assert_eq!(action.form_value["reason"].as_str(), Some("approved"));
}

#[test]
fn card_action_trigger_preserves_string_callback_values() {
    let request: CardActionTriggerRequest = serde_json::from_value(serde_json::json!({
        "token": "token",
        "action": {"tag": "button", "value": "approve"}
    }))
    .expect("deserialize string callback value");
    let action = request.action.expect("action");
    assert!(action.value.is_empty());
    assert_eq!(
        action.raw_value.expect("raw string value").as_str(),
        Some("approve")
    );
}

#[test]
fn card_action_callback_can_return_typed_raw_v1_card() {
    let raw = CallbackCard::raw(
        CardV1::new().element(ElementV1::Div(DivV1::new(TextV1::plain("Updated")))),
    )
    .expect("serialize typed Card JSON 1.0 callback response");
    let value = serde_json::to_value(raw).expect("serialize callback card");
    assert_eq!(value["type"], "raw");
    assert_eq!(value["data"]["elements"][0]["tag"], "div");
}

#[test]
fn modern_v1_serializes_every_documented_component_tag() {
    use larksuite_oapi_sdk_rs::card::v1 as card;

    let text = card::Text::plain("text");
    let elements = vec![
        ("div", card::Element::Div(card::Div::new(text.clone()))),
        (
            "markdown",
            card::Element::Markdown(card::Markdown::new("markdown")),
        ),
        (
            "img",
            card::Element::Img(card::Image::new("img", text.clone())),
        ),
        (
            "img_combination",
            card::Element::ImgCombination(card::ImageCombination::new()),
        ),
        (
            "person",
            card::Element::Person(card::Person::new("ou_user")),
        ),
        (
            "person_list",
            card::Element::PersonList(card::PersonList::new()),
        ),
        (
            "chart",
            card::Element::Chart(card::Chart::new(serde_json::json!({}).into())),
        ),
        ("table", card::Element::Table(card::Table::new())),
        ("hr", card::Element::Hr),
        ("note", card::Element::Note(card::Note::new())),
        (
            "column_set",
            card::Element::ColumnSet(card::ColumnSet::new()),
        ),
        (
            "collapsible_panel",
            card::Element::CollapsiblePanel(card::CollapsiblePanel::new(
                card::CollapsiblePanelHeader::new(text.clone()),
            )),
        ),
        (
            "interactive_container",
            card::Element::InteractiveContainer(card::InteractiveContainer::new()),
        ),
        ("form", card::Element::Form(card::Form::new("form"))),
        ("action", card::Element::Action(card::Action::new())),
        (
            "button",
            card::Element::Button(card::Button::new(text.clone())),
        ),
        ("overflow", card::Element::Overflow(card::Overflow::new())),
        (
            "select_static",
            card::Element::SelectStatic(card::StaticSelect::new()),
        ),
        (
            "multi_select_static",
            card::Element::MultiSelectStatic(card::MultiStaticSelect::default()),
        ),
        (
            "select_person",
            card::Element::SelectPerson(card::PersonSelect::default()),
        ),
        (
            "multi_select_person",
            card::Element::MultiSelectPerson(card::MultiPersonSelect::default()),
        ),
        (
            "date_picker",
            card::Element::DatePicker(card::DatePicker::default()),
        ),
        (
            "picker_time",
            card::Element::PickerTime(card::TimePicker::default()),
        ),
        (
            "picker_datetime",
            card::Element::PickerDatetime(card::DatetimePicker::default()),
        ),
        ("input", card::Element::Input(card::Input::new())),
        (
            "select_img",
            card::Element::SelectImg(card::ImageSelect::new()),
        ),
        ("checker", card::Element::Checker(card::Checker::default())),
    ];

    for (expected_tag, element) in elements {
        let value = serde_json::to_value(element).expect("serialize Card JSON 1.0 element");
        assert_eq!(value["tag"], expected_tag, "wrong tag for {expected_tag}");
    }
}

#[test]
fn modern_v1_builders_serialize_optional_protocol_shapes() {
    use larksuite_oapi_sdk_rs::card::v1 as card;

    let url = card::MultiUrl::new("https://example.com")
        .android_url("https://example.com/android")
        .ios_url("https://example.com/ios")
        .pc_url("https://example.com/pc");
    let icon = card::HeaderIcon::standard("approval_colorful");
    let custom_icon = card::HeaderIcon::custom("img_icon");
    let standard_icon = card::StandardIcon::new("approval_colorful")
        .color(card::HeaderTagColor::Green)
        .size("16px");
    let header = card::Header::new("Title")
        .subtitle("Subtitle")
        .title_text(card::HeaderText::lark_md("**Title**"))
        .subtitle_text(
            card::HeaderText::plain("Subtitle")
                .i18n(BTreeMap::from([(card::Locale::ZhCn, "副标题".to_string())])),
        )
        .template(TemplateColor::Blue)
        .icon(custom_icon.clone())
        .i18n_text_tag_list(BTreeMap::from([(
            card::Locale::ZhCn,
            vec![card::HeaderTag::new("标签", card::HeaderTagColor::Green)],
        )]));
    let text = card::Text::lark_md("**text**")
        .i18n_content(BTreeMap::from([(card::Locale::ZhCn, "文本".to_string())]))
        .text_size("heading-1")
        .text_color(card::Color::Default)
        .text_align(card::TextAlign::Center)
        .lines(2);
    let div = card::Div::new(text.clone())
        .field(card::DivField::new(card::Text::plain("field")).short(true))
        .extra(card::Extra::Button(card::Button::new(card::Text::plain(
            "Open",
        ))))
        .icon(icon.clone());
    let markdown = card::Markdown::new("[link](https://example.com)")
        .text_size("heading-2")
        .text_align(card::TextAlign::Right)
        .icon(icon.clone())
        .href(BTreeMap::from([("link".to_string(), url.clone())]));
    let image = card::Image::new("img_key", card::Text::plain("Image"))
        .title(card::Text::plain("Image title"))
        .i18n_img_key(BTreeMap::from([(
            card::Locale::ZhCn,
            "img_key_zh".to_string(),
        )]))
        .scale_type(card::ImageScale::CropTop)
        .custom_width(320)
        .compact_width(true)
        .preview(false);
    let combination = card::ImageCombination::new()
        .image(card::ImageReference::new("img_a"))
        .image(card::ImageReference::new("img_b"))
        .combination_mode(card::ImageCombinationMode::Double);
    let chart = card::Chart::new(serde_json::json!({"type": "bar"}).into())
        .aspect_ratio("4:3")
        .color_theme(card::ChartColorTheme::Rainbow)
        .preview(false)
        .height("240px");
    let table = card::Table::new()
        .column(card::TableColumn::new("status").data_type(card::TableDataType::Text))
        .row(BTreeMap::from([(
            "status".to_string(),
            serde_json::json!("ready").into(),
        )]));
    let note = card::Note::new()
        .element(card::NoteElement::Text(card::Text::plain("Note")))
        .element(card::NoteElement::Image(image.clone()));
    let column = card::Column::new().element(card::Element::Div(div.clone()));
    let column_set = card::ColumnSet::new().column(column);
    let container = card::InteractiveContainer::new()
        .element(card::Element::Markdown(markdown.clone()))
        .action(card::OpenUrl::new(url.clone()));
    let panel = card::CollapsiblePanel::new(card::CollapsiblePanelHeader::new(card::Text::plain(
        "Panel",
    )))
    .element(card::Element::Div(div.clone()));
    let form =
        card::Form::new("form").element(card::Element::Input(card::Input::new().name("reason")));
    let confirm = card::Confirm::new(card::Text::plain("Confirm"), card::Text::plain("Continue?"));
    let button = card::Button::new(card::Text::plain("Submit"))
        .button_type(card::ButtonType::Primary)
        .value(serde_json::json!({"submit": true}).into())
        .url("https://example.com")
        .multi_url(url.clone())
        .confirm(confirm.clone())
        .name("submit");
    let option = card::SelectOption::new(card::Text::plain("Option"), "option");
    let overflow = card::Overflow::new().option(option.clone());
    let select = card::StaticSelect::new().option(option.clone());
    let image_select = card::ImageSelect::new().option(card::ImageSelectOption::new("img_a"));
    let action = card::Action::new().action(card::ActionComponent::Button(button.clone()));

    let card = card::Card::new()
        .config(
            card::Config::new()
                .enable_forward(true)
                .update_multi(true)
                .width_mode(card::WidthMode::Fill)
                .compact_width(false)
                .use_custom_translation(true)
                .enable_forward_interaction(true)
                .style(
                    card::CardStyle::new()
                        .text_size(
                            "cus-size",
                            card::CustomTextSize::new()
                                .default("medium")
                                .pc("large")
                                .mobile("small"),
                        )
                        .color(
                            "cus-color",
                            card::CustomColor::new("rgba(1,2,3,0.5)", "rgba(4,5,6,0.5)"),
                        ),
                ),
        )
        .card_link(url)
        .header(header)
        .element(card::Element::Div(div))
        .element(card::Element::Markdown(markdown))
        .element(card::Element::Img(image))
        .element(card::Element::ImgCombination(combination))
        .element(card::Element::Chart(chart))
        .element(card::Element::Table(table))
        .element(card::Element::Note(note))
        .element(card::Element::ColumnSet(column_set))
        .element(card::Element::InteractiveContainer(container))
        .element(card::Element::CollapsiblePanel(panel))
        .element(card::Element::Form(form))
        .element(card::Element::Action(action))
        .element(card::Element::Overflow(overflow))
        .element(card::Element::SelectStatic(select))
        .element(card::Element::SelectImg(image_select))
        .i18n_elements(BTreeMap::from([(
            card::Locale::ZhCn,
            vec![card::Element::Button(button)],
        )]))
        .fallback(
            card::Fallback::new()
                .trigger_condition(card::FallbackCondition::MinClientVersion("7.9".into()))
                .trigger_condition(card::FallbackCondition::ElementTags(vec!["chart".into()])),
        );

    assert!(card.validate().is_ok());
    assert!(card.to_json().as_value().is_object());
    assert!(serde_json::to_value(standard_icon).unwrap().is_object());
    assert!(
        serde_json::to_value(card::CustomIcon::new("img_icon"))
            .unwrap()
            .is_object()
    );
    assert!(
        serde_json::to_value(card::UdIcon::new("icon_token"))
            .unwrap()
            .is_object()
    );
}

#[test]
fn modern_v1_serializes_color_domains_and_validation_errors() {
    use larksuite_oapi_sdk_rs::card::v1 as card;

    for color in [
        card::Color::Default,
        card::Color::Transparent,
        card::Color::White,
        card::Color::Black,
        card::Color::Grey,
        card::Color::Neutral,
        card::Color::Blue,
        card::Color::Turquoise,
        card::Color::Lime,
        card::Color::Orange,
        card::Color::Violet,
        card::Color::Indigo,
        card::Color::Wathet,
        card::Color::Green,
        card::Color::Yellow,
        card::Color::Red,
        card::Color::Purple,
        card::Color::Carmine,
        card::Color::custom("cus-color"),
    ] {
        let json = serde_json::to_value(&color).unwrap();
        assert_eq!(serde_json::from_value::<card::Color>(json).unwrap(), color);
    }
    for color in [
        card::HeaderTagColor::Neutral,
        card::HeaderTagColor::Blue,
        card::HeaderTagColor::Turquoise,
        card::HeaderTagColor::Lime,
        card::HeaderTagColor::Orange,
        card::HeaderTagColor::Violet,
        card::HeaderTagColor::Indigo,
        card::HeaderTagColor::Wathet,
        card::HeaderTagColor::Green,
        card::HeaderTagColor::Yellow,
        card::HeaderTagColor::Red,
        card::HeaderTagColor::Purple,
        card::HeaderTagColor::Carmine,
    ] {
        assert!(
            serde_json::to_value(card::Color::from(color))
                .unwrap()
                .is_string()
        );
    }

    let option = card::SelectOption::new(card::Text::plain("Option"), "option");
    let invalid_cards = [
        card::Card::new().element(card::Element::ColumnSet(card::ColumnSet::new())),
        card::Card::new().element(card::Element::Action(card::Action::new())),
        card::Card::new().element(card::Element::Form(card::Form::new("form"))),
        card::Card::new().element(card::Element::SelectImg(card::ImageSelect::new())),
        card::Card::new().element(card::Element::ImgCombination(card::ImageCombination::new())),
        card::Card::new().element(card::Element::ImgCombination(
            card::ImageCombination::new().image(card::ImageReference::new("img")),
        )),
        card::Card::new().element(card::Element::Person(card::Person::new(""))),
        card::Card::new().element(card::Element::PersonList(card::PersonList::new())),
        card::Card::new().element(card::Element::Overflow(card::Overflow::new())),
        card::Card::new().element(card::Element::SelectStatic(card::StaticSelect::new())),
        card::Card::new().element(card::Element::MultiSelectStatic(
            card::MultiStaticSelect::default(),
        )),
        card::Card::new().element(card::Element::Action(
            card::Action::new().action(card::ActionComponent::Overflow(card::Overflow::new())),
        )),
        card::Card::new().element(card::Element::Action(card::Action::new().action(
            card::ActionComponent::SelectStatic(card::StaticSelect::new()),
        ))),
        card::Card::new().element(card::Element::Action(card::Action::new().action(
            card::ActionComponent::MultiSelectStatic(card::MultiStaticSelect::default()),
        ))),
        card::Card::new().element(card::Element::Table(card::Table {
            page_size: Some(0),
            columns: vec![card::TableColumn::new("known")],
            ..Default::default()
        })),
        card::Card::new().element(card::Element::Table(card::Table {
            columns: vec![card::TableColumn::new("known")],
            rows: vec![BTreeMap::from([(
                "unknown".to_string(),
                serde_json::json!(true).into(),
            )])],
            ..Default::default()
        })),
    ];
    for card in invalid_cards {
        assert!(card.validate().is_err());
    }
    for (mode, count) in [
        (card::ImageCombinationMode::Double, 2),
        (card::ImageCombinationMode::Triple, 3),
        (card::ImageCombinationMode::Bisect, 6),
        (card::ImageCombinationMode::Trisect, 9),
    ] {
        let combination = (0..count).fold(
            card::ImageCombination::new().combination_mode(mode),
            |combination, index| {
                combination.image(card::ImageReference::new(format!("img_{index}")))
            },
        );
        assert!(
            card::Card::new()
                .element(card::Element::ImgCombination(combination))
                .validate()
                .is_ok()
        );
    }
    for error in [
        card::ValidationError::InvalidCardLink,
        card::ValidationError::EmptyFallback,
        card::ValidationError::HeaderTagLocalizationConflict,
        card::ValidationError::EmptyColumns,
        card::ValidationError::EmptyActionRow,
        card::ValidationError::EmptyForm {
            name: "form".to_string(),
        },
        card::ValidationError::EmptyImageSelect,
        card::ValidationError::EmptyImageCombination,
        card::ValidationError::MissingImageCombinationMode,
        card::ValidationError::InvalidImageCombinationCount,
        card::ValidationError::EmptyPersonId,
        card::ValidationError::EmptyPersonList,
        card::ValidationError::MissingPanelHeader,
        card::ValidationError::EmptyOverflow,
        card::ValidationError::EmptyStaticSelect,
        card::ValidationError::InvalidTablePageSize(0),
        card::ValidationError::TableHasNoColumns,
        card::ValidationError::UnknownTableRowColumn {
            column: "unknown".to_string(),
        },
    ] {
        assert!(!error.to_string().is_empty());
    }
    assert!(serde_json::to_value(option).unwrap().is_object());
}

#[test]
fn modern_v1_validation_rejects_cross_field_protocol_errors() {
    use larksuite_oapi_sdk_rs::card::v1 as card;

    let fallback = CardV1::new().fallback(Fallback::new());
    assert!(matches!(
        fallback.validate(),
        Err(card::ValidationError::EmptyFallback)
    ));

    let conflicting_tags = CardV1::new().header(
        HeaderV1::new("title")
            .text_tag(HeaderTag::new("tag", HeaderTagColor::Blue))
            .i18n_text_tag_list(BTreeMap::from([(
                Locale::ZhCn,
                vec![HeaderTag::new("标签", HeaderTagColor::Blue)],
            )])),
    );
    assert!(matches!(
        conflicting_tags.validate(),
        Err(card::ValidationError::HeaderTagLocalizationConflict)
    ));

    let invalid_table = CardV1::new().element(ElementV1::Table(card::Table::new()));
    assert!(matches!(
        invalid_table.validate(),
        Err(card::ValidationError::TableHasNoColumns)
    ));

    let invalid_link = CardV1::new().card_link(MultiUrl::default());
    assert!(matches!(
        invalid_link.validate(),
        Err(card::ValidationError::InvalidCardLink)
    ));

    let invalid_images =
        CardV1::new().element(ElementV1::ImgCombination(card::ImageCombination::new()));
    assert!(matches!(
        invalid_images.validate(),
        Err(card::ValidationError::EmptyImageCombination)
    ));
}

#[test]
fn modern_v1_color_preserves_known_tokens_and_custom_style_tokens() {
    use larksuite_oapi_sdk_rs::card::v1::{
        Color, HeaderIcon, StandardIcon, TableHeaderStyle, Text,
    };

    let known =
        HeaderIcon::StandardIcon(StandardIcon::new("approval_colorful").color(Color::Wathet));
    assert_eq!(serde_json::to_value(known).unwrap()["color"], "wathet");

    let custom = Color::custom("cus-0");
    assert_eq!(
        serde_json::to_value(custom).unwrap(),
        serde_json::json!("cus-0")
    );

    assert_eq!(
        serde_json::to_value(Text::plain("Status").text_color(Color::Default)).unwrap()["text_color"],
        "default"
    );
    assert_eq!(
        serde_json::to_value(TableHeaderStyle::new().text_color(Color::custom("cus-0"))).unwrap()["text_color"],
        "cus-0"
    );
}
