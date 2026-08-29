# Tutorial: build a Lark / Feishu (飞书) bot

In this tutorial, you will build a bot that receives a text message and replies
in the same chat. It uses a WebSocket long connection, so you do not need to
expose an HTTP callback URL while developing. By the end, you will have a
running Rust process that prints each received message and sends a reply through
the typed IM API.

## What you need

- Rust 1.95.0 or newer.
- A Lark / Feishu Open Platform app with a bot capability.
- The app's `APP_ID` and `APP_SECRET`.
- Permission to subscribe the app to events and to send messages as its bot.

This tutorial subscribes to `im.message.receive_v1` and sends messages with the
tenant permission `im:message:send_as_bot`. Configure those two items in your
app's Open Platform console, choose WebSocket (long connection) event delivery,
then make the app available to the test tenant. The console labels differ
slightly between Lark and Feishu regions, but the event and permission names are
the same.

## Step 1: create the app and record its credentials

Create an app in the Lark or Feishu Open Platform console and add its bot
capability. In the app configuration:

1. Subscribe to the tenant event `im.message.receive_v1`.
2. Grant the tenant permission `im:message:send_as_bot`.
3. Select WebSocket (long connection) as the event-delivery method.
4. Publish or enable the app for the tenant where you will test it.

Copy the App ID and App Secret somewhere safe. They will become environment
variables in step 3. Add the bot to a test chat, so you have somewhere to send
it a message.

## Step 2: create the Rust project and add the bot

Create a binary project and enable the SDK's `ws` feature. The feature includes
the WebSocket event client; normal typed REST resources, including IM, are
available without an additional feature.

```bash
cargo new feishu-bot
cd feishu-bot
cargo add larksuite-oapi-sdk-rs@0.3.10 --features ws
cargo add tokio --features macros,rt-multi-thread
```

Replace `src/main.rs` with the following program. Its generated, typed event
handler only queues incoming events. A worker receives the queue, filters for
text messages, and uses the chat ID from the event to send a reply. Keeping the
event handler short lets the WebSocket client acknowledge delivery before an
outbound API request or slower application work begins.

```rust,no_run
use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;
use larksuite_oapi_sdk_rs::service::im::v1::CreateMessageReqBody;
use larksuite_oapi_sdk_rs::{EventDispatcher, LarkClient, LarkError, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let client = LarkClient::builder(app_id, app_secret).build()?;

    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<P2MessageReceiveV1>(128);
    let worker_client = client.clone();
    let worker = tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            if event.message.message_type != "text" {
                continue;
            }

            let message_id = event.message.message_id;
            let chat_id = event.message.chat_id;
            println!("received: id={message_id} content={}", event.message.content);

            let body = CreateMessageReqBody {
                receive_id: Some(chat_id),
                msg_type: Some("text".to_string()),
                content: Some(r#"{"text":"Hello from Rust!"}"#.to_string()),
                uuid: Some(format!("reply-{message_id}")),
            };

            match worker_client
                .im()
                .message
                .create("chat_id", &body, &RequestOption::default())
                .await
            {
                Ok(response) if response.success() => println!("reply sent"),
                Ok(response) => eprintln!("reply rejected: {:?}", response.code_error),
                Err(error) => eprintln!("reply failed: {error}"),
            }
        }
    });

    let dispatcher = EventDispatcher::new("", "").on_p2_im_message_receive_v1(
        move |event: P2MessageReceiveV1| {
            let event_tx = event_tx.clone();
            async move {
                event_tx.try_send(event).map_err(|error| {
                    LarkError::Event(format!("event queue is full or closed: {error}"))
                })
            }
        },
    );

    println!("starting bot; press Ctrl+C to stop");
    client
        .ws_client(dispatcher)
        .on_ready(|| println!("websocket connected"))
        .on_disconnected(|| eprintln!("websocket disconnected"))
        .on_error(|error| eprintln!("websocket error: {error}"))
        .start()
        .await?;
    worker.await?;
    Ok(())
}
```

The empty strings passed to `EventDispatcher::new` are intentional for this
long-connection example: the WebSocket client obtains its gateway connection
from the app credentials. For an HTTP webhook, provide the verification token
and encryption key configured in the Open Platform console instead; see
[Events, WebSockets, and channels](events-and-channels.md).

The bounded queue makes pressure explicit: if its worker cannot keep up, the
handler returns an error instead of silently accepting work it cannot process.
For a production bot, add policy before enqueueing (for example, ignore the
bot's own messages and require a mention in group chats), deduplicate by
`message_id`, and select queue capacity and worker ownership for your traffic.
The reply UUID is derived from that same message ID, so a retry of the same
delivery does not create another message within Lark's idempotency window.

## Step 3: run it and send the bot a message

Start the process with the credentials from step 1:

```bash
APP_ID='cli_your_app_id' \
APP_SECRET='your_app_secret' \
cargo run
```

When the process prints `starting bot`, send a text message in the test chat
that contains the bot. The terminal prints the message event, and the bot sends
`Hello from Rust!` to the same chat.

If the terminal starts but no event arrives, check that the app is enabled for
the test tenant, the bot belongs to the chat, WebSocket delivery is selected,
and `im.message.receive_v1` is subscribed. If the event arrives but the reply
does not, confirm that `im:message:send_as_bot` is approved for the app and that
the bot has permission to speak in that chat.

## What you built

You now have a typed event handler and a typed message send in one Tokio
process. From here, you can:

- handle another event by registering one of the typed methods on
  [`EventDispatcher`](https://docs.rs/larksuite-oapi-sdk-rs/latest/larksuite_oapi_sdk_rs/struct.EventDispatcher.html);
- send richer messages or interactive cards with
  [`examples/send_message.rs`](../examples/send_message.rs) and the
  [Cards guide](cards.md);
- switch to a verified HTTP webhook when your deployment already has a public
  endpoint; see [Events, WebSockets, and channels](events-and-channels.md);
- automate other Open Platform services through the curated
  [examples](../examples/README.md) and [documentation index](README.md).
