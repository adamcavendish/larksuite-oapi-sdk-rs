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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CallbackAction {
    #[serde(default)]
    pub value: BTreeMap<String, crate::JsonValue>,
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
