# Docs AI document content

`client.docs_ai()` exposes the official Lark CLI's reusable Docs AI v1 Open
Platform operations. It is intentionally separate from legacy `client.docs()`,
the structured-block `client.docx()`, and OCR-oriented `client.document_ai()`
services.

The official Go SDK catalog does not currently model these endpoints. The SDK
therefore provides stable route, credential, pagination, and task-lookup seams
while preserving the service-owned document payloads and responses as
`Serialize` inputs and `JsonResp` values.

| Resource | Operations |
| --- | --- |
| `document` | `create`, `fetch`, `update` |
| `history` | `list`, `revert`, `revert_status` |

Use either an explicit user access token or a tenant access token for a bot.
The client obtains the appropriate `Authorization` header from
`RequestOption`; callers do not need to send an app ID separately.

```rust,no_run
use larksuite_oapi_sdk_rs::service::docs_ai::v1::ListDocumentHistoryQuery;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let option = RequestOption {
    user_access_token: Some("USER_ACCESS_TOKEN".into()),
    ..RequestOption::default()
};

let document_id = "doxcn_document_id";
let fetched = client.docs_ai().document.fetch(
    document_id,
    serde_json::json!({
        "format": "xml",
        "extra_param": r#"{"enable_user_cite_reference_map":true,"include_comments":true,"return_html5_block_data":true}"#,
    }),
    &option,
).await?;
let history = client.docs_ai().history.list(
    &ListDocumentHistoryQuery::new(document_id).page(PageQuery::new().page_size(20)),
    &option,
).await?;
println!("{:#?} {:#?}", fetched.data, history.data);
# Ok(())
# }
```

## Dynamic payloads and mutation safety

`create`, `fetch`, `update`, and `revert` accept any `Serialize` value. This
keeps Docs AI request bodies forward-compatible: `fetch` may carry
`read_option`, `export_option`, or `extra_param`, while update commands may
include block ranges such as `start_block_id` and `end_block_id`.

`history.revert` starts an asynchronous history restore. Include its returned
task ID in `GetDocumentHistoryRevertStatusQuery::new(document_id, task_id)` and
poll `history.revert_status` until the service reports completion. Create,
update, and revert calls have server-side effects; confirm document IDs, block
ranges, and command bodies before issuing them.

The SDK does not include CLI-only local-file rewrites, URL parsing, prompts,
permission-grant orchestration, or output formatting. Those behaviors do not
belong in a reusable HTTP client.

See [`examples/docs_ai_fetch.rs`](../examples/docs_ai_fetch.rs) for a runnable,
read-only document fetch.
