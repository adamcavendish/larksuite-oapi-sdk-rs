//! Typed Card JSON 1.0 composition.
//!
//! This module is the modern Card JSON 1.0 root. It deliberately does not add
//! new fields to the historical [`Card`](crate::card::Card) builder, so a future
//! `card::v2` root can model its incompatible structure directly.

use std::collections::BTreeMap;

use crate::JsonValue;
use serde::{Deserialize, Serialize};

pub use super::TemplateColor;

/// Card JSON 1.0 root.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Config>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_link: Option<MultiUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Header>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Element>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_elements: Option<BTreeMap<Locale, Vec<Element>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<Fallback>,
}

impl Card {
    pub fn new() -> Self {
        Self::default()
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

    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }

    pub fn i18n_elements(mut self, elements: BTreeMap<Locale, Vec<Element>>) -> Self {
        self.i18n_elements = Some(elements);
        self
    }

    pub fn fallback(mut self, fallback: Fallback) -> Self {
        self.fallback = Some(fallback);
        self
    }

    pub fn to_json(&self) -> JsonValue {
        JsonValue::from_serializable(self).expect("Card JSON 1.0 is serializable")
    }
}

/// Card JSON 1.0 configuration supported by the modern root.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_forward: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_multi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_mode: Option<WidthMode>,
    /// Deprecated by `width_mode`, retained for wire compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compact_width: Option<bool>,
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

    pub fn enable_forward(mut self, enabled: bool) -> Self {
        self.enable_forward = Some(enabled);
        self
    }

    pub fn update_multi(mut self, enabled: bool) -> Self {
        self.update_multi = Some(enabled);
        self
    }

    pub fn width_mode(mut self, width_mode: WidthMode) -> Self {
        self.width_mode = Some(width_mode);
        self
    }

    pub fn compact_width(mut self, compact: bool) -> Self {
        self.compact_width = Some(compact);
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

/// Named font-size and color definitions available to Card JSON 1.0 elements.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// Desktop, mobile, and compatibility font-size token values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// Per-theme RGBA color definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Global Card JSON 1.0 fallback configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Fallback {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub trigger_conditions: Vec<FallbackCondition>,
}

impl Fallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trigger_condition(mut self, condition: FallbackCondition) -> Self {
        self.trigger_conditions.push(condition);
        self
    }
}

/// One condition that triggers the client-wide fallback display.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum FallbackCondition {
    MinClientVersion(String),
    ElementTags(Vec<String>),
}

/// Width modes defined by Card JSON 1.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidthMode {
    Default,
    Compact,
    Fill,
}

/// Platform-specific navigation target.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// Full Card JSON 1.0 header baseline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Header {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<HeaderText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<HeaderText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplateColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ud_icon: Option<UdIcon>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub text_tag_list: Vec<HeaderTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_text_tag_list: Option<BTreeMap<Locale, Vec<HeaderTag>>>,
}

impl Header {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: Some(HeaderText::plain(title)),
            ..Self::default()
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(HeaderText::plain(subtitle));
        self
    }

    pub fn title_text(mut self, title: HeaderText) -> Self {
        self.title = Some(title);
        self
    }

    pub fn subtitle_text(mut self, subtitle: HeaderText) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn template(mut self, template: TemplateColor) -> Self {
        self.template = Some(template);
        self
    }

    pub fn icon(mut self, icon: HeaderIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn text_tag(mut self, text_tag: HeaderTag) -> Self {
        self.text_tag_list.push(text_tag);
        self
    }

    pub fn i18n_text_tag_list(mut self, tags: BTreeMap<Locale, Vec<HeaderTag>>) -> Self {
        self.i18n_text_tag_list = Some(tags);
        self
    }
}

/// Header text uses the `i18n` field defined for header title and subtitle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeaderText {
    pub tag: TextTag,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n: Option<BTreeMap<Locale, String>>,
}

impl HeaderText {
    pub fn plain(content: impl Into<String>) -> Self {
        Self {
            tag: TextTag::PlainText,
            content: content.into(),
            i18n: None,
        }
    }

    pub fn lark_md(content: impl Into<String>) -> Self {
        Self {
            tag: TextTag::LarkMd,
            content: content.into(),
            i18n: None,
        }
    }

    pub fn i18n(mut self, content: BTreeMap<Locale, String>) -> Self {
        self.i18n = Some(content);
        self
    }
}

/// A header icon is either a standard token or an uploaded image key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum HeaderIcon {
    StandardIcon(StandardIcon),
    CustomIcon(CustomIcon),
}

impl HeaderIcon {
    pub fn standard(token: impl Into<String>) -> Self {
        Self::StandardIcon(StandardIcon::new(token))
    }

    pub fn custom(img_key: impl Into<String>) -> Self {
        Self::CustomIcon(CustomIcon::new(img_key))
    }
}

/// Standard icon token and optional semantic color.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StandardIcon {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

impl StandardIcon {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            color: None,
            size: None,
        }
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }
}

/// Uploaded icon reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomIcon {
    pub img_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

impl CustomIcon {
    pub fn new(img_key: impl Into<String>) -> Self {
        Self {
            img_key: img_key.into(),
            size: None,
        }
    }
}

/// A newer icon-library reference used by headers and person lists. Unlike a
/// `standard_icon`, this object intentionally has no `tag` member on the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UdIcon {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<IconStyle>,
}

impl UdIcon {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            style: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IconStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}

/// Header suffix tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeaderTag {
    pub tag: HeaderTagKind,
    pub text: HeaderText,
    pub color: HeaderTagColor,
}

impl HeaderTag {
    pub fn new(text: impl Into<String>, color: HeaderTagColor) -> Self {
        Self {
            tag: HeaderTagKind::TextTag,
            text: HeaderText::plain(text),
            color,
        }
    }
}

/// The only Card JSON header-tag shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeaderTagKind {
    TextTag,
}

/// Documented semantic colors for header suffix tags and standard icons.
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

/// The documented palette, with an explicit escape hatch for custom style
/// tokens. Fixed palette values remain closed enum variants instead of
/// unchecked strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Default,
    Transparent,
    White,
    Black,
    Grey,
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
    Custom(String),
}

impl From<HeaderTagColor> for Color {
    fn from(color: HeaderTagColor) -> Self {
        match color {
            HeaderTagColor::Neutral => Self::Neutral,
            HeaderTagColor::Blue => Self::Blue,
            HeaderTagColor::Turquoise => Self::Turquoise,
            HeaderTagColor::Lime => Self::Lime,
            HeaderTagColor::Orange => Self::Orange,
            HeaderTagColor::Violet => Self::Violet,
            HeaderTagColor::Indigo => Self::Indigo,
            HeaderTagColor::Wathet => Self::Wathet,
            HeaderTagColor::Green => Self::Green,
            HeaderTagColor::Yellow => Self::Yellow,
            HeaderTagColor::Red => Self::Red,
            HeaderTagColor::Purple => Self::Purple,
            HeaderTagColor::Carmine => Self::Carmine,
        }
    }
}

impl Color {
    /// Reference a named color from [`CardStyle::color`].
    pub fn custom(token: impl Into<String>) -> Self {
        Self::Custom(token.into())
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Default => "default",
            Self::Transparent => "transparent",
            Self::White => "white",
            Self::Black => "black",
            Self::Grey => "grey",
            Self::Neutral => "neutral",
            Self::Blue => "blue",
            Self::Turquoise => "turquoise",
            Self::Lime => "lime",
            Self::Orange => "orange",
            Self::Violet => "violet",
            Self::Indigo => "indigo",
            Self::Wathet => "wathet",
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
            Self::Purple => "purple",
            Self::Carmine => "carmine",
            Self::Custom(value) => value,
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "default" => Self::Default,
            "transparent" => Self::Transparent,
            "white" => Self::White,
            "black" => Self::Black,
            "grey" => Self::Grey,
            "neutral" => Self::Neutral,
            "blue" => Self::Blue,
            "turquoise" => Self::Turquoise,
            "lime" => Self::Lime,
            "orange" => Self::Orange,
            "violet" => Self::Violet,
            "indigo" => Self::Indigo,
            "wathet" => Self::Wathet,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "red" => Self::Red,
            "purple" => Self::Purple,
            "carmine" => Self::Carmine,
            _ => Self::Custom(value),
        })
    }
}

/// Locales documented for Card JSON 1.0 global and header localization.
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

/// Card text object with a closed text tag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Text {
    pub tag: TextTag,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_content: Option<BTreeMap<Locale, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<TextAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<u32>,
}

impl Text {
    pub fn plain(content: impl Into<String>) -> Self {
        Self {
            tag: TextTag::PlainText,
            content: content.into(),
            i18n_content: None,
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
            i18n_content: None,
            text_size: None,
            text_color: None,
            text_align: None,
            lines: None,
        }
    }

    pub fn i18n_content(mut self, content: BTreeMap<Locale, String>) -> Self {
        self.i18n_content = Some(content);
        self
    }

    pub fn text_size(mut self, size: impl Into<String>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn text_align(mut self, alignment: TextAlign) -> Self {
        self.text_align = Some(alignment);
        self
    }

    pub fn lines(mut self, lines: u32) -> Self {
        self.lines = Some(lines);
        self
    }
}

/// Text tags supported by Card JSON 1.0 text objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextTag {
    PlainText,
    LarkMd,
}

/// Horizontal alignment shared by textual Card JSON 1.0 components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// Card JSON 1.0 display elements implemented by the initial modern root.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)] // Inline variants keep Card JSON composition ergonomic.
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum Element {
    Div(Div),
    Markdown(Markdown),
    Hr,
    Img(Image),
    ImgCombination(ImageCombination),
    Person(Person),
    PersonList(PersonList),
    Chart(Chart),
    Table(Table),
    Note(Note),
    ColumnSet(ColumnSet),
    InteractiveContainer(InteractiveContainer),
    CollapsiblePanel(CollapsiblePanel),
    Form(Form),
    Action(Action),
    Button(Button),
    Overflow(Overflow),
    SelectStatic(StaticSelect),
    MultiSelectStatic(MultiStaticSelect),
    SelectPerson(PersonSelect),
    MultiSelectPerson(MultiPersonSelect),
    DatePicker(DatePicker),
    PickerTime(TimePicker),
    PickerDatetime(DatetimePicker),
    Input(Input),
    SelectImg(ImageSelect),
    Checker(Checker),
}

/// Ordinary text block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Div {
    pub text: Text,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub fields: Vec<DivField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Extra>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
}

impl Div {
    pub fn new(text: Text) -> Self {
        Self {
            text,
            fields: Vec::new(),
            extra: None,
            icon: None,
        }
    }

    pub fn field(mut self, field: DivField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn extra(mut self, extra: Extra) -> Self {
        self.extra = Some(extra);
        self
    }

    pub fn icon(mut self, icon: HeaderIcon) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Secondary text field in a [`Div`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivField {
    pub is_short: bool,
    pub text: Text,
}

impl DivField {
    pub fn new(text: Text) -> Self {
        Self {
            is_short: false,
            text,
        }
    }

    pub fn short(mut self, is_short: bool) -> Self {
        self.is_short = is_short;
        self
    }
}

/// Markdown block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Markdown {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<TextAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<BTreeMap<String, MultiUrl>>,
}

impl Markdown {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            text_size: None,
            text_align: None,
            icon: None,
            href: None,
        }
    }

    pub fn text_size(mut self, size: impl Into<String>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    pub fn text_align(mut self, alignment: TextAlign) -> Self {
        self.text_align = Some(alignment);
        self
    }

    pub fn icon(mut self, icon: HeaderIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn href(mut self, href: BTreeMap<String, MultiUrl>) -> Self {
        self.href = Some(href);
        self
    }
}

/// Image display element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub img_key: String,
    pub alt: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_img_key: Option<BTreeMap<Locale, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<ImageScale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compact_width: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<LegacyImageMode>,
}

impl Image {
    pub fn new(img_key: impl Into<String>, alt: Text) -> Self {
        Self {
            img_key: img_key.into(),
            alt,
            i18n_img_key: None,
            title: None,
            corner_radius: None,
            scale_type: None,
            custom_width: None,
            compact_width: None,
            preview: None,
            size: None,
            transparent: None,
            mode: None,
        }
    }

    pub fn title(mut self, title: Text) -> Self {
        self.title = Some(title);
        self
    }

    pub fn i18n_img_key(mut self, keys: BTreeMap<Locale, String>) -> Self {
        self.i18n_img_key = Some(keys);
        self
    }

    pub fn scale_type(mut self, scale_type: ImageScale) -> Self {
        self.scale_type = Some(scale_type);
        self
    }

    pub fn custom_width(mut self, width: u32) -> Self {
        self.custom_width = Some(width);
        self
    }

    pub fn compact_width(mut self, compact: bool) -> Self {
        self.compact_width = Some(compact);
        self
    }

    pub fn preview(mut self, preview: bool) -> Self {
        self.preview = Some(preview);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegacyImageMode {
    CropCenter,
    FitHorizontal,
    Stretch,
    Large,
    Medium,
    Small,
    Tiny,
}

/// Modern image scaling modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageScale {
    CropCenter,
    CropTop,
    FitHorizontal,
}

/// VChart color themes documented by Lark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartColorTheme {
    Brand,
    Rainbow,
    Complementary,
    Converse,
    Primary,
}

/// Native VChart display element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub chart_spec: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_theme: Option<ChartColorTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
}

impl Chart {
    pub fn new(chart_spec: JsonValue) -> Self {
        Self {
            chart_spec,
            aspect_ratio: None,
            color_theme: None,
            preview: None,
            height: None,
        }
    }

    pub fn aspect_ratio(mut self, aspect_ratio: impl Into<String>) -> Self {
        self.aspect_ratio = Some(aspect_ratio.into());
        self
    }

    pub fn color_theme(mut self, color_theme: ChartColorTheme) -> Self {
        self.color_theme = Some(color_theme);
        self
    }

    pub fn preview(mut self, preview: bool) -> Self {
        self.preview = Some(preview);
        self
    }

    pub fn height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(height.into());
        self
    }
}

/// A compact multi-image layout.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImageCombination {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub img_list: Vec<ImageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combination_mode: Option<ImageCombinationMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
}

impl ImageCombination {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn image(mut self, image: ImageReference) -> Self {
        self.img_list.push(image);
        self
    }
    pub fn combination_mode(mut self, mode: ImageCombinationMode) -> Self {
        self.combination_mode = Some(mode);
        self
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

/// A person identity displayed by a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<PersonSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_avatar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<PersonStyle>,
}

impl Person {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            size: None,
            show_avatar: None,
            show_name: None,
            style: None,
        }
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

/// A horizontal list of card users.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonList {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub persons: Vec<PersonReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_invalid_user_id: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_avatar: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<PersonSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ud_icon: Option<UdIcon>,
}

impl PersonList {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn person(mut self, person: PersonReference) -> Self {
        self.persons.push(person);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonReference {
    pub id: String,
}
impl PersonReference {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

/// A data table. Rows are intentionally JSON objects because their keys are
/// defined by the caller's [`TableColumn::name`] values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Table {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_style: Option<TableHeaderStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_first_column: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub columns: Vec<TableColumn>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub rows: Vec<BTreeMap<String, JsonValue>>,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableHeaderStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_align: Option<TextAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<u32>,
}

impl TableHeaderStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<TextAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<TableDataType>,
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
            width: None,
            vertical_align: None,
            horizontal_align: None,
            data_type: None,
            format: None,
            date_format: None,
        }
    }
    pub fn data_type(mut self, value: TableDataType) -> Self {
        self.data_type = Some(value);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableDataType {
    Text,
    LarkMd,
    Options,
    Number,
    Persons,
    Date,
    Markdown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NumberFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
}

/// Inline note content permits text and images only.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Note {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<NoteElement>,
}

impl Note {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn element(mut self, element: NoteElement) -> Self {
        self.elements.push(element);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)] // `note` has a deliberately richer image form.
#[serde(untagged)]
pub enum NoteElement {
    Text(Text),
    Image(Image),
}

/// Click action supported by display containers in Card JSON 1.0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenUrl {
    pub multi_url: MultiUrl,
}

impl OpenUrl {
    pub fn new(multi_url: MultiUrl) -> Self {
        Self { multi_url }
    }
}

/// A responsive group of columns.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ColumnSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<TextAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_mode: Option<ColumnFlexMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<OpenUrl>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub columns: Vec<Column>,
}

impl ColumnSet {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnFlexMode {
    None,
    Stretch,
    Flow,
    Bisect,
    Trisect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Column {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<OpenUrl>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Element>,
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }
}

/// A card container whose contents have one shared link action.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InteractiveContainer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_border: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<OpenUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Element>,
}

impl InteractiveContainer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }
    pub fn action(mut self, action: OpenUrl) -> Self {
        self.action = Some(action);
        self
    }
}

/// A collapsible section. The header is a normal text object and the body is
/// a list of v1 card elements.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollapsiblePanel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<CollapsiblePanelHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<PanelBorder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Element>,
}

impl CollapsiblePanel {
    pub fn new(header: CollapsiblePanelHeader) -> Self {
        Self {
            header: Some(header),
            ..Self::default()
        }
    }
    pub fn element(mut self, element: Element) -> Self {
        self.elements.push(element);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapsiblePanelHeader {
    pub title: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<HeaderIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_position: Option<PanelIconPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_expanded_angle: Option<i16>,
}

impl CollapsiblePanelHeader {
    pub fn new(title: Text) -> Self {
        Self {
            title,
            background_color: None,
            vertical_align: None,
            padding: None,
            icon: None,
            icon_position: None,
            icon_expanded_angle: None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelIconPosition {
    Left,
    Right,
    FollowText,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PanelBorder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
}

/// Collects a group of form controls for one callback submission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub elements: Vec<Element>,
}

impl Form {
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

/// An action row, including all Card JSON 1.0 controls that can appear there.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Action {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub actions: Vec<ActionComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<ActionLayout>,
}

impl Action {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn action(mut self, action: ActionComponent) -> Self {
        self.actions.push(action);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionLayout {
    Bisected,
    Trisection,
    Flow,
}

/// Components permitted in an `action` block.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)] // Public action rows should not require Box wrappers.
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum ActionComponent {
    Button(Button),
    Overflow(Overflow),
    SelectStatic(StaticSelect),
    MultiSelectStatic(MultiStaticSelect),
    SelectPerson(PersonSelect),
    MultiSelectPerson(MultiPersonSelect),
    DatePicker(DatePicker),
    PickerTime(TimePicker),
    PickerDatetime(DatetimePicker),
}

/// Components that may be placed in a `div.extra` slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)] // `div.extra` shares the action-component ergonomics.
#[serde(tag = "tag", rename_all = "snake_case")]
pub enum Extra {
    Button(Button),
    Overflow(Overflow),
    SelectStatic(StaticSelect),
    MultiSelectStatic(MultiStaticSelect),
    SelectPerson(PersonSelect),
    MultiSelectPerson(MultiPersonSelect),
    DatePicker(DatePicker),
    PickerTime(TimePicker),
    PickerDatetime(DatetimePicker),
}

/// The confirmation dialog used by callback-capable controls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Confirm {
    pub title: Text,
    pub text: Text,
}

impl Confirm {
    pub fn new(title: Text, text: Text) -> Self {
        Self { title, text }
    }
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

/// A direct URL or callback button. Card JSON 1.0 uses `url`, `multi_url`,
/// and `value` directly; it intentionally has no Card JSON 2.0 `behaviors`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub text: Text,
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
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_url: Option<MultiUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<FormActionType>,
}

impl Button {
    pub fn new(text: Text) -> Self {
        Self {
            text,
            button_type: None,
            size: None,
            width: None,
            icon: None,
            disabled: None,
            disabled_tips: None,
            hover_tips: None,
            value: None,
            confirm: None,
            url: None,
            multi_url: None,
            name: None,
            required: None,
            action_type: None,
        }
    }
    pub fn button_type(mut self, value: ButtonType) -> Self {
        self.button_type = Some(value);
        self
    }
    pub fn value(mut self, value: JsonValue) -> Self {
        self.value = Some(value);
        self
    }
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }
    pub fn multi_url(mut self, value: MultiUrl) -> Self {
        self.multi_url = Some(value);
        self
    }
    pub fn confirm(mut self, value: Confirm) -> Self {
        self.confirm = Some(value);
        self
    }
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormActionType {
    Link,
    Request,
    Multi,
    FormSubmit,
    FormReset,
}

/// A selectable option shared by static selects and overflow controls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub text: Text,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_url: Option<MultiUrl>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub button_type: Option<ButtonType>,
}

impl SelectOption {
    pub fn new(text: Text, value: impl Into<String>) -> Self {
        Self {
            text,
            value: value.into(),
            url: None,
            multi_url: None,
            button_type: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Overflow {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
}

impl Overflow {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn option(mut self, option: SelectOption) -> Self {
        self.options.push(option);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StaticSelect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Text>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub options: Vec<SelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

impl StaticSelect {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn option(mut self, option: SelectOption) -> Self {
        self.options.push(option);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiStaticSelect {
    #[serde(flatten)]
    pub select: StaticSelect,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonSelect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiPersonSelect {
    #[serde(flatten)]
    pub select: PersonSelect,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_values: Option<Vec<String>>,
}

/// Common fields of the three Card JSON 1.0 date/time pickers.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PickerBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatePicker {
    #[serde(flatten)]
    pub base: PickerBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimePicker {
    #[serde(flatten)]
    pub base: PickerBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatetimePicker {
    #[serde(flatten)]
    pub base: PickerBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_datetime: Option<String>,
}

/// A free-text input. It is an element because it may be displayed outside an
/// action row; `name` and `required` become meaningful inside a form.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Input {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageSelectStyle {
    Default,
    Laser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageSelectLayout {
    Stretch,
    Bisect,
    Trisect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSelectOption {
    pub img_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_tips: Option<Text>,
}

impl ImageSelectOption {
    pub fn new(img_key: impl Into<String>) -> Self {
        Self {
            img_key: img_key.into(),
            value: None,
            disabled: None,
            disabled_tips: None,
            hover_tips: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImageSelect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageSelectStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_select: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<ImageSelectLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub options: Vec<ImageSelectOption>,
}

impl ImageSelect {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn option(mut self, option: ImageSelectOption) -> Self {
        self.options.push(option);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckedStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
}

/// Checkbox-like task control. Unlike Card JSON 2.0 this v1 model exposes its
/// callback payload through `value` and does not serialize `behaviors`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Checker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    pub margin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Confirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_tips: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_tips: Option<Text>,
}

/// Validation errors for constraints that cannot be represented by Rust types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    InvalidCardLink,
    EmptyFallback,
    HeaderTagLocalizationConflict,
    EmptyColumns,
    EmptyActionRow,
    EmptyForm { name: String },
    EmptyImageSelect,
    EmptyImageCombination,
    MissingImageCombinationMode,
    InvalidImageCombinationCount,
    EmptyPersonId,
    EmptyPersonList,
    MissingPanelHeader,
    EmptyOverflow,
    EmptyStaticSelect,
    InvalidTablePageSize(u8),
    TableHasNoColumns,
    UnknownTableRowColumn { column: String },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCardLink => {
                formatter.write_str("card_link needs url or all platform-specific URL fields")
            }
            Self::EmptyFallback => {
                formatter.write_str("fallback.trigger_conditions must not be empty")
            }
            Self::HeaderTagLocalizationConflict => formatter
                .write_str("header cannot contain both text_tag_list and i18n_text_tag_list"),
            Self::EmptyColumns => formatter.write_str("column_set.columns must not be empty"),
            Self::EmptyActionRow => formatter.write_str("action.actions must not be empty"),
            Self::EmptyForm { name } => {
                write!(formatter, "form {name:?} must contain at least one element")
            }
            Self::EmptyImageSelect => formatter.write_str("select_img.options must not be empty"),
            Self::EmptyImageCombination => {
                formatter.write_str("img_combination.img_list must not be empty")
            }
            Self::MissingImageCombinationMode => {
                formatter.write_str("img_combination.combination_mode is required")
            }
            Self::InvalidImageCombinationCount => {
                formatter.write_str("img_combination has too many images for its combination_mode")
            }
            Self::EmptyPersonId => formatter.write_str("person.user_id must not be empty"),
            Self::EmptyPersonList => formatter.write_str("person_list.persons must not be empty"),
            Self::MissingPanelHeader => formatter.write_str("collapsible_panel.header is required"),
            Self::EmptyOverflow => formatter.write_str("overflow.options must not be empty"),
            Self::EmptyStaticSelect => {
                formatter.write_str("select_static.options must not be empty")
            }
            Self::InvalidTablePageSize(size) => {
                write!(formatter, "table.page_size {size} is outside 1..=10")
            }
            Self::TableHasNoColumns => formatter.write_str("table.columns must not be empty"),
            Self::UnknownTableRowColumn { column } => {
                write!(formatter, "table row contains undeclared column {column:?}")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

impl Card {
    /// Check the v1 cross-field constraints before serializing or sending a card.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.card_link.as_ref().is_some_and(|link| {
            link.url.is_none()
                && (link.android_url.is_none() || link.ios_url.is_none() || link.pc_url.is_none())
        }) {
            return Err(ValidationError::InvalidCardLink);
        }
        if self
            .fallback
            .as_ref()
            .is_some_and(|fallback| fallback.trigger_conditions.is_empty())
        {
            return Err(ValidationError::EmptyFallback);
        }
        if self.header.as_ref().is_some_and(|header| {
            !header.text_tag_list.is_empty() && header.i18n_text_tag_list.is_some()
        }) {
            return Err(ValidationError::HeaderTagLocalizationConflict);
        }
        for element in &self.elements {
            validate_element(element)?;
        }
        if let Some(localized) = &self.i18n_elements {
            for elements in localized.values() {
                for element in elements {
                    validate_element(element)?;
                }
            }
        }
        Ok(())
    }
}

fn validate_element(element: &Element) -> Result<(), ValidationError> {
    match element {
        Element::ColumnSet(set) if set.columns.is_empty() => Err(ValidationError::EmptyColumns),
        Element::ColumnSet(set) => {
            for column in &set.columns {
                for element in &column.elements {
                    validate_element(element)?;
                }
            }
            Ok(())
        }
        Element::InteractiveContainer(container) => {
            for element in &container.elements {
                validate_element(element)?;
            }
            Ok(())
        }
        Element::CollapsiblePanel(panel) if panel.header.is_none() => {
            Err(ValidationError::MissingPanelHeader)
        }
        Element::CollapsiblePanel(panel) => {
            for element in &panel.elements {
                validate_element(element)?;
            }
            Ok(())
        }
        Element::Form(form) if form.elements.is_empty() => Err(ValidationError::EmptyForm {
            name: form.name.clone(),
        }),
        Element::Form(form) => {
            for element in &form.elements {
                validate_element(element)?;
            }
            Ok(())
        }
        Element::Action(action) if action.actions.is_empty() => {
            Err(ValidationError::EmptyActionRow)
        }
        Element::Action(action) => {
            for action in &action.actions {
                validate_action(action)?;
            }
            Ok(())
        }
        Element::SelectImg(select) if select.options.is_empty() => {
            Err(ValidationError::EmptyImageSelect)
        }
        Element::ImgCombination(combination) => validate_image_combination(combination),
        Element::Person(person) if person.user_id.is_empty() => Err(ValidationError::EmptyPersonId),
        Element::PersonList(list) if list.persons.is_empty() => {
            Err(ValidationError::EmptyPersonList)
        }
        Element::Overflow(overflow) if overflow.options.is_empty() => {
            Err(ValidationError::EmptyOverflow)
        }
        Element::SelectStatic(select) if select.options.is_empty() => {
            Err(ValidationError::EmptyStaticSelect)
        }
        Element::MultiSelectStatic(select) if select.select.options.is_empty() => {
            Err(ValidationError::EmptyStaticSelect)
        }
        Element::Table(table) => validate_table(table),
        _ => Ok(()),
    }
}

fn validate_image_combination(combination: &ImageCombination) -> Result<(), ValidationError> {
    if combination.img_list.is_empty() {
        return Err(ValidationError::EmptyImageCombination);
    }
    let Some(mode) = combination.combination_mode else {
        return Err(ValidationError::MissingImageCombinationMode);
    };
    let maximum = match mode {
        ImageCombinationMode::Double => 2,
        ImageCombinationMode::Triple => 3,
        ImageCombinationMode::Bisect => 6,
        ImageCombinationMode::Trisect => 9,
    };
    if combination.img_list.len() > maximum {
        return Err(ValidationError::InvalidImageCombinationCount);
    }
    Ok(())
}

fn validate_action(action: &ActionComponent) -> Result<(), ValidationError> {
    match action {
        ActionComponent::Overflow(overflow) if overflow.options.is_empty() => {
            Err(ValidationError::EmptyOverflow)
        }
        ActionComponent::SelectStatic(select) if select.options.is_empty() => {
            Err(ValidationError::EmptyStaticSelect)
        }
        ActionComponent::MultiSelectStatic(select) if select.select.options.is_empty() => {
            Err(ValidationError::EmptyStaticSelect)
        }
        _ => Ok(()),
    }
}

fn validate_table(table: &Table) -> Result<(), ValidationError> {
    if table.columns.is_empty() {
        return Err(ValidationError::TableHasNoColumns);
    }
    if let Some(size) = table.page_size
        && !(1..=10).contains(&size)
    {
        return Err(ValidationError::InvalidTablePageSize(size));
    }
    for row in &table.rows {
        for name in row.keys() {
            if !table.columns.iter().any(|column| column.name == *name) {
                return Err(ValidationError::UnknownTableRowColumn {
                    column: name.clone(),
                });
            }
        }
    }
    Ok(())
}
