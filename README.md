# larksuite-oapi-sdk-rs

Lark/Feishu OpenAPI SDK for Rust.

A Rust port of the official [Go SDK](https://github.com/larksuite/oapi-sdk-go).

## Features

- Self-built and marketplace app support
- Automatic token management with caching and retry on expiry
- Event dispatching with AES-256-CBC decryption and signature verification
- Card action handling
- Configurable HTTP client, timeouts, and default headers

## Usage

```rust
use larksuite_oapi_sdk_rs::{Client, RequestOption};

#[tokio::main]
async fn main() {
    let client = Client::builder("app_id", "app_secret")
        .build();

    let option = RequestOption::default();
    let resp = client.get("/open-apis/contact/v3/users/me", &option).await;
    println!("{resp:?}");
}
```

## Requirements

- Rust 1.94.0+
- Edition 2024

## License

MIT
