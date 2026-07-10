# larksuite-oapi-sdk-rs

Lark/Feishu OpenAPI SDK for Rust.

[![Crates.io](https://img.shields.io/crates/v/larksuite-oapi-sdk-rs)](https://crates.io/crates/larksuite-oapi-sdk-rs)
[![Docs](https://docs.rs/larksuite-oapi-sdk-rs/badge.svg)](https://docs.rs/larksuite-oapi-sdk-rs)
[![CI](https://github.com/adamcavendish/larksuite-oapi-sdk-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/adamcavendish/larksuite-oapi-sdk-rs/actions/workflows/ci.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A Rust port of the official [Go SDK](https://github.com/larksuite/oapi-sdk-go).
The SDK handles token acquisition and caching, request construction, event
decryption, callback signature verification, and typed OpenAPI responses so
application code can stay close to the Lark/Feishu domain operation it is
performing.

## Features

- **REST API client** with automatic token management, caching, and retry
- **Named query helpers** across generated REST services, with compatibility
  positional adapters for existing callers
- **Client assertion (JWT bearer)** for secretless deployments
- **OAuth flows** — authorization code exchange and refresh token rotation
- **Event dispatching** with AES-256-CBC decryption and SHA-1/SHA-256
  signature verification
- **Card action handling** with typed callback actions
- **WebSocket client** (`ws` feature) for long-connection event streams with
  protobuf framing and fragment reassembly
- **Axum adapter** (`axum` feature) for HTTP server integration
- **Message Card builder** for constructing interactive Lark card messages
- **Go SDK parity escape hatch** via `client.go_v397()` for newer generated
  endpoints that are not yet promoted to dedicated Rust resources
- **56+ service modules** covering IM, Calendar, Drive, Sheets, Docs,
  Contacts, Approval, VC, Wiki, and more

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
larksuite-oapi-sdk-rs = "0.2"
```

For runnable examples, see the curated [examples index](examples/README.md).

### Client configuration

`Client::builder` mirrors the Go SDK's `lark.NewClient(..., With...)` option
style while keeping configuration immutable after build.

```rust
use std::time::Duration;

use larksuite_oapi_sdk_rs::{Client, LARK_BASE_URL};

fn build_client() -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::builder("APP_ID", "APP_SECRET")
        .base_url(LARK_BASE_URL)
        .timeout(Duration::from_secs(3))
        .disable_token_cache()
        .max_retries(3)
        .log_req_at_debug()
        .build()?;

    Ok(client)
}
```

Use `.marketplace()` for marketplace apps, `.helpdesk_credential(...)` for
helpdesk APIs, `.default_headers(...)` for extra headers, and
`.client_assertion_provider(...)` for JWT bearer deployments. See
[`examples/client_config.rs`](examples/client_config.rs) for a runnable setup
sample.

### Typed REST API

Prefer generated service resources when they exist. They provide typed request
and response structs plus query helpers for endpoints with path, query, body,
or file inputs.

```rust,no_run
use larksuite_oapi_sdk_rs::service::im::v1::CreateMessageReqBody;
use larksuite_oapi_sdk_rs::{Client, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("APP_ID", "APP_SECRET").build()?;
    let option = RequestOption::default();

    let body = CreateMessageReqBody {
        receive_id: Some("CHAT_ID".to_string()),
        msg_type: Some("text".to_string()),
        content: Some(r#"{"text":"Hello from Rust"}"#.to_string()),
        uuid: None,
    };

    let resp = client
        .im()
        .message
        .create("chat_id", &body, &option)
        .await?;

    println!("success: {}", resp.success());
    Ok(())
}
```

### Raw API escape hatch

Use raw requests for OpenAPI endpoints that do not have a generated Rust
resource yet. The SDK still applies token handling, request IDs, retries, and
response helpers.

```rust,no_run
use larksuite_oapi_sdk_rs::{
    AccessTokenType, ApiReq, Client, HttpMethod, RequestOption,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("APP_ID", "APP_SECRET").build()?;
    let option = RequestOption {
        user_access_token: Some("USER_ACCESS_TOKEN".to_string()),
        ..Default::default()
    };

    let mut req = ApiReq::new(HttpMethod::GET, "/open-apis/contact/v3/users/:user_id");
    req.path_params.set("user_id", "ou_xxx");
    req.query_params.set("user_id_type", "open_id");

    let (api_resp, raw) = client
        .raw_request_typed_with_token::<serde_json::Value>(
            req,
            AccessTokenType::User,
            &option,
        )
        .await?;

    println!("request_id: {:?}", api_resp.request_id());
    println!("data: {:?}", raw.data);
    Ok(())
}
```

See [`examples/raw_api.rs`](examples/raw_api.rs) for both user-token and
tenant-token raw calls.

### One-click app registration

The `registration` module mirrors the Go SDK's `scene/registration` package.
It uses the OAuth 2.0 device-code flow to create or update an app after the
user opens the verification URL or scans it as a QR code.

```rust,no_run
use larksuite_oapi_sdk_rs::registration::{Options, register_app};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = register_app(Options {
        source: "quickstart".to_string(),
        domain: String::new(),
        lark_domain: String::new(),
        app_preset: None,
        addons: None,
        create_only: false,
        app_id: String::new(),
        on_qr_code: Box::new(|info| {
            println!("open or scan this URL: {}", info.url);
            println!("expires in {} seconds", info.expire_in);
        }),
        on_status_change: None,
    })
    .await?;

    println!("App ID: {}", result.client_id);
    println!("App Secret: {}", result.client_secret);
    Ok(())
}
```

See [`examples/app_registration.rs`](examples/app_registration.rs) for custom
scopes, events, callbacks, app preset values, and existing-app update inputs.

### WebSocket events

```rust,no_run
use larksuite_oapi_sdk_rs::{Client, EventDispatcher};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("APP_ID", "APP_SECRET").build()?;
    let dispatcher = EventDispatcher::new("VERIFICATION_TOKEN", "ENCRYPT_KEY");
    client.ws_client(dispatcher).start().await?;
    Ok(())
}
```

### Channel helpers

The optional `channel` feature adds Go-SDK-style message normalization and
higher-level send helpers on top of IM and WebSocket APIs.

```rust,no_run
use larksuite_oapi_sdk_rs::channel::{Channel, SendInput};
use larksuite_oapi_sdk_rs::{Client, EventDispatcher, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("APP_ID", "APP_SECRET").build()?;
    let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

    let sent = channel.send(
        &SendInput {
            chat_id: Some("oc_xxx".into()),
            markdown: Some("**hello**".into()),
            title: Some("Rust SDK".into()),
            ..Default::default()
        },
        &RequestOption::default(),
    ).await?;

    println!("message_id: {}", sent.message_id);
    Ok(())
}
```

Incoming channel messages keep the original Lark JSON in `content` and expose
normalized user-facing text plus attached resources through `text` and
`resources`.

```rust,no_run
use larksuite_oapi_sdk_rs::channel::{Channel, ChannelPolicy};
use larksuite_oapi_sdk_rs::{Client, EventDispatcher, LarkError};

fn channel_with_normalized_messages(client: &Client, dispatcher: EventDispatcher) -> Channel<'_> {
    Channel::builder(client, dispatcher)
        .policy(ChannelPolicy::default().require_mention(false))
        .on_message(|message| async move {
            println!("raw: {}", message.content);
            println!("text: {}", message.text.unwrap_or_default());
            for resource in message.resources {
                println!("resource: {} {}", resource.resource_type, resource.file_key);
            }
            Ok::<(), LarkError>(())
        })
        .build()
}
```

For uploads, `SendInput` accepts pre-uploaded image/file/audio/video keys,
`image_path`/`file_path`, or `UploadInput` with `source_path`, `source_bytes`,
or `source_url`. URL uploads are fetched with a conservative SSRF guard and a
bounded response size. Audio/video uploads may pass `duration` explicitly; when
omitted, the channel helper infers duration for Opus/OGG audio and MP4 video.

### OAuth authorization code

```rust,no_run
use larksuite_oapi_sdk_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("APP_ID", "APP_SECRET").build()?;
    let v1 = client.authen();
    let resp = v1.oauth.retrieve_by_authorization_code(
        "AUTH_CODE",
        Some("https://example.com/callback"),
        None, None,
        &Default::default(),
    ).await?;
    println!("access_token: {:?}", resp.data.and_then(|d| d.access_token));
    Ok(())
}
```

### Card message building

```rust
use larksuite_oapi_sdk_rs::card::{
    Card, CardConfig, CardHeader, div, md, button
};

let card = Card::new()
    .config(CardConfig::new().wide_screen_mode(true))
    .header(CardHeader::new("Hello").template("blue"))
    .element(div("**Welcome**"))
    .element(md("*This is markdown*"));
```

For card callbacks, use `CardActionHandler` directly or pass it through
`http_handler::card_action_handler`; the optional `axum` feature also exposes
`axum_handler::card_action_handler`. See
[`examples/card_action_handler.rs`](examples/card_action_handler.rs).

## Features (Cargo)

| Feature | Description |
|---------|-------------|
| `ws`    | WebSocket long-connection event client (protobuf framing) |
| `channel` | Channel message normalization and send helpers; enables `ws` |
| `axum`  | Axum HTTP adapter for event/card action handlers |

```toml
[dependencies]
larksuite-oapi-sdk-rs = { version = "0.2", features = ["ws", "axum"] }
```

## Generated API coverage

Dedicated service modules are the preferred API surface when a resource exists.
Generated services expose named query structs and `*_by_query` methods for
request shapes that have path, query, body, or file inputs. Existing positional
methods remain as compatibility adapters where they already existed.
Streaming download helpers follow the same query-helper convention and expose
HTTP metadata before the response body is consumed. Because the body is not
pre-read, Lark JSON code errors returned as a 2xx response body are surfaced to
the caller as body bytes rather than retried after metadata has been exposed.
Non-success JSON responses may still be read before returning so token-refresh
retries keep the same behavior as buffered requests. See
[`examples/im_stream_download.rs`](examples/im_stream_download.rs) for a
message-resource stream that writes chunks to disk, hashes incrementally, and
enforces an optional byte limit.

Paginated resources may also expose `*_by_iterator` helpers. These iterators
own their request inputs, fetch pages lazily, and provide `limit`,
`page_token`, and `next_page_token` controls for resumable scans. Contact and
Bitable list/search resources are the first generated-service areas promoted to
this pattern, alongside the existing IM, Drive, and CoreHR iterators.
Hire v1 catalog resources such as registration schemas, resume sources, job
functions, job types, locations, roles, and websites also expose typed response
data and matching iterator helpers. Lightweight Hire reference lists now cover
subjects, talent folders, termination reasons, todos, and user-role assignments
with typed response data, plus iterators where the Go SDK exposes
`ListByIterator`. Hire task lists for evaluation, exam marking, and interview
work also expose typed response data and iterator helpers. Hire interview
support lists cover application interviews, interviewers, feedback forms,
registration schemas, and round types, with iterators where the Go SDK exposes
them. Hire website posting resources now type website job posts, referral job
posts, portal apply schemas, and website channels, with iterators for the
paginated job-posting and portal schema scans. The typed Hire surface also
includes read/list coverage for job process and schema config, Offer
application forms and approval templates, questionnaires, talent tags,
employees, evaluations, notes, interview records, tripartite agreements, and
background check orders, with iterators for the Go-backed paginated lists.
Referral and talent read/search helpers cover referral lookup/search, talent
object schemas, talent operation logs, and talent pools, including an iterator
for talent pool search. External Hire read helpers now type external
applications, background checks, interviews, offers, and talent interview
lookups, with iterators for the Go-backed external list and batch-query
surfaces. Job utility read/search helpers now type job publish record search,
job requirement lookup by ID, and location query responses. Agency read/query
helpers now type supplier, agency detail, account, and protection payloads.
Auxiliary Hire reads now type diversity inclusion, interview attachment,
minutes, and website delivery task payloads. Hire utility helpers now type job
manager, referral account asset, talent ID lookup, test search, and website
delivery creation payloads, with an iterator for test search. Hire write
utility helpers now type employee, interviewer, note, attachment, and website
channel write responses. Hire external write helpers now type external
application, background check, interview, offer, interview assessment, referral
reward, talent external info, and website site-user responses. Existing-model
Hire write helpers now type job manager batch updates, job requirement
creation, referral account write actions, reconciliation, withdrawals, and
tripartite agreement creation. Simple Talent and Exam write helpers now type
folder updates, combined talent create/update, talent-pool moves, and exam
creation. Hire action helpers now type application transfer-onboard and job
recruiter response payloads. Hire Job helpers now type configuration reads and
updates, plus combined job create and update response payloads. Application
Offer lookups now return structured offer, compensation, and send-record data.
Job detail lookups return structured job, manager, requirement, address,
configuration, tag, and stage-count data.

For newer Go SDK endpoints that have not yet been promoted to dedicated Rust
resources, use `GoV397Endpoint` through `client.go_v397()` to make typed raw
requests with the same token handling as the rest of the SDK.

This crate keeps a curated set of examples instead of porting the Go SDK's full
generated `sample/apiall` tree. Use the [examples index](examples/README.md)
for representative generated-service calls, and the generated service smoke
tests for request-shape coverage across the broader API surface.

## Go SDK sample map

The official Go SDK includes both hand-written samples and a generated
`sample/apiall` tree. This crate mirrors the hand-written sample categories
with focused Rust examples:

| Go SDK sample | Rust entry point | Notes |
| --- | --- | --- |
| `sample/client/main.go` | [`examples/client_config.rs`](examples/client_config.rs) | Client builder options, timeouts, headers, retries, logging, token cache toggles |
| `sample/callrawapi/api.go` | [`examples/raw_api.rs`](examples/raw_api.rs) | `ApiReq`, path/query params, user and tenant token raw calls |
| `sample/api/im/im.go` | [`examples/send_message.rs`](examples/send_message.rs), [`examples/im_message_query.rs`](examples/im_message_query.rs), [`examples/im_upload_download.rs`](examples/im_upload_download.rs), [`examples/im_stream_download.rs`](examples/im_stream_download.rs) | Typed IM send, list, upload, buffered download, and streaming download flows |
| `sample/event/event.go` | [`examples/event_handler.rs`](examples/event_handler.rs) | Webhook event dispatcher and callback payload handling |
| `sample/ws/sample.go` | [`examples/ws_client.rs`](examples/ws_client.rs) | Long-connection event stream; enable the `ws` feature |
| `sample/card/card.go` | [`examples/card_action_handler.rs`](examples/card_action_handler.rs), card builder APIs | Card callback handling and interactive card JSON construction |
| `sample/channel/main.go` | [`examples/channel_send.rs`](examples/channel_send.rs), [`examples/channel_normalize.rs`](examples/channel_normalize.rs) | Channel send helpers and normalized incoming messages; enable the `channel` feature |
| `sample/api/bitable2.go`, `sheets.go`, `docx.go`, `application.go`, `calendar` samples | Generated service examples in [`examples/README.md`](examples/README.md) | Representative typed resources across common services |
| `sample/apiall/...` | Generated service smoke tests and `client.go_v397()` | Broad request-shape coverage is kept in tests instead of hand-maintained examples |

## Requirements

- Rust 1.95.0+
- Edition 2024

## License

MIT
