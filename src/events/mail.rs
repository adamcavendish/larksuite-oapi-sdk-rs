//! Mail v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MailboxCreatedV1 {
    #[serde(default)]
    pub mailbox_id: String,
    #[serde(default)]
    pub mailbox: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2MailboxDeletedV1 {
    #[serde(default)]
    pub mailbox_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PublicMailboxCreatedV1 {
    #[serde(default)]
    pub public_mailbox_id: String,
    #[serde(default)]
    pub public_mailbox: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PublicMailboxDeletedV1 {
    #[serde(default)]
    pub public_mailbox_id: String,
}

// ── Handler registration helpers ──

fn wrap_handler<T, F, Fut>(
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
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => Box::pin(handler(typed))
                as Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>,
            Err(e) => Box::pin(async move {
                Err(LarkError::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher extension methods ──

impl EventDispatcher {
    pub fn on_p2_mail_mailbox_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MailboxCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("mail.mailbox.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_mail_mailbox_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2MailboxDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("mail.mailbox.deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_mail_public_mailbox_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2PublicMailboxCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("mail.public_mailbox.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_mail_public_mailbox_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2PublicMailboxDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("mail.public_mailbox.deleted_v1", wrap_handler(handler))
    }
}
