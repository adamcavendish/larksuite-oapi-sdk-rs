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

## Author Card JSON 2.0 layouts safely

`CardDocument::new` remains the final strict protocol gate. For dynamic
authoring, `CardDocument::new_with_diagnostic` preserves the original
`ValidationError` and also returns a stable code, a JSON Pointer, the violated
constraint, and relevant legal values. Do not parse its display string.

`ColumnSet` has constructors for the three legal width branches. They emit the
wire shape required by Card JSON 2.0 and leave document validation enabled:

```rust
use larksuite_oapi_sdk_rs::card::v2::{
    AutoColumn, Body, Card, CardDocument, ColumnSet, ColumnWeight, Config,
    Element, FixedColumn, FixedColumnWidth, Markdown, WeightedColumn,
};

let automatic = Element::ColumnSet(ColumnSet::automatic([
    AutoColumn::new().element(Element::Markdown(Markdown::new("auto"))),
]));

let fixed = Element::ColumnSet(ColumnSet::fixed([
    FixedColumn::new(FixedColumnWidth::pixels(160)?)
        .element(Element::Markdown(Markdown::new("fixed"))),
]));

let weighted = Element::ColumnSet(ColumnSet::weighted([
    WeightedColumn::new(ColumnWeight::new(3)?)
        .element(Element::Markdown(Markdown::new("weighted"))),
]));

let card = Card::new()
    .config(Config::new().update_multi())
    .body(Body::new().element(automatic).element(fixed).element(weighted));
let _document = CardDocument::new(card)?;
# Ok::<(), larksuite_oapi_sdk_rs::card::v2::ValidationError>(())
```

The `Padding`, `Margin`, `Spacing::pixels`, `FixedColumnWidth`, and
`ColumnWeight` constructors encode the documented finite pixel/range grammar.
Existing string-oriented builders remain supported for compatibility and are
still validated by `CardDocument`.

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
