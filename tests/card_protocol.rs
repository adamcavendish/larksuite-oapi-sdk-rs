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

const V1_MANIFEST: &str = include_str!("fixtures/card_protocol/card_json_v1.json");
const V2_MANIFEST: &str = include_str!("fixtures/card_protocol/card_json_v2.json");
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

const REQUIRED_V2_SURFACES: &[&str] = &[
    "card",
    "config",
    "card_link",
    "header",
    "body",
    "column_set",
    "column",
    "collapsible_panel",
    "form",
    "interactive_container",
    "recycling_container",
    "div",
    "markdown",
    "img",
    "img_combination",
    "person",
    "person_list",
    "chart",
    "table",
    "hr",
    "button",
    "input",
    "overflow",
    "select_static",
    "multi_select_static",
    "select_person",
    "multi_select_person",
    "date_picker",
    "picker_time",
    "picker_datetime",
    "select_img",
    "checker",
    "behaviors",
    "card.action.trigger",
];

const REQUIRED_V2_FIELDS: &[&str] = &[
    "card.schema",
    "card.config",
    "card.card_link",
    "card.header",
    "card.body",
    "config.streaming_mode",
    "config.streaming_config",
    "config.summary",
    "config.locales",
    "config.enable_forward",
    "config.update_multi",
    "config.width_mode",
    "config.use_custom_translation",
    "config.enable_forward_interaction",
    "config.style",
    "card_link.url",
    "card_link.android_url",
    "card_link.ios_url",
    "card_link.pc_url",
    "streaming_config.print_frequency_ms",
    "streaming_config.print_step",
    "streaming_config.print_strategy",
    "summary.content",
    "summary.i18n_content",
    "style.text_size",
    "style.color",
    "body.elements",
    "body.direction",
    "body.padding",
    "body.horizontal_spacing",
    "body.vertical_spacing",
    "body.horizontal_align",
    "body.vertical_align",
    "element.tag",
    "element.element_id",
    "element.margin",
    "header.title",
    "header.subtitle",
    "header.template",
    "header.text_tag_list",
    "header.i18n_text_tag_list",
    "header.icon",
    "header.padding",
    "header.title.lines",
    "header.subtitle.lines",
    "header.text_tag.text",
    "header.text_tag.color",
    "header.icon.tag",
    "header.icon.token",
    "header.icon.color",
    "header.icon.size",
    "behavior.type",
    "behavior.value",
    "behavior.default_url",
    "behavior.pc_url",
    "behavior.ios_url",
    "behavior.android_url",
    "collapsible_panel.header",
    "collapsible_panel.elements",
    "collapsible_panel.expanded",
    "collapsible_panel.background_color",
    "collapsible_panel.border",
    "collapsible_panel.direction",
    "collapsible_panel.vertical_spacing",
    "collapsible_panel.horizontal_spacing",
    "collapsible_panel.padding",
    "collapsible_panel.margin",
    "collapsible_panel.header.title",
    "collapsible_panel.header.background_color",
    "collapsible_panel.header.width",
    "collapsible_panel.header.vertical_align",
    "collapsible_panel.header.icon",
    "collapsible_panel.header.icon_position",
    "collapsible_panel.header.icon_expanded_angle",
    "interactive_container.elements",
    "interactive_container.behaviors",
    "interactive_container.width",
    "interactive_container.height",
    "interactive_container.direction",
    "interactive_container.horizontal_align",
    "interactive_container.vertical_align",
    "interactive_container.background_style",
    "interactive_container.has_border",
    "interactive_container.border_color",
    "interactive_container.corner_radius",
    "interactive_container.padding",
    "interactive_container.margin",
    "interactive_container.disabled",
    "interactive_container.disabled_tips",
    "interactive_container.hover_tips",
    "interactive_container.confirm",
    "form.name",
    "button.form_action_type",
    "input.name",
    "input.required",
    "column_set.columns",
    "column_set.flex_mode",
    "column_set.horizontal_spacing",
    "column_set.horizontal_align",
    "column_set.background_style",
    "column_set.action",
    "column.width",
    "column.weight",
    "column.vertical_align",
    "column.direction",
    "column.horizontal_spacing",
    "column.vertical_spacing",
    "column.padding",
    "column.background_style",
    "column.action",
    "column.elements",
    "div.text",
    "div.fields",
    "div.icon",
    "div.width",
    "markdown.content",
    "markdown.text_size",
    "markdown.text_align",
    "markdown.icon",
    "img.img_key",
    "img.alt",
    "img.title",
    "img.scale_type",
    "img.size",
    "img.corner_radius",
    "img.transparent",
    "img.preview",
    "img_combination.combination_mode",
    "img_combination.img_list",
    "img_combination.combination_transparent",
    "img_combination.corner_radius",
    "person.user_id",
    "person.size",
    "person.show_avatar",
    "person.show_name",
    "person.style",
    "person_list.persons",
    "person_list.show_name",
    "person_list.show_avatar",
    "person_list.size",
    "person_list.lines",
    "person_list.drop_invalid_user_id",
    "person_list.icon",
    "person_list.ud_icon",
    "chart.chart_spec",
    "chart.aspect_ratio",
    "chart.color_theme",
    "chart.height",
    "chart.preview",
    "table.columns",
    "table.rows",
    "table.page_size",
    "table.row_height",
    "table.row_max_height",
    "table.freeze_first_column",
    "table.header_style",
];

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    protocol: String,
    version: String,
    sources: Sources,
    surfaces: Vec<Surface>,
    fixtures: Vec<Fixture>,
    #[serde(default)]
    fields: Vec<ProtocolField>,
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
    #[serde(default)]
    expected_constraints: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ProtocolField {
    surface: String,
    name: String,
    domain: String,
    constraints: Vec<String>,
    sources: Vec<String>,
}

fn v1_manifest() -> Manifest {
    serde_json::from_str(V1_MANIFEST).expect("Card JSON 1.0 protocol manifest must be valid JSON")
}

fn v2_manifest() -> Manifest {
    serde_json::from_str(V2_MANIFEST).expect("Card JSON 2.0 protocol manifest must be valid JSON")
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
    let manifest = v1_manifest();
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
    let manifest = v1_manifest();
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
fn card_json_v2_inventory_is_complete_and_traceable() {
    let manifest = v2_manifest();
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.protocol, "card-json");
    assert_eq!(manifest.version, "2.0");

    assert_eq!(manifest.sources.official_docs.role, "normative");
    assert_eq!(manifest.sources.official_docs.accessed_on, "2026-08-25");
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
    let expected: BTreeSet<_> = REQUIRED_V2_SURFACES.iter().copied().collect();
    assert_eq!(
        ids, expected,
        "inventory must classify every Card JSON 2.0 surface"
    );

    for surface in &manifest.surfaces {
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
        assert!(surface.sources.contains(&"official_docs".to_string()));
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
        if surface.id == "recycling_container" {
            assert_eq!(surface.status, "known_divergence");
            assert!(surface.wire_tag.is_none());
        } else if matches!(surface.kind.as_str(), "component" | "container") {
            assert!(
                surface.wire_tag.is_some(),
                "{} is missing a wire tag",
                surface.id
            );
        }
    }

    let fields: BTreeSet<_> = manifest
        .fields
        .iter()
        .map(|field| format!("{}.{}", field.surface, field.name))
        .collect();
    let expected_fields: BTreeSet<_> = REQUIRED_V2_FIELDS
        .iter()
        .map(|field| (*field).to_string())
        .collect();
    assert_eq!(
        fields, expected_fields,
        "inventory must retain v2 root and cross-field rules"
    );
    for field in &manifest.fields {
        assert!(!field.domain.is_empty());
        assert!(!field.constraints.is_empty());
        assert!(field.sources.contains(&"official_docs".to_string()));
    }
}

#[test]
fn card_json_v2_fixtures_are_exact_protocol_contracts() {
    let manifest = v2_manifest();
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
        assert!(entry.path.starts_with("card_json_v2/"));
        let value = fixture(&entry.path);
        assert!(value.is_object(), "{} must be a JSON object", entry.id);
        assert_eq!(value["schema"], "2.0", "{} must be a v2 card", entry.id);
        let elements = value["body"]["elements"].as_array();
        let tags: Vec<_> = elements
            .into_iter()
            .flatten()
            .map(|element| element["tag"].as_str().expect("element tag"))
            .collect();
        assert_eq!(
            tags, entry.expected_tags,
            "{} has unexpected tags",
            entry.id
        );

        match entry.kind.as_str() {
            "outbound_card" => {
                use larksuite_oapi_sdk_rs::card::v2 as card;

                assert!(entry.expected_constraints.is_empty());
                let card: card::Card = serde_json::from_value(value.clone())
                    .expect("outbound fixture must deserialize into the typed v2 AST");
                card.validate()
                    .expect("outbound fixture must satisfy v2 validation");
                assert_eq!(
                    card.to_json(),
                    value.clone().into(),
                    "outbound fixture must round-trip without changing the wire shape"
                );
            }
            "invalid_card" => {
                assert!(!entry.expected_constraints.is_empty());
                for constraint in &entry.expected_constraints {
                    match constraint.as_str() {
                        "shared_only" => assert_ne!(value["config"]["update_multi"], true),
                        "required_shared_config" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(
                                card.validate(),
                                Err(card::ValidationError::V2RequiresSharedCard)
                            );
                        }
                        "required_body" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(card.validate(), Err(card::ValidationError::MissingBody));
                        }
                        "unique_element_ids" => {
                            let elements = elements.expect("constraint requires body.elements");
                            let ids: Vec<_> = elements
                                .iter()
                                .filter_map(|element| element["element_id"].as_str())
                                .collect();
                            assert!(ids.len() > BTreeSet::from_iter(ids.iter()).len());
                        }
                        "element_id_format" => {
                            let elements = elements.expect("constraint requires body.elements");
                            assert!(elements.iter().any(|element| {
                                element["element_id"]
                                    .as_str()
                                    .is_some_and(|id| !id.starts_with(char::is_alphabetic))
                            }));
                        }
                        "img_combination_limit" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(
                                card.validate(),
                                Err(card::ValidationError::TooManyImagesInCombination {
                                    mode: card::ImageCombinationMode::Double,
                                    count: 3,
                                })
                            );
                        }
                        "img_size_requires_crop_scale" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(
                                card.validate(),
                                Err(card::ValidationError::ImageSizeRequiresCropScale)
                            );
                        }
                        "table_row_max_height_requires_auto" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(
                                card.validate(),
                                Err(card::ValidationError::TableRowMaxHeightRequiresAutoRowHeight)
                            );
                        }
                        "card_link_url" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(
                                card.validate(),
                                Err(card::ValidationError::InvalidCardLink)
                            );
                        }
                        "streaming_config_mode" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(
                                card.validate(),
                                Err(card::ValidationError::StreamingConfigRequiresStreamingMode)
                            );
                        }
                        "header_title_lines" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(
                                card.validate(),
                                Err(card::ValidationError::InvalidHeaderTitleLines(5))
                            );
                        }
                        "open_url_default" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(
                                card.validate(),
                                Err(card::ValidationError::InvalidOpenUrl(String::new()))
                            );
                        }
                        "panel_disallows_form" => {
                            use larksuite_oapi_sdk_rs::card::v2 as card;

                            let card: card::Card = serde_json::from_value(value.clone())
                                .expect("fixture must deserialize before validation");
                            assert_eq!(
                                card.validate(),
                                Err(card::ValidationError::FormNestedOutsideBody)
                            );
                        }
                        other => panic!("unknown v2 fixture constraint {other}"),
                    }
                }
            }
            other => panic!("unknown v2 fixture kind {other}"),
        }
    }
}

#[test]
fn modern_v2_root_matches_complete_fixture() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let card = card::Card::new()
        .config(
            card::Config::new()
                .streaming_mode(true)
                .streaming_config(
                    card::StreamingConfig::new()
                        .print_frequency_ms(card::ClientValue::new().default(70).pc(70))
                        .print_strategy(card::StreamingPrintStrategy::Fast),
                )
                .summary(
                    card::Summary::new("Release health").i18n_content(BTreeMap::from([(
                        card::Locale::ZhCn,
                        "发布健康度".to_string(),
                    )])),
                )
                .locales([card::Locale::EnUs, card::Locale::ZhCn])
                .enable_forward(true)
                .update_multi()
                .width_mode(card::WidthMode::Fill)
                .use_custom_translation(true)
                .enable_forward_interaction(true)
                .style(
                    card::CardStyle::new()
                        .text_size(
                            "title",
                            card::CustomTextSize::new()
                                .default("heading-2")
                                .pc("heading-2")
                                .mobile("heading-3"),
                        )
                        .color(
                            "cus-primary",
                            card::CustomColor::new("rgba(30,120,255,1)", "rgba(80,150,255,1)"),
                        ),
                ),
        )
        .card_link(
            card::MultiUrl::new("https://example.com/release")
                .pc_url("https://example.com/release/desktop"),
        )
        .header(
            card::Header::new(card::Text::plain("Release health"))
                .subtitle(card::Text::plain("All systems normal"))
                .template(TemplateColor::Blue)
                .icon(card::HeaderIcon::standard("approval_colorful"))
                .text_tag(
                    card::HeaderTag::new(card::Text::plain("Stable"), card::HeaderTagColor::Green)
                        .element_id("status_tag"),
                )
                .padding("12px 12px 8px 12px"),
        )
        .body(
            card::Body::new()
                .direction(card::Direction::Vertical)
                .padding("12px")
                .horizontal_spacing(card::Spacing::Medium)
                .vertical_spacing(card::Spacing::Large)
                .horizontal_align(card::HorizontalAlign::Left)
                .vertical_align(card::VerticalAlign::Top)
                .element(card::Element::Markdown(
                    card::Markdown::new("**All checks are passing.**")
                        .element_id("summary_md")
                        .text_size("title")
                        .margin("0px"),
                ))
                .element(card::Element::ColumnSet(
                    card::ColumnSet::new()
                        .element_id("metrics_cols")
                        .flex_mode(card::ColumnFlexMode::None)
                        .column(
                            card::Column::new()
                                .width(card::ColumnWidth::Weighted)
                                .weight(1)
                                .element(card::Element::Markdown(
                                    card::Markdown::new("Latency: 92ms").element_id("metric_one"),
                                )),
                        )
                        .column(
                            card::Column::new()
                                .width(card::ColumnWidth::Weighted)
                                .weight(1)
                                .element(card::Element::Markdown(
                                    card::Markdown::new("Errors: 0").element_id("metric_two"),
                                )),
                        ),
                ))
                .element(card::Element::Form(
                    card::Form::new("feedback")
                        .element_id("feedback_form")
                        .element(card::Element::Input(
                            card::Input::new("reason")
                                .element_id("reason_input")
                                .required(true)
                                .placeholder(card::Text::plain("Reason")),
                        ))
                        .element(card::Element::Button(
                            card::Button::new(card::Text::plain("Submit"))
                                .element_id("submit_button")
                                .name("submit")
                                .form_action(card::FormActionType::Submit)
                                .button_type(card::ButtonType::PrimaryFilled),
                        )),
                )),
        );

    card.validate().expect("complete v2 card is valid");
    let fixture = fixture("card_json_v2/complete_root.json");
    assert_eq!(card.to_json(), fixture.clone().into());
    let decoded: card::Card = serde_json::from_value(fixture.clone())
        .expect("deserialize complete Card JSON 2.0 fixture");
    assert_eq!(
        decoded.to_json(),
        fixture.into(),
        "complete Card JSON 2.0 fixture must round-trip"
    );
}

#[test]
fn modern_v2_display_layout_fixture_round_trips_and_validates() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let fixture = fixture("card_json_v2/display_layout.json");
    let card: card::Card = serde_json::from_value(fixture.clone())
        .expect("deserialize complete Card JSON 2.0 display/layout fixture");
    card.validate()
        .expect("display/layout fixture satisfies protocol constraints");
    assert_eq!(
        card.to_json(),
        fixture.into(),
        "display/layout fixture must preserve every documented field"
    );
}

#[test]
fn modern_v2_display_layout_validation_rejects_documented_ranges() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let shared_card = |element| {
        card::Card::new()
            .config(card::Config::new().update_multi())
            .body(card::Body::new().element(element))
    };

    let mut person_list = card::PersonList::new();
    person_list.lines = Some(0);
    assert_eq!(
        shared_card(card::Element::PersonList(person_list)).validate(),
        Err(card::ValidationError::InvalidPersonListLines)
    );

    let column_set = card::ColumnSet::new().column(
        card::Column::new()
            .width(card::ColumnWidth::Pixels("15px".into()))
            .element(card::Element::Markdown(card::Markdown::new("too narrow"))),
    );
    assert_eq!(
        shared_card(card::Element::ColumnSet(column_set)).validate(),
        Err(card::ValidationError::InvalidColumnWidth("15px".into()))
    );

    let default_weight_column = card::ColumnSet::new().column(
        card::Column::new()
            .width(card::ColumnWidth::Weighted)
            .element(card::Element::Markdown(card::Markdown::new(
                "default weight",
            ))),
    );
    shared_card(card::Element::ColumnSet(default_weight_column))
        .validate()
        .expect("weighted columns may use the documented default weight");

    let chart = card::Chart::new(serde_json::json!([]).into());
    assert_eq!(
        shared_card(card::Element::Chart(chart)).validate(),
        Err(card::ValidationError::InvalidChartSpec)
    );

    let table = card::Table::new()
        .column(card::TableColumn::new("service"))
        .row(BTreeMap::from([(
            "service".into(),
            serde_json::json!("API").into(),
        )]));
    let mut table = table;
    table.row_height = Some(card::TableRowHeight::Pixels("12px".into()));
    assert_eq!(
        shared_card(card::Element::Table(table)).validate(),
        Err(card::ValidationError::InvalidTableRowHeight("12px".into()))
    );

    let mut image = card::Image::new("img", card::Text::plain("Image"));
    image.scale_type = Some(card::ImageScale::FitHorizontal);
    image.size = Some("large".into());
    assert_eq!(
        shared_card(card::Element::Img(image)).validate(),
        Err(card::ValidationError::ImageSizeRequiresCropScale)
    );

    let table = card::Table::new()
        .column(card::TableColumn::new("service"))
        .row(BTreeMap::from([(
            "service".into(),
            serde_json::json!("API").into(),
        )]));
    let mut table = table;
    table.row_height = Some(card::TableRowHeight::Middle);
    table.row_max_height = Some("200px".into());
    assert_eq!(
        shared_card(card::Element::Table(table)).validate(),
        Err(card::ValidationError::TableRowMaxHeightRequiresAutoRowHeight)
    );

    let column_set = card::ColumnSet::new()
        .flex_mode(card::ColumnFlexMode::Flow)
        .column(
            card::Column::new()
                .width(card::ColumnWidth::Auto)
                .element(card::Element::Markdown(card::Markdown::new("flow"))),
        );
    assert_eq!(
        shared_card(card::Element::ColumnSet(column_set)).validate(),
        Err(card::ValidationError::ColumnWidthRequiresFixedFlexMode)
    );

    let error = serde_json::from_value::<card::ChartAspectRatio>(serde_json::json!("3:2"))
        .expect_err("undocumented chart aspect ratios must be rejected");
    assert!(error.to_string().contains("unknown variant"));
}

#[test]
fn modern_v2_root_header_validation_rejects_documented_constraints() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let shared_card = |header| {
        card::Card::new()
            .config(card::Config::new().update_multi())
            .header(header)
            .body(card::Body::new())
    };

    assert!(
        shared_card(
            card::Header::new(card::Text::plain("Title"))
                .text_tag(card::HeaderTag::new(
                    card::Text::plain("Stable"),
                    card::HeaderTagColor::Green,
                ))
                .i18n_text_tag_list(BTreeMap::from([(
                    card::Locale::ZhCn,
                    vec![card::HeaderTag::new(
                        card::Text::plain("稳定"),
                        card::HeaderTagColor::Green,
                    )],
                )])),
        )
        .validate()
        .is_ok()
    );

    assert_eq!(
        shared_card(
            card::Header::new(card::Text::plain("Title"))
                .subtitle(card::Text::plain("Subtitle").lines(2),)
        )
        .validate(),
        Err(card::ValidationError::InvalidHeaderSubtitleLines(2))
    );

    assert_eq!(
        shared_card(
            card::Header::new(card::Text::plain("Title")).text_tag(card::HeaderTag::new(
                card::Text::lark_md("**not a plain tag**"),
                card::HeaderTagColor::Green,
            )),
        )
        .validate(),
        Err(card::ValidationError::HeaderTagRequiresPlainText)
    );

    assert_eq!(
        shared_card(card::Header::new(card::Text::plain("Title")).padding("100px")).validate(),
        Err(card::ValidationError::InvalidPadding("100px".into()))
    );
}

#[test]
fn modern_v2_serializes_every_handwritten_component_tag() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let text = card::Text::plain("text");
    let callback_value: larksuite_oapi_sdk_rs::JsonValue =
        serde_json::json!({"operation": "run"}).into();
    let callback = card::Behavior::callback(callback_value);
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
            card::Element::ImgCombination(card::ImageCombination::new(
                card::ImageCombinationMode::Double,
            )),
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
        ("hr", card::Element::Hr(card::Hr::new())),
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
        ("form", card::Element::Form(card::Form::new("form"))),
        (
            "interactive_container",
            card::Element::InteractiveContainer(card::InteractiveContainer::new(callback.clone())),
        ),
        (
            "button",
            card::Element::Button(card::Button::new(text.clone())),
        ),
        ("input", card::Element::Input(card::Input::new("input"))),
        ("overflow", card::Element::Overflow(card::Overflow::new())),
        (
            "select_static",
            card::Element::SelectStatic(card::StaticSelect::new()),
        ),
        (
            "multi_select_static",
            card::Element::MultiSelectStatic(card::MultiStaticSelect::new()),
        ),
        (
            "select_person",
            card::Element::SelectPerson(card::PersonSelect::new()),
        ),
        (
            "multi_select_person",
            card::Element::MultiSelectPerson(card::MultiPersonSelect::new()),
        ),
        (
            "date_picker",
            card::Element::DatePicker(card::DatePicker::new()),
        ),
        (
            "picker_time",
            card::Element::PickerTime(card::TimePicker::new()),
        ),
        (
            "picker_datetime",
            card::Element::PickerDatetime(card::DatetimePicker::new()),
        ),
        (
            "select_img",
            card::Element::SelectImg(card::ImageSelect::new(
                vec![card::ImageSelectOption {
                    img_key: "img".into(),
                    value: "value".into(),
                    disabled: None,
                    disabled_tips: None,
                    hover_tips: None,
                }],
                callback.clone(),
            )),
        ),
        ("checker", card::Element::Checker(card::Checker::new())),
    ];

    for (expected_tag, element) in elements {
        let value = serde_json::to_value(element).expect("serialize Card JSON 2.0 element");
        assert_eq!(value["tag"], expected_tag, "wrong tag for {expected_tag}");
        let decoded: card::Element =
            serde_json::from_value(value.clone()).expect("deserialize Card JSON 2.0 element");
        assert_eq!(
            serde_json::to_value(decoded).expect("reserialize Card JSON 2.0 element"),
            value,
            "round-trip changed {expected_tag}"
        );
    }
}

#[test]
fn modern_v2_rejects_unknown_component_tag() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let error = serde_json::from_value::<card::Element>(serde_json::json!({"tag": "unknown"}))
        .expect_err("unknown Card JSON 2.0 tag must be rejected");
    assert!(error.to_string().contains("unknown variant"));

    let error = serde_json::from_value::<card::Element>(serde_json::json!({
        "tag": "markdown",
        "content": "Known tag with unsupported property",
        "unsupported": true
    }))
    .expect_err("unknown component property must be rejected");
    assert!(error.to_string().contains("unknown field"));

    let error = serde_json::from_value::<card::Element>(serde_json::json!({
        "tag": "overflow",
        "options": [],
        "unsupported": true
    }))
    .expect_err("unknown flattened-control property must be rejected");
    assert!(error.to_string().contains("unknown field"));

    let error = serde_json::from_value::<card::Card>(serde_json::json!({
        "schema": "2.0",
        "config": {"update_multi": true},
        "body": {"elements": []},
        "unsupported": true
    }))
    .expect_err("unknown root property must be rejected");
    assert!(error.to_string().contains("unknown field"));
}

#[test]
fn modern_v2_callback_behavior_and_raw_response_match_protocol() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let callback_value: larksuite_oapi_sdk_rs::JsonValue =
        serde_json::json!({"operation": "approve"}).into();
    let card = card::Card::new()
        .config(card::Config::new().update_multi())
        .header(card::Header::new(card::Text::plain("Approval")))
        .body(
            card::Body::new().element(card::Element::Button(
                card::Button::new(card::Text::plain("Approve"))
                    .element_id("approve_button")
                    .button_type(card::ButtonType::PrimaryFilled)
                    .behavior(card::Behavior::callback(callback_value))
                    .behavior(card::Behavior::open_url("https://example.com/approval")),
            )),
        );
    card.validate().expect("v2 callback card is valid");
    assert_eq!(
        card.to_json(),
        fixture("card_json_v2/callback_button.json").into()
    );

    let request: CardActionTriggerRequest = serde_json::from_value(serde_json::json!({
        "token": "callback-token",
        "action": {"tag": "button", "value": {"operation": "approve"}}
    }))
    .expect("deserialize v2 callback");
    assert_eq!(
        request.action.expect("callback action").value["operation"].as_str(),
        Some("approve")
    );
    let response = CallbackCard::raw(card).expect("serialize v2 raw callback response");
    let response = serde_json::to_value(response).expect("serialize response");
    assert_eq!(response["data"]["schema"], "2.0");
}

#[test]
fn modern_v2_button_and_input_preserve_optional_interaction_fields() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let mut button = card::Button::new(card::Text::plain("Open"))
        .button_type(card::ButtonType::Laser)
        .behavior(card::Behavior::open_url("https://example.com"));
    button.size = Some(card::ControlSize::Large);
    button.width = Some("fill".into());
    button.disabled = Some(false);
    button.confirm = Some(card::Confirm::new(
        card::Text::plain("Continue?"),
        card::Text::plain("Open the linked page"),
    ));
    let button = serde_json::to_value(card::Element::Button(button)).expect("serialize button");
    assert_eq!(button["type"], "laser");
    assert_eq!(button["size"], "large");
    assert_eq!(button["width"], "fill");
    assert_eq!(button["disabled"], false);
    assert_eq!(button["behaviors"][0]["type"], "open_url");
    assert_eq!(button["confirm"]["title"]["content"], "Continue?");

    let mut input = card::Input::new("reason")
        .required(true)
        .placeholder(card::Text::plain("Reason"));
    input.default_value = Some("initial".into());
    input.label = Some(card::Text::plain("Reason"));
    input.label_position = Some(card::LabelPosition::Left);
    input.input_type = Some(card::InputType::MultilineText);
    input.multiline = Some(true);
    input.rows = Some(3);
    input.auto_resize = Some(true);
    input.max_rows = Some(6);
    input.max_length = Some(500);
    input.width = Some("fill".into());
    input.disabled = Some(false);
    input.behaviors.push(card::Behavior::callback(
        serde_json::json!({"field": "reason"}),
    ));
    let input = serde_json::to_value(card::Element::Input(input)).expect("serialize input");
    assert_eq!(input["default_value"], "initial");
    assert_eq!(input["multiline"], true);
    assert_eq!(input["rows"], 3);
    assert_eq!(input["max_length"], 500);
    assert_eq!(input["label"]["content"], "Reason");
    assert_eq!(input["label_position"], "left");
    assert_eq!(input["input_type"], "multiline_text");
    assert_eq!(input["auto_resize"], true);
    assert_eq!(input["max_rows"], 6);
    assert_eq!(input["behaviors"][0]["type"], "callback");

    let mut select = card::StaticSelect::new();
    select.r#type = Some(card::SelectType::Text);
    let select =
        serde_json::to_value(card::Element::SelectStatic(select)).expect("serialize static select");
    assert_eq!(select["type"], "text");
    assert!(select.get("select_type").is_none());
}

#[test]
fn modern_v2_controls_match_fixture() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let mut input = card::Input::new("reason")
        .required(true)
        .placeholder(card::Text::plain("Reason"));
    input.label = Some(card::Text::plain("Release reason"));
    input.label_position = Some(card::LabelPosition::Left);
    input.input_type = Some(card::InputType::MultilineText);
    input.rows = Some(3);
    input.auto_resize = Some(true);
    input.max_rows = Some(6);
    input.max_length = Some(500);
    input.width = Some("fill".into());

    let mut select = card::StaticSelect::new();
    select.r#type = Some(card::SelectType::Text);
    select.control.placeholder = Some(card::Text::plain("Severity"));
    select.options = Some(vec![
        card::SelectOption::new(card::Text::plain("S1"), "s1"),
        card::SelectOption::new(card::Text::plain("S2"), "s2"),
    ]);
    select.initial_option = Some("s2".into());

    let mut checker = card::Checker::new();
    checker.control.name = Some("confirmed".into());
    checker.checked = Some(false);
    checker.text = Some(card::Text::plain("I confirmed the release"));
    checker.overall_checkable = Some(true);
    checker.checked_style = Some(card::CheckedStyle {
        show_strikethrough: Some(true),
        opacity: Some(0.5),
    });

    let card = card::Card::new()
        .config(card::Config::new().update_multi())
        .body(
            card::Body::new()
                .element(card::Element::Input(input))
                .element(card::Element::SelectStatic(select))
                .element(card::Element::Checker(checker)),
        );
    card.validate().expect("controls fixture is valid");
    assert_eq!(card.to_json(), fixture("card_json_v2/controls.json").into());
}

#[test]
fn modern_v2_container_optional_fields_preserve_wire_shape() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let icon = card::HeaderIcon::standard("right_small")
        .color(card::HeaderTagColor::Blue)
        .size("16px");
    let mut panel = card::CollapsiblePanel::new(card::CollapsiblePanelHeader::new(
        card::Text::plain("Details"),
    ))
    .element(card::Element::Markdown(card::Markdown::new("content")));
    panel.header.icon_expanded_angle = Some(card::PanelIconExpandedAngle::NegativeNinety);
    panel.border = Some(card::Border::new(card::Color::Grey));
    let panel = serde_json::to_value(card::Element::CollapsiblePanel(panel))
        .expect("serialize collapsible panel");
    assert_eq!(panel["border"]["color"], "grey");
    assert_eq!(panel["header"]["icon_expanded_angle"], "-90");

    let callback_value: larksuite_oapi_sdk_rs::JsonValue = serde_json::json!({}).into();
    let mut container = card::InteractiveContainer::new(card::Behavior::callback(callback_value))
        .element(card::Element::Div(card::Div::new(card::Text::plain(
            "content",
        ))));
    container.disabled = Some(false);
    container.disabled_tips = Some(card::Text::plain("Unavailable"));
    container.hover_tips = Some(card::Text::plain("Open details"));
    container.confirm = Some(card::Confirm::new(
        card::Text::plain("Continue?"),
        card::Text::plain("Open this item"),
    ));
    let container = serde_json::to_value(card::Element::InteractiveContainer(container))
        .expect("serialize interactive container");
    assert_eq!(container["disabled"], false);
    assert_eq!(container["disabled_tips"]["content"], "Unavailable");
    assert_eq!(container["hover_tips"]["content"], "Open details");
    assert_eq!(container["confirm"]["title"]["content"], "Continue?");
    assert_eq!(
        serde_json::to_value(icon).expect("serialize icon")["size"],
        "16px"
    );
}

#[test]
fn modern_v2_container_behavior_validation_rejects_documented_constraints() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let shared_card = |element| {
        card::Card::new()
            .config(card::Config::new().update_multi())
            .body(card::Body::new().element(element))
    };
    let callback = || card::Behavior::callback(serde_json::json!({}));
    let interactive = |behavior| {
        card::InteractiveContainer::new(behavior)
            .element(card::Element::Markdown(card::Markdown::new("content")))
    };

    assert_eq!(
        shared_card(card::Element::InteractiveContainer(interactive(
            card::Behavior::open_url("")
        )))
        .validate(),
        Err(card::ValidationError::InvalidOpenUrl(String::new()))
    );

    let mut no_behavior = interactive(callback());
    no_behavior.behaviors.clear();
    assert_eq!(
        shared_card(card::Element::InteractiveContainer(no_behavior)).validate(),
        Err(card::ValidationError::MissingInteractiveContainerBehavior)
    );

    let mut narrow = interactive(callback());
    narrow.width = Some("15px".into());
    assert_eq!(
        shared_card(card::Element::InteractiveContainer(narrow)).validate(),
        Err(card::ValidationError::InvalidInteractiveContainerWidth(
            "15px".into()
        ))
    );

    let mut short = interactive(callback());
    short.height = Some("9px".into());
    assert_eq!(
        shared_card(card::Element::InteractiveContainer(short)).validate(),
        Err(card::ValidationError::InvalidInteractiveContainerHeight(
            "9px".into()
        ))
    );

    let mut square = interactive(callback());
    square.corner_radius = Some("101%".into());
    assert_eq!(
        shared_card(card::Element::InteractiveContainer(square)).validate(),
        Err(card::ValidationError::InvalidCornerRadius("101%".into()))
    );

    let mut panel = card::CollapsiblePanel::new(card::CollapsiblePanelHeader::new(
        card::Text::plain("Details"),
    ));
    panel.border = Some(card::Border {
        color: card::Color::Grey,
        corner_radius: Some("4em".into()),
    });
    assert_eq!(
        shared_card(card::Element::CollapsiblePanel(panel)).validate(),
        Err(card::ValidationError::InvalidCornerRadius("4em".into()))
    );

    let error = serde_json::from_value::<card::PanelIconExpandedAngle>(serde_json::json!("0"))
        .expect_err("undocumented panel icon angles must be rejected");
    assert!(error.to_string().contains("unknown variant"));
}

#[test]
fn modern_v2_validation_rejects_documented_component_placement_errors() {
    use larksuite_oapi_sdk_rs::card::v2 as card;

    let shared_card = |body| {
        card::Card::new()
            .config(card::Config::new().update_multi())
            .body(body)
    };
    let callback_value: larksuite_oapi_sdk_rs::JsonValue = serde_json::json!({}).into();
    let callback = card::Behavior::callback(callback_value);
    let image_select = card::ImageSelect::new(
        vec![card::ImageSelectOption {
            img_key: "img".into(),
            value: "img-1".into(),
            disabled: None,
            disabled_tips: None,
            hover_tips: None,
        }],
        callback.clone(),
    );
    let mut image_select = image_select;
    image_select.multi_select = Some(true);
    let image_card = shared_card(card::Body::new().element(card::Element::SelectImg(image_select)));
    assert_eq!(
        image_card.validate(),
        Err(card::ValidationError::MultiSelectImageOutsideForm)
    );

    let nested_table = shared_card(
        card::Body::new()
            .element(card::Element::ColumnSet(card::ColumnSet::new().column(
                card::Column::new().element(card::Element::Table(card::Table::new())),
            ))),
    );
    assert_eq!(
        nested_table.validate(),
        Err(card::ValidationError::TableNestedOutsideBody)
    );

    let empty_container = shared_card(card::Body::new().element(
        card::Element::InteractiveContainer(card::InteractiveContainer::new(callback)),
    ));
    assert_eq!(
        empty_container.validate(),
        Err(card::ValidationError::EmptyInteractiveContainer)
    );

    let empty_overflow =
        shared_card(card::Body::new().element(card::Element::Overflow(card::Overflow::new())));
    assert_eq!(
        empty_overflow.validate(),
        Err(card::ValidationError::EmptyOptions("overflow"))
    );

    let empty_picker =
        shared_card(card::Body::new().element(card::Element::DatePicker(card::DatePicker::new())));
    assert_eq!(
        empty_picker.validate(),
        Err(card::ValidationError::MissingPickerValue("date_picker"))
    );

    let mut duplicate_options = card::StaticSelect::new();
    duplicate_options.options = Some(vec![
        card::SelectOption::new(card::Text::plain("one"), "duplicate"),
        card::SelectOption::new(card::Text::plain("two"), "duplicate"),
    ]);
    let duplicate_options =
        shared_card(card::Body::new().element(card::Element::SelectStatic(duplicate_options)));
    assert_eq!(
        duplicate_options.validate(),
        Err(card::ValidationError::DuplicateOptionValue(
            "duplicate".into()
        ))
    );

    let button_without_behavior = shared_card(card::Body::new().element(card::Element::Button(
        card::Button::new(card::Text::plain("button")),
    )));
    assert_eq!(
        button_without_behavior.validate(),
        Err(card::ValidationError::MissingButtonBehavior)
    );

    let conflicting_button = shared_card(
        card::Body::new().element(card::Element::Form(
            card::Form::new("form").element(card::Element::Button(
                card::Button::new(card::Text::plain("button"))
                    .form_action(card::FormActionType::Submit)
                    .behavior(card::Behavior::callback(serde_json::json!({}))),
            )),
        )),
    );
    assert_eq!(
        conflicting_button.validate(),
        Err(card::ValidationError::ButtonBehaviorConflict)
    );

    let empty_table =
        shared_card(card::Body::new().element(card::Element::Table(card::Table::new())));
    assert_eq!(
        empty_table.validate(),
        Err(card::ValidationError::EmptyTableColumns)
    );

    let mut invalid_page_size = card::Table::new()
        .column(card::TableColumn::new("status"))
        .row(BTreeMap::from([(
            "status".to_string(),
            serde_json::json!("ok").into(),
        )]));
    invalid_page_size.page_size = Some(11);
    let invalid_page_size =
        shared_card(card::Body::new().element(card::Element::Table(invalid_page_size)));
    assert_eq!(
        invalid_page_size.validate(),
        Err(card::ValidationError::InvalidTablePageSize(11))
    );

    let duplicate_form_name = shared_card(
        card::Body::new().element(card::Element::Form(
            card::Form::new("form")
                .element(card::Element::Input(card::Input::new("duplicate")))
                .element(card::Element::Input(card::Input::new("duplicate"))),
        )),
    );
    assert_eq!(
        duplicate_form_name.validate(),
        Err(card::ValidationError::DuplicateFormControlName(
            "duplicate".into()
        ))
    );

    let callback_value: larksuite_oapi_sdk_rs::JsonValue = serde_json::json!({}).into();
    let mut deeply_nested = card::Element::Markdown(card::Markdown::new("leaf"));
    for _ in 0..6 {
        deeply_nested = card::Element::InteractiveContainer(
            card::InteractiveContainer::new(card::Behavior::callback(callback_value.clone()))
                .element(deeply_nested),
        );
    }
    let deeply_nested = shared_card(card::Body::new().element(deeply_nested));
    assert_eq!(
        deeply_nested.validate(),
        Err(card::ValidationError::TooDeeplyNestedContainer(6))
    );

    let form_without_submit = shared_card(card::Body::new().element(card::Element::Form(
        card::Form::new("form").element(card::Element::Input(card::Input::new("reason"))),
    )));
    assert_eq!(
        form_without_submit.validate(),
        Err(card::ValidationError::MissingFormSubmit("form".into()))
    );
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
