//! Typed Card JSON 2.0 composition.
//!
//! Card JSON 2.0 is not a schema flag on the historical card builder or on
//! [`crate::card::v1`]. Its `body.elements` root, shared-card-only rule, and globally
//! unique element identifiers require a separate model.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{JsonValue, LarkError};

pub use super::TemplateColor;
pub use super::v1::Color;

/// A stable, machine-readable explanation of a Card JSON 2.0 validation failure.
///
/// `path` is a JSON Pointer.  When validation has no document instance to
/// inspect (for example, [`ValidationError::diagnostic`]), a `*` segment
/// denotes the matching member of a repeated collection.  Use
/// [`Card::validate_with_diagnostic`] or
/// [`CardDocument::new_with_diagnostic`] to obtain the concrete pointer for
/// layout and spacing failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationDiagnostic {
    /// Stable identifier for programmatic handling.
    pub code: &'static str,
    /// JSON Pointer for the invalid field or the field that must be changed.
    pub path: String,
    /// The protocol rule that was violated.
    pub constraint: Option<String>,
    /// Closed values, a numeric range, or a minimal valid shape when useful.
    pub allowed_values: Option<Vec<String>>,
}

impl ValidationDiagnostic {
    fn new(
        code: &'static str,
        path: impl Into<String>,
        constraint: impl Into<Option<String>>,
        allowed_values: Option<Vec<String>>,
    ) -> Self {
        Self {
            code,
            path: path.into(),
            constraint: constraint.into(),
            allowed_values,
        }
    }
}

impl std::fmt::Display for ValidationDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.code, self.path)
    }
}

impl std::error::Error for ValidationDiagnostic {}

/// A validation failure together with its concrete document location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardValidationError {
    pub error: ValidationError,
    pub diagnostic: ValidationDiagnostic,
}

impl std::fmt::Display for CardValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}

impl std::error::Error for CardValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// A Card JSON 2.0 document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Card {
    pub schema: SchemaVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_link: Option<MultiUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Header>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
}

impl Card {
    /// Start a Card JSON 2.0 document with its required schema discriminator.
    pub fn new() -> Self {
        Self {
            schema: SchemaVersion::V2,
            config: None,
            card_link: None,
            header: None,
            body: None,
        }
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn card_link(mut self, card_link: MultiUrl) -> Self {
        self.card_link = Some(card_link);
        self
    }

    pub fn header(mut self, header: Header) -> Self {
        self.header = Some(header);
        self
    }

    pub fn body(mut self, body: Body) -> Self {
        self.body = Some(body);
        self
    }

    /// Check the Card JSON 2.0 root and component constraints before sending.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.schema != SchemaVersion::V2 {
            return Err(ValidationError::InvalidSchema);
        }
        if self
            .config
            .as_ref()
            .is_none_or(|config| config.update_multi != Some(true))
        {
            return Err(ValidationError::V2RequiresSharedCard);
        }
        if self
            .card_link
            .as_ref()
            .is_some_and(|link| link.url.as_deref().is_none_or(str::is_empty))
        {
            return Err(ValidationError::InvalidCardLink);
        }
        if self.config.as_ref().is_some_and(|config| {
            config.streaming_config.is_some() && config.streaming_mode != Some(true)
        }) {
            return Err(ValidationError::StreamingConfigRequiresStreamingMode);
        }

        let mut element_ids = BTreeSet::new();
        let mut form_state = FormValidationState::default();
        let mut element_count = 0;
        if let Some(header) = &self.header {
            validate_header(header, &mut element_ids)?;
        }
        let body = self.body.as_ref().ok_or(ValidationError::MissingBody)?;
        validate_body(body)?;
        let table_count = body
            .elements
            .iter()
            .filter(|element| matches!(element, Element::Table(_)))
            .count();
        if table_count > 5 {
            return Err(ValidationError::TooManyTables(table_count));
        }
        for element in &body.elements {
            element.validate(
                &mut element_ids,
                &mut form_state,
                &mut element_count,
                true,
                false,
                0,
            )?;
        }
        if element_count > 200 {
            return Err(ValidationError::TooManyElements(element_count));
        }
        Ok(())
    }

    /// Validate the card and return a machine-readable diagnostic on failure.
    ///
    /// This is additive to [`Card::validate`], whose `ValidationError` return
    /// type remains the compatibility API.
    pub fn validate_with_diagnostic(&self) -> Result<(), ValidationDiagnostic> {
        self.validate().map_err(|error| self.diagnostic_for(&error))
    }

    fn diagnostic_for(&self, error: &ValidationError) -> ValidationDiagnostic {
        let mut diagnostic = error.diagnostic();
        if let Some(path) = self.concrete_validation_path(error) {
            diagnostic.path = path;
        }
        diagnostic
    }

    fn concrete_validation_path(&self, error: &ValidationError) -> Option<String> {
        match error {
            ValidationError::InvalidSchema => Some("/schema".to_string()),
            ValidationError::V2RequiresSharedCard => Some("/config/update_multi".to_string()),
            ValidationError::MissingBody => Some("/body".to_string()),
            ValidationError::InvalidCardLink => Some("/card_link/url".to_string()),
            ValidationError::StreamingConfigRequiresStreamingMode => {
                Some("/config/streaming_mode".to_string())
            }
            ValidationError::HeaderTagRequiresPlainText => {
                Some("/header/text_tag_list".to_string())
            }
            ValidationError::InvalidHeaderTitleLines(_) => Some("/header/title/lines".to_string()),
            ValidationError::InvalidHeaderSubtitleLines(_) => {
                Some("/header/subtitle/lines".to_string())
            }
            ValidationError::TooManyTables(_) | ValidationError::TooManyElements(_) => {
                Some("/body/elements".to_string())
            }
            ValidationError::InvalidColumnWidth(_) | ValidationError::InvalidColumnWeight(_) => {
                find_column_path(
                    self.body.as_ref()?.elements.as_slice(),
                    error,
                    "/body/elements",
                )
            }
            ValidationError::ColumnWidthRequiresFixedFlexMode => find_column_set_flex_mode_path(
                self.body.as_ref()?.elements.as_slice(),
                "/body/elements",
            ),
            ValidationError::InvalidMargin(_)
            | ValidationError::InvalidPadding(_)
            | ValidationError::InvalidSpacing(_) => self
                .header
                .as_ref()
                .and_then(|header| find_header_layout_path(header, error))
                .or_else(|| find_typed_layout_path(self.body.as_ref()?, error)),
            _ => self
                .body
                .as_ref()
                .and_then(|body| find_element_error_path(&body.elements, error, "/body/elements"))
                .or_else(|| Some("/body".to_string())),
        }
    }

    pub fn to_json(&self) -> JsonValue {
        JsonValue::from_serializable(self).expect("Card JSON 2.0 is serializable")
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

/// An immutable Card JSON 2.0 document that has passed protocol validation.
#[derive(Debug, Clone)]
pub struct CardDocument {
    card: Card,
}

impl CardDocument {
    /// Validate a Card JSON 2.0 document before an outbound Card transport uses it.
    pub fn new(card: Card) -> Result<Self, ValidationError> {
        card.validate()?;
        Ok(Self { card })
    }

    /// Validate a Card JSON 2.0 document and retain structured failure data.
    ///
    /// Unlike [`CardDocument::new`], this constructor exposes a concrete JSON
    /// Pointer for authoring tools without changing the established error type
    /// of the compatibility constructor.
    pub fn new_with_diagnostic(card: Card) -> Result<Self, Box<CardValidationError>> {
        card.validate().map_err(|error| {
            Box::new(CardValidationError {
                diagnostic: card.diagnostic_for(&error),
                error,
            })
        })?;
        Ok(Self { card })
    }

    /// Inspect the validated Card JSON 2.0 document.
    pub fn card(&self) -> &Card {
        &self.card
    }

    /// Return to an editable Card JSON 2.0 document.
    ///
    /// The returned Card must be validated again before it can be sent.
    pub fn into_card(self) -> Card {
        self.card
    }

    pub(crate) fn encoded_content(&self) -> Result<String, LarkError> {
        serde_json::to_string(&self.card).map_err(LarkError::Json)
    }
}

impl TryFrom<Card> for CardDocument {
    type Error = ValidationError;

    fn try_from(card: Card) -> Result<Self, Self::Error> {
        Self::new(card)
    }
}

/// The only schema value produced by this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchemaVersion {
    #[serde(rename = "2.0")]
    V2,
}

/// Global Card JSON 2.0 configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_config: Option<StreamingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Summary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locales: Option<Vec<Locale>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_forward: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_multi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_mode: Option<WidthMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_custom_translation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_forward_interaction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CardStyle>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn streaming_mode(mut self, enabled: bool) -> Self {
        self.streaming_mode = Some(enabled);
        self
    }

    pub fn streaming_config(mut self, config: StreamingConfig) -> Self {
        self.streaming_config = Some(config);
        self
    }

    pub fn summary(mut self, summary: Summary) -> Self {
        self.summary = Some(summary);
        self
    }

    pub fn locales(mut self, locales: impl IntoIterator<Item = Locale>) -> Self {
        self.locales = Some(locales.into_iter().collect());
        self
    }

    pub fn enable_forward(mut self, enabled: bool) -> Self {
        self.enable_forward = Some(enabled);
        self
    }

    /// Set the only supported v2 update mode.
    pub fn update_multi(mut self) -> Self {
        self.update_multi = Some(true);
        self
    }

    pub fn width_mode(mut self, width_mode: WidthMode) -> Self {
        self.width_mode = Some(width_mode);
        self
    }

    pub fn use_custom_translation(mut self, enabled: bool) -> Self {
        self.use_custom_translation = Some(enabled);
        self
    }

    pub fn enable_forward_interaction(mut self, enabled: bool) -> Self {
        self.enable_forward_interaction = Some(enabled);
        self
    }

    pub fn style(mut self, style: CardStyle) -> Self {
        self.style = Some(style);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreamingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_frequency_ms: Option<ClientValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_step: Option<ClientValue<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_strategy: Option<StreamingPrintStrategy>,
}

impl StreamingConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_frequency_ms(mut self, value: ClientValue<u32>) -> Self {
        self.print_frequency_ms = Some(value);
        self
    }

    pub fn print_step(mut self, value: ClientValue<u32>) -> Self {
        self.print_step = Some(value);
        self
    }

    pub fn print_strategy(mut self, strategy: StreamingPrintStrategy) -> Self {
        self.print_strategy = Some(strategy);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientValue<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<T>,
}

impl<T> Default for ClientValue<T> {
    fn default() -> Self {
        Self {
            default: None,
            android: None,
            ios: None,
            pc: None,
        }
    }
}

impl<T> ClientValue<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn default(mut self, value: T) -> Self {
        self.default = Some(value);
        self
    }

    pub fn android(mut self, value: T) -> Self {
        self.android = Some(value);
        self
    }

    pub fn ios(mut self, value: T) -> Self {
        self.ios = Some(value);
        self
    }

    pub fn pc(mut self, value: T) -> Self {
        self.pc = Some(value);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamingPrintStrategy {
    Fast,
    Delay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Summary {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_content: Option<BTreeMap<Locale, String>>,
}

impl Summary {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            i18n_content: None,
        }
    }

    pub fn i18n_content(mut self, content: BTreeMap<Locale, String>) -> Self {
        self.i18n_content = Some(content);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardStyle {
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub text_size: BTreeMap<String, CustomTextSize>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub color: BTreeMap<String, CustomColor>,
}

impl CardStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text_size(mut self, name: impl Into<String>, size: CustomTextSize) -> Self {
        self.text_size.insert(name.into(), size);
        self
    }

    pub fn color(mut self, name: impl Into<String>, color: CustomColor) -> Self {
        self.color.insert(name.into(), color);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomTextSize {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
}

impl CustomTextSize {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }

    pub fn pc(mut self, value: impl Into<String>) -> Self {
        self.pc = Some(value.into());
        self
    }

    pub fn mobile(mut self, value: impl Into<String>) -> Self {
        self.mobile = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomColor {
    pub light_mode: String,
    pub dark_mode: String,
}

impl CustomColor {
    pub fn new(light_mode: impl Into<String>, dark_mode: impl Into<String>) -> Self {
        Self {
            light_mode: light_mode.into(),
            dark_mode: dark_mode.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidthMode {
    Compact,
    Default,
    Fill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Locale {
    ZhCn,
    EnUs,
    JaJp,
    ZhHk,
    ZhTw,
    IdId,
    ViVn,
    ThTh,
    PtBr,
    EsEs,
    KoKr,
    DeDe,
    FrFr,
    ItIt,
    RuRu,
    MsMy,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
}

impl MultiUrl {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            ..Self::default()
        }
    }

    pub fn android_url(mut self, url: impl Into<String>) -> Self {
        self.android_url = Some(url.into());
        self
    }

    pub fn ios_url(mut self, url: impl Into<String>) -> Self {
        self.ios_url = Some(url.into());
        self
    }

    pub fn pc_url(mut self, url: impl Into<String>) -> Self {
        self.pc_url = Some(url.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header {
    pub title: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplateColor>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub text_tag_list: Vec<HeaderTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_text_tag_list: Option<BTreeMap<Locale, Vec<HeaderTag>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
}

impl Header {
    pub fn new(title: Text) -> Self {
        Self {
            title,
            subtitle: None,
            template: None,
            text_tag_list: Vec::new(),
            i18n_text_tag_list: None,
            icon: None,
            padding: None,
        }
    }

    pub fn subtitle(mut self, subtitle: Text) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn template(mut self, template: TemplateColor) -> Self {
        self.template = Some(template);
        self
    }

    pub fn text_tag(mut self, tag: HeaderTag) -> Self {
        self.text_tag_list.push(tag);
        self
    }

    pub fn i18n_text_tag_list(mut self, tags: BTreeMap<Locale, Vec<HeaderTag>>) -> Self {
        self.i18n_text_tag_list = Some(tags);
        self
    }

    pub fn icon(mut self, icon: HeaderIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn padding(mut self, padding: impl Into<String>) -> Self {
        self.padding = Some(padding.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    pub tag: TextTag,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<u32>,
}

impl Text {
    pub fn plain(content: impl Into<String>) -> Self {
        Self {
            tag: TextTag::PlainText,
            content: content.into(),
            text_size: None,
            text_color: None,
            text_align: None,
            lines: None,
        }
    }

    pub fn lark_md(content: impl Into<String>) -> Self {
        Self {
            tag: TextTag::LarkMd,
            content: content.into(),
            text_size: None,
            text_color: None,
            text_align: None,
            lines: None,
        }
    }

    pub fn text_size(mut self, size: impl Into<String>) -> Self {
        self.text_size = Some(size.into());
        self
    }
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }
    pub fn text_align(mut self, align: HorizontalAlign) -> Self {
        self.text_align = Some(align);
        self
    }
    pub fn lines(mut self, lines: u32) -> Self {
        self.lines = Some(lines);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextTag {
    PlainText,
    LarkMd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeaderTag {
    pub tag: HeaderTagKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub text: Text,
    pub color: HeaderTagColor,
}

impl HeaderTag {
    pub fn new(text: Text, color: HeaderTagColor) -> Self {
        Self {
            tag: HeaderTagKind::TextTag,
            element_id: None,
            text,
            color,
        }
    }

    pub fn element_id(mut self, element_id: impl Into<String>) -> Self {
        self.element_id = Some(element_id.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeaderTagKind {
    TextTag,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeaderTagColor {
    Neutral,
    Blue,
    Turquoise,
    Lime,
    Orange,
    Violet,
    Indigo,
    Wathet,
    Green,
    Yellow,
    Red,
    Purple,
    Carmine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum HeaderIcon {
    StandardIcon {
        token: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        color: Option<HeaderTagColor>,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<String>,
    },
    CustomIcon {
        img_key: String,
    },
}

impl HeaderIcon {
    pub fn standard(token: impl Into<String>) -> Self {
        Self::StandardIcon {
            token: token.into(),
            color: None,
            size: None,
        }
    }
    pub fn custom(img_key: impl Into<String>) -> Self {
        Self::CustomIcon {
            img_key: img_key.into(),
        }
    }
    pub fn color(self, color: HeaderTagColor) -> Self {
        match self {
            Self::StandardIcon { token, size, .. } => Self::StandardIcon {
                token,
                color: Some(color),
                size,
            },
            custom => custom,
        }
    }
    pub fn size(self, size: impl Into<String>) -> Self {
        match self {
            Self::StandardIcon { token, color, .. } => Self::StandardIcon {
                token,
                color,
                size: Some(size.into()),
            },
            custom => custom,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(default)]
    pub elements: Vec<Element>,
}

impl Body {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }
    pub fn padding(mut self, padding: impl Into<String>) -> Self {
        self.padding = Some(padding.into());
        self
    }
    pub fn horizontal_spacing(mut self, spacing: Spacing) -> Self {
        self.horizontal_spacing = Some(spacing);
        self
    }
    pub fn vertical_spacing(mut self, spacing: Spacing) -> Self {
        self.vertical_spacing = Some(spacing);
        self
    }
    pub fn horizontal_align(mut self, alignment: HorizontalAlign) -> Self {
        self.horizontal_align = Some(alignment);
        self
    }
    pub fn vertical_align(mut self, alignment: VerticalAlign) -> Self {
        self.vertical_align = Some(alignment);
        self
    }
    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Vertical,
    Horizontal,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spacing {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Pixels(String),
}

impl Spacing {
    /// Construct a protocol-valid pixel spacing value.
    ///
    /// The public `Pixels(String)` variant remains for deserializing existing
    /// documents; new authored cards should prefer this constructor.
    pub fn pixels(value: u8) -> Result<Self, ValidationDiagnostic> {
        if value <= 99 {
            Ok(Self::Pixels(format!("{value}px")))
        } else {
            Err(ValidationDiagnostic::new(
                "invalid_spacing",
                "/horizontal_spacing",
                Some("spacing_pixels_must_be_between_0_and_99".to_string()),
                Some(vec!["0px through 99px".to_string()]),
            ))
        }
    }
}

/// A protocol-valid one-, two-, or four-value padding declaration.
///
/// Construct it with [`Padding::uniform`], [`Padding::symmetric`], or
/// [`Padding::sides`] and pass it to existing `padding(...)` builders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Padding(String);

impl Padding {
    pub fn uniform(value: u8) -> Result<Self, ValidationDiagnostic> {
        Self::from_values(&[value])
    }

    pub fn symmetric(vertical: u8, horizontal: u8) -> Result<Self, ValidationDiagnostic> {
        Self::from_values(&[vertical, horizontal])
    }

    pub fn sides(top: u8, right: u8, bottom: u8, left: u8) -> Result<Self, ValidationDiagnostic> {
        Self::from_values(&[top, right, bottom, left])
    }

    fn from_values(values: &[u8]) -> Result<Self, ValidationDiagnostic> {
        if values.iter().all(|value| *value <= 99) {
            Ok(Self(
                values
                    .iter()
                    .map(|value| format!("{value}px"))
                    .collect::<Vec<_>>()
                    .join(" "),
            ))
        } else {
            Err(ValidationDiagnostic::new(
                "invalid_padding",
                "/padding",
                Some("padding_pixels_must_be_between_0_and_99".to_string()),
                Some(vec!["0px through 99px".to_string()]),
            ))
        }
    }
}

impl From<Padding> for String {
    fn from(value: Padding) -> Self {
        value.0
    }
}

/// A protocol-valid one-, two-, or four-value margin declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Margin(String);

impl Margin {
    pub fn uniform(value: i16) -> Result<Self, ValidationDiagnostic> {
        Self::from_values(&[value])
    }

    pub fn symmetric(vertical: i16, horizontal: i16) -> Result<Self, ValidationDiagnostic> {
        Self::from_values(&[vertical, horizontal])
    }

    pub fn sides(
        top: i16,
        right: i16,
        bottom: i16,
        left: i16,
    ) -> Result<Self, ValidationDiagnostic> {
        Self::from_values(&[top, right, bottom, left])
    }

    fn from_values(values: &[i16]) -> Result<Self, ValidationDiagnostic> {
        if values.iter().all(|value| (-99..=99).contains(value)) {
            Ok(Self(
                values
                    .iter()
                    .map(|value| format!("{value}px"))
                    .collect::<Vec<_>>()
                    .join(" "),
            ))
        } else {
            Err(ValidationDiagnostic::new(
                "invalid_margin",
                "/margin",
                Some("margin_pixels_must_be_between_negative_99_and_99".to_string()),
                Some(vec!["-99px through 99px".to_string()]),
            ))
        }
    }
}

impl From<Margin> for String {
    fn from(value: Margin) -> Self {
        value.0
    }
}

impl Serialize for Spacing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra_large",
            Self::Pixels(value) => value,
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for Spacing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "small" => Self::Small,
            "medium" => Self::Medium,
            "large" => Self::Large,
            "extra_large" => Self::ExtraLarge,
            _ => Self::Pixels(value),
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

/// Responsive layout mode for a Card JSON 2.0 column set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnFlexMode {
    None,
    Stretch,
    Flow,
    Bisect,
    Trisect,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Element {
    Div(Div),
    Markdown(Markdown),
    Img(Image),
    ImgCombination(ImageCombination),
    Person(Person),
    PersonList(PersonList),
    Chart(Chart),
    Table(Table),
    Hr(Hr),
    ColumnSet(ColumnSet),
    CollapsiblePanel(CollapsiblePanel),
    Form(Form),
    InteractiveContainer(InteractiveContainer),
    Button(Button),
    Input(Input),
    Overflow(Overflow),
    SelectStatic(StaticSelect),
    MultiSelectStatic(MultiStaticSelect),
    SelectPerson(PersonSelect),
    MultiSelectPerson(MultiPersonSelect),
    DatePicker(DatePicker),
    PickerTime(TimePicker),
    PickerDatetime(DatetimePicker),
    SelectImg(ImageSelect),
    Checker(Checker),
}

impl Element {
    fn validate_wire_fields(value: &serde_json::Value, tag: &str) -> Result<(), String> {
        let extra_fields: &[&str] = match tag {
            "overflow" => &["options"],
            "select_static" => &["options", "initial_option", "initial_index", "type"],
            "multi_select_static" => &["options", "selected_values", "type"],
            "select_person" => &["options", "initial_option", "type"],
            "multi_select_person" => &["options", "selected_values", "type"],
            "date_picker" => &["initial_date"],
            "picker_time" => &["initial_time"],
            "picker_datetime" => &["initial_datetime"],
            "select_img" => &[
                "options",
                "multi_select",
                "layout",
                "aspect_ratio",
                "can_preview",
                "value",
            ],
            "checker" => &[
                "checked",
                "text",
                "overall_checkable",
                "button_area",
                "checked_style",
                "padding",
            ],
            _ => return Ok(()),
        };
        const CONTROL_FIELDS: &[&str] = &[
            "tag",
            "element_id",
            "name",
            "required",
            "placeholder",
            "width",
            "disabled",
            "disabled_tips",
            "hover_tips",
            "behaviors",
            "confirm",
            "margin",
        ];

        let object = value
            .as_object()
            .ok_or_else(|| format!("component {tag:?} must be a JSON object"))?;
        if let Some(field) = object.keys().find(|field| {
            !CONTROL_FIELDS.contains(&field.as_str()) && !extra_fields.contains(&field.as_str())
        }) {
            return Err(format!("unknown field {field:?} for component {tag:?}"));
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let tag = value
            .get("tag")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| serde::de::Error::missing_field("tag"))?;
        Self::validate_wire_fields(&value, tag).map_err(serde::de::Error::custom)?;
        let decode = |error: serde_json::Error| serde::de::Error::custom(error);
        match tag {
            "div" => serde_json::from_value(value).map(Self::Div).map_err(decode),
            "markdown" => serde_json::from_value(value)
                .map(Self::Markdown)
                .map_err(decode),
            "img" => serde_json::from_value(value).map(Self::Img).map_err(decode),
            "img_combination" => serde_json::from_value(value)
                .map(Self::ImgCombination)
                .map_err(decode),
            "person" => serde_json::from_value(value)
                .map(Self::Person)
                .map_err(decode),
            "person_list" => serde_json::from_value(value)
                .map(Self::PersonList)
                .map_err(decode),
            "chart" => serde_json::from_value(value)
                .map(Self::Chart)
                .map_err(decode),
            "table" => serde_json::from_value(value)
                .map(Self::Table)
                .map_err(decode),
            "hr" => serde_json::from_value(value).map(Self::Hr).map_err(decode),
            "column_set" => serde_json::from_value(value)
                .map(Self::ColumnSet)
                .map_err(decode),
            "form" => serde_json::from_value(value)
                .map(Self::Form)
                .map_err(decode),
            "collapsible_panel" => serde_json::from_value(value)
                .map(Self::CollapsiblePanel)
                .map_err(decode),
            "interactive_container" => serde_json::from_value(value)
                .map(Self::InteractiveContainer)
                .map_err(decode),
            "button" => serde_json::from_value(value)
                .map(Self::Button)
                .map_err(decode),
            "input" => serde_json::from_value(value)
                .map(Self::Input)
                .map_err(decode),
            "overflow" => serde_json::from_value(value)
                .map(Self::Overflow)
                .map_err(decode),
            "select_static" => serde_json::from_value(value)
                .map(Self::SelectStatic)
                .map_err(decode),
            "multi_select_static" => serde_json::from_value(value)
                .map(Self::MultiSelectStatic)
                .map_err(decode),
            "select_person" => serde_json::from_value(value)
                .map(Self::SelectPerson)
                .map_err(decode),
            "multi_select_person" => serde_json::from_value(value)
                .map(Self::MultiSelectPerson)
                .map_err(decode),
            "date_picker" => serde_json::from_value(value)
                .map(Self::DatePicker)
                .map_err(decode),
            "picker_time" => serde_json::from_value(value)
                .map(Self::PickerTime)
                .map_err(decode),
            "picker_datetime" => serde_json::from_value(value)
                .map(Self::PickerDatetime)
                .map_err(decode),
            "select_img" => serde_json::from_value(value)
                .map(Self::SelectImg)
                .map_err(decode),
            "checker" => serde_json::from_value(value)
                .map(Self::Checker)
                .map_err(decode),
            tag => Err(serde::de::Error::unknown_variant(
                tag,
                &[
                    "div",
                    "markdown",
                    "img",
                    "img_combination",
                    "person",
                    "person_list",
                    "chart",
                    "table",
                    "hr",
                    "column_set",
                    "form",
                    "collapsible_panel",
                    "interactive_container",
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
                ],
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Markdown {
    pub tag: MarkdownTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl Markdown {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            tag: MarkdownTag::Markdown,
            element_id: None,
            content: content.into(),
            text_size: None,
            text_align: None,
            icon: None,
            margin: None,
        }
    }
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.element_id = Some(id.into());
        self
    }
    pub fn text_size(mut self, size: impl Into<String>) -> Self {
        self.text_size = Some(size.into());
        self
    }
    pub fn margin(mut self, margin: impl Into<String>) -> Self {
        self.margin = Some(margin.into());
        self
    }
    pub fn text_align(mut self, text_align: HorizontalAlign) -> Self {
        self.text_align = Some(text_align);
        self
    }
    pub fn icon(mut self, icon: HeaderIcon) -> Self {
        self.icon = Some(icon);
        self
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkdownTag {
    #[serde(rename = "markdown")]
    Markdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColumnSet {
    pub tag: ColumnSetTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_mode: Option<ColumnFlexMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<BackgroundStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<NavigationAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    pub columns: Vec<Column>,
}
impl ColumnSet {
    pub fn new() -> Self {
        Self {
            tag: ColumnSetTag::ColumnSet,
            element_id: None,
            flex_mode: None,
            horizontal_spacing: None,
            horizontal_align: None,
            background_style: None,
            action: None,
            margin: None,
            columns: Vec::new(),
        }
    }
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.element_id = Some(id.into());
        self
    }
    pub fn column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }
    pub fn flex_mode(mut self, flex_mode: ColumnFlexMode) -> Self {
        self.flex_mode = Some(flex_mode);
        self
    }

    /// Build the automatic-width branch of the column layout grammar.
    pub fn automatic(columns: impl IntoIterator<Item = AutoColumn>) -> Self {
        Self::new().with_columns(columns.into_iter().map(AutoColumn::into_column))
    }

    /// Build the fixed-width branch of the column layout grammar.
    ///
    /// This always emits the required `flex_mode: none`.
    pub fn fixed(columns: impl IntoIterator<Item = FixedColumn>) -> Self {
        Self::new()
            .flex_mode(ColumnFlexMode::None)
            .with_columns(columns.into_iter().map(FixedColumn::into_column))
    }

    /// Build the weighted-width branch of the column layout grammar.
    ///
    /// This always emits `flex_mode: none`, `width: weighted`, and a weight in
    /// the protocol's one-through-five range.
    pub fn weighted(columns: impl IntoIterator<Item = WeightedColumn>) -> Self {
        Self::new()
            .flex_mode(ColumnFlexMode::None)
            .with_columns(columns.into_iter().map(WeightedColumn::into_column))
    }

    fn with_columns(mut self, columns: impl IntoIterator<Item = Column>) -> Self {
        self.columns.extend(columns);
        self
    }
}
impl Default for ColumnSet {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnSetTag {
    #[serde(rename = "column_set")]
    ColumnSet,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Column {
    pub tag: ColumnTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<ColumnWidth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<BackgroundStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<NavigationAction>,
    #[serde(default)]
    pub elements: Vec<Element>,
}
impl Column {
    pub fn new() -> Self {
        Self {
            tag: ColumnTag::Column,
            width: None,
            weight: None,
            vertical_align: None,
            direction: None,
            horizontal_spacing: None,
            vertical_spacing: None,
            padding: None,
            margin: None,
            background_style: None,
            action: None,
            elements: Vec::new(),
        }
    }
    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }
    pub fn width(mut self, width: ColumnWidth) -> Self {
        self.width = Some(width);
        self
    }
    pub fn weight(mut self, weight: u8) -> Self {
        self.weight = Some(weight);
        self
    }
}
impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

/// A column in the automatic-width layout branch.
#[derive(Debug, Clone)]
pub struct AutoColumn(Column);

impl AutoColumn {
    pub fn new() -> Self {
        Self(Column::new())
    }

    pub fn element(mut self, element: Element) -> Self {
        self.0.elements.push(element);
        self
    }

    fn into_column(self) -> Column {
        self.0
    }
}

impl Default for AutoColumn {
    fn default() -> Self {
        Self::new()
    }
}

/// A checked Card JSON 2.0 fixed column width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedColumnWidth(u16);

impl FixedColumnWidth {
    /// Accept a documented fixed width of 16 through 600 pixels.
    pub fn pixels(value: u16) -> Result<Self, ValidationDiagnostic> {
        if (16..=600).contains(&value) {
            Ok(Self(value))
        } else {
            Err(ValidationDiagnostic::new(
                "invalid_column_width",
                "/width",
                Some("column_width_pixels_must_be_between_16_and_600".to_string()),
                Some(vec!["16px through 600px".to_string()]),
            ))
        }
    }
}

/// A column in the fixed-width layout branch.
#[derive(Debug, Clone)]
pub struct FixedColumn(Column);

impl FixedColumn {
    pub fn new(width: FixedColumnWidth) -> Self {
        Self(Column::new().width(ColumnWidth::Pixels(format!("{}px", width.0))))
    }

    pub fn element(mut self, element: Element) -> Self {
        self.0.elements.push(element);
        self
    }

    fn into_column(self) -> Column {
        self.0
    }
}

/// A checked Card JSON 2.0 weighted column value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnWeight(u8);

impl ColumnWeight {
    /// Accept a documented relative weight of one through five.
    pub fn new(value: u8) -> Result<Self, ValidationDiagnostic> {
        if (1..=5).contains(&value) {
            Ok(Self(value))
        } else {
            Err(ValidationDiagnostic::new(
                "invalid_column_weight",
                "/weight",
                Some("column_weight_must_be_between_1_and_5".to_string()),
                Some((1..=5).map(|value| value.to_string()).collect()),
            ))
        }
    }
}

/// A column in the weighted-width layout branch.
#[derive(Debug, Clone)]
pub struct WeightedColumn(Column);

impl WeightedColumn {
    pub fn new(weight: ColumnWeight) -> Self {
        Self(Column::new().width(ColumnWidth::Weighted).weight(weight.0))
    }

    pub fn element(mut self, element: Element) -> Self {
        self.0.elements.push(element);
        self
    }

    fn into_column(self) -> Column {
        self.0
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnTag {
    #[serde(rename = "column")]
    Column,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnWidth {
    Auto,
    Weighted,
    /// A documented fixed pixel width. Validate it with [`Card::validate`]
    /// before sending: Card JSON 2.0 accepts 16px through 600px.
    Pixels(String),
}

/// The document's fixed chart aspect-ratio vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChartAspectRatio {
    #[serde(rename = "1:1")]
    OneToOne,
    #[serde(rename = "2:1")]
    TwoToOne,
    #[serde(rename = "4:3")]
    FourToThree,
    #[serde(rename = "16:9")]
    SixteenToNine,
}
impl Serialize for ColumnWidth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Auto => "auto",
            Self::Weighted => "weighted",
            Self::Pixels(value) => value,
        })
    }
}
impl<'de> Deserialize<'de> for ColumnWidth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "auto" => Self::Auto,
            "weighted" => Self::Weighted,
            _ => Self::Pixels(value),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Form {
    pub tag: FormTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(default)]
    pub elements: Vec<Element>,
}
impl Form {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            tag: FormTag::Form,
            element_id: None,
            name: name.into(),
            direction: None,
            horizontal_spacing: None,
            vertical_spacing: None,
            horizontal_align: None,
            vertical_align: None,
            padding: None,
            margin: None,
            elements: Vec::new(),
        }
    }
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.element_id = Some(id.into());
        self
    }
    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormTag {
    #[serde(rename = "form")]
    Form,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Button {
    pub tag: ButtonTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub button_type: Option<ButtonType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ControlSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_action_type: Option<FormActionType>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub behaviors: Vec<Behavior>,
}
impl Button {
    pub fn new(text: Text) -> Self {
        Self {
            tag: ButtonTag::Button,
            element_id: None,
            text: Some(text),
            button_type: None,
            size: None,
            width: None,
            icon: None,
            disabled: None,
            disabled_tips: None,
            hover_tips: None,
            confirm: None,
            margin: None,
            name: None,
            form_action_type: None,
            behaviors: Vec::new(),
        }
    }
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.element_id = Some(id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn form_action(mut self, action: FormActionType) -> Self {
        self.form_action_type = Some(action);
        self
    }
    pub fn behavior(mut self, behavior: Behavior) -> Self {
        self.behaviors.push(behavior);
        self
    }
    pub fn button_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = Some(button_type);
        self
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonTag {
    #[serde(rename = "button")]
    Button,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormActionType {
    Submit,
    Reset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonType {
    Default,
    Primary,
    Danger,
    Text,
    PrimaryText,
    DangerText,
    PrimaryFilled,
    DangerFilled,
    Laser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlSize {
    Tiny,
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub tag: InputTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_position: Option<LabelPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<InputType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_resize: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rows: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_icon: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_tips: Option<Text>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub behaviors: Vec<Behavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl Input {
    pub fn new(name: impl Into<String>) -> Self {
        Self::unnamed().name(name)
    }
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.element_id = Some(id.into());
        self
    }
    pub fn unnamed() -> Self {
        Self {
            tag: InputTag::Input,
            element_id: None,
            name: None,
            placeholder: None,
            label: None,
            label_position: None,
            required: None,
            default_value: None,
            input_type: None,
            rows: None,
            auto_resize: None,
            max_rows: None,
            max_length: None,
            show_icon: None,
            width: None,
            disabled: None,
            disabled_tips: None,
            hover_tips: None,
            behaviors: Vec::new(),
            confirm: None,
            margin: None,
        }
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn required(mut self, required: bool) -> Self {
        self.required = Some(required);
        self
    }
    pub fn placeholder(mut self, placeholder: Text) -> Self {
        self.placeholder = Some(placeholder);
        self
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputTag {
    #[serde(rename = "input")]
    Input,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelPosition {
    Top,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputType {
    Text,
    MultilineText,
    Password,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Behavior {
    Callback {
        value: JsonValue,
    },
    OpenUrl {
        default_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pc_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ios_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        android_url: Option<String>,
    },
}
impl Behavior {
    pub fn callback(value: impl Into<JsonValue>) -> Self {
        Self::Callback {
            value: value.into(),
        }
    }
    pub fn open_url(default_url: impl Into<String>) -> Self {
        Self::OpenUrl {
            default_url: default_url.into(),
            pc_url: None,
            ios_url: None,
            android_url: None,
        }
    }
}

/// The closed Card JSON 2.0 component discriminator set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ComponentTag {
    Div,
    Img,
    ImgCombination,
    Person,
    PersonList,
    Chart,
    Table,
    Hr,
    CollapsiblePanel,
    InteractiveContainer,
    Overflow,
    SelectStatic,
    MultiSelectStatic,
    SelectPerson,
    MultiSelectPerson,
    DatePicker,
    PickerTime,
    PickerDatetime,
    SelectImg,
    Checker,
}

/// A labelled text field used by [`Div`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_short: Option<bool>,
    pub text: Text,
}

impl TextField {
    pub fn new(text: Text) -> Self {
        Self {
            is_short: None,
            text,
        }
    }
    pub fn short(mut self, is_short: bool) -> Self {
        self.is_short = Some(is_short);
        self
    }
}

/// A regular text component with optional icon and label/value fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Div {
    tag: ComponentTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub fields: Vec<TextField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}

impl Div {
    pub fn new(text: Text) -> Self {
        Self {
            tag: ComponentTag::Div,
            element_id: None,
            text: Some(text),
            fields: Vec::new(),
            icon: None,
            width: None,
            margin: None,
        }
    }
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.element_id = Some(id.into());
        self
    }
    pub fn field(mut self, field: TextField) -> Self {
        self.fields.push(field);
        self
    }
    pub fn icon(mut self, icon: HeaderIcon) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
    tag: ComponentTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub img_key: String,
    pub alt: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<ImageScale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl Image {
    pub fn new(img_key: impl Into<String>, alt: Text) -> Self {
        Self {
            tag: ComponentTag::Img,
            element_id: None,
            img_key: img_key.into(),
            alt,
            title: None,
            scale_type: None,
            size: None,
            corner_radius: None,
            transparent: None,
            preview: None,
            margin: None,
        }
    }
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.element_id = Some(id.into());
        self
    }
    pub fn scale_type(mut self, value: ImageScale) -> Self {
        self.scale_type = Some(value);
        self
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageScale {
    CropCenter,
    CropTop,
    FitHorizontal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageReference {
    pub img_key: String,
}
impl ImageReference {
    pub fn new(img_key: impl Into<String>) -> Self {
        Self {
            img_key: img_key.into(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageCombinationMode {
    Double,
    Triple,
    Bisect,
    Trisect,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageCombination {
    tag: ComponentTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub combination_mode: ImageCombinationMode,
    pub img_list: Vec<ImageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combination_transparent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl ImageCombination {
    pub fn new(mode: ImageCombinationMode) -> Self {
        Self {
            tag: ComponentTag::ImgCombination,
            element_id: None,
            combination_mode: mode,
            img_list: Vec::new(),
            combination_transparent: None,
            corner_radius: None,
            margin: None,
        }
    }
    pub fn image(mut self, image: ImageReference) -> Self {
        self.img_list.push(image);
        self
    }
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.element_id = Some(id.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonStyle {
    Normal,
    Capsule,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Person {
    tag: ComponentTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<PersonSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_avatar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<PersonStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl Person {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            tag: ComponentTag::Person,
            element_id: None,
            user_id: user_id.into(),
            size: None,
            show_avatar: None,
            show_name: None,
            style: None,
            margin: None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonReference {
    pub id: String,
}
impl PersonReference {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonList {
    tag: ComponentTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub persons: Vec<PersonReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_avatar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<PersonSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_invalid_user_id: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ud_icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl PersonList {
    pub fn new() -> Self {
        Self {
            tag: ComponentTag::PersonList,
            element_id: None,
            persons: Vec::new(),
            show_name: None,
            show_avatar: None,
            size: None,
            lines: None,
            drop_invalid_user_id: None,
            icon: None,
            ud_icon: None,
            margin: None,
        }
    }
    pub fn person(mut self, person: PersonReference) -> Self {
        self.persons.push(person);
        self
    }
}
impl Default for PersonList {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartColorTheme {
    Brand,
    Rainbow,
    Complementary,
    Converse,
    Primary,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chart {
    tag: ComponentTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub chart_spec: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<ChartAspectRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_theme: Option<ChartColorTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl Chart {
    pub fn new(chart_spec: JsonValue) -> Self {
        Self {
            tag: ComponentTag::Chart,
            element_id: None,
            chart_spec,
            aspect_ratio: None,
            color_theme: None,
            height: None,
            preview: None,
            margin: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableDataType {
    Text,
    LarkMd,
    Number,
    Options,
    Persons,
    Date,
    Markdown,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableColumn {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<TableDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<NumberFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
}
impl TableColumn {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            display_name: None,
            data_type: None,
            width: None,
            horizontal_align: None,
            vertical_align: None,
            format: None,
            date_format: None,
        }
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NumberFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableHeaderStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<TableHeaderBackground>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<u32>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableHeaderBackground {
    Grey,
    None,
}

/// The documented table row-height options, with a pixel escape hatch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableRowHeight {
    Low,
    Middle,
    High,
    Auto,
    Pixels(String),
}

impl Serialize for TableRowHeight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Low => "low",
            Self::Middle => "middle",
            Self::High => "high",
            Self::Auto => "auto",
            Self::Pixels(value) => value,
        })
    }
}

impl<'de> Deserialize<'de> for TableRowHeight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "low" => Self::Low,
            "middle" => Self::Middle,
            "high" => Self::High,
            "auto" => Self::Auto,
            _ => Self::Pixels(value),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    tag: ComponentTag,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<BTreeMap<String, JsonValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_height: Option<TableRowHeight>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_max_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_first_column: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_style: Option<TableHeaderStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl Table {
    pub fn new() -> Self {
        Self {
            tag: ComponentTag::Table,
            columns: Vec::new(),
            rows: Vec::new(),
            page_size: None,
            row_height: None,
            row_max_height: None,
            freeze_first_column: None,
            header_style: None,
            margin: None,
        }
    }
    pub fn column(mut self, column: TableColumn) -> Self {
        self.columns.push(column);
        self
    }
    pub fn row(mut self, row: BTreeMap<String, JsonValue>) -> Self {
        self.rows.push(row);
        self
    }
}
impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hr {
    tag: ComponentTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl Hr {
    pub fn new() -> Self {
        Self {
            tag: ComponentTag::Hr,
            element_id: None,
            margin: None,
        }
    }
}
impl Default for Hr {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollapsiblePanelHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<PanelHeaderWidth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_position: Option<IconPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_expanded_angle: Option<PanelIconExpandedAngle>,
}
impl CollapsiblePanelHeader {
    pub fn new(title: Text) -> Self {
        Self {
            title: Some(title),
            background_color: None,
            width: None,
            vertical_align: None,
            icon: None,
            icon_position: None,
            icon_expanded_angle: None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelHeaderWidth {
    Fill,
    Auto,
    AutoWhenFold,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IconPosition {
    Left,
    Right,
    FollowText,
}

/// The only rotation angles supported by a collapsible-panel header icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PanelIconExpandedAngle {
    #[serde(rename = "-180")]
    NegativeOneEighty,
    #[serde(rename = "-90")]
    NegativeNinety,
    #[serde(rename = "90")]
    Ninety,
    #[serde(rename = "180")]
    OneEighty,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollapsiblePanel {
    tag: ComponentTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub header: CollapsiblePanelHeader,
    #[serde(default)]
    pub elements: Vec<Element>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<Border>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
impl CollapsiblePanel {
    pub fn new(header: CollapsiblePanelHeader) -> Self {
        Self {
            tag: ComponentTag::CollapsiblePanel,
            element_id: None,
            header,
            elements: Vec::new(),
            expanded: None,
            background_color: None,
            border: None,
            direction: None,
            vertical_spacing: None,
            horizontal_spacing: None,
            padding: None,
            margin: None,
        }
    }
    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Border {
    pub color: Color,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
}

/// A column or column-set click target.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NavigationAction {
    pub multi_url: MultiUrl,
}

impl NavigationAction {
    pub fn new(multi_url: MultiUrl) -> Self {
        Self { multi_url }
    }
}

impl Border {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            corner_radius: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InteractiveContainer {
    tag: ComponentTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    pub elements: Vec<Element>,
    pub behaviors: Vec<Behavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<HorizontalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<BackgroundStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_border: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
}
impl InteractiveContainer {
    pub fn new(behavior: Behavior) -> Self {
        Self {
            tag: ComponentTag::InteractiveContainer,
            element_id: None,
            elements: Vec::new(),
            behaviors: vec![behavior],
            width: None,
            height: None,
            direction: None,
            horizontal_align: None,
            vertical_align: None,
            background_style: None,
            has_border: None,
            border_color: None,
            corner_radius: None,
            padding: None,
            margin: None,
            disabled: None,
            disabled_tips: None,
            hover_tips: None,
            confirm: None,
        }
    }
    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }
    pub fn behavior(mut self, behavior: Behavior) -> Self {
        self.behaviors.push(behavior);
        self
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundStyle {
    Default,
    Laser,
    Color(Color),
    Rgba(String),
}
impl Serialize for BackgroundStyle {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Default => s.serialize_str("default"),
            Self::Laser => s.serialize_str("laser"),
            Self::Color(v) => v.serialize(s),
            Self::Rgba(v) => s.serialize_str(v),
        }
    }
}
impl<'de> Deserialize<'de> for BackgroundStyle {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = String::deserialize(d)?;
        Ok(match v.as_str() {
            "default" => Self::Default,
            "laser" => Self::Laser,
            _ if v.starts_with("rgba(") => Self::Rgba(v),
            _ => Self::Color(
                serde_json::from_value(serde_json::Value::String(v))
                    .map_err(serde::de::Error::custom)?,
            ),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Confirm {
    pub title: Text,
    pub text: Text,
}
impl Confirm {
    pub fn new(title: Text, text: Text) -> Self {
        Self { title, text }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectOption {
    pub text: Text,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_url: Option<MultiUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
}
impl SelectOption {
    pub fn new(text: Text, value: impl Into<String>) -> Self {
        Self {
            text,
            value: value.into(),
            multi_url: None,
            icon: None,
        }
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Control {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_tips: Option<Text>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub behaviors: Vec<Behavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectType {
    Default,
    Text,
}
macro_rules! control_element { ($name:ident, $tag:ident, { $($field:ident : $type:ty),* $(,)? }) => { #[derive(Debug, Clone, Serialize, Deserialize)] #[serde(deny_unknown_fields)] pub struct $name { tag: ComponentTag, #[serde(flatten)] pub control: Control, $(#[serde(skip_serializing_if = "Option::is_none")] pub $field: Option<$type>,)* } impl $name { pub fn new() -> Self { Self { tag: ComponentTag::$tag, control: Control::default(), $($field: None,)* } } } impl Default for $name { fn default() -> Self { Self::new() } } }; }
control_element!(Overflow, Overflow, { options: Vec<SelectOption> });
control_element!(StaticSelect, SelectStatic, { options: Vec<SelectOption>, initial_option: String, initial_index: u32, r#type: SelectType });
control_element!(MultiStaticSelect, MultiSelectStatic, { options: Vec<SelectOption>, selected_values: Vec<String>, r#type: SelectType });
control_element!(PersonSelect, SelectPerson, { options: Vec<PersonOption>, initial_option: String, r#type: SelectType });
control_element!(MultiPersonSelect, MultiSelectPerson, { options: Vec<PersonOption>, selected_values: Vec<String>, r#type: SelectType });
control_element!(DatePicker, DatePicker, { initial_date: String });
control_element!(TimePicker, PickerTime, { initial_time: String });
control_element!(DatetimePicker, PickerDatetime, { initial_datetime: String });
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonOption {
    pub value: String,
}
impl PersonOption {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageSelectOption {
    pub img_key: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_tips: Option<Text>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageSelectLayout {
    Stretch,
    Bisect,
    Trisect,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageSelectAspectRatio {
    #[serde(rename = "1:1")]
    OneToOne,
    #[serde(rename = "4:3")]
    FourToThree,
    #[serde(rename = "16:9")]
    SixteenToNine,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageSelect {
    tag: ComponentTag,
    #[serde(flatten)]
    pub control: Control,
    pub options: Vec<ImageSelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_select: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<ImageSelectLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<ImageSelectAspectRatio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
}
impl ImageSelect {
    pub fn new(options: Vec<ImageSelectOption>, behavior: Behavior) -> Self {
        Self {
            tag: ComponentTag::SelectImg,
            control: Control {
                behaviors: vec![behavior],
                ..Control::default()
            },
            options,
            multi_select: None,
            layout: None,
            aspect_ratio: None,
            can_preview: None,
            value: None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checker {
    tag: ComponentTag,
    #[serde(flatten)]
    pub control: Control,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_checkable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_area: Option<CheckerButtonArea>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_style: Option<CheckedStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckedStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f32>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckerButtonArea {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_display_rule: Option<CheckerButtonDisplayRule>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub buttons: Vec<Button>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckerButtonDisplayRule {
    Always,
    OnHover,
}
impl Checker {
    pub fn new() -> Self {
        Self {
            tag: ComponentTag::Checker,
            control: Control::default(),
            checked: None,
            text: None,
            overall_checkable: None,
            button_area: None,
            checked_style: None,
            padding: None,
        }
    }
}
impl Default for Checker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    InvalidSchema,
    V2RequiresSharedCard,
    MissingBody,
    InvalidCardLink,
    StreamingConfigRequiresStreamingMode,
    HeaderTagRequiresPlainText,
    InvalidHeaderTitleLines(u32),
    InvalidHeaderSubtitleLines(u32),
    InvalidSpacing(String),
    InvalidPadding(String),
    InvalidMargin(String),
    EmptyColumnSet,
    InvalidColumnWidth(String),
    InvalidColumnWeight(u8),
    ColumnWidthRequiresFixedFlexMode,
    InvalidDivWidth(String),
    ImageSizeRequiresCropScale,
    TooManyImagesInCombination {
        mode: ImageCombinationMode,
        count: usize,
    },
    InvalidPersonListLines,
    InvalidChartSpec,
    InvalidChartHeight(String),
    InvalidElementId(String),
    DuplicateElementId(String),
    TooManyElements(usize),
    EmptyForm(String),
    InvalidFormName(String),
    DuplicateFormName(String),
    FormNestedOutsideBody,
    TableNestedOutsideBody,
    MultiSelectImageOutsideForm,
    EmptyInteractiveContainer,
    MissingInteractiveContainerBehavior,
    InvalidOpenUrl(String),
    InvalidInteractiveContainerWidth(String),
    InvalidInteractiveContainerHeight(String),
    InvalidCornerRadius(String),
    EmptyOptions(&'static str),
    DuplicateOptionValue(String),
    MissingPickerValue(&'static str),
    MissingFormControlName(&'static str),
    ButtonBehaviorConflict,
    FormActionOutsideForm,
    MissingButtonBehavior,
    TooManyCheckerButtons(usize),
    TooManyHeaderTags(usize),
    TooManyTables(usize),
    EmptyTableColumns,
    EmptyTableRows,
    TooManyTableColumns(usize),
    DuplicateTableColumn(String),
    InvalidTablePageSize(u8),
    InvalidTableColumnWidth(String),
    InvalidTableRowHeight(String),
    InvalidTableRowMaxHeight(String),
    TableRowMaxHeightRequiresAutoRowHeight,
    UnknownTableRowColumn(String),
    DuplicateFormControlName(String),
    TooDeeplyNestedContainer(usize),
    MissingFormSubmit(String),
    MissingFormButtonAction,
    ButtonTextRequiresPlainText,
    ButtonTextTooLong(usize),
    PlainTextRequired(&'static str),
    TextTooLong {
        field: &'static str,
        length: usize,
    },
    InvalidControlWidth(String),
    InvalidInputMaxLength(u32),
    InvalidInitialOption(&'static str, String),
    InvalidInitialIndex(u32),
    InvalidPickerInitialValue(&'static str, String),
    MissingImageSelectBehavior,
    ImagePreviewOutsideForm,
}

impl ValidationError {
    /// Stable machine-readable identifier for this validation rule.
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidSchema => "invalid_schema",
            Self::V2RequiresSharedCard => "v2_requires_shared_card",
            Self::MissingBody => "missing_body",
            Self::InvalidCardLink => "invalid_card_link",
            Self::StreamingConfigRequiresStreamingMode => {
                "streaming_config_requires_streaming_mode"
            }
            Self::HeaderTagRequiresPlainText => "header_tag_requires_plain_text",
            Self::InvalidHeaderTitleLines(_) => "invalid_header_title_lines",
            Self::InvalidHeaderSubtitleLines(_) => "invalid_header_subtitle_lines",
            Self::InvalidSpacing(_) => "invalid_spacing",
            Self::InvalidPadding(_) => "invalid_padding",
            Self::InvalidMargin(_) => "invalid_margin",
            Self::EmptyColumnSet => "empty_column_set",
            Self::InvalidColumnWidth(_) => "invalid_column_width",
            Self::InvalidColumnWeight(_) => "invalid_column_weight",
            Self::ColumnWidthRequiresFixedFlexMode => "column_layout_requires_fixed_flex_mode",
            Self::InvalidDivWidth(_) => "invalid_div_width",
            Self::ImageSizeRequiresCropScale => "image_size_requires_crop_scale",
            Self::TooManyImagesInCombination { .. } => "too_many_images_in_combination",
            Self::InvalidPersonListLines => "invalid_person_list_lines",
            Self::InvalidChartSpec => "invalid_chart_spec",
            Self::InvalidChartHeight(_) => "invalid_chart_height",
            Self::InvalidElementId(_) => "invalid_element_id",
            Self::DuplicateElementId(_) => "duplicate_element_id",
            Self::TooManyElements(_) => "too_many_elements",
            Self::EmptyForm(_) => "empty_form",
            Self::InvalidFormName(_) => "invalid_form_name",
            Self::DuplicateFormName(_) => "duplicate_form_name",
            Self::FormNestedOutsideBody => "form_nested_outside_body",
            Self::TableNestedOutsideBody => "table_nested_outside_body",
            Self::MultiSelectImageOutsideForm => "multi_select_image_outside_form",
            Self::EmptyInteractiveContainer => "empty_interactive_container",
            Self::MissingInteractiveContainerBehavior => "missing_interactive_container_behavior",
            Self::InvalidOpenUrl(_) => "invalid_open_url",
            Self::InvalidInteractiveContainerWidth(_) => "invalid_interactive_container_width",
            Self::InvalidInteractiveContainerHeight(_) => "invalid_interactive_container_height",
            Self::InvalidCornerRadius(_) => "invalid_corner_radius",
            Self::EmptyOptions(_) => "empty_options",
            Self::DuplicateOptionValue(_) => "duplicate_option_value",
            Self::MissingPickerValue(_) => "missing_picker_value",
            Self::MissingFormControlName(_) => "missing_form_control_name",
            Self::ButtonBehaviorConflict => "button_behavior_conflict",
            Self::FormActionOutsideForm => "form_action_outside_form",
            Self::MissingButtonBehavior => "missing_button_behavior",
            Self::TooManyCheckerButtons(_) => "too_many_checker_buttons",
            Self::TooManyHeaderTags(_) => "too_many_header_tags",
            Self::TooManyTables(_) => "too_many_tables",
            Self::EmptyTableColumns => "empty_table_columns",
            Self::EmptyTableRows => "empty_table_rows",
            Self::TooManyTableColumns(_) => "too_many_table_columns",
            Self::DuplicateTableColumn(_) => "duplicate_table_column",
            Self::InvalidTablePageSize(_) => "invalid_table_page_size",
            Self::InvalidTableColumnWidth(_) => "invalid_table_column_width",
            Self::InvalidTableRowHeight(_) => "invalid_table_row_height",
            Self::InvalidTableRowMaxHeight(_) => "invalid_table_row_max_height",
            Self::TableRowMaxHeightRequiresAutoRowHeight => {
                "table_row_max_height_requires_auto_row_height"
            }
            Self::UnknownTableRowColumn(_) => "unknown_table_row_column",
            Self::DuplicateFormControlName(_) => "duplicate_form_control_name",
            Self::TooDeeplyNestedContainer(_) => "too_deeply_nested_container",
            Self::MissingFormSubmit(_) => "missing_form_submit",
            Self::MissingFormButtonAction => "missing_form_button_action",
            Self::ButtonTextRequiresPlainText => "button_text_requires_plain_text",
            Self::ButtonTextTooLong(_) => "button_text_too_long",
            Self::PlainTextRequired(_) => "plain_text_required",
            Self::TextTooLong { .. } => "text_too_long",
            Self::InvalidControlWidth(_) => "invalid_control_width",
            Self::InvalidInputMaxLength(_) => "invalid_input_max_length",
            Self::InvalidInitialOption(_, _) => "invalid_initial_option",
            Self::InvalidInitialIndex(_) => "invalid_initial_index",
            Self::InvalidPickerInitialValue(_, _) => "invalid_picker_initial_value",
            Self::MissingImageSelectBehavior => "missing_image_select_behavior",
            Self::ImagePreviewOutsideForm => "image_preview_outside_form",
        }
    }

    /// A stable, machine-readable description of this rule.
    ///
    /// This method is available even when the error originated from a local
    /// component validator.  For a concrete JSON Pointer in a Card document,
    /// use [`Card::validate_with_diagnostic`].
    pub fn diagnostic(&self) -> ValidationDiagnostic {
        match self {
            Self::InvalidMargin(_) => ValidationDiagnostic::new(
                self.code(),
                "/body/elements/*/margin",
                Some("margin_pixels_must_be_between_negative_99_and_99".to_string()),
                Some(vec!["-99px through 99px".to_string()]),
            ),
            Self::InvalidPadding(_) => ValidationDiagnostic::new(
                self.code(),
                "/body/padding",
                Some("padding_pixels_must_be_between_0_and_99".to_string()),
                Some(vec!["0px through 99px".to_string()]),
            ),
            Self::InvalidSpacing(_) => ValidationDiagnostic::new(
                self.code(),
                "/body/horizontal_spacing",
                Some("spacing_pixels_must_be_between_0_and_99".to_string()),
                Some(
                    vec![
                        "small",
                        "medium",
                        "large",
                        "extra_large",
                        "0px through 99px",
                    ]
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                ),
            ),
            Self::InvalidColumnWidth(_) => ValidationDiagnostic::new(
                self.code(),
                "/body/elements/*/columns/*/width",
                Some("column_width_must_be_auto_weighted_or_16_to_600px".to_string()),
                Some(
                    vec!["auto", "weighted", "16px through 600px"]
                        .into_iter()
                        .map(str::to_string)
                        .collect(),
                ),
            ),
            Self::InvalidColumnWeight(_) => ValidationDiagnostic::new(
                self.code(),
                "/body/elements/*/columns/*/weight",
                Some("weight_requires_weighted_width_and_value_1_through_5".to_string()),
                Some((1..=5).map(|value| value.to_string()).collect()),
            ),
            Self::ColumnWidthRequiresFixedFlexMode => ValidationDiagnostic::new(
                self.code(),
                "/body/elements/*/flex_mode",
                Some("width_or_weight_requires_flex_mode_none".to_string()),
                Some(vec!["none".to_string()]),
            ),
            _ => ValidationDiagnostic::new(self.code(), "/", Some(self.code().to_string()), None),
        }
    }
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSchema => f.write_str("Card JSON v2 requires schema 2.0"),
            Self::V2RequiresSharedCard => f.write_str("Card JSON v2 supports shared cards only"),
            Self::MissingBody => f.write_str("Card JSON v2 requires body.elements"),
            Self::InvalidCardLink => f.write_str("card_link requires a non-empty url"),
            Self::StreamingConfigRequiresStreamingMode => {
                f.write_str("streaming_config requires streaming_mode true")
            }
            Self::HeaderTagRequiresPlainText => f.write_str("header text tags require plain_text"),
            Self::InvalidHeaderTitleLines(lines) => {
                write!(f, "header title supports at most four lines, got {lines}")
            }
            Self::InvalidHeaderSubtitleLines(lines) => {
                write!(f, "header subtitle supports at most one line, got {lines}")
            }
            Self::InvalidSpacing(value) => write!(f, "invalid spacing {value:?}"),
            Self::InvalidPadding(value) => write!(f, "invalid padding {value:?}"),
            Self::InvalidMargin(value) => write!(f, "invalid margin {value:?}"),
            Self::EmptyColumnSet => f.write_str("column_set requires at least one column"),
            Self::InvalidColumnWidth(value) => write!(f, "invalid column width {value:?}"),
            Self::InvalidColumnWeight(weight) => {
                write!(
                    f,
                    "column weight requires width weighted and a value from 1 through 5, got {weight}"
                )
            }
            Self::ColumnWidthRequiresFixedFlexMode => {
                f.write_str("column width and weight require column_set flex_mode none")
            }
            Self::InvalidDivWidth(value) => write!(f, "invalid div width {value:?}"),
            Self::ImageSizeRequiresCropScale => {
                f.write_str("img size requires crop_center or crop_top scale_type")
            }
            Self::TooManyImagesInCombination { mode, count } => {
                write!(
                    f,
                    "img_combination {mode:?} supports at most {} images, got {count}",
                    image_combination_limit(*mode)
                )
            }
            Self::InvalidPersonListLines => f.write_str("person_list lines must not be zero"),
            Self::InvalidChartSpec => f.write_str("chart_spec must be a JSON object"),
            Self::InvalidChartHeight(value) => write!(f, "invalid chart height {value:?}"),
            Self::InvalidElementId(id) => write!(f, "invalid element_id {id:?}"),
            Self::DuplicateElementId(id) => write!(f, "duplicate element_id {id:?}"),
            Self::TooManyElements(count) => {
                write!(f, "Card JSON v2 supports at most 200 elements, got {count}")
            }
            Self::EmptyForm(name) => write!(f, "form {name:?} has no elements"),
            Self::InvalidFormName(name) => write!(f, "invalid form name {name:?}"),
            Self::DuplicateFormName(name) => write!(f, "duplicate form name {name:?}"),
            Self::FormNestedOutsideBody => f.write_str("form may only appear in body.elements"),
            Self::TableNestedOutsideBody => f.write_str("table may only appear in body.elements"),
            Self::MultiSelectImageOutsideForm => {
                f.write_str("select_img.multi_select requires a form")
            }
            Self::EmptyInteractiveContainer => {
                f.write_str("interactive_container requires at least one child element")
            }
            Self::MissingInteractiveContainerBehavior => {
                f.write_str("interactive_container requires at least one behavior")
            }
            Self::InvalidOpenUrl(url) => {
                write!(f, "open_url requires a non-empty default_url, got {url:?}")
            }
            Self::InvalidInteractiveContainerWidth(width) => {
                write!(f, "invalid interactive_container width {width:?}")
            }
            Self::InvalidInteractiveContainerHeight(height) => {
                write!(f, "invalid interactive_container height {height:?}")
            }
            Self::InvalidCornerRadius(value) => write!(f, "invalid corner radius {value:?}"),
            Self::EmptyOptions(tag) => write!(f, "{tag} requires at least one option"),
            Self::DuplicateOptionValue(value) => write!(f, "duplicate option value {value:?}"),
            Self::MissingPickerValue(tag) => {
                write!(f, "{tag} requires initial value or placeholder")
            }
            Self::MissingFormControlName(tag) => {
                write!(f, "{tag} requires name inside a form")
            }
            Self::ButtonBehaviorConflict => {
                f.write_str("form button must use form_action_type instead of behaviors")
            }
            Self::FormActionOutsideForm => {
                f.write_str("form_action_type is only valid for buttons inside a form")
            }
            Self::MissingButtonBehavior => f.write_str("button outside a form requires a behavior"),
            Self::TooManyCheckerButtons(count) => {
                write!(
                    f,
                    "checker button_area supports at most three buttons, got {count}"
                )
            }
            Self::TooManyHeaderTags(count) => {
                write!(f, "header supports at most three text tags, got {count}")
            }
            Self::TooManyTables(count) => {
                write!(f, "Card JSON v2 supports at most five tables, got {count}")
            }
            Self::EmptyTableColumns => f.write_str("table requires at least one column"),
            Self::EmptyTableRows => f.write_str("table requires at least one row"),
            Self::TooManyTableColumns(count) => {
                write!(f, "table supports at most 50 columns, got {count}")
            }
            Self::DuplicateTableColumn(column) => write!(f, "duplicate table column {column:?}"),
            Self::InvalidTablePageSize(size) => {
                write!(f, "table page_size must be 1 through 10, got {size}")
            }
            Self::InvalidTableColumnWidth(value) => {
                write!(f, "invalid table column width {value:?}")
            }
            Self::InvalidTableRowHeight(value) => write!(f, "invalid table row_height {value:?}"),
            Self::InvalidTableRowMaxHeight(value) => {
                write!(f, "invalid table row_max_height {value:?}")
            }
            Self::TableRowMaxHeightRequiresAutoRowHeight => {
                f.write_str("table row_max_height requires row_height auto")
            }
            Self::UnknownTableRowColumn(column) => {
                write!(f, "table row has no declared column {column:?}")
            }
            Self::DuplicateFormControlName(name) => {
                write!(f, "duplicate form control name {name:?}")
            }
            Self::TooDeeplyNestedContainer(depth) => {
                write!(f, "containers support at most five levels, got {depth}")
            }
            Self::MissingFormSubmit(name) => {
                write!(f, "form {name:?} requires a submit button")
            }
            Self::MissingFormButtonAction => {
                f.write_str("button inside a form requires form_action_type")
            }
            Self::ButtonTextRequiresPlainText => f.write_str("button text requires plain_text"),
            Self::ButtonTextTooLong(length) => {
                write!(
                    f,
                    "button text supports at most 100 characters, got {length}"
                )
            }
            Self::PlainTextRequired(field) => write!(f, "{field} requires plain_text"),
            Self::TextTooLong { field, length } => {
                write!(
                    f,
                    "{field} exceeds its maximum length with {length} characters"
                )
            }
            Self::InvalidControlWidth(width) => write!(f, "invalid control width {width:?}"),
            Self::InvalidInputMaxLength(length) => {
                write!(f, "input max_length must be 1 through 1000, got {length}")
            }
            Self::InvalidInitialOption(tag, value) => {
                write!(f, "{tag} initial option {value:?} is not in options")
            }
            Self::InvalidInitialIndex(index) => {
                write!(f, "select_static initial_index {index} is outside options")
            }
            Self::InvalidPickerInitialValue(tag, value) => {
                write!(f, "invalid {tag} initial value {value:?}")
            }
            Self::MissingImageSelectBehavior => {
                f.write_str("select_img requires at least one behavior")
            }
            Self::ImagePreviewOutsideForm => {
                f.write_str("select_img.can_preview is only supported inside a form")
            }
        }
    }
}
impl std::error::Error for ValidationError {}

#[derive(Default)]
struct FormValidationState {
    names: BTreeSet<String>,
}

fn find_column_path(elements: &[Element], error: &ValidationError, base: &str) -> Option<String> {
    for (element_index, element) in elements.iter().enumerate() {
        let element_path = format!("{base}/{element_index}");
        if let Element::ColumnSet(column_set) = element {
            for (column_index, column) in column_set.columns.iter().enumerate() {
                let column_path = format!("{element_path}/columns/{column_index}");
                match error {
                    ValidationError::InvalidColumnWidth(value) if matches!(&column.width, Some(ColumnWidth::Pixels(width)) if width == value) =>
                    {
                        return Some(format!("{column_path}/width"));
                    }
                    ValidationError::InvalidColumnWeight(weight)
                        if column.weight == Some(*weight)
                            && (!(1..=5).contains(weight)
                                || !matches!(column.width, Some(ColumnWidth::Weighted))) =>
                    {
                        return Some(format!("{column_path}/weight"));
                    }
                    _ => {}
                }
                if let Some(path) =
                    find_column_path(&column.elements, error, &format!("{column_path}/elements"))
                {
                    return Some(path);
                }
            }
        }
        if let Some(children) = element_children(element)
            && let Some(path) =
                find_column_path(children, error, &format!("{element_path}/elements"))
        {
            return Some(path);
        }
    }
    None
}

fn find_column_set_flex_mode_path(elements: &[Element], base: &str) -> Option<String> {
    for (element_index, element) in elements.iter().enumerate() {
        let element_path = format!("{base}/{element_index}");
        if let Element::ColumnSet(column_set) = element {
            if column_set
                .flex_mode
                .is_some_and(|mode| mode != ColumnFlexMode::None)
                && column_set
                    .columns
                    .iter()
                    .any(|column| column.width.is_some() || column.weight.is_some())
            {
                return Some(format!("{element_path}/flex_mode"));
            }
            for (column_index, column) in column_set.columns.iter().enumerate() {
                if let Some(path) = find_column_set_flex_mode_path(
                    &column.elements,
                    &format!("{element_path}/columns/{column_index}/elements"),
                ) {
                    return Some(path);
                }
            }
        }
        if let Some(children) = element_children(element)
            && let Some(path) =
                find_column_set_flex_mode_path(children, &format!("{element_path}/elements"))
        {
            return Some(path);
        }
    }
    None
}

fn element_children(element: &Element) -> Option<&[Element]> {
    match element {
        Element::CollapsiblePanel(panel) => Some(&panel.elements),
        Element::Form(form) => Some(&form.elements),
        Element::InteractiveContainer(container) => Some(&container.elements),
        _ => None,
    }
}

fn find_element_error_path(
    elements: &[Element],
    error: &ValidationError,
    base: &str,
) -> Option<String> {
    for (index, element) in elements.iter().enumerate() {
        let path = format!("{base}/{index}");
        if let Some(field) = element_error_field(element, error) {
            return Some(format!("{path}/{field}"));
        }
        if let Element::ColumnSet(column_set) = element {
            for (column_index, column) in column_set.columns.iter().enumerate() {
                let column_path = format!("{path}/columns/{column_index}");
                if matches!(error, ValidationError::EmptyColumnSet) {
                    return Some(format!("{path}/columns"));
                }
                if let Some(path) = find_element_error_path(
                    &column.elements,
                    error,
                    &format!("{column_path}/elements"),
                ) {
                    return Some(path);
                }
            }
        }
        if let Some(children) = element_children(element)
            && let Some(path) =
                find_element_error_path(children, error, &format!("{path}/elements"))
        {
            return Some(path);
        }
    }
    None
}

fn element_error_field(element: &Element, error: &ValidationError) -> Option<&'static str> {
    match (element, error) {
        (Element::Div(_), ValidationError::InvalidDivWidth(_)) => Some("width"),
        (Element::Img(_), ValidationError::ImageSizeRequiresCropScale) => Some("scale_type"),
        (Element::Img(_), ValidationError::InvalidCornerRadius(_)) => Some("corner_radius"),
        (Element::ImgCombination(_), ValidationError::TooManyImagesInCombination { .. }) => {
            Some("img_list")
        }
        (Element::ImgCombination(_), ValidationError::InvalidCornerRadius(_)) => {
            Some("corner_radius")
        }
        (Element::PersonList(_), ValidationError::InvalidPersonListLines) => Some("lines"),
        (Element::Chart(_), ValidationError::InvalidChartSpec) => Some("chart_spec"),
        (Element::Chart(_), ValidationError::InvalidChartHeight(_)) => Some("height"),
        (Element::Table(_), ValidationError::EmptyTableColumns)
        | (Element::Table(_), ValidationError::TooManyTableColumns(_))
        | (Element::Table(_), ValidationError::DuplicateTableColumn(_))
        | (Element::Table(_), ValidationError::InvalidTableColumnWidth(_)) => Some("columns"),
        (Element::Table(_), ValidationError::EmptyTableRows)
        | (Element::Table(_), ValidationError::UnknownTableRowColumn(_)) => Some("rows"),
        (Element::Table(_), ValidationError::InvalidTablePageSize(_)) => Some("page_size"),
        (Element::Table(_), ValidationError::InvalidTableRowHeight(_)) => Some("row_height"),
        (Element::Table(_), ValidationError::InvalidTableRowMaxHeight(_))
        | (Element::Table(_), ValidationError::TableRowMaxHeightRequiresAutoRowHeight) => {
            Some("row_max_height")
        }
        (Element::ColumnSet(_), ValidationError::EmptyColumnSet) => Some("columns"),
        (Element::Form(_), ValidationError::EmptyForm(_))
        | (Element::Form(_), ValidationError::InvalidFormName(_))
        | (Element::Form(_), ValidationError::DuplicateFormName(_)) => Some("name"),
        (Element::Form(_), ValidationError::FormNestedOutsideBody) => Some("tag"),
        (Element::Form(_), ValidationError::MissingFormSubmit(_)) => Some("elements"),
        (Element::InteractiveContainer(_), ValidationError::EmptyInteractiveContainer) => {
            Some("elements")
        }
        (
            Element::InteractiveContainer(_),
            ValidationError::MissingInteractiveContainerBehavior,
        ) => Some("behaviors"),
        (
            Element::InteractiveContainer(_),
            ValidationError::InvalidInteractiveContainerWidth(_),
        ) => Some("width"),
        (
            Element::InteractiveContainer(_),
            ValidationError::InvalidInteractiveContainerHeight(_),
        ) => Some("height"),
        (Element::InteractiveContainer(_), ValidationError::InvalidCornerRadius(_)) => {
            Some("corner_radius")
        }
        (Element::Button(_), ValidationError::ButtonTextRequiresPlainText)
        | (Element::Button(_), ValidationError::ButtonTextTooLong(_)) => Some("text"),
        (Element::Button(_), ValidationError::MissingButtonBehavior) => Some("behaviors"),
        (Element::Button(_), ValidationError::MissingFormButtonAction)
        | (Element::Button(_), ValidationError::FormActionOutsideForm) => Some("form_action_type"),
        (Element::Button(_), ValidationError::ButtonBehaviorConflict) => Some("behaviors"),
        (Element::Input(_), ValidationError::InvalidInputMaxLength(_)) => Some("max_length"),
        (Element::Input(_), ValidationError::InvalidControlWidth(_)) => Some("width"),
        (Element::SelectImg(_), ValidationError::MissingImageSelectBehavior) => Some("behaviors"),
        (Element::SelectImg(_), ValidationError::ImagePreviewOutsideForm) => Some("can_preview"),
        (Element::CollapsiblePanel(_), ValidationError::InvalidCornerRadius(_)) => {
            Some("border/corner_radius")
        }
        (Element::Table(_), ValidationError::TableNestedOutsideBody) => Some("tag"),
        (Element::SelectImg(_), ValidationError::MultiSelectImageOutsideForm) => {
            Some("multi_select")
        }
        (Element::Checker(_), ValidationError::TooManyCheckerButtons(_)) => {
            Some("button_area/buttons")
        }
        (Element::Button(_), ValidationError::InvalidControlWidth(_)) => Some("width"),
        (_, ValidationError::EmptyOptions(tag)) if element_wire_tag(element) == *tag => {
            Some("options")
        }
        (_, ValidationError::MissingPickerValue(tag)) if element_wire_tag(element) == *tag => {
            Some("placeholder")
        }
        (_, ValidationError::MissingFormControlName(tag)) if element_wire_tag(element) == *tag => {
            Some("name")
        }
        (_, ValidationError::InvalidInitialOption(tag, _)) if element_wire_tag(element) == *tag => {
            Some(initial_option_field(tag))
        }
        (_, ValidationError::InvalidInitialIndex(_))
            if matches!(element, Element::SelectStatic(_)) =>
        {
            Some("initial_index")
        }
        (_, ValidationError::InvalidPickerInitialValue(tag, _))
            if element_wire_tag(element) == *tag =>
        {
            Some(initial_picker_field(tag))
        }
        (_, ValidationError::DuplicateOptionValue(_)) if element_has_options(element) => {
            Some("options")
        }
        (_, ValidationError::PlainTextRequired(field))
            if field.starts_with("input.") && matches!(element, Element::Input(_)) =>
        {
            Some("placeholder")
        }
        (_, ValidationError::TextTooLong { field, .. })
            if field.starts_with("input.") && matches!(element, Element::Input(_)) =>
        {
            Some("placeholder")
        }
        (_, ValidationError::InvalidOpenUrl(_)) if element_has_behaviors(element) => {
            Some("behaviors")
        }
        (_, ValidationError::DuplicateFormControlName(_))
            if matches!(element, Element::Form(_)) =>
        {
            Some("elements")
        }
        (_, ValidationError::TooDeeplyNestedContainer(_))
            if element_children(element).is_some() || matches!(element, Element::ColumnSet(_)) =>
        {
            Some("elements")
        }
        _ => element_id_path(element, error),
    }
}

fn element_wire_tag(element: &Element) -> &'static str {
    match element {
        Element::Div(_) => "div",
        Element::Markdown(_) => "markdown",
        Element::Img(_) => "img",
        Element::ImgCombination(_) => "img_combination",
        Element::Person(_) => "person",
        Element::PersonList(_) => "person_list",
        Element::Chart(_) => "chart",
        Element::Table(_) => "table",
        Element::Hr(_) => "hr",
        Element::ColumnSet(_) => "column_set",
        Element::CollapsiblePanel(_) => "collapsible_panel",
        Element::Form(_) => "form",
        Element::InteractiveContainer(_) => "interactive_container",
        Element::Button(_) => "button",
        Element::Input(_) => "input",
        Element::Overflow(_) => "overflow",
        Element::SelectStatic(_) => "select_static",
        Element::MultiSelectStatic(_) => "multi_select_static",
        Element::SelectPerson(_) => "select_person",
        Element::MultiSelectPerson(_) => "multi_select_person",
        Element::DatePicker(_) => "date_picker",
        Element::PickerTime(_) => "picker_time",
        Element::PickerDatetime(_) => "picker_datetime",
        Element::SelectImg(_) => "select_img",
        Element::Checker(_) => "checker",
    }
}

fn element_has_options(element: &Element) -> bool {
    matches!(
        element,
        Element::Overflow(_)
            | Element::SelectStatic(_)
            | Element::MultiSelectStatic(_)
            | Element::SelectPerson(_)
            | Element::MultiSelectPerson(_)
            | Element::SelectImg(_)
    )
}

fn element_has_behaviors(element: &Element) -> bool {
    matches!(
        element,
        Element::Button(_)
            | Element::Input(_)
            | Element::Overflow(_)
            | Element::SelectStatic(_)
            | Element::MultiSelectStatic(_)
            | Element::SelectPerson(_)
            | Element::MultiSelectPerson(_)
            | Element::DatePicker(_)
            | Element::PickerTime(_)
            | Element::PickerDatetime(_)
            | Element::SelectImg(_)
            | Element::Checker(_)
            | Element::InteractiveContainer(_)
    )
}

fn initial_option_field(tag: &str) -> &'static str {
    match tag {
        "multi_select_static" | "multi_select_person" => "selected_values",
        _ => "initial_option",
    }
}

fn initial_picker_field(tag: &str) -> &'static str {
    match tag {
        "date_picker" => "initial_date",
        "picker_time" => "initial_time",
        "picker_datetime" => "initial_datetime",
        _ => "value",
    }
}

fn element_id_path(element: &Element, error: &ValidationError) -> Option<&'static str> {
    let id = match element {
        Element::Div(value) => value.element_id.as_deref(),
        Element::Markdown(value) => value.element_id.as_deref(),
        Element::Img(value) => value.element_id.as_deref(),
        Element::ImgCombination(value) => value.element_id.as_deref(),
        Element::Person(value) => value.element_id.as_deref(),
        Element::PersonList(value) => value.element_id.as_deref(),
        Element::Chart(value) => value.element_id.as_deref(),
        Element::Hr(value) => value.element_id.as_deref(),
        Element::ColumnSet(value) => value.element_id.as_deref(),
        Element::CollapsiblePanel(value) => value.element_id.as_deref(),
        Element::Form(value) => value.element_id.as_deref(),
        Element::InteractiveContainer(value) => value.element_id.as_deref(),
        Element::Button(value) => value.element_id.as_deref(),
        Element::Input(value) => value.element_id.as_deref(),
        Element::Overflow(value) => value.control.element_id.as_deref(),
        Element::SelectStatic(value) => value.control.element_id.as_deref(),
        Element::MultiSelectStatic(value) => value.control.element_id.as_deref(),
        Element::SelectPerson(value) => value.control.element_id.as_deref(),
        Element::MultiSelectPerson(value) => value.control.element_id.as_deref(),
        Element::DatePicker(value) => value.control.element_id.as_deref(),
        Element::PickerTime(value) => value.control.element_id.as_deref(),
        Element::PickerDatetime(value) => value.control.element_id.as_deref(),
        Element::SelectImg(value) => value.control.element_id.as_deref(),
        Element::Checker(value) => value.control.element_id.as_deref(),
        Element::Table(_) => None,
    };
    matches!(error, ValidationError::InvalidElementId(value) | ValidationError::DuplicateElementId(value) if id == Some(value))
        .then_some("element_id")
}

fn find_header_layout_path(header: &Header, error: &ValidationError) -> Option<String> {
    matches!(error, ValidationError::InvalidPadding(value) if header.padding.as_deref() == Some(value))
        .then(|| "/header/padding".to_string())
}

fn find_typed_layout_path(body: &Body, error: &ValidationError) -> Option<String> {
    match error {
        ValidationError::InvalidPadding(value) if body.padding.as_deref() == Some(value) => {
            Some("/body/padding".to_string())
        }
        ValidationError::InvalidSpacing(value) if matches!(&body.horizontal_spacing, Some(Spacing::Pixels(spacing)) if spacing == value) => {
            Some("/body/horizontal_spacing".to_string())
        }
        ValidationError::InvalidSpacing(value) if matches!(&body.vertical_spacing, Some(Spacing::Pixels(spacing)) if spacing == value) => {
            Some("/body/vertical_spacing".to_string())
        }
        _ => find_element_layout_path(&body.elements, error, "/body/elements"),
    }
}

fn find_element_layout_path(
    elements: &[Element],
    error: &ValidationError,
    base: &str,
) -> Option<String> {
    for (index, element) in elements.iter().enumerate() {
        let path = format!("{base}/{index}");
        if let Some(field) = direct_layout_field(element, error) {
            return Some(format!("{path}/{field}"));
        }
        if let Element::ColumnSet(column_set) = element {
            for (column_index, column) in column_set.columns.iter().enumerate() {
                let column_path = format!("{path}/columns/{column_index}");
                if let Some(field) = column_layout_field(column, error) {
                    return Some(format!("{column_path}/{field}"));
                }
                if let Some(path) = find_element_layout_path(
                    &column.elements,
                    error,
                    &format!("{column_path}/elements"),
                ) {
                    return Some(path);
                }
            }
        }
        if let Some(children) = element_children(element)
            && let Some(path) =
                find_element_layout_path(children, error, &format!("{path}/elements"))
        {
            return Some(path);
        }
    }
    None
}

fn direct_layout_field(element: &Element, error: &ValidationError) -> Option<&'static str> {
    let margin = match element {
        Element::Div(value) => value.margin.as_deref(),
        Element::Markdown(value) => value.margin.as_deref(),
        Element::Img(value) => value.margin.as_deref(),
        Element::ImgCombination(value) => value.margin.as_deref(),
        Element::Person(value) => value.margin.as_deref(),
        Element::PersonList(value) => value.margin.as_deref(),
        Element::Chart(value) => value.margin.as_deref(),
        Element::Table(value) => value.margin.as_deref(),
        Element::Hr(value) => value.margin.as_deref(),
        Element::ColumnSet(value) => value.margin.as_deref(),
        Element::CollapsiblePanel(value) => value.margin.as_deref(),
        Element::Form(value) => value.margin.as_deref(),
        Element::InteractiveContainer(value) => value.margin.as_deref(),
        Element::Button(value) => value.margin.as_deref(),
        Element::Input(value) => value.margin.as_deref(),
        Element::Overflow(value) => value.control.margin.as_deref(),
        Element::SelectStatic(value) => value.control.margin.as_deref(),
        Element::MultiSelectStatic(value) => value.control.margin.as_deref(),
        Element::SelectPerson(value) => value.control.margin.as_deref(),
        Element::MultiSelectPerson(value) => value.control.margin.as_deref(),
        Element::DatePicker(value) => value.control.margin.as_deref(),
        Element::PickerTime(value) => value.control.margin.as_deref(),
        Element::PickerDatetime(value) => value.control.margin.as_deref(),
        Element::SelectImg(value) => value.control.margin.as_deref(),
        Element::Checker(value) => value.control.margin.as_deref(),
    };
    if matches!(error, ValidationError::InvalidMargin(value) if margin == Some(value)) {
        return Some("margin");
    }

    let padding = match element {
        Element::CollapsiblePanel(value) => value.padding.as_deref(),
        Element::Form(value) => value.padding.as_deref(),
        Element::InteractiveContainer(value) => value.padding.as_deref(),
        Element::Checker(value) => value.padding.as_deref(),
        _ => None,
    };
    if matches!(error, ValidationError::InvalidPadding(value) if padding == Some(value)) {
        return Some("padding");
    }

    let (horizontal_spacing, vertical_spacing) = match element {
        Element::ColumnSet(value) => (value.horizontal_spacing.as_ref(), None),
        Element::CollapsiblePanel(value) => (
            value.horizontal_spacing.as_ref(),
            value.vertical_spacing.as_ref(),
        ),
        Element::Form(value) => (
            value.horizontal_spacing.as_ref(),
            value.vertical_spacing.as_ref(),
        ),
        _ => (None, None),
    };
    if let ValidationError::InvalidSpacing(value) = error {
        if matches!(horizontal_spacing, Some(Spacing::Pixels(spacing)) if spacing == value) {
            return Some("horizontal_spacing");
        }
        if matches!(vertical_spacing, Some(Spacing::Pixels(spacing)) if spacing == value) {
            return Some("vertical_spacing");
        }
    }
    None
}

fn column_layout_field(column: &Column, error: &ValidationError) -> Option<&'static str> {
    if matches!(error, ValidationError::InvalidPadding(value) if column.padding.as_deref() == Some(value))
    {
        return Some("padding");
    }
    if matches!(error, ValidationError::InvalidMargin(value) if column.margin.as_deref() == Some(value))
    {
        return Some("margin");
    }
    if let ValidationError::InvalidSpacing(value) = error {
        if matches!(&column.horizontal_spacing, Some(Spacing::Pixels(spacing)) if spacing == value)
        {
            return Some("horizontal_spacing");
        }
        if matches!(&column.vertical_spacing, Some(Spacing::Pixels(spacing)) if spacing == value) {
            return Some("vertical_spacing");
        }
    }
    None
}

fn validate_optional_element_id(
    id: Option<&str>,
    ids: &mut BTreeSet<String>,
) -> Result<(), ValidationError> {
    if let Some(id) = id {
        if id.is_empty()
            || id.len() > 20
            || !id.starts_with(char::is_alphabetic)
            || !id.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            return Err(ValidationError::InvalidElementId(id.to_string()));
        }
        if !ids.insert(id.to_string()) {
            return Err(ValidationError::DuplicateElementId(id.to_string()));
        }
    }
    Ok(())
}

fn validate_header_tag(tag: &HeaderTag, ids: &mut BTreeSet<String>) -> Result<(), ValidationError> {
    if tag.text.tag != TextTag::PlainText {
        return Err(ValidationError::HeaderTagRequiresPlainText);
    }
    validate_optional_element_id(tag.element_id.as_deref(), ids)
}

fn validate_header_tag_list(
    tags: &[HeaderTag],
    ids: &mut BTreeSet<String>,
) -> Result<(), ValidationError> {
    if tags.len() > 3 {
        return Err(ValidationError::TooManyHeaderTags(tags.len()));
    }
    for tag in tags {
        validate_header_tag(tag, ids)?;
    }
    Ok(())
}

fn validate_header(header: &Header, ids: &mut BTreeSet<String>) -> Result<(), ValidationError> {
    validate_padding(header.padding.as_deref())?;
    validate_header_tag_list(&header.text_tag_list, ids)?;
    if let Some(lines) = header.title.lines
        && lines > 4
    {
        return Err(ValidationError::InvalidHeaderTitleLines(lines));
    }
    if let Some(subtitle) = &header.subtitle
        && let Some(lines) = subtitle.lines
        && lines > 1
    {
        return Err(ValidationError::InvalidHeaderSubtitleLines(lines));
    }
    if let Some(localized) = &header.i18n_text_tag_list {
        for tags in localized.values() {
            validate_header_tag_list(tags, ids)?;
        }
    }
    Ok(())
}

fn validate_control(
    control: &Control,
    tag: &'static str,
    in_form: bool,
) -> Result<(), ValidationError> {
    validate_behaviors(&control.behaviors, false)?;
    validate_optional_plain_text(control.placeholder.as_ref(), "control.placeholder", None)?;
    validate_optional_plain_text(
        control.disabled_tips.as_ref(),
        "control.disabled_tips",
        None,
    )?;
    validate_optional_plain_text(control.hover_tips.as_ref(), "control.hover_tips", None)?;
    validate_optional_confirm(control.confirm.as_ref())?;
    if in_form && control.name.as_deref().is_none_or(str::is_empty) {
        return Err(ValidationError::MissingFormControlName(tag));
    }
    validate_margin(control.margin.as_deref())?;
    validate_control_width(control.width.as_deref())?;
    Ok(())
}

fn validate_plain_text(
    text: &Text,
    field: &'static str,
    maximum_length: Option<usize>,
) -> Result<(), ValidationError> {
    if text.tag != TextTag::PlainText {
        return Err(ValidationError::PlainTextRequired(field));
    }
    if let Some(maximum_length) = maximum_length {
        let length = text.content.chars().count();
        if length > maximum_length {
            return Err(ValidationError::TextTooLong { field, length });
        }
    }
    Ok(())
}

fn validate_optional_plain_text(
    text: Option<&Text>,
    field: &'static str,
    maximum_length: Option<usize>,
) -> Result<(), ValidationError> {
    text.map_or(Ok(()), |text| {
        validate_plain_text(text, field, maximum_length)
    })
}

fn validate_optional_confirm(confirm: Option<&Confirm>) -> Result<(), ValidationError> {
    if let Some(confirm) = confirm {
        validate_plain_text(&confirm.title, "confirm.title", None)?;
        validate_plain_text(&confirm.text, "confirm.text", None)?;
    }
    Ok(())
}

fn validate_behaviors(behaviors: &[Behavior], required: bool) -> Result<(), ValidationError> {
    if required && behaviors.is_empty() {
        return Err(ValidationError::MissingInteractiveContainerBehavior);
    }
    for behavior in behaviors {
        if let Behavior::OpenUrl { default_url, .. } = behavior
            && default_url.is_empty()
        {
            return Err(ValidationError::InvalidOpenUrl(default_url.clone()));
        }
    }
    Ok(())
}

fn valid_pixels(value: &str, min: i16, max: i16) -> bool {
    value
        .strip_suffix("px")
        .and_then(|value| value.parse::<i16>().ok())
        .is_some_and(|value| (min..=max).contains(&value))
}

fn valid_min_pixels(value: &str, min: u32) -> bool {
    value
        .strip_suffix("px")
        .and_then(|value| value.parse::<u32>().ok())
        .is_some_and(|value| value >= min)
}

fn validate_control_width(value: Option<&str>) -> Result<(), ValidationError> {
    if let Some(value) = value
        && value != "default"
        && value != "fill"
        && !valid_min_pixels(value, 100)
    {
        return Err(ValidationError::InvalidControlWidth(value.to_string()));
    }
    Ok(())
}

fn valid_box_pixels(value: &str, min: i16, max: i16) -> bool {
    let values: Vec<_> = value.split_ascii_whitespace().collect();
    (1..=4).contains(&values.len())
        && values
            .into_iter()
            .all(|value| valid_pixels(value, min, max))
}

fn valid_corner_radius(value: &str) -> bool {
    value
        .strip_suffix("px")
        .and_then(|value| value.parse::<u32>().ok())
        .is_some()
        || value
            .strip_suffix('%')
            .and_then(|value| value.parse::<u8>().ok())
            .is_some_and(|value| value <= 100)
}

fn validate_spacing(spacing: &Spacing) -> Result<(), ValidationError> {
    match spacing {
        Spacing::Pixels(value) if !valid_pixels(value, 0, 99) => {
            Err(ValidationError::InvalidSpacing(value.clone()))
        }
        _ => Ok(()),
    }
}

fn validate_padding(value: Option<&str>) -> Result<(), ValidationError> {
    if let Some(value) = value
        && !valid_box_pixels(value, 0, 99)
    {
        return Err(ValidationError::InvalidPadding(value.to_string()));
    }
    Ok(())
}

fn validate_margin(value: Option<&str>) -> Result<(), ValidationError> {
    if let Some(value) = value
        && !valid_box_pixels(value, -99, 99)
    {
        return Err(ValidationError::InvalidMargin(value.to_string()));
    }
    Ok(())
}

fn validate_layout(
    padding: Option<&str>,
    margin: Option<&str>,
    horizontal_spacing: Option<&Spacing>,
    vertical_spacing: Option<&Spacing>,
) -> Result<(), ValidationError> {
    validate_padding(padding)?;
    validate_margin(margin)?;
    if let Some(spacing) = horizontal_spacing {
        validate_spacing(spacing)?;
    }
    if let Some(spacing) = vertical_spacing {
        validate_spacing(spacing)?;
    }
    Ok(())
}

fn validate_body(body: &Body) -> Result<(), ValidationError> {
    validate_layout(
        body.padding.as_deref(),
        None,
        body.horizontal_spacing.as_ref(),
        body.vertical_spacing.as_ref(),
    )
}

fn validate_column(column: &Column) -> Result<(), ValidationError> {
    validate_layout(
        column.padding.as_deref(),
        column.margin.as_deref(),
        column.horizontal_spacing.as_ref(),
        column.vertical_spacing.as_ref(),
    )?;
    match &column.width {
        Some(ColumnWidth::Pixels(value)) if !valid_pixels(value, 16, 600) => {
            Err(ValidationError::InvalidColumnWidth(value.clone()))
        }
        _ if column
            .weight
            .is_some_and(|weight| !(1..=5).contains(&weight)) =>
        {
            Err(ValidationError::InvalidColumnWeight(
                column.weight.unwrap_or_default(),
            ))
        }
        _ if column.weight.is_some() && !matches!(column.width, Some(ColumnWidth::Weighted)) => {
            Err(ValidationError::InvalidColumnWeight(
                column.weight.unwrap_or_default(),
            ))
        }
        _ => Ok(()),
    }
}

fn validate_column_set(column_set: &ColumnSet) -> Result<(), ValidationError> {
    if column_set.columns.is_empty() {
        return Err(ValidationError::EmptyColumnSet);
    }
    validate_margin(column_set.margin.as_deref())?;
    if let Some(spacing) = &column_set.horizontal_spacing {
        validate_spacing(spacing)?;
    }
    for column in &column_set.columns {
        validate_column(column)?;
        if column_set
            .flex_mode
            .is_some_and(|mode| mode != ColumnFlexMode::None)
            && (column.width.is_some() || column.weight.is_some())
        {
            return Err(ValidationError::ColumnWidthRequiresFixedFlexMode);
        }
    }
    Ok(())
}

fn image_combination_limit(mode: ImageCombinationMode) -> usize {
    match mode {
        ImageCombinationMode::Double => 2,
        ImageCombinationMode::Triple => 3,
        ImageCombinationMode::Bisect => 6,
        ImageCombinationMode::Trisect => 9,
    }
}

fn validate_image_combination(element: &ImageCombination) -> Result<(), ValidationError> {
    validate_margin(element.margin.as_deref())?;
    if let Some(corner_radius) = &element.corner_radius
        && !valid_corner_radius(corner_radius)
    {
        return Err(ValidationError::InvalidCornerRadius(corner_radius.clone()));
    }
    let limit = image_combination_limit(element.combination_mode);
    if element.img_list.len() > limit {
        return Err(ValidationError::TooManyImagesInCombination {
            mode: element.combination_mode,
            count: element.img_list.len(),
        });
    }
    Ok(())
}

fn validate_chart(chart: &Chart) -> Result<(), ValidationError> {
    validate_margin(chart.margin.as_deref())?;
    if !matches!(chart.chart_spec.as_value(), serde_json::Value::Object(_)) {
        return Err(ValidationError::InvalidChartSpec);
    }
    if let Some(height) = &chart.height
        && height != "auto"
        && !valid_pixels(height, 1, 999)
    {
        return Err(ValidationError::InvalidChartHeight(height.clone()));
    }
    Ok(())
}

fn validate_div(div: &Div) -> Result<(), ValidationError> {
    validate_margin(div.margin.as_deref())?;
    if let Some(width) = &div.width
        && width != "fill"
        && width != "auto"
        && !valid_pixels(width, 16, 999)
    {
        return Err(ValidationError::InvalidDivWidth(width.clone()));
    }
    Ok(())
}

fn validate_image(image: &Image) -> Result<(), ValidationError> {
    validate_margin(image.margin.as_deref())?;
    validate_plain_text(&image.alt, "img.alt", None)?;
    validate_optional_plain_text(image.title.as_ref(), "img.title", None)?;
    if let Some(corner_radius) = &image.corner_radius
        && !valid_corner_radius(corner_radius)
    {
        return Err(ValidationError::InvalidCornerRadius(corner_radius.clone()));
    }
    if image.size.is_some() && matches!(image.scale_type, Some(ImageScale::FitHorizontal)) {
        return Err(ValidationError::ImageSizeRequiresCropScale);
    }
    Ok(())
}

fn validate_person(person: &Person) -> Result<(), ValidationError> {
    validate_margin(person.margin.as_deref())
}

fn validate_person_list(person_list: &PersonList) -> Result<(), ValidationError> {
    validate_margin(person_list.margin.as_deref())?;
    if person_list.lines == Some(0) {
        return Err(ValidationError::InvalidPersonListLines);
    }
    Ok(())
}

fn validate_collapsible_panel(panel: &CollapsiblePanel) -> Result<(), ValidationError> {
    validate_layout(
        panel.padding.as_deref(),
        panel.margin.as_deref(),
        panel.horizontal_spacing.as_ref(),
        panel.vertical_spacing.as_ref(),
    )?;
    if let Some(border) = &panel.border
        && let Some(corner_radius) = &border.corner_radius
        && !valid_corner_radius(corner_radius)
    {
        return Err(ValidationError::InvalidCornerRadius(corner_radius.clone()));
    }
    Ok(())
}

fn validate_interactive_container(container: &InteractiveContainer) -> Result<(), ValidationError> {
    validate_behaviors(&container.behaviors, true)?;
    validate_optional_plain_text(
        container.disabled_tips.as_ref(),
        "interactive_container.disabled_tips",
        None,
    )?;
    validate_optional_plain_text(
        container.hover_tips.as_ref(),
        "interactive_container.hover_tips",
        None,
    )?;
    validate_optional_confirm(container.confirm.as_ref())?;
    validate_layout(
        container.padding.as_deref(),
        container.margin.as_deref(),
        None,
        None,
    )?;
    if let Some(width) = &container.width
        && width != "fill"
        && width != "auto"
        && !valid_pixels(width, 16, 999)
    {
        return Err(ValidationError::InvalidInteractiveContainerWidth(
            width.clone(),
        ));
    }
    if let Some(height) = &container.height
        && height != "auto"
        && !valid_pixels(height, 10, 999)
    {
        return Err(ValidationError::InvalidInteractiveContainerHeight(
            height.clone(),
        ));
    }
    if let Some(corner_radius) = &container.corner_radius
        && !valid_corner_radius(corner_radius)
    {
        return Err(ValidationError::InvalidCornerRadius(corner_radius.clone()));
    }
    Ok(())
}

fn validate_form(form: &Form, form_names: &mut BTreeSet<String>) -> Result<(), ValidationError> {
    if form.name.is_empty() {
        return Err(ValidationError::InvalidFormName(form.name.clone()));
    }
    if !form_names.insert(form.name.clone()) {
        return Err(ValidationError::DuplicateFormName(form.name.clone()));
    }
    if let Some(padding) = form.padding.as_deref()
        && !valid_box_pixels(padding, -99, 99)
    {
        return Err(ValidationError::InvalidPadding(padding.to_string()));
    }
    validate_margin(form.margin.as_deref())?;
    if let Some(spacing) = &form.horizontal_spacing {
        validate_spacing(spacing)?;
    }
    if let Some(spacing) = &form.vertical_spacing {
        validate_spacing(spacing)?;
    }
    Ok(())
}

fn validate_button(button: &Button) -> Result<(), ValidationError> {
    if let Some(text) = &button.text {
        if text.tag != TextTag::PlainText {
            return Err(ValidationError::ButtonTextRequiresPlainText);
        }
        let length = text.content.chars().count();
        if length > 100 {
            return Err(ValidationError::ButtonTextTooLong(length));
        }
    }
    validate_optional_plain_text(button.disabled_tips.as_ref(), "button.disabled_tips", None)?;
    validate_optional_plain_text(button.hover_tips.as_ref(), "button.hover_tips", None)?;
    validate_optional_confirm(button.confirm.as_ref())?;
    validate_behaviors(&button.behaviors, false)?;
    validate_margin(button.margin.as_deref())?;
    validate_control_width(button.width.as_deref())
}

fn validate_input(input: &Input) -> Result<(), ValidationError> {
    validate_behaviors(&input.behaviors, false)?;
    validate_optional_plain_text(input.placeholder.as_ref(), "input.placeholder", Some(100))?;
    validate_optional_plain_text(input.label.as_ref(), "input.label", None)?;
    validate_optional_plain_text(input.disabled_tips.as_ref(), "input.disabled_tips", None)?;
    validate_optional_plain_text(input.hover_tips.as_ref(), "input.hover_tips", None)?;
    validate_optional_confirm(input.confirm.as_ref())?;
    validate_margin(input.margin.as_deref())?;
    validate_control_width(input.width.as_deref())?;
    if input
        .max_length
        .is_some_and(|length| !(1..=1000).contains(&length))
    {
        return Err(ValidationError::InvalidInputMaxLength(
            input.max_length.unwrap_or_default(),
        ));
    }
    Ok(())
}

fn validate_options(
    options: &[SelectOption],
    maximum_text_length: Option<usize>,
) -> Result<(), ValidationError> {
    let mut values = BTreeSet::new();
    for option in options {
        validate_plain_text(&option.text, "select_option.text", maximum_text_length)?;
        if !values.insert(&option.value) {
            return Err(ValidationError::DuplicateOptionValue(option.value.clone()));
        }
    }
    Ok(())
}

fn validate_person_options(options: &[PersonOption]) -> Result<(), ValidationError> {
    let mut values = BTreeSet::new();
    for option in options {
        if !values.insert(&option.value) {
            return Err(ValidationError::DuplicateOptionValue(option.value.clone()));
        }
    }
    Ok(())
}

fn validate_image_options(options: &[ImageSelectOption]) -> Result<(), ValidationError> {
    let mut values = BTreeSet::new();
    for option in options {
        validate_optional_plain_text(
            option.disabled_tips.as_ref(),
            "select_img.option.disabled_tips",
            None,
        )?;
        validate_optional_plain_text(
            option.hover_tips.as_ref(),
            "select_img.option.hover_tips",
            None,
        )?;
        if !values.insert(&option.value) {
            return Err(ValidationError::DuplicateOptionValue(option.value.clone()));
        }
    }
    Ok(())
}

fn validate_initial_option(
    tag: &'static str,
    initial: Option<&str>,
    options: Option<&[SelectOption]>,
) -> Result<(), ValidationError> {
    if let (Some(initial), Some(options)) = (initial, options)
        && !options.iter().any(|option| option.value == initial)
    {
        return Err(ValidationError::InvalidInitialOption(
            tag,
            initial.to_string(),
        ));
    }
    Ok(())
}

fn validate_selected_values(
    tag: &'static str,
    selected: Option<&[String]>,
    options: Option<&[SelectOption]>,
) -> Result<(), ValidationError> {
    if let (Some(selected), Some(options)) = (selected, options) {
        for value in selected {
            if !options.iter().any(|option| option.value == *value) {
                return Err(ValidationError::InvalidInitialOption(tag, value.clone()));
            }
        }
    }
    Ok(())
}

fn validate_person_initial_option(
    tag: &'static str,
    initial: Option<&str>,
    options: Option<&[PersonOption]>,
) -> Result<(), ValidationError> {
    if let (Some(initial), Some(options)) = (initial, options)
        && !options.iter().any(|option| option.value == initial)
    {
        return Err(ValidationError::InvalidInitialOption(
            tag,
            initial.to_string(),
        ));
    }
    Ok(())
}

fn validate_person_selected_values(
    tag: &'static str,
    selected: Option<&[String]>,
    options: Option<&[PersonOption]>,
) -> Result<(), ValidationError> {
    if let (Some(selected), Some(options)) = (selected, options) {
        for value in selected {
            if !options.iter().any(|option| option.value == *value) {
                return Err(ValidationError::InvalidInitialOption(tag, value.clone()));
            }
        }
    }
    Ok(())
}

fn valid_date(value: &str) -> bool {
    let mut pieces = value.split('-');
    let year = pieces.next();
    let month = pieces.next();
    let day = pieces.next();
    value.len() == 10
        && pieces.next().is_none()
        && year.is_some_and(|value| {
            value.len() == 4 && value.bytes().all(|byte| byte.is_ascii_digit())
        })
        && month
            .filter(|value| value.len() == 2 && value.bytes().all(|byte| byte.is_ascii_digit()))
            .and_then(|value| value.parse::<u8>().ok())
            .is_some_and(|value| (1..=12).contains(&value))
        && day
            .filter(|value| value.len() == 2 && value.bytes().all(|byte| byte.is_ascii_digit()))
            .and_then(|value| value.parse::<u8>().ok())
            .is_some_and(|value| (1..=31).contains(&value))
}

fn valid_time(value: &str) -> bool {
    let mut pieces = value.split(':');
    let hour = pieces.next();
    let minute = pieces.next();
    value.len() == 5
        && pieces.next().is_none()
        && hour
            .filter(|value| value.len() == 2 && value.bytes().all(|byte| byte.is_ascii_digit()))
            .and_then(|value| value.parse::<u8>().ok())
            .is_some_and(|value| value <= 23)
        && minute
            .filter(|value| value.len() == 2 && value.bytes().all(|byte| byte.is_ascii_digit()))
            .and_then(|value| value.parse::<u8>().ok())
            .is_some_and(|value| value <= 59)
}

fn validate_image_select(select: &ImageSelect) -> Result<(), ValidationError> {
    validate_control(&select.control, "select_img", false)?;
    if select.control.behaviors.is_empty() {
        return Err(ValidationError::MissingImageSelectBehavior);
    }
    validate_image_options(&select.options)
}

fn validate_checker(checker: &Checker) -> Result<(), ValidationError> {
    validate_control(&checker.control, "checker", false)?;
    if let Some(padding) = checker.padding.as_deref()
        && !valid_box_pixels(padding, -99, 99)
    {
        return Err(ValidationError::InvalidPadding(padding.to_string()));
    }
    if let Some(button_area) = &checker.button_area {
        for button in &button_area.buttons {
            validate_button(button)?;
        }
    }
    Ok(())
}

fn validate_table(table: &Table) -> Result<(), ValidationError> {
    if table.columns.is_empty() {
        return Err(ValidationError::EmptyTableColumns);
    }
    if table.rows.is_empty() {
        return Err(ValidationError::EmptyTableRows);
    }
    if table.columns.len() > 50 {
        return Err(ValidationError::TooManyTableColumns(table.columns.len()));
    }
    let mut columns = BTreeSet::new();
    for column in &table.columns {
        if !columns.insert(&column.name) {
            return Err(ValidationError::DuplicateTableColumn(column.name.clone()));
        }
        if let Some(width) = &column.width
            && width != "auto"
            && !width.ends_with('%')
            && !valid_pixels(width, 80, 600)
        {
            return Err(ValidationError::InvalidTableColumnWidth(width.clone()));
        }
    }
    if let Some(page_size) = table.page_size
        && !(1..=10).contains(&page_size)
    {
        return Err(ValidationError::InvalidTablePageSize(page_size));
    }
    for row in &table.rows {
        if let Some(column) = row.keys().find(|column| !columns.contains(*column)) {
            return Err(ValidationError::UnknownTableRowColumn(column.clone()));
        }
    }
    if let Some(TableRowHeight::Pixels(height)) = &table.row_height
        && !valid_pixels(height, 32, 124)
    {
        return Err(ValidationError::InvalidTableRowHeight(height.clone()));
    }
    if let Some(height) = &table.row_max_height
        && !valid_pixels(height, 32, 999)
    {
        return Err(ValidationError::InvalidTableRowMaxHeight(height.clone()));
    }
    if table.row_max_height.is_some() && table.row_height != Some(TableRowHeight::Auto) {
        return Err(ValidationError::TableRowMaxHeightRequiresAutoRowHeight);
    }
    validate_margin(table.margin.as_deref())?;
    Ok(())
}

fn validate_form_control_name(
    name: Option<&str>,
    tag: &'static str,
    names: &mut BTreeSet<String>,
) -> Result<(), ValidationError> {
    let name = name
        .filter(|name| !name.is_empty())
        .ok_or(ValidationError::MissingFormControlName(tag))?;
    if !names.insert(name.to_string()) {
        return Err(ValidationError::DuplicateFormControlName(name.to_string()));
    }
    Ok(())
}

fn validate_form_control_names(
    element: &Element,
    names: &mut BTreeSet<String>,
) -> Result<(), ValidationError> {
    match element {
        Element::Button(element) => {
            validate_form_control_name(element.name.as_deref(), "button", names)
        }
        Element::Overflow(element) => element.control.name.as_deref().map_or_else(
            || Err(ValidationError::MissingFormControlName("overflow")),
            |name| validate_form_control_name(Some(name), "overflow", names),
        ),
        Element::Input(element) => {
            validate_form_control_name(element.name.as_deref(), "input", names)
        }
        Element::SelectStatic(element) => element.control.name.as_deref().map_or_else(
            || Err(ValidationError::MissingFormControlName("select_static")),
            |name| validate_form_control_name(Some(name), "select_static", names),
        ),
        Element::MultiSelectStatic(element) => element.control.name.as_deref().map_or_else(
            || {
                Err(ValidationError::MissingFormControlName(
                    "multi_select_static",
                ))
            },
            |name| validate_form_control_name(Some(name), "multi_select_static", names),
        ),
        Element::SelectPerson(element) => element.control.name.as_deref().map_or_else(
            || Err(ValidationError::MissingFormControlName("select_person")),
            |name| validate_form_control_name(Some(name), "select_person", names),
        ),
        Element::MultiSelectPerson(element) => element.control.name.as_deref().map_or_else(
            || {
                Err(ValidationError::MissingFormControlName(
                    "multi_select_person",
                ))
            },
            |name| validate_form_control_name(Some(name), "multi_select_person", names),
        ),
        Element::DatePicker(element) => element.control.name.as_deref().map_or_else(
            || Err(ValidationError::MissingFormControlName("date_picker")),
            |name| validate_form_control_name(Some(name), "date_picker", names),
        ),
        Element::PickerTime(element) => element.control.name.as_deref().map_or_else(
            || Err(ValidationError::MissingFormControlName("picker_time")),
            |name| validate_form_control_name(Some(name), "picker_time", names),
        ),
        Element::PickerDatetime(element) => element.control.name.as_deref().map_or_else(
            || Err(ValidationError::MissingFormControlName("picker_datetime")),
            |name| validate_form_control_name(Some(name), "picker_datetime", names),
        ),
        Element::SelectImg(element) => element.control.name.as_deref().map_or_else(
            || Err(ValidationError::MissingFormControlName("select_img")),
            |name| validate_form_control_name(Some(name), "select_img", names),
        ),
        Element::Checker(element) => element.control.name.as_deref().map_or_else(
            || Err(ValidationError::MissingFormControlName("checker")),
            |name| validate_form_control_name(Some(name), "checker", names),
        ),
        Element::ColumnSet(element) => {
            for column in &element.columns {
                for child in &column.elements {
                    validate_form_control_names(child, names)?;
                }
            }
            Ok(())
        }
        Element::CollapsiblePanel(element) => {
            for child in &element.elements {
                validate_form_control_names(child, names)?;
            }
            Ok(())
        }
        Element::InteractiveContainer(element) => {
            for child in &element.elements {
                validate_form_control_names(child, names)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn next_container_depth(depth: usize) -> Result<usize, ValidationError> {
    let next = depth + 1;
    if next > 5 {
        return Err(ValidationError::TooDeeplyNestedContainer(next));
    }
    Ok(next)
}

fn has_form_submit(element: &Element) -> bool {
    match element {
        Element::Button(button) => button.form_action_type == Some(FormActionType::Submit),
        Element::ColumnSet(column_set) => column_set
            .columns
            .iter()
            .any(|column| column.elements.iter().any(has_form_submit)),
        Element::CollapsiblePanel(panel) => panel.elements.iter().any(has_form_submit),
        Element::InteractiveContainer(container) => container.elements.iter().any(has_form_submit),
        _ => false,
    }
}

impl Element {
    fn validate(
        &self,
        ids: &mut BTreeSet<String>,
        form_state: &mut FormValidationState,
        count: &mut usize,
        root: bool,
        in_form: bool,
        container_depth: usize,
    ) -> Result<(), ValidationError> {
        let element = self;
        *count += 1;
        match element {
            Element::Div(element) => {
                validate_div(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::Markdown(element) => {
                validate_margin(element.margin.as_deref())?;
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::Img(element) => {
                validate_image(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::ImgCombination(element) => {
                validate_image_combination(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::Person(element) => {
                validate_person(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::PersonList(element) => {
                validate_person_list(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::Chart(element) => {
                validate_chart(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::Hr(element) => {
                validate_margin(element.margin.as_deref())?;
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::Button(element) => {
                validate_button(element)?;
                if !in_form && element.form_action_type.is_some() {
                    return Err(ValidationError::FormActionOutsideForm);
                }
                if in_form && element.form_action_type.is_some() && !element.behaviors.is_empty() {
                    return Err(ValidationError::ButtonBehaviorConflict);
                }
                if in_form && element.form_action_type.is_none() {
                    return Err(ValidationError::MissingFormButtonAction);
                }
                if !in_form && element.behaviors.is_empty() {
                    return Err(ValidationError::MissingButtonBehavior);
                }
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::Input(element) => {
                validate_input(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)
            }
            Element::ColumnSet(element) => {
                validate_column_set(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)?;
                let child_depth = next_container_depth(container_depth)?;
                for column in &element.columns {
                    for child in &column.elements {
                        child.validate(ids, form_state, count, false, in_form, child_depth)?;
                    }
                }
                Ok(())
            }
            Element::Form(element) => {
                if !root {
                    return Err(ValidationError::FormNestedOutsideBody);
                }
                if element.elements.is_empty() {
                    return Err(ValidationError::EmptyForm(element.name.clone()));
                }
                validate_form(element, &mut form_state.names)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)?;
                let child_depth = next_container_depth(container_depth)?;
                let mut control_names = BTreeSet::new();
                for child in &element.elements {
                    validate_form_control_names(child, &mut control_names)?;
                    child.validate(ids, form_state, count, false, true, child_depth)?;
                }
                if !element.elements.iter().any(has_form_submit) {
                    return Err(ValidationError::MissingFormSubmit(element.name.clone()));
                }
                Ok(())
            }
            Element::CollapsiblePanel(element) => {
                validate_collapsible_panel(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)?;
                let child_depth = next_container_depth(container_depth)?;
                for child in &element.elements {
                    child.validate(ids, form_state, count, false, in_form, child_depth)?;
                }
                Ok(())
            }
            Element::InteractiveContainer(element) => {
                if element.elements.is_empty() {
                    return Err(ValidationError::EmptyInteractiveContainer);
                }
                validate_interactive_container(element)?;
                validate_optional_element_id(element.element_id.as_deref(), ids)?;
                let child_depth = next_container_depth(container_depth)?;
                for child in &element.elements {
                    child.validate(ids, form_state, count, false, in_form, child_depth)?;
                }
                Ok(())
            }
            Element::Table(table) if root => validate_table(table),
            Element::Table(_) => Err(ValidationError::TableNestedOutsideBody),
            Element::Overflow(element) => {
                validate_control(&element.control, "overflow", in_form)?;
                let options = element
                    .options
                    .as_deref()
                    .ok_or(ValidationError::EmptyOptions("overflow"))?;
                if options.is_empty() {
                    return Err(ValidationError::EmptyOptions("overflow"));
                }
                validate_options(options, Some(100))?;
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
            Element::SelectStatic(element) => {
                validate_control(&element.control, "select_static", in_form)?;
                if let Some(options) = &element.options {
                    validate_options(options, None)?;
                }
                validate_initial_option(
                    "select_static",
                    element.initial_option.as_deref(),
                    element.options.as_deref(),
                )?;
                if element.initial_option.is_none()
                    && let (Some(index), Some(options)) =
                        (element.initial_index, element.options.as_deref())
                    && index > options.len() as u32
                {
                    return Err(ValidationError::InvalidInitialIndex(index));
                }
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
            Element::MultiSelectStatic(element) => {
                validate_control(&element.control, "multi_select_static", in_form)?;
                if let Some(options) = &element.options {
                    validate_options(options, None)?;
                }
                validate_selected_values(
                    "multi_select_static",
                    element.selected_values.as_deref(),
                    element.options.as_deref(),
                )?;
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
            Element::SelectPerson(element) => {
                validate_control(&element.control, "select_person", in_form)?;
                if let Some(options) = &element.options {
                    validate_person_options(options)?;
                }
                validate_person_initial_option(
                    "select_person",
                    element.initial_option.as_deref(),
                    element.options.as_deref(),
                )?;
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
            Element::MultiSelectPerson(element) => {
                validate_control(&element.control, "multi_select_person", in_form)?;
                if let Some(options) = &element.options {
                    validate_person_options(options)?;
                }
                validate_person_selected_values(
                    "multi_select_person",
                    element.selected_values.as_deref(),
                    element.options.as_deref(),
                )?;
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
            Element::DatePicker(element) => {
                validate_control(&element.control, "date_picker", in_form)?;
                if element.initial_date.is_none() && element.control.placeholder.is_none() {
                    return Err(ValidationError::MissingPickerValue("date_picker"));
                }
                if let Some(value) = element.initial_date.as_deref()
                    && !valid_date(value)
                {
                    return Err(ValidationError::InvalidPickerInitialValue(
                        "date_picker",
                        value.to_string(),
                    ));
                }
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
            Element::PickerTime(element) => {
                validate_control(&element.control, "picker_time", in_form)?;
                if element.initial_time.is_none() && element.control.placeholder.is_none() {
                    return Err(ValidationError::MissingPickerValue("picker_time"));
                }
                if let Some(value) = element.initial_time.as_deref()
                    && !valid_time(value)
                {
                    return Err(ValidationError::InvalidPickerInitialValue(
                        "picker_time",
                        value.to_string(),
                    ));
                }
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
            Element::PickerDatetime(element) => {
                validate_control(&element.control, "picker_datetime", in_form)?;
                if element.initial_datetime.is_none() && element.control.placeholder.is_none() {
                    return Err(ValidationError::MissingPickerValue("picker_datetime"));
                }
                if let Some(value) = element.initial_datetime.as_deref() {
                    let (date, time) = value.split_once(' ').unwrap_or_default();
                    if !valid_date(date) || !valid_time(time) {
                        return Err(ValidationError::InvalidPickerInitialValue(
                            "picker_datetime",
                            value.to_string(),
                        ));
                    }
                }
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
            Element::SelectImg(element) => {
                validate_image_select(element)?;
                if in_form && element.control.name.as_deref().is_none_or(str::is_empty) {
                    return Err(ValidationError::MissingFormControlName("select_img"));
                }
                if element.multi_select == Some(true) && !in_form {
                    return Err(ValidationError::MultiSelectImageOutsideForm);
                }
                if element.can_preview.is_some() && !in_form {
                    return Err(ValidationError::ImagePreviewOutsideForm);
                }
                if element.options.is_empty() {
                    return Err(ValidationError::EmptyOptions("select_img"));
                }
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
            Element::Checker(element) => {
                validate_checker(element)?;
                if in_form && element.control.name.as_deref().is_none_or(str::is_empty) {
                    return Err(ValidationError::MissingFormControlName("checker"));
                }
                if let Some(button_area) = &element.button_area
                    && button_area.buttons.len() > 3
                {
                    return Err(ValidationError::TooManyCheckerButtons(
                        button_area.buttons.len(),
                    ));
                }
                validate_optional_element_id(element.control.element_id.as_deref(), ids)
            }
        }
    }
}

#[cfg(test)]
#[path = "v2_diagnostic_path_tests.rs"]
mod diagnostic_path_tests;
