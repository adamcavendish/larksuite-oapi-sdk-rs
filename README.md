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
| `axum`  | Axum HTTP adapter for event/card action handlers |

```toml
[dependencies]
larksuite-oapi-sdk-rs = { version = "0.1", features = ["ws", "axum"] }
```

## Requirements

- Rust 1.94.0+
- Edition 2024

## License

MIT
