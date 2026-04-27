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

// ── Template color constants (matching Go SDK) ──

pub const TEMPLATE_BLUE: &str = "blue";
pub const TEMPLATE_WATHET: &str = "wathet";
pub const TEMPLATE_TURQUOISE: &str = "turquoise";
pub const TEMPLATE_GREEN: &str = "green";
pub const TEMPLATE_YELLOW: &str = "yellow";
pub const TEMPLATE_ORANGE: &str = "orange";
pub const TEMPLATE_RED: &str = "red";
pub const TEMPLATE_CARMINE: &str = "carmine";
pub const TEMPLATE_VIOLET: &str = "violet";
pub const TEMPLATE_PURPLE: &str = "purple";
pub const TEMPLATE_INDIGO: &str = "indigo";
pub const TEMPLATE_GREY: &str = "grey";

// ── Card top-level ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<CardConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<CardHeader>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Element>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_elements: Option<I18nElements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_link: Option<CardUrl>,
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

    pub fn i18n_elements(mut self, i18n: I18nElements) -> Self {
        self.i18n_elements = Some(i18n);
        self
    }

    pub fn card_link(mut self, link: CardUrl) -> Self {
        self.card_link = Some(link);
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_multi: Option<bool>,
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

    pub fn update_multi(mut self, v: bool) -> Self {
        self.update_multi = Some(v);
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n: Option<Box<TextI18n>>,
}

/// Per-locale plain text content for [`TextObject`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextI18n {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_hk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_tw: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vi_vn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th_th: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt_br: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_es: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ko_kr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de_de: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_fr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub it_it: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru_ru: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_my: Option<String>,
}

impl TextObject {
    pub fn plain(content: impl Into<String>) -> Self {
        Self {
            tag: "plain_text".to_string(),
            content: content.into(),
            lines: None,
            i18n: None,
        }
    }

    pub fn lark_md(content: impl Into<String>) -> Self {
        Self {
            tag: "lark_md".to_string(),
            content: content.into(),
            lines: None,
            i18n: None,
        }
    }

    pub fn lines(mut self, n: i32) -> Self {
        self.lines = Some(n);
        self
    }

    pub fn i18n(mut self, i18n: TextI18n) -> Self {
        self.i18n = Some(Box::new(i18n));
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
    SelectPerson(SelectPersonComponent),
    DatePicker(DatePickerComponent),
    TimePicker(TimePickerComponent),
    PickerDatetime(DatetimePickerComponent),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_url: Option<CardUrl>,
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

    pub fn multi_url(mut self, multi_url: CardUrl) -> Self {
        self.multi_url = Some(multi_url);
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

// ── DatetimePicker ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatetimePickerComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

impl DatetimePickerComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn placeholder(mut self, text: TextObject) -> Self {
        self.placeholder = Some(text);
        self
    }

    pub fn initial_datetime(mut self, dt: impl Into<String>) -> Self {
        self.initial_datetime = Some(dt.into());
        self
    }
}

// ── SelectPerson ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SelectPersonComponent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

impl SelectPersonComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn placeholder(mut self, text: TextObject) -> Self {
        self.placeholder = Some(text);
        self
    }
}

// ── CardUrl (multi-platform URL) ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
}

impl CardUrl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
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

// ── I18nElements (per-locale element arrays) ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nElements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_hk: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_tw: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_id: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vi_vn: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th_th: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt_br: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_es: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ko_kr: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de_de: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_fr: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub it_it: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ru_ru: Option<Vec<Element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_my: Option<Vec<Element>>,
}

impl I18nElements {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn zh_cn(mut self, elements: Vec<Element>) -> Self {
        self.zh_cn = Some(elements);
        self
    }

    pub fn en_us(mut self, elements: Vec<Element>) -> Self {
        self.en_us = Some(elements);
        self
    }

    pub fn ja_jp(mut self, elements: Vec<Element>) -> Self {
        self.ja_jp = Some(elements);
        self
    }

    pub fn zh_hk(mut self, elements: Vec<Element>) -> Self {
        self.zh_hk = Some(elements);
        self
    }

    pub fn zh_tw(mut self, elements: Vec<Element>) -> Self {
        self.zh_tw = Some(elements);
        self
    }

    pub fn id_id(mut self, elements: Vec<Element>) -> Self {
        self.id_id = Some(elements);
        self
    }

    pub fn vi_vn(mut self, elements: Vec<Element>) -> Self {
        self.vi_vn = Some(elements);
        self
    }

    pub fn th_th(mut self, elements: Vec<Element>) -> Self {
        self.th_th = Some(elements);
        self
    }

    pub fn pt_br(mut self, elements: Vec<Element>) -> Self {
        self.pt_br = Some(elements);
        self
    }

    pub fn es_es(mut self, elements: Vec<Element>) -> Self {
        self.es_es = Some(elements);
        self
    }

    pub fn ko_kr(mut self, elements: Vec<Element>) -> Self {
        self.ko_kr = Some(elements);
        self
    }

    pub fn de_de(mut self, elements: Vec<Element>) -> Self {
        self.de_de = Some(elements);
        self
    }

    pub fn fr_fr(mut self, elements: Vec<Element>) -> Self {
        self.fr_fr = Some(elements);
        self
    }

    pub fn it_it(mut self, elements: Vec<Element>) -> Self {
        self.it_it = Some(elements);
        self
    }

    pub fn ru_ru(mut self, elements: Vec<Element>) -> Self {
        self.ru_ru = Some(elements);
        self
    }

    pub fn ms_my(mut self, elements: Vec<Element>) -> Self {
        self.ms_my = Some(elements);
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

// ── MessageCard (legacy IM card builder, matches Go SDK) ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<MessageCardConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<MessageCardHeader>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_elements: Option<MessageCardI18nElements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_link: Option<MessageCardURL>,
}

impl MessageCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(mut self, config: MessageCardConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn header(mut self, header: MessageCardHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn element(mut self, element: impl Serialize) -> Self {
        self.elements
            .push(serde_json::to_value(element).unwrap_or_default());
        self
    }

    pub fn i18n_elements(mut self, i18n: MessageCardI18nElements) -> Self {
        self.i18n_elements = Some(i18n);
        self
    }

    pub fn card_link(mut self, link: MessageCardURL) -> Self {
        self.card_link = Some(link);
        self
    }

    pub fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// --- PLACEHOLDER_MSG_CARD_SUBTYPES ---

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_forward: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_multi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wide_screen_mode: Option<bool>,
}

impl MessageCardConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_forward(mut self, v: bool) -> Self {
        self.enable_forward = Some(v);
        self
    }

    pub fn update_multi(mut self, v: bool) -> Self {
        self.update_multi = Some(v);
        self
    }

    pub fn wide_screen_mode(mut self, v: bool) -> Self {
        self.wide_screen_mode = Some(v);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<MessageCardPlainText>,
}

impl MessageCardHeader {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            template: None,
            title: Some(MessageCardPlainText::new(title)),
        }
    }

    pub fn template(mut self, template: impl Into<String>) -> Self {
        self.template = Some(template.into());
        self
    }
}

// --- PLACEHOLDER_MSG_CARD_SUBTYPES_2 ---

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardPlainText {
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n: Option<MessageCardPlainTextI18n>,
}

impl MessageCardPlainText {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            tag: "plain_text".to_string(),
            content: content.into(),
            lines: None,
            i18n: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardLarkMd {
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n: Option<MessageCardPlainTextI18n>,
}

impl MessageCardLarkMd {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            tag: "lark_md".to_string(),
            content: content.into(),
            lines: None,
            i18n: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardPlainTextI18n {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardURL {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
}

impl MessageCardURL {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            android_url: None,
            ios_url: None,
            pc_url: None,
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
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

fn message_card_action_tag() -> String {
    "action".to_string()
}

fn message_card_select_static_tag() -> String {
    "select_static".to_string()
}

fn message_card_multi_select_static_tag() -> String {
    "multi_select_static".to_string()
}

fn message_card_select_person_tag() -> String {
    "select_person".to_string()
}

/// Layout for a legacy message card action block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageCardActionLayout {
    Bisected,
    Trisection,
    Flow,
}

/// Confirmation dialog shown before running a legacy message card action.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardActionConfirm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<MessageCardPlainText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<MessageCardPlainText>,
}

impl MessageCardActionConfirm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: MessageCardPlainText) -> Self {
        self.title = Some(title);
        self
    }

    pub fn text(mut self, text: MessageCardPlainText) -> Self {
        self.text = Some(text);
        self
    }
}

/// Option item used by legacy embedded select menus and overflow menus.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardEmbedSelectOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<MessageCardPlainText>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_url: Option<MessageCardURL>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl MessageCardEmbedSelectOption {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: MessageCardPlainText) -> Self {
        self.text = Some(text);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn multi_url(mut self, multi_url: MessageCardURL) -> Self {
        self.multi_url = Some(multi_url);
        self
    }

    pub fn option_type(mut self, option_type: impl Into<String>) -> Self {
        self.r#type = Some(option_type.into());
        self
    }
}

/// Shared fields for legacy embedded select menu variants.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardEmbedSelectMenuBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<MessageCardPlainText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub options: Vec<MessageCardEmbedSelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<MessageCardActionConfirm>,
}

impl MessageCardEmbedSelectMenuBase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn placeholder(mut self, placeholder: MessageCardPlainText) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn initial_option(mut self, initial_option: impl Into<String>) -> Self {
        self.initial_option = Some(initial_option.into());
        self
    }

    pub fn initial_options(mut self, initial_options: Vec<String>) -> Self {
        self.initial_options = Some(initial_options);
        self
    }

    pub fn option(mut self, option: MessageCardEmbedSelectOption) -> Self {
        self.options.push(option);
        self
    }

    pub fn options(mut self, options: Vec<MessageCardEmbedSelectOption>) -> Self {
        self.options = options;
        self
    }

    pub fn value(mut self, value: Value) -> Self {
        self.value = Some(value);
        self
    }

    pub fn confirm(mut self, confirm: MessageCardActionConfirm) -> Self {
        self.confirm = Some(confirm);
        self
    }
}

/// Single-select static menu embedded in a legacy message card action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCardEmbedSelectMenuStatic {
    #[serde(default = "message_card_select_static_tag")]
    pub tag: String,
    #[serde(flatten)]
    pub base: MessageCardEmbedSelectMenuBase,
}

impl Default for MessageCardEmbedSelectMenuStatic {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageCardEmbedSelectMenuStatic {
    pub fn new() -> Self {
        Self {
            tag: message_card_select_static_tag(),
            base: MessageCardEmbedSelectMenuBase::new(),
        }
    }

    pub fn base(mut self, base: MessageCardEmbedSelectMenuBase) -> Self {
        self.base = base;
        self
    }
}

/// Multi-select static menu embedded in a legacy message card action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCardEmbedSelectMenuMulti {
    #[serde(default = "message_card_multi_select_static_tag")]
    pub tag: String,
    #[serde(flatten)]
    pub base: MessageCardEmbedSelectMenuBase,
}

impl Default for MessageCardEmbedSelectMenuMulti {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageCardEmbedSelectMenuMulti {
    pub fn new() -> Self {
        Self {
            tag: message_card_multi_select_static_tag(),
            base: MessageCardEmbedSelectMenuBase::new(),
        }
    }

    pub fn base(mut self, base: MessageCardEmbedSelectMenuBase) -> Self {
        self.base = base;
        self
    }
}

/// Person selector embedded in a legacy message card action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCardEmbedSelectMenuPerson {
    #[serde(default = "message_card_select_person_tag")]
    pub tag: String,
    #[serde(flatten)]
    pub base: MessageCardEmbedSelectMenuBase,
}

impl Default for MessageCardEmbedSelectMenuPerson {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageCardEmbedSelectMenuPerson {
    pub fn new() -> Self {
        Self {
            tag: message_card_select_person_tag(),
            base: MessageCardEmbedSelectMenuBase::new(),
        }
    }

    pub fn base(mut self, base: MessageCardEmbedSelectMenuBase) -> Self {
        self.base = base;
        self
    }
}

/// Multi-action layout block for legacy message cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCardAction {
    #[serde(default = "message_card_action_tag")]
    pub tag: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub actions: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<MessageCardActionLayout>,
}

impl Default for MessageCardAction {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageCardAction {
    pub fn new() -> Self {
        Self {
            tag: message_card_action_tag(),
            actions: Vec::new(),
            layout: None,
        }
    }

    pub fn action(mut self, action: impl Serialize) -> Self {
        self.actions
            .push(serde_json::to_value(action).unwrap_or_default());
        self
    }

    pub fn actions(mut self, actions: Vec<Value>) -> Self {
        self.actions = actions;
        self
    }

    pub fn layout(mut self, layout: MessageCardActionLayout) -> Self {
        self.layout = Some(layout);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageCardI18nElements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<Vec<Value>>,
}
