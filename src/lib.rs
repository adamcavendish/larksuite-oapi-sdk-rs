//! Lark/Feishu OpenAPI SDK for Rust.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use larksuite_oapi_sdk_rs::Client;
//!
//! let client = Client::builder("APP_ID", "APP_SECRET").build();
//! ```
//!
//! # Features
//!
//! - **REST services** — 56+ service modules under [`service`] covering IM, Calendar, Drive, etc.
//! - **Event dispatch** — webhook and WebSocket event routing via [`EventDispatcher`].
//! - **Typed events** — 200+ typed event structs under [`events`].
//! - **Card builder** — interactive Lark card construction via [`card`].
//! - **WebSocket client** — feature-gated (`ws`) persistent long-connection event stream via [`ws::WsClient`].
//! - **Axum adapter** — feature-gated (`axum`) integration for HTTP servers.

pub mod cache;
pub mod card;
pub mod client;
pub mod config;
pub mod constants;
pub mod crypto;
pub mod error;
pub mod event;
pub mod events;
#[macro_use]
mod macros;
pub mod req;
pub mod resp;
pub mod service;
pub mod token;
pub(crate) mod transport;
#[cfg(feature = "ws")]
pub mod ws;

#[cfg(feature = "axum")]
pub mod axum_handler;

pub use cache::{Cache, LocalCache};
pub use client::{Client, ClientBuilder};
pub use config::Config;
pub use constants::{AccessTokenType, AppType, FEISHU_BASE_URL, LARK_BASE_URL};
pub use crypto::{event_decrypt, event_encrypt, verify_signature_sha1, verify_signature_sha256};
pub use error::{Error, Result};
pub use event::{
    CallbackAction, CallbackCard, CallbackContext, CallbackHandlerFn, CallbackOperator, CardAction,
    CardActionHandler, CardActionTriggerRequest, CardActionTriggerResponse, CardHandlerFn,
    CardHandlerResult, CustomResp, CustomizedEventHandlerFn, EventDispatcher, EventHandlerFn,
    EventHeader, EventReq, EventResp, EventV2Body, InlinePreview, PreviewUrl, TemplateCard, Toast,
    URLPreviewGetRequest, URLPreviewGetResponse,
};
pub use req::{
    ApiReq, FormDataBuilder, FormDataField, FormDataValue, PathParams, QueryParams, ReqBody,
    RequestOption,
};
pub use resp::{ApiResp, CodeError, CodeErrorInfo, RawResponse};
pub use token::{
    AppTicketManager, AppTokenResponse, MarketplaceAppTokenReq, MarketplaceTenantTokenReq,
    ResendAppTicketRequest, ResendAppTicketResponse, SelfBuiltAppTokenReq, SelfBuiltTenantTokenReq,
    TenantTokenResponse, TokenManager,
};
