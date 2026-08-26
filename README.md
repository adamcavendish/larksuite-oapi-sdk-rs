# larksuite-oapi-sdk-rs

[![Crates.io](https://img.shields.io/crates/v/larksuite-oapi-sdk-rs)](https://crates.io/crates/larksuite-oapi-sdk-rs)
[![Docs](https://docs.rs/larksuite-oapi-sdk-rs/badge.svg)](https://docs.rs/larksuite-oapi-sdk-rs)
[![CI](https://github.com/adamcavendish/larksuite-oapi-sdk-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/adamcavendish/larksuite-oapi-sdk-rs/actions/workflows/ci.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Lark/Feishu OpenAPI SDK for Rust. It follows the official
[Go SDK](https://github.com/larksuite/oapi-sdk-go) closely while providing
idiomatic Rust configuration, typed REST resources, event handling, and optional
WebSocket, Axum, and channel integration.

## Highlights

- Tenant, user, app, and marketplace token acquisition with configurable
  caching and retry behavior.
- Typed REST resources with query inputs, upload/download helpers, and lazy
  iterators for supported paginated endpoints.
- Raw requests and a Go-compatibility bridge for endpoints not yet promoted to
  dedicated Rust resources.
- Webhook event dispatch with signature verification and encrypted payload
  decryption.
- Optional WebSocket long connections, Axum handlers, and higher-level channel
  message helpers.

## Install

```toml
[dependencies]
larksuite-oapi-sdk-rs = "0.3.7"
```

The minimum supported Rust version is 1.95.0.

## Quick Start

Build a client once, then call a typed resource. The SDK handles tenant-token
acquisition and request authentication for this endpoint.

```rust,no_run
use larksuite_oapi_sdk_rs::service::im::v1::CreateMessageReqBody;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
    let body = CreateMessageReqBody {
        receive_id: Some("CHAT_ID".to_string()),
        msg_type: Some("text".to_string()),
        content: Some(r#"{"text":"Hello from Rust"}"#.to_string()),
        uuid: None,
    };

    let response = client
        .im()
        .message
        .create("chat_id", &body, &RequestOption::default())
        .await?;

    println!("success: {}", response.success());
    Ok(())
}
```

Use `LarkClient::builder` to set a base URL, timeout, retry limit, token cache,
custom headers, marketplace mode, helpdesk credentials, or a JWT client
assertion provider. See [`examples/client_config.rs`](examples/client_config.rs)
for the complete setup.

## Common Flows

| Need | Start here |
| --- | --- |
| Call a typed REST endpoint | [`examples/send_message.rs`](examples/send_message.rs) or [generated examples](examples/README.md) |
| Call an uncovered endpoint | [`examples/raw_api.rs`](examples/raw_api.rs) |
| Discover or manage BaseApps | [`examples/base_v3_app_read.rs`](examples/base_v3_app_read.rs) and [Base v3 application mode](docs/base-v3-apps.md) |
| Read or update Base dashboard or form sharing | [Base v3 sharing](docs/base-v3-sharing.md) |
| Read or update modern Docx content | [`examples/docs_ai_fetch.rs`](examples/docs_ai_fetch.rs) and [Docs AI document content](docs/docs-ai.md) |
| Read, edit, or render Slides AI presentations | [`examples/slides_ai_read.rs`](examples/slides_ai_read.rs), [`examples/slides_ai_render.rs`](examples/slides_ai_render.rs), and [Slides AI presentation content](docs/slides-ai.md) |
| Exchange or refresh OAuth tokens | [`examples/authen_oauth.rs`](examples/authen_oauth.rs) and [OAuth SSO guide](docs/oauth-sso.md) |
| Receive webhook events | [`examples/event_handler.rs`](examples/event_handler.rs) |
| Send a validated interactive card | [`examples/card_send.rs`](examples/card_send.rs) and [Cards guide](docs/cards.md) |
| Stream a CardKit document | [`examples/cardkit_stream.rs`](examples/cardkit_stream.rs) and [Cards guide](docs/cards.md) |
| Send a published Card Builder template | [`examples/card_template_send.rs`](examples/card_template_send.rs) and [Cards guide](docs/cards.md) |
| Handle interactive card callbacks | [`examples/card_action_handler.rs`](examples/card_action_handler.rs) |
| Run a WebSocket event client | [`examples/ws_client.rs`](examples/ws_client.rs) with `ws` enabled |
| Send channel messages | [`examples/channel_send.rs`](examples/channel_send.rs) with `messaging` enabled |
| Normalize channel messages | [`examples/channel_normalize.rs`](examples/channel_normalize.rs) with `channel` enabled |
| Search bots or work with OKRs | [`examples/bot_search.rs`](examples/bot_search.rs) and [`examples/okr_v2.rs`](examples/okr_v2.rs) |

## Guides

- [Go service and event compatibility](docs/go-compatibility.md): raw
  requests, the compatibility bridge, and deterministic Go-to-Rust checks.
- [Cards](docs/cards.md): versioned Card JSON, CardKit streaming, published
  Card Builder templates, and callbacks.
- [Card protocol alignment](docs/card-protocol.md): source hierarchy and
  verification workflow for Card JSON and card callbacks.
- [Events, WebSockets, and channels](docs/events-and-channels.md): webhook
  dispatch, long connections, trusted user channels, replies, and updates.
- [OAuth SSO](docs/oauth-sso.md): Feishu and Lark browser login, PKCE, token
  refresh, and application-owned session handling.
- [Base v3 application mode](docs/base-v3-apps.md): user-token workspace,
  BaseApp, page, and block operations.
- [Base v3 sharing](docs/base-v3-sharing.md): dashboard and form sharing
  reads and partial updates with user or tenant credentials.
- [Docs AI document content](docs/docs-ai.md): modern Docx content fetches,
  updates, and version-history operations.
- [Slides AI presentation content](docs/slides-ai.md): XML presentation and
  slide operations with version-history support.
- [Documentation index](docs/README.md): detailed guides and runnable examples.

## Migration from 0.2

Version 0.3 makes API-breaking corrections so the public Rust surface matches
the upstream Lark contracts more closely.

- Replace `Client` with `LarkClient` and `ClientBuilder` with
  `LarkClientBuilder`.
- Replace `client.wiki()` with `client.wiki_v1()` or `client.wiki_v2()`.
- Replace `do_req`, `do_req_typed`, and verb helpers with `raw_request*` and an
  `ApiReq` when a dedicated typed resource is unavailable.
- Treat Hire v1 `IdNameObject.name` as `Option<I18n>` with `zh_cn` and `en_us`
  fields.
- Handle JSON construction failures as `LarkError::Json`; legacy card element
  helpers are fallible for the same reason.
- Use `JsonValue` for intentionally dynamic API data instead of public
  `serde_json::Value` model fields.

Typed REST responses and incoming event payloads are `#[non_exhaustive]`.
Deserialize them and read their public fields as before, but do not use struct
literals to create server-owned values.

## Cargo Features

| Feature | Description |
| --- | --- |
| `ws` | WebSocket long-connection event client with protobuf framing. |
| `messaging` | High-level outbound messaging and resource operations without WebSocket dependencies. |
| `channel` | Inbound normalization and runtime policy helpers; enables `messaging` and `ws`. |
| `axum` | Axum adapters for webhook and card-action handlers. |

Cargo unifies enabled features across the dependency graph. Enabling `channel`
therefore enables both `messaging` and `ws`; enabling `messaging` and `ws`
separately does not enable the `channel` runtime API.

```toml
[dependencies]
larksuite-oapi-sdk-rs = { version = "0.3.7", features = ["ws", "axum"] }
```

## API Coverage

The SDK provides dedicated resources across IM, Calendar, Drive, Docs, Sheets,
Bitable, Contacts, Approval, VC, Wiki, Hire, and the broader Lark/Feishu
OpenAPI surface. Generated smoke tests verify request paths, query inputs,
bodies, uploads, downloads, and response handling. See the
[changelog](CHANGELOG.md) for release detail and the
[examples index](examples/README.md) for runnable calls.

## Requirements

- Rust 1.95.0 or newer
- Rust edition 2024

## License

Licensed under [MIT](LICENSE).
