//! SDK-owned representation for intentionally dynamic JSON values.

use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::LarkError;

/// A JSON value used where the Lark API intentionally permits an open-ended
/// payload or response shape.
///
/// Closed API shapes use their dedicated request and response models instead.
/// This wrapper keeps the SDK's public API independent of `serde_json` while
/// preserving exact JSON serialization and deserialization behavior.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonValue(serde_json::Value);

impl JsonValue {
    /// Creates a JSON value from any serializable Rust value.
    ///
    /// Returns [`LarkError::Json`] when serialization fails.
    pub fn from_serializable(value: impl Serialize) -> Result<Self, LarkError> {
        Ok(Self(serde_json::to_value(value)?))
    }

    /// Borrows the underlying JSON representation for integration boundaries.
    pub fn as_value(&self) -> &serde_json::Value {
        &self.0
    }

    /// Consumes the wrapper and returns the underlying JSON representation.
    pub fn into_value(self) -> serde_json::Value {
        self.0
    }
}

impl From<serde_json::Value> for JsonValue {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}

impl From<JsonValue> for serde_json::Value {
    fn from(value: JsonValue) -> Self {
        value.0
    }
}

impl AsRef<serde_json::Value> for JsonValue {
    fn as_ref(&self) -> &serde_json::Value {
        self.as_value()
    }
}

impl Deref for JsonValue {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        self.as_value()
    }
}
