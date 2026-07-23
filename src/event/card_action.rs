use std::collections::{BTreeMap, HashMap};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;

use super::callback::CallbackAction;
use super::{DispatchPipeline, EventReq, EventResp, PipelineResult, SignAlgorithm};

/// Return type for card action handlers.
///
/// Handlers can return either a JSON value (served as HTTP 200) or a
/// [`CustomResp`] with a custom HTTP status code and body.
#[derive(Debug, Clone)]
pub enum CardHandlerResult {
    Json(crate::JsonValue),
    Custom(CustomResp),
}

impl From<crate::JsonValue> for CardHandlerResult {
    fn from(val: crate::JsonValue) -> Self {
        Self::Json(val)
    }
}

impl From<CustomResp> for CardHandlerResult {
    fn from(resp: CustomResp) -> Self {
        Self::Custom(resp)
    }
}

/// Custom card response with an explicit HTTP status code.
#[derive(Debug, Clone)]
pub struct CustomResp {
    pub status_code: u16,
    pub body: BTreeMap<String, crate::JsonValue>,
}

pub type CardHandlerFn = Arc<
    dyn Fn(CardAction) -> Pin<Box<dyn Future<Output = Result<CardHandlerResult, LarkError>> + Send>>
        + Send
        + Sync,
>;

#[must_use]
pub struct CardActionHandler {
    pipeline: DispatchPipeline,
    handler: CardHandlerFn,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CardAction {
    #[serde(default)]
    pub open_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub open_message_id: String,
    #[serde(default)]
    pub open_chat_id: String,
    #[serde(default)]
    pub tenant_key: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub timezone: String,
    #[serde(default)]
    pub action: CallbackAction,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub delivery_type: String,
    /// The raw HTTP request that delivered this card action.
    /// Populated by [`CardActionHandler`] before calling the handler.
    #[serde(skip)]
    pub req: Option<EventReq>,
}

impl CardActionHandler {
    /// Create a card action handler that returns a JSON response with HTTP 200.
    pub fn new<F, Fut, T>(
        verification_token: impl Into<String>,
        event_encrypt_key: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: Fn(CardAction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, LarkError>> + Send + 'static,
        T: Into<crate::JsonValue> + Send + 'static,
    {
        Self {
            pipeline: DispatchPipeline::new(
                verification_token.into(),
                event_encrypt_key.into(),
                SignAlgorithm::Sha1,
            ),
            handler: Arc::new(
                move |action: CardAction| -> Pin<
                    Box<dyn Future<Output = Result<CardHandlerResult, LarkError>> + Send>,
                > {
                    let fut = handler(action);
                    Box::pin(
                        async move { fut.await.map(|value| CardHandlerResult::Json(value.into())) },
                    )
                },
            ),
        }
    }

    /// Create a card action handler that can return a [`CustomResp`] with a
    /// custom HTTP status code and body.
    pub fn new_custom<F, Fut>(
        verification_token: impl Into<String>,
        event_encrypt_key: impl Into<String>,
        handler: F,
    ) -> Self
    where
        F: Fn(CardAction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<CardHandlerResult, LarkError>> + Send + 'static,
    {
        Self {
            pipeline: DispatchPipeline::new(
                verification_token.into(),
                event_encrypt_key.into(),
                SignAlgorithm::Sha1,
            ),
            handler: Arc::new(
                move |action: CardAction| -> Pin<
                    Box<dyn Future<Output = Result<CardHandlerResult, LarkError>> + Send>,
                > { Box::pin(handler(action)) },
            ),
        }
    }

    pub fn skip_sign_verify(mut self) -> Self {
        self.pipeline.skip_sign_verify = true;
        self
    }

    pub async fn handle(&self, req: EventReq) -> EventResp {
        match self.do_handle(req).await {
            Ok(resp) => resp,
            Err(e) => {
                tracing::error!("card action handler error: {e}");
                EventResp::error(500, &e.to_string())
            }
        }
    }

    async fn do_handle(&self, req: EventReq) -> Result<EventResp, LarkError> {
        let (body_str, req) = match self.pipeline.process(req)? {
            PipelineResult::Challenge(resp) => return Ok(resp),
            PipelineResult::Event { body_str, req } => (body_str, req),
        };

        let parsed: crate::JsonValue = serde_json::from_str(&body_str)
            .map_err(|e| LarkError::Event(format!("failed to parse card body: {e}")))?;

        let mut action: CardAction = serde_json::from_value(parsed.into())
            .map_err(|e| LarkError::Event(format!("failed to parse card action: {e}")))?;
        action.req = Some(req);

        let result = (self.handler)(action).await?;
        match result {
            CardHandlerResult::Json(val) => Ok(EventResp::success(val)),
            CardHandlerResult::Custom(custom) => {
                let status = if custom.status_code == 0 {
                    200
                } else {
                    custom.status_code
                };
                let body_bytes = serde_json::to_vec(&custom.body).unwrap_or_default();
                Ok(EventResp {
                    status_code: status,
                    headers: HashMap::from([(
                        "Content-Type".to_string(),
                        "application/json".to_string(),
                    )]),
                    body: body_bytes,
                })
            }
        }
    }
}
