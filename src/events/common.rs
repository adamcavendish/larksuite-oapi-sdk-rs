//! Shared building blocks for typed event payloads.
//!
//! Most event modules need the same user-identity struct and the same handler
//! adapter that turns a typed handler into the raw `serde_json::Value` callback
//! the dispatcher stores. They live here so each module re-exports or references
//! them instead of redefining them.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;

/// User identity carried by most events, matching the Go SDK `UserId` shape.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
}

impl UserId {
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    pub fn open_id(&self) -> Option<&str> {
        self.open_id.as_deref()
    }

    pub fn union_id(&self) -> Option<&str> {
        self.union_id.as_deref()
    }
}

/// A list of user identities, used by events that target multiple users.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserIdList {
    #[serde(default)]
    pub user_id_list: Vec<UserId>,
}

pub(crate) fn decode_payload<T>(val: serde_json::Value) -> Result<T, LarkError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    serde_json::from_value(val)
        .map_err(|e| LarkError::Event(format!("failed to deserialize event payload: {e}")))
}

/// Adapt a typed event handler into the raw `serde_json::Value` callback the
/// dispatcher stores. Deserialization failures surface as [`LarkError::Event`].
pub(crate) fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
+ Send
+ Sync
+ 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
{
    move |val: serde_json::Value| match decode_payload(val) {
        Ok(typed) => {
            Box::pin(handler(typed)) as Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
        }
        Err(e) => Box::pin(async move { Err(e) }),
    }
}
