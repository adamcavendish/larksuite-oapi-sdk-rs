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
control.close_and_wait().await;
task.await??;
# Ok(())
# }
```

`WsClientControl::close` requests shutdown without waiting, which is safe from
lifecycle callbacks. `close_and_wait` is for external teardown and waits until
the running client has stopped. The client honors a finite reconnect count from
the gateway bootstrap configuration, bounds each connection write to ten
seconds by default, and accepts `write_timeout` for a different bound. Use
`websocket_connector` when a custom proxy, TLS, or test dial transport is
needed; it affects only the WebSocket gateway dial, not the bootstrap HTTP
client.

## Calendar share-token joins

Calendar invitations provide an opaque share token through their link, QR
code, or RSVP card. Join with that token; the endpoint deliberately has no
event-ID alternative.

```rust,no_run
use larksuite_oapi_sdk_rs::service::calendar::v4::JoinCalendarEventReqBody;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn example() -> Result<(), larksuite_oapi_sdk_rs::LarkError> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
client
    .calendar()
    .calendar
    .join_event(
        &JoinCalendarEventReqBody::new("CALENDAR_SHARE_TOKEN"),
        &RequestOption::default(),
    )
    .await?;
# Ok(())
# }
```

Use the optional `axum` feature for Axum HTTP adapters. The optional `messaging`
feature provides outbound send, reply, update, upload, and download operations
without WebSocket dependencies. The optional `channel` feature adds inbound
message normalization and runtime policy controls and enables both `messaging`
and `ws`.

## Replies

`ChannelMessaging::send` preserves convenience behavior: a reply whose target has
disappeared may retry as a top-level send. For workflows that must never escape
a thread, use `ChannelMessaging::reply` or `ChannelMessaging::reply_in_thread`.
Both methods only call the reply endpoint and return its error unchanged. Set
`uuid` to opt into Lark's one-hour idempotency window; automatic split messages
derive distinct UUIDs, so the supplied UUID must leave room for a `-N` suffix.

```rust,no_run
use larksuite_oapi_sdk_rs::channel::SendInput;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let messaging = client.channel_messaging();
messaging
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

Use `ChannelMessaging::edit_text` to update a text message. Use
`ChannelMessaging::edit_card` only for validated interactive-card documents; it
uses the separate card patch operation. See
[`examples/card_send.rs`](../examples/card_send.rs) for Card JSON delivery and
[`examples/card_action_handler.rs`](../examples/card_action_handler.rs) for
interactive callback responses.

```rust,no_run
use larksuite_oapi_sdk_rs::card::v1::{Card, CardDocument, Div, Element, Header, Text};
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let messaging = client.channel_messaging();
let card = CardDocument::new(
    Card::new()
        .header(Header::new("Done"))
        .element(Element::Div(Div::new(Text::lark_md("Complete")))),
)?;

messaging
    .edit_text("om_text_message", "Updated status", &RequestOption::default())
    .await?;
messaging
    .edit_card(
        "om_card_message",
        &card,
        &RequestOption::default(),
    )
    .await?;
# Ok(())
# }
```

## Cards

Use `card::v1` or `card::v2` to build versioned Card JSON, and validate it with
their `CardDocument` types before sending. `CardActionHandler` handles callback
payloads. See the [Cards guide](cards.md) for CardKit and published Card Builder
templates.
