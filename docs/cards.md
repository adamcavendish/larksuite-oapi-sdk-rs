# Cards

The SDK keeps Card JSON 1.0, Card JSON 2.0, CardKit, and published Card Builder
templates as separate protocols. Choose the smallest surface that matches the
message lifecycle you need.

## Send a Card JSON document

Build a versioned Card JSON value, then create a `CardDocument` before passing
it to IM or channel message helpers. `CardDocument::new` validates the document;
outbound interactive-card helpers deliberately reject arbitrary serializable
values.

[`examples/card_send.rs`](../examples/card_send.rs) sends a validated Card JSON
1.0 document through the IM create-message API:

```sh
APP_ID=... APP_SECRET=... CHAT_ID=oc_... cargo run --example card_send
```

Use `card::v1` for Card JSON 1.0. Use `card::v2` for Card JSON 2.0; its root,
shared-card requirements, and element-identifier rules are incompatible with
1.0.

## Stream a CardKit document

CardKit accepts a validated Card JSON 2.0 `card::cardkit::CardDocument`.
`LarkClient::cardkit_cards()` creates the entity; send that entity once with a
`card::cardkit::CardEntityMessage` through IM before beginning ordered updates.
Keep one update session for full-document, settings, element, and content
updates; each successful mutation advances its sequence. CardKit entities have
the documented 14-day lifetime and may be sent only once.

[`examples/cardkit_stream.rs`](../examples/cardkit_stream.rs) creates a
streaming CardKit card and replaces a Markdown element's complete content:

```sh
APP_ID=... APP_SECRET=... CHAT_ID=oc_... cargo run --example cardkit_stream
```

## Send a published Card Builder template

Card Builder's editor and builder-only components are not a Card JSON AST.
Send a published template with `TemplateMessage<T>`, where `T: Serialize` is
your application's template-variable object. The SDK requires that it serialize
to a JSON object; `TemplateMessage<JsonValue>` remains available for dynamic
bindings.

[`examples/card_template_send.rs`](../examples/card_template_send.rs) sends a
typed published template:

```sh
APP_ID=... APP_SECRET=... CHAT_ID=oc_... CARD_TEMPLATE_ID=... \
  cargo run --example card_template_send
```

## Handle callbacks

Card callbacks are inbound events. Use
[`examples/card_action_handler.rs`](../examples/card_action_handler.rs) for a
typed `CardActionHandler` that returns a callback response. Application-level
authorization, routing, deduplication, and state management remain the
application's responsibility.
