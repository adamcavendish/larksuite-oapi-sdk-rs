# Slides AI presentation content

`client.slides_ai()` exposes the official Lark CLI's reusable Slides AI v1 Open
Platform operations. It is deliberately separate from Docs AI document content,
structured Docx blocks, and the CLI's local XML authoring workflow.

The official Go SDK catalog does not currently model these endpoints. The SDK
therefore owns the stable route, token, revision, selector, pagination, and
history-task interface while leaving XML presentation and slide-part schemas as
caller-provided `Serialize` values and service-provided `JsonResp` values.

| Resource | Operations |
| --- | --- |
| `presentation` | `create`, `get` |
| `slide` | `get`, `add`, `delete`, `replace` |
| `history` | `list`, `revert`, `revert_status` |

Use an explicit user access token or a tenant access token for a bot. The
client derives the `Authorization` header from `RequestOption`; callers do not
need to add an app ID separately.

```rust,no_run
use larksuite_oapi_sdk_rs::service::slides_ai::v1::{
    GetSlideQuery, GetXmlPresentationQuery, ListXmlPresentationHistoryQuery,
};
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let option = RequestOption {
    user_access_token: Some("USER_ACCESS_TOKEN".into()),
    ..RequestOption::default()
};

let presentation_id = "xml_presentation_id";
let presentation = client.slides_ai().presentation.get(
    &GetXmlPresentationQuery::new(presentation_id),
    &option,
).await?;
let slide = client.slides_ai().slide.get(
    &GetSlideQuery::by_number(presentation_id, 1),
    &option,
).await?;
let history = client.slides_ai().history.list(
    &ListXmlPresentationHistoryQuery::new(presentation_id)
        .page(PageQuery::new().page_size(20)),
    &option,
).await?;
println!("{:#?} {:#?} {:#?}", presentation.data, slide.data, history.data);
# Ok(())
# }
```

## Revisions, selectors, and mutation safety

The presentation and slide query structs default `revision_id` to `-1`, the
server's latest-revision value. Set a specific revision with `.revision_id(...)`
when coordinating concurrent edits. `GetSlideQuery` uses either a slide ID or
a one-based slide number, preventing invalid two-selector requests at the type
level.

`presentation.create`, `slide.add`, and `slide.replace` accept any `Serialize`
value. For replacement, supply the full service body, including its `parts`
array. The SDK forwards those parts unchanged: it does not parse XML, insert
block IDs, normalize content, or construct a `block_replace` request for you.

`slide.delete`, `slide.replace`, and `history.revert` have immediate server-side
effects. Confirm the presentation ID, slide ID, revision, and XML/parts body
before issuing them. A history revert is asynchronous; pass its task ID to
`GetXmlPresentationHistoryRevertStatusQuery::new(presentation_id, task_id)` and
poll `history.revert_status` until it completes.

## Deliberate exclusions

The SDK does not copy the CLI's XML linting, `@path` image uploads, wiki
resolution, permission grants, prompts, or filesystem output. Screenshot and
render endpoints are a separate follow-up because their Base64 image result
needs a focused Rust result interface.

See [`examples/slides_ai_read.rs`](../examples/slides_ai_read.rs) for a
runnable, read-only presentation fetch.
