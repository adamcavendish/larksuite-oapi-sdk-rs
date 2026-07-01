# larksuite-oapi-sdk-rs

Lark/Feishu OpenAPI SDK for Rust.

[![Crates.io](https://img.shields.io/crates/v/larksuite-oapi-sdk-rs)](https://crates.io/crates/larksuite-oapi-sdk-rs)
[![Docs](https://docs.rs/larksuite-oapi-sdk-rs/badge.svg)](https://docs.rs/larksuite-oapi-sdk-rs)
[![CI](https://github.com/adamcavendish/larksuite-oapi-sdk-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/adamcavendish/larksuite-oapi-sdk-rs/actions/workflows/ci.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A Rust port of the official [Go SDK](https://github.com/larksuite/oapi-sdk-go).

## Features

- **REST API client** with automatic token management, caching, and retry
- **Client assertion (JWT bearer)** for secretless deployments
- **OAuth flows** — authorization code exchange and refresh token rotation
- **Event dispatching** with AES-256-CBC decryption and SHA-1/SHA-256
  signature verification
- **Card action handling** with typed callback actions
- **WebSocket client** (`ws` feature) for long-connection event streams with
  protobuf framing and fragment reassembly
- **Axum adapter** (`axum` feature) for HTTP server integration
- **Message Card builder** for constructing interactive Lark card messages
- **56+ service modules** covering IM, Calendar, Drive, Sheets, Docs,
  Contacts, Approval, VC, Wiki, and more

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
larksuite-oapi-sdk-rs = "0.1"
```

### REST API

```rust
use larksuite_oapi_sdk_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("APP_ID", "APP_SECRET").build()?;
    let option = larksuite_oapi_sdk_rs::RequestOption::default();
    let resp = client.get("/open-apis/contact/v3/users/me", &option).await?;
    println!("{resp:?}");
    Ok(())
}
```

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
`image_path`/`file_path`, or `UploadInput` with `source_path`/`source_bytes`.
URL uploads and automatic audio/video duration detection are intentionally
deferred; pass an explicit key, local path, or byte buffer, and provide
`duration` for audio/video uploads.

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

## Features (Cargo)

| Feature | Description |
|---------|-------------|
| `ws`    | WebSocket long-connection event client (protobuf framing) |
| `channel` | Channel message normalization and send helpers; enables `ws` |
| `axum`  | Axum HTTP adapter for event/card action handlers |

```toml
[dependencies]
larksuite-oapi-sdk-rs = { version = "0.1", features = ["ws", "axum"] }
```

## Generated API coverage

Dedicated service modules are the preferred API surface when a resource exists.
For newer Go SDK endpoints that have not yet been promoted to dedicated Rust
resources, use `GoV397Endpoint` through `client.go_v397()` to make typed raw
requests with the same token handling as the rest of the SDK.

## Requirements

- Rust 1.94.0+
- Edition 2024

## License

MIT
