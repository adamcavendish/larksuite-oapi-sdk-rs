//! Published Card Builder template messages.
//!
//! Card Builder owns the visual template and any builder-only components. This
//! module only models the typed message envelope used to send a published
//! template through the IM `interactive` message APIs.

use serde::Serialize;
use serde_json::{Map, Value};

use crate::{JsonValue, LarkError};

/// An interactive message that references a published Card Builder template.
///
/// `T` is the caller-defined, published-template variable object. Template
/// variable names and shapes are owned by the published template rather than
/// Card JSON, so the SDK validates only that `T` serializes to a JSON object.
/// Pass this value to an IM `interactive_card` request helper.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateMessage<T = JsonValue> {
    #[serde(rename = "type")]
    message_type: &'static str,
    data: TemplateMessageData<T>,
}

#[derive(Debug, Clone, Serialize)]
struct TemplateMessageData<T> {
    template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_version_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_variable: Option<T>,
}

impl TemplateMessage<JsonValue> {
    /// Reference a non-empty Card Builder template ID.
    pub fn new(template_id: impl Into<String>) -> Result<Self, LarkError> {
        let template_id = template_id.into();
        validate_non_empty("Card Builder template_id", &template_id)?;
        Ok(Self {
            message_type: "template",
            data: TemplateMessageData {
                template_id,
                template_version_name: None,
                template_variable: None,
            },
        })
    }

    /// Set the complete object of template-variable bindings.
    ///
    /// Values are intentionally open JSON. The top-level value must be an
    /// object because Card Builder identifies bindings by variable name.
    pub fn template_variables(
        mut self,
        variables: impl Into<JsonValue>,
    ) -> Result<Self, LarkError> {
        let variables = variables.into();
        if !variables.as_value().is_object() {
            return Err(LarkError::IllegalParam(
                "Card Builder template variables must be a JSON object".to_string(),
            ));
        }
        self.data.template_variable = Some(variables);
        Ok(self)
    }

    /// Replace raw JSON bindings with a caller-defined variable object.
    ///
    /// Use [`TemplateMessage::with_variables`] when constructing a typed
    /// message directly.
    pub fn typed_variables<T>(self, variables: T) -> Result<TemplateMessage<T>, LarkError>
    where
        T: Serialize,
    {
        validate_template_variables(&variables)?;
        Ok(TemplateMessage {
            message_type: self.message_type,
            data: TemplateMessageData {
                template_id: self.data.template_id,
                template_version_name: self.data.template_version_name,
                template_variable: Some(variables),
            },
        })
    }

    /// Add or replace one named template-variable binding.
    pub fn template_variable(
        mut self,
        name: impl Into<String>,
        value: impl Into<JsonValue>,
    ) -> Result<Self, LarkError> {
        let name = name.into();
        validate_non_empty("Card Builder template variable name", &name)?;

        let mut variables = self
            .data
            .template_variable
            .take()
            .map(JsonValue::into_value)
            .unwrap_or_else(|| Value::Object(Map::new()));
        variables
            .as_object_mut()
            .expect("template variables are always an object")
            .insert(name, value.into().into_value());
        self.data.template_variable = Some(variables.into());
        Ok(self)
    }
}

impl<T> TemplateMessage<T> {
    /// Pin the published template version sent in this message.
    pub fn template_version_name(
        mut self,
        version_name: impl Into<String>,
    ) -> Result<Self, LarkError> {
        let version_name = version_name.into();
        validate_non_empty("Card Builder template_version_name", &version_name)?;
        self.data.template_version_name = Some(version_name);
        Ok(self)
    }
}

impl<T: Serialize> TemplateMessage<T> {
    /// Construct a message with a caller-defined published-template variable
    /// object.
    pub fn with_variables(template_id: impl Into<String>, variables: T) -> Result<Self, LarkError> {
        let template_id = template_id.into();
        validate_non_empty("Card Builder template_id", &template_id)?;
        validate_template_variables(&variables)?;
        Ok(Self {
            message_type: "template",
            data: TemplateMessageData {
                template_id,
                template_version_name: None,
                template_variable: Some(variables),
            },
        })
    }

    /// Return the serialized IM `interactive` message content.
    pub fn to_content(&self) -> Result<String, LarkError> {
        if let Some(variables) = &self.data.template_variable {
            validate_template_variables(variables)?;
        }
        serde_json::to_string(self).map_err(LarkError::Json)
    }
}

fn validate_template_variables(variables: &impl Serialize) -> Result<(), LarkError> {
    if !serde_json::to_value(variables)?.is_object() {
        return Err(LarkError::IllegalParam(
            "Card Builder template variables must serialize to a JSON object".to_string(),
        ));
    }
    Ok(())
}

fn validate_non_empty(field: &str, value: &str) -> Result<(), LarkError> {
    if value.is_empty() {
        return Err(LarkError::IllegalParam(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}
