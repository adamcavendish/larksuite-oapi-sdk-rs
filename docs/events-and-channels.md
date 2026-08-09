# Events, WebSockets, and channels

Webhook handlers use `EventDispatcher` for typed event callbacks, signature
verification, and encrypted payload processing. The optional `ws` feature adds
long connections for event delivery; see
[`examples/ws_client.rs`](../examples/ws_client.rs).

Trusted user channels match the upstream Go SDK behavior. Configure the channel
before obtaining its control handle, start the client, then bind a user token
after the connection becomes ready. See
[`examples/ws_client.rs`](../examples/ws_client.rs) for the connection lifecycle
and [`examples/channel_send.rs`](../examples/channel_send.rs) for higher-level
messages.

```rust,no_run
use std::collections::HashMap;
use std::time::Duration;

# use larksuite_oapi_sdk_rs::{EventDispatcher, LarkClient};
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

## Replies

`Channel::send` preserves convenience behavior: a reply whose target has
disappeared may retry as a top-level send. For workflows that must never escape
a thread, use `Channel::reply` or `Channel::reply_in_thread`. Both methods only
call the reply endpoint and return its error unchanged. Set `uuid` to opt into
Lark's one-hour idempotency window; automatic split messages derive distinct
UUIDs, so the supplied UUID must leave room for a `-N` suffix.

```rust,no_run
use larksuite_oapi_sdk_rs::channel::{Channel, SendInput};
use larksuite_oapi_sdk_rs::{EventDispatcher, LarkClient, RequestOption};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();
channel
    .reply_in_thread(
        "om_parent_message",
        &SendInput {
            markdown: Some("A reply that stays in the topic".into()),
            uuid: Some("order-42-status".into()),
            ..Default::default()
        },
        &RequestOption::default(),
    )
    .await?;
# Ok(())
# }
```

## Message updates

Use `Channel::edit_text` to update a text message. Use `Channel::edit_card`
only for interactive cards; it uses the separate card patch operation. See
[`examples/channel_send.rs`](../examples/channel_send.rs) for sending and
[`examples/card_action_handler.rs`](../examples/card_action_handler.rs) for
interactive callback responses.

```rust,no_run
use larksuite_oapi_sdk_rs::card::{div, Card, CardHeader};
use larksuite_oapi_sdk_rs::channel::Channel;
use larksuite_oapi_sdk_rs::{EventDispatcher, LarkClient, RequestOption};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let channel = Channel::builder(&client, EventDispatcher::new("", "")).build();

channel
    .edit_text("om_text_message", "Updated status", &RequestOption::default())
    .await?;
channel
    .edit_card(
        "om_card_message",
        Card::new().header(CardHeader::new("Done")).element(div("Complete")),
        &RequestOption::default(),
    )
    .await?;
# Ok(())
# }
```

## Cards

The `card` module builds interactive Lark cards, and `CardActionHandler` handles
callback payloads. Consult the generated API documentation for card builders.
