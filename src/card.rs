//! Lark interactive card builder.
//!
//! Build message cards using a fluent API, then serialize to JSON for
//! the Lark messaging API.
//!
//! # Example
//!
//! ```rust
//! use larksuite_oapi_sdk_rs::card::{Card, CardConfig, CardHeader};
//!
//! let card = Card::new()
//!     .config(CardConfig::new().wide_screen_mode(true))
//!     .header(CardHeader::new("Hello").template("blue"))
//!     .element(larksuite_oapi_sdk_rs::card::md("**world**"));
//!
//! let json = card.to_json();
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ── Card top-level ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<CardConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<CardHeader>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Element>,
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(mut self, config: CardConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn header(mut self, header: CardHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }

    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ── Card config ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wide_screen_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_forward: Option<bool>,
}

impl CardConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn wide_screen_mode(mut self, v: bool) -> Self {
        self.wide_screen_mode = Some(v);
        self
    }

    pub fn enable_forward(mut self, v: bool) -> Self {
        self.enable_forward = Some(v);
        self
    }
}

// ── Card header ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

impl CardHeader {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(TextObject::plain(title)),
            template: None,
        }
    }

    pub fn template(mut self, template: impl Into<String>) -> Self {
        self.template = Some(template.into());
        self
    }
}

// ── Text objects ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextObject {
    pub tag: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<i32>,
}

impl TextObject {
    pub fn plain(content: impl Into<String>) -> Self {
        Self {
            tag: "plain_text".to_string(),
            content: content.into(),
            lines: None,
        }
    }

    pub fn lark_md(content: impl Into<String>) -> Self {
        Self {
            tag: "lark_md".to_string(),
            content: content.into(),
            lines: None,
        }
    }

    pub fn lines(mut self, n: i32) -> Self {
        self.lines = Some(n);
        self
    }
}

// ── Elements ──

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum Element {
    Div(DivElement),
    Hr(HrElement),
    Img(ImgElement),
    Action(ActionElement),
    Note(NoteElement),
    Markdown(MarkdownElement),
    ColumnSet(ColumnSetElement),
    Form(FormElement),
    ChartMd(ChartMdElement),
}

// ── Div (content block) ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DivElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextObject>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub fields: Vec<DivField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
}

impl DivElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: TextObject) -> Self {
        self.text = Some(text);
        self
    }

    pub fn field(mut self, field: DivField) -> Self {
        self.fields.push(field);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DivField {
    pub is_short: bool,
    pub text: TextObject,
}

impl DivField {
    pub fn new(text: TextObject) -> Self {
        Self {
            is_short: false,
            text,
        }
    }

    pub fn short(mut self) -> Self {
        self.is_short = true;
        self
    }
}

// ── Hr (divider) ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HrElement {}

// ── Img ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImgElement {
    pub img_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl ImgElement {
    pub fn new(img_key: impl Into<String>) -> Self {
        Self {
            img_key: img_key.into(),
            ..Default::default()
        }
    }

    pub fn alt(mut self, alt: TextObject) -> Self {
        self.alt = Some(alt);
        self
    }
}

// ── Action ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActionElement {
    pub actions: Vec<ActionComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
}

impl ActionElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action(mut self, action: ActionComponent) -> Self {
        self.actions.push(action);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum ActionComponent {
    Button(ButtonComponent),
    SelectStatic(SelectStaticComponent),
    MultiSelectStatic(MultiSelectStaticComponent),
    DatePicker(DatePickerComponent),
    TimePicker(TimePickerComponent),
    Overflow(OverflowComponent),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ButtonComponent {
    pub text: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ButtonComponent {
    pub fn new(text: TextObject) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }

    pub fn button_type(mut self, t: impl Into<String>) -> Self {
        self.r#type = Some(t.into());
        self
    }

    pub fn value(mut self, v: Value) -> Self {
        self.value = Some(v);
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SelectStaticComponent {
    pub placeholder: TextObject,
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SelectOption {
    pub text: TextObject,
    pub value: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatePickerComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OverflowComponent {
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

// ── Note ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NoteElement {
    pub elements: Vec<NoteContent>,
}

impl NoteElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: TextObject) -> Self {
        self.elements.push(NoteContent::Text(text));
        self
    }

    pub fn img(mut self, img: ImgElement) -> Self {
        self.elements.push(NoteContent::Img(img));
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NoteContent {
    Text(TextObject),
    Img(ImgElement),
}

// ── Markdown ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarkdownElement {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<String>,
}

impl MarkdownElement {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            text_align: None,
        }
    }

    pub fn text_align(mut self, align: impl Into<String>) -> Self {
        self.text_align = Some(align.into());
        self
    }
}

// ── ColumnSet / Column ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColumnSetElement {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub columns: Vec<ColumnElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<String>,
}

impl ColumnSetElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn column(mut self, col: ColumnElement) -> Self {
        self.columns.push(col);
        self
    }

    pub fn flex_mode(mut self, mode: impl Into<String>) -> Self {
        self.flex_mode = Some(mode.into());
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColumnElement {
    pub tag: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Element>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

impl ColumnElement {
    pub fn new() -> Self {
        Self {
            tag: "column".to_string(),
            ..Default::default()
        }
    }

    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }

    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }
}

// ── Form / InputForm ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormElement {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Element>,
}

impl FormElement {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            elements: Vec::new(),
        }
    }

    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }
}

// ── ChartMd ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChartMdElement {
    pub chart_spec: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
}

impl ChartMdElement {
    pub fn new(chart_spec: Value) -> Self {
        Self {
            chart_spec,
            ..Default::default()
        }
    }

    pub fn color_theme(mut self, theme: impl Into<String>) -> Self {
        self.color_theme = Some(theme.into());
        self
    }

    pub fn height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(height.into());
        self
    }
}

// ── MultiSelectStatic ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiSelectStaticComponent {
    pub placeholder: TextObject,
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_values: Option<Vec<String>>,
}

impl MultiSelectStaticComponent {
    pub fn new(placeholder: TextObject) -> Self {
        Self {
            placeholder,
            ..Default::default()
        }
    }

    pub fn option(mut self, opt: SelectOption) -> Self {
        self.options.push(opt);
        self
    }
}

// ── TimePicker ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimePickerComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

impl TimePickerComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn placeholder(mut self, text: TextObject) -> Self {
        self.placeholder = Some(text);
        self
    }

    pub fn initial_time(mut self, time: impl Into<String>) -> Self {
        self.initial_time = Some(time.into());
        self
    }
}

// ── Convenience element constructors ──

pub fn div(text: impl Into<String>) -> Element {
    Element::Div(DivElement::new().text(TextObject::plain(text)))
}

pub fn md(content: impl Into<String>) -> Element {
    Element::Markdown(MarkdownElement::new(content))
}

pub fn hr() -> Element {
    Element::Hr(HrElement {})
}

pub fn button(text: impl Into<String>, value: Value) -> ActionComponent {
    ActionComponent::Button(
        ButtonComponent::new(TextObject::plain(text))
            .button_type("default")
            .value(value),
    )
}
