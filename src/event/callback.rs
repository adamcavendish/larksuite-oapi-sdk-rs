use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

// ── Typed callback types ──

/// Operator info for card action trigger and URL preview callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CallbackOperator {
    #[serde(default)]
    pub tenant_key: Option<String>,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub open_id: String,
}

/// Context for card action trigger and URL preview callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CallbackContext {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub preview_token: String,
    #[serde(default)]
    pub open_message_id: String,
    #[serde(default)]
    pub open_chat_id: String,
}

/// Action detail for card action trigger callbacks.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct CallbackAction {
    /// Historical object-shaped callback payload retained for source
    /// compatibility with the channel-normalized action API.
    #[serde(default)]
    pub value: BTreeMap<String, crate::JsonValue>,
    /// Exact callback payload, including the string form permitted by the
    /// current `card.action.trigger` schema.
    #[serde(skip)]
    pub raw_value: Option<crate::JsonValue>,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub option: String,
    #[serde(default)]
    pub timezone: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub form_value: BTreeMap<String, crate::JsonValue>,
    #[serde(default)]
    pub input_value: String,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub checked: bool,
}

impl<'de> Deserialize<'de> for CallbackAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireAction {
            #[serde(default)]
            value: Option<crate::JsonValue>,
            #[serde(default)]
            tag: String,
            #[serde(default)]
            option: String,
            #[serde(default)]
            timezone: String,
            #[serde(default)]
            name: String,
            #[serde(default)]
            form_value: BTreeMap<String, crate::JsonValue>,
            #[serde(default)]
            input_value: String,
            #[serde(default)]
            options: Vec<String>,
            #[serde(default)]
            checked: bool,
        }

        let wire = WireAction::deserialize(deserializer)?;
        let value = wire
            .value
            .as_ref()
            .and_then(|value| match value.as_value() {
                serde_json::Value::Object(values) => Some(
                    values
                        .iter()
                        .map(|(key, value)| (key.clone(), value.clone().into()))
                        .collect(),
                ),
                _ => None,
            })
            .unwrap_or_default();
        Ok(Self {
            value,
            raw_value: wire.value,
            tag: wire.tag,
            option: wire.option,
            timezone: wire.timezone,
            name: wire.name,
            form_value: wire.form_value,
            input_value: wire.input_value,
            options: wire.options,
            checked: wire.checked,
        })
    }
}

/// Request payload for `card.action.trigger` callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CardActionTriggerRequest {
    #[serde(default)]
    pub operator: Option<CallbackOperator>,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub action: Option<CallbackAction>,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub delivery_type: String,
    #[serde(default)]
    pub context: Option<CallbackContext>,
}

/// Toast notification in a card action trigger response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Toast {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub toast_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n: Option<HashMap<String, String>>,
}

impl Toast {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            toast_type: Some("info".to_string()),
            content: Some(content.into()),
            i18n: None,
        }
    }

    pub fn toast_type(mut self, t: impl Into<String>) -> Self {
        self.toast_type = Some(t.into());
        self
    }

    pub fn i18n(mut self, i18n: ToastI18n) -> Self {
        self.i18n = Some(i18n.into_map());
        self
    }
}

card_locale_struct!(
    /// Typed locale fields for toast I18n, matching Go SDK's `I18n` struct.
    ToastI18n, String
);

impl ToastI18n {
    fn into_map(self) -> HashMap<String, String> {
        [
            ("zh_cn", self.zh_cn),
            ("en_us", self.en_us),
            ("ja_jp", self.ja_jp),
            ("zh_hk", self.zh_hk),
            ("zh_tw", self.zh_tw),
            ("id_id", self.id_id),
            ("vi_vn", self.vi_vn),
            ("th_th", self.th_th),
            ("pt_br", self.pt_br),
            ("es_es", self.es_es),
            ("ko_kr", self.ko_kr),
            ("de_de", self.de_de),
            ("fr_fr", self.fr_fr),
            ("it_it", self.it_it),
            ("ru_ru", self.ru_ru),
            ("ms_my", self.ms_my),
        ]
        .into_iter()
        .filter_map(|(locale, value)| value.map(|value| (locale.to_string(), value)))
        .collect()
    }
}

/// Card reference in callback responses (template or raw card JSON).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackCard {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub card_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::JsonValue>,
}

impl CallbackCard {
    /// Create a template-based card response.
    pub fn template(card: TemplateCard) -> Self {
        Self {
            card_type: Some("template".to_string()),
            data: crate::JsonValue::from_serializable(card).ok(),
        }
    }

    /// Create a raw Card JSON callback response.
    pub fn raw(card: impl Serialize) -> Result<Self, crate::LarkError> {
        Ok(Self {
            card_type: Some("raw".to_string()),
            data: Some(crate::JsonValue::from_serializable(card)?),
        })
    }
}

/// Template card with ID, version, and variable bindings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<HashMap<String, crate::JsonValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version_name: Option<String>,
}

/// Response for `card.action.trigger` callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardActionTriggerResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toast: Option<Toast>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CallbackCard>,
}

/// Request payload for `url.preview.get` callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct URLPreviewGetRequest {
    #[serde(default)]
    pub operator: Option<CallbackOperator>,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub context: Option<CallbackContext>,
}

/// Inline preview in a URL preview response.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlinePreview {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<PreviewUrl>,
}

/// Multi-platform URL for inline previews.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreviewUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}

/// Response for `url.preview.get` callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct URLPreviewGetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<InlinePreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CallbackCard>,
}
