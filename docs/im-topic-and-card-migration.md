# IM topic replies and live card migration

This SDK exposes the Lark IM message fields needed by daemons that route
normal group messages, topic/thread messages, and live card updates without
falling back to raw JSON.

## Receive event fields

Old raw JSON path used by downstream code:

```rust
let message = &event["message"];
let thread_id = message["thread_id"].as_str();
let root_id = message["root_id"].as_str();
let parent_id = message["parent_id"].as_str();
let message_id = message["message_id"].as_str();
```

New typed SDK path:

```rust
use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;

async fn handle(event: P2MessageReceiveV1) {
    let message_id = &event.message.message_id;
    let root_id = &event.message.root_id;
    let parent_id = &event.message.parent_id;
    let thread_id = &event.message.thread_id;
    let sender_open_id = event.sender.open_id();
}
```

Topic gotcha: Lark topic-group traffic can arrive as `chat_type = "group"`.
Preserve both IDs in your normalized event. `thread_id` (`omt_...`) identifies
the topic, while reply APIs may need `root_id` or the original `message_id`
(`om_...`) as the reply anchor. Do not fall back to a different chat or parent
when both the topic and same-anchor reply IDs are missing.

Mentions are typed as `events::im::Mention`; use `mention.open_id()`,
`mention.user_id()`, or `mention.union_id()` instead of indexing raw JSON.

## Reply in thread

```rust
use larksuite_oapi_sdk_rs::service::im::v1::ReplyMessageReqBody;

let anchor = if event.message.root_id.is_empty() {
    &event.message.message_id
} else {
    &event.message.root_id
};

let body = ReplyMessageReqBody {
    msg_type: Some("text".to_string()),
    content: Some(r#"{"text":"working"}"#.to_string()),
    reply_in_thread: Some(true),
    uuid: None,
};

client.im().message.reply(anchor, &body, &Default::default()).await?;
```

## Live card reply and patch

```rust
use larksuite_oapi_sdk_rs::card::{Card, CardHeader, div};
use larksuite_oapi_sdk_rs::service::im::v1::{
    PatchMessageReqBody, ReplyMessageReqBody,
};

let card = Card::new()
    .header(CardHeader::new("Working"))
    .element(div("Preparing answer"));
let body = ReplyMessageReqBody::interactive_card(&card)?.reply_in_thread(true);
let sent = client.im().message.reply(anchor, &body, &Default::default()).await?;
let live_message_id = sent.data.and_then(|data| data.message_id).unwrap();

let final_card = Card::new()
    .header(CardHeader::new("Done"))
    .element(div("Final answer"));
let patch = PatchMessageReqBody::interactive_card(&final_card)?;
client.im().message.patch(&live_message_id, &patch, &Default::default()).await?;
```

If you already have a serialized Lark card JSON string, use the `_content`
helpers to avoid double-encoding it:

```rust
let body = ReplyMessageReqBody::interactive_card_content(card_json).reply_in_thread(true);
```

## Raw escape hatch

If an endpoint is not generated yet, build `ApiReq` and call
`Client::raw_request_with_token` or `Client::raw_request_typed_with_token` with
the token type accepted by that endpoint. These methods reuse SDK auth, token
caching, retries, base URL handling, and request options.

```rust
use larksuite_oapi_sdk_rs::{AccessTokenType, ApiReq, HttpMethod};

let req = ApiReq::new(HttpMethod::POST, "/open-apis/example/v1/missing");
let resp = client
    .raw_request_with_token(req, AccessTokenType::Tenant, &Default::default())
    .await?;
```
