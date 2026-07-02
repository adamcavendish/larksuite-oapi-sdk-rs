use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ── Typed callback types ──

/// Operator info for card action trigger and URL preview callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
pub struct CallbackAction {
    #[serde(default)]
    pub value: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub option: String,
    #[serde(default)]
    pub timezone: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub form_value: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub input_value: String,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub checked: bool,
}

/// Request payload for `card.action.trigger` callbacks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
        let mut map = HashMap::new();
        if let Some(v) = i18n.zh_cn {
            map.insert("zh_cn".to_string(), v);
        }
        if let Some(v) = i18n.en_us {
            map.insert("en_us".to_string(), v);
        }
        if let Some(v) = i18n.ja_jp {
            map.insert("ja_jp".to_string(), v);
        }
        if let Some(v) = i18n.zh_hk {
            map.insert("zh_hk".to_string(), v);
        }
        if let Some(v) = i18n.zh_tw {
            map.insert("zh_tw".to_string(), v);
        }
        if let Some(v) = i18n.id_id {
            map.insert("id_id".to_string(), v);
        }
        if let Some(v) = i18n.vi_vn {
            map.insert("vi_vn".to_string(), v);
        }
        if let Some(v) = i18n.th_th {
            map.insert("th_th".to_string(), v);
        }
        if let Some(v) = i18n.pt_br {
            map.insert("pt_br".to_string(), v);
        }
        if let Some(v) = i18n.es_es {
            map.insert("es_es".to_string(), v);
        }
        if let Some(v) = i18n.ko_kr {
            map.insert("ko_kr".to_string(), v);
        }
        if let Some(v) = i18n.de_de {
            map.insert("de_de".to_string(), v);
        }
        if let Some(v) = i18n.fr_fr {
            map.insert("fr_fr".to_string(), v);
        }
        if let Some(v) = i18n.it_it {
            map.insert("it_it".to_string(), v);
        }
        if let Some(v) = i18n.ru_ru {
            map.insert("ru_ru".to_string(), v);
        }
        if let Some(v) = i18n.ms_my {
            map.insert("ms_my".to_string(), v);
        }
        self.i18n = Some(map);
        self
    }
}

/// Typed locale fields for toast I18n, matching Go SDK's `I18n` struct.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToastI18n {
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

/// Card reference in callback responses (template or raw card JSON).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackCard {
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub card_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl CallbackCard {
    /// Create a template-based card response.
    pub fn template(card: TemplateCard) -> Self {
        Self {
            card_type: Some("template".to_string()),
            data: serde_json::to_value(card).ok(),
        }
    }
}

/// Template card with ID, version, and variable bindings.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<HashMap<String, serde_json::Value>>,
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
