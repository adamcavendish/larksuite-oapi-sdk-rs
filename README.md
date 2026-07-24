# larksuite-oapi-sdk-rs

[![Crates.io](https://img.shields.io/crates/v/larksuite-oapi-sdk-rs)](https://crates.io/crates/larksuite-oapi-sdk-rs)
[![Docs](https://docs.rs/larksuite-oapi-sdk-rs/badge.svg)](https://docs.rs/larksuite-oapi-sdk-rs)
[![CI](https://github.com/adamcavendish/larksuite-oapi-sdk-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/adamcavendish/larksuite-oapi-sdk-rs/actions/workflows/ci.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Lark/Feishu OpenAPI SDK for Rust. It follows the official
[Go SDK](https://github.com/larksuite/oapi-sdk-go) closely while providing
idiomatic Rust client configuration, typed REST resources, event handling, and
optional WebSocket and Axum integration.

## Highlights

- Tenant, user, app, and marketplace token acquisition with configurable
  caching and retry behavior.
- Typed REST resources with named query inputs, upload/download helpers, and
  lazy iterators for supported paginated endpoints.
- Raw request and Go-compatibility bridge APIs for endpoints that have not yet
  been promoted to dedicated Rust resources.
- Webhook event dispatch with signature verification and encrypted-payload
  decryption.
- Optional WebSocket long connections, Axum handlers, and higher-level channel
  message helpers.
- Typed Hire v1 and v2 catalogs, lists, and detail responses across the
  Go-backed surface.

## Install

```toml
[dependencies]
larksuite-oapi-sdk-rs = "0.3.1"
```

The minimum supported Rust version is 1.95.0.

## Quick Start

Build a client once, then call a typed resource. The SDK handles the tenant
access token and request authentication for this endpoint.

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

Use `LarkClient::builder` to set a base URL, request timeout, retry limit, token
cache, custom headers, marketplace mode, helpdesk credentials, or a JWT client
assertion provider. See [`examples/client_config.rs`](examples/client_config.rs)
for the complete setup.

Marketplace clients can be constructed outside a Tokio runtime. Their app ticket
is acquired lazily on first use; call `LarkClient::resend_app_ticket` when an
application needs to prewarm the ticket explicitly.

### Migrating to 0.3

Version 0.3 intentionally makes API-breaking corrections so the public Rust
surface matches the upstream Lark contracts more closely.

- Replace `Client` with `LarkClient` and `ClientBuilder` with
  `LarkClientBuilder`. The old names are not aliases.
- Replace `client.wiki()` with `client.wiki_v1()` or `client.wiki_v2()`.
- Replace `do_req`, `do_req_typed`, and verb helpers with `raw_request*` and an
  `ApiReq` when a dedicated typed resource is unavailable.
- Treat Hire v1 `IdNameObject.name` as `Option<I18n>` with `zh_cn` and `en_us`
  fields. The old `zh_name` and `en_name` fields are removed.
- Handle JSON construction failures as `LarkError::Json`. Legacy card element
  helpers are fallible for the same reason.
- Use `JsonValue` for intentionally dynamic API data instead of public
  `serde_json::Value` model fields.

Typed REST responses and incoming event payloads are now `#[non_exhaustive]`.
Continue to deserialize them and read their public fields as before. Do not use
struct literals to create server-owned values; build request models, callback
replies, transport inputs, and domain errors directly as needed.

## Common Flows

| Need | Start here |
| --- | --- |
| Call a typed REST endpoint | [`examples/send_message.rs`](examples/send_message.rs) or the [generated examples](examples/README.md) |
| Call an uncovered endpoint | [`examples/raw_api.rs`](examples/raw_api.rs) |
| Exchange or refresh OAuth tokens | [`examples/authen_oauth.rs`](examples/authen_oauth.rs) |
| Receive webhook events | [`examples/event_handler.rs`](examples/event_handler.rs) |
| Handle interactive card callbacks | [`examples/card_action_handler.rs`](examples/card_action_handler.rs) |
| Create or update an app by device code | [`examples/app_registration.rs`](examples/app_registration.rs) |
| Run a WebSocket event client | [`examples/ws_client.rs`](examples/ws_client.rs) with `ws` enabled |
| Send and normalize channel messages | [`examples/channel_send.rs`](examples/channel_send.rs) and [`examples/channel_normalize.rs`](examples/channel_normalize.rs) with `channel` enabled |

### Raw Requests and the Go Bridge

Prefer a dedicated service resource when one exists. Named `*_by_query` methods
keep request paths, filters, and bodies explicit, while older positional methods
remain as compatibility adapters where they were already public.

For an endpoint without a generated Rust resource, use `ApiReq` through the raw
request APIs. Token selection, request IDs, retries, and error handling remain
the same as for typed resources. The `client.go_v397()` bridge provides the
same transport behavior for its curated newer-Go-SDK endpoint set. See
[`examples/raw_api.rs`](examples/raw_api.rs) and
[`examples/go_v397_endpoint.rs`](examples/go_v397_endpoint.rs).

The GoV397 endpoint metadata is generated from the Go SDK's `v3.6.1..v3.9.7`
resource delta. Regenerate it or verify that it is current with a local Go SDK
checkout:

```sh
GO_SDK_DIR=/path/to/larksuite-oapi-sdk-go
go run ./tools/generate_go_v397_metadata.go --go-sdk "$GO_SDK_DIR"
just go-v397-check "$GO_SDK_DIR"
```

The checked-in Go service contract catalog records the complete Go `v3.9.9`
resource request surface: source resource, receiver, operation, HTTP method,
path, token types, and upload behavior. It is a reproducible tooling input for
future service generation; it does not alter runtime request dispatch or
generate Rust resource implementations yet. Refresh it or verify that it is
current with the same Go checkout:

```sh
GO_SDK_DIR=/path/to/larksuite-oapi-sdk-go
go run ./tools/generate_go_service_catalog.go --go-sdk "$GO_SDK_DIR" --revision v3.9.9
just go-service-catalog-check "$GO_SDK_DIR"
```

`tools/go_rust_service_parity.json` compares that Go catalog with Rust's
literal and `format!`-based `RestRequest` wiring, supported local
macro-generated request methods, and the GoV397 bridge. It classifies typed
matches, bridge matches, metadata mismatches, missing Go contracts, and any
unsupported request templates that need explicit extractor support. The report
is a deterministic request-contract baseline, not a generator for Rust
resource implementations. Regenerate or verify it after either catalog or
request-wiring changes:

```sh
go run ./tools/generate_go_rust_service_parity.go
just go-rust-service-parity-check
```

CI runs `just go-contract-provenance-check` against a full-history checkout of
the pinned Go `v3.9.9` reference. That gate also runs both Go extractor test
suites and verifies the GoV397 metadata and full service catalog before
accepting the Rust parity report.

### Dynamic JSON Values

Closed API contracts use dedicated request and response models. Where the
upstream contract is intentionally open-ended, the SDK exposes `JsonValue`
instead of `serde_json::Value`. Construct one from any serializable value or
convert an existing JSON value at an integration boundary:

```rust
use larksuite_oapi_sdk_rs::JsonValue;

let value = JsonValue::from(serde_json::json!({"custom_field": "value"}));
assert_eq!(value["custom_field"], "value");
```

### Pagination and Transfers

Supported list and search resources expose `*_by_iterator` helpers. Iterators
fetch pages lazily and support `limit`, `page_token`, and `next_page_token` for
bounded or resumable scans. See [`examples/bitable_records.rs`](examples/bitable_records.rs).

Upload and download helpers use the same typed-resource pattern. Buffered
downloads are available where appropriate; streaming message-resource downloads
expose response metadata before the body is consumed. See
[`examples/im_upload_download.rs`](examples/im_upload_download.rs) and
[`examples/im_stream_download.rs`](examples/im_stream_download.rs).

Endpoints that accept binary content take `Vec<FormDataField>` values. Build
them with `FormDataBuilder` so files and accompanying text fields are encoded
as `multipart/form-data`.

### App Registration

The `registration` module mirrors the Go SDK device-code flow. It opens a QR or
verification URL, then polls until the app credentials are available. Use
`AppAddons::preset` to control whether the default add-on template is applied:

```rust
use larksuite_oapi_sdk_rs::registration::AppAddons;

let addons = AppAddons {
    preset: Some(false),
    ..Default::default()
};
```

`Some(false)` permits an otherwise empty add-on configuration. An omitted or
`Some(true)` preset still requires at least one scope, event, or callback.

### Events and WebSockets

Webhook handlers use `EventDispatcher` for typed event callbacks, signature
verification, and encrypted payload processing. The optional `ws` feature adds
long connections for event delivery:

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, EventDispatcher};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
    let dispatcher = EventDispatcher::new("VERIFICATION_TOKEN", "ENCRYPT_KEY");
    client.ws_client(dispatcher).start().await?;
    Ok(())
}
```

Trusted user channels are available as parity with the upstream Go SDK branch.
Configure the channel before obtaining its control handle, start the client,
then bind a user token after the connection becomes ready:

```rust,no_run
use std::collections::HashMap;
use std::time::Duration;

# use larksuite_oapi_sdk_rs::{LarkClient, EventDispatcher};
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
# let dispatcher = EventDispatcher::new("VERIFICATION_TOKEN", "ENCRYPT_KEY");
let ws_client = client
    .ws_client(dispatcher)
    .channel_tag("trusted_channel")
    .websocket_query_params(HashMap::from([("env".into(), "staging".into())]));
let control = ws_client.control();
let task = tokio::spawn(ws_client.start());

while control.connection_id().is_none() {
    tokio::time::sleep(Duration::from_millis(50)).await;
}
control.attach_user("USER_ACCESS_TOKEN").await?;
// Later, before dropping the user session:
// control.detach_user("USER_ACCESS_TOKEN").await?;
task.await??;
# Ok(())
# }
```

Use the optional `axum` feature for Axum HTTP adapters. The optional `channel`
feature builds on WebSocket and IM APIs with message normalization, send helpers,
upload inputs, and runtime policy controls.

### Cards

The `card` module builds interactive Lark cards, and `CardActionHandler`
handles callback payloads. See [`examples/card_action_handler.rs`](examples/card_action_handler.rs)
for a runnable callback response and the API documentation for card builders.

## Cargo Features

| Feature | Description |
| --- | --- |
| `ws` | WebSocket long-connection event client with protobuf framing. |
| `channel` | Higher-level send and normalized receive helpers; enables `ws`. |
| `axum` | Axum adapters for webhook and card-action handlers. |

```toml
[dependencies]
larksuite-oapi-sdk-rs = { version = "0.3.1", features = ["ws", "axum"] }
```

## API Coverage

The SDK provides dedicated resources across the Lark/Feishu OpenAPI service
surface, including IM, Calendar, Drive, Docs, Sheets, Bitable, Contacts,
Approval, VC, Wiki, and Hire. Generated service smoke tests verify request
paths, query inputs, bodies, uploads, downloads, and response handling across
the broader API surface.

Generated V2 response wrappers use Go-shaped top-level data structs when the
reference SDK defines response data, and unit responses where it does not.
Their nested response objects follow the Go SDK model graph; intentionally
open-ended upstream shapes use `JsonValue`.

Legacy response wrappers follow the same rule. Sheets range-value operations
provide typed plain-text and rich-text cells, range metadata, update metadata,
and typed find/replace, dimension, filter, and dropdown payloads. Open-ended
batch-operation and conditional-format extension fields use `JsonValue`.

Task v2, Directory v1, Application v6, IM v2, and CardKit v1 mutation
resources use Go-shaped request models for their closed API contracts. Dynamic
or unmodeled upstream contracts use `JsonValue`, including:
CardKit template variables, Task's file-bearing attachment upload, and the
legacy Directory user status field.

Hire v1 is a particular focus: catalog, reference, task, website-posting,
external, agency, job, talent, application, interview, and background-check
resources provide typed Go-backed responses. Where the Go SDK exposes iterator
support, the Rust resource follows with lazy pagination helpers.

Hire v2 provides typed interview-record and composite-talent responses,
including nested assessment and I18n data.

See the [changelog](CHANGELOG.md) for release-by-release detail and the
[examples index](examples/README.md) for runnable service calls.

## Requirements

- Rust 1.95.0 or newer
- Rust edition 2024

## License

Licensed under [MIT](LICENSE).
