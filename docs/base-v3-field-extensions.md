# Base v3 field extensions and record share links

`client.base_v3()` provides typed operations for the official Base field
extension endpoints and batch record share-link generation. These requests need
an explicit user or tenant token in `RequestOption`; the client supplies
`X-App-Id` from its configuration.

Field extensions currently support the platform's built-in LLM completion
configuration. A default `UpdateFieldExtensionReqBody::clear()` serializes to
an empty object and removes the configuration. Updating cells is intentionally
separate: a column operation may target a view, while a row operation requires
the record IDs to process.

```rust,no_run
use larksuite_oapi_sdk_rs::service::base::v3::{
    CreateRecordShareLinksReqBody, FieldExtensionCompletionInput,
    FieldExtensionPromptSegment, UpdateFieldExtensionCellsReqBody,
    UpdateFieldExtensionReqBody,
};
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn example() -> Result<(), larksuite_oapi_sdk_rs::LarkError> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let option = RequestOption {
    tenant_access_token: Some("TENANT_ACCESS_TOKEN".into()),
    ..RequestOption::default()
};
let extension = UpdateFieldExtensionReqBody::builtin_llm_completion(
    FieldExtensionCompletionInput::new([
        FieldExtensionPromptSegment::text("Summarize: "),
        FieldExtensionPromptSegment::field_ref("Description"),
    ]),
);

client
    .base_v3()
    .field_extension
    .update("base_token", "table_id", "field_id", &extension, &option)
    .await?;
client
    .base_v3()
    .field_extension
    .update_cells(
        "base_token",
        "table_id",
        "field_id",
        &UpdateFieldExtensionCellsReqBody::rows(["rec_1", "rec_2"]),
        &option,
    )
    .await?;

let links = client
    .base_v3()
    .record
    .create_share_links(
        "base_token",
        "table_id",
        &CreateRecordShareLinksReqBody::new(["rec_1", "rec_2"]),
        &option,
    )
    .await?;
println!("{:#?}", links.data);
# Ok(())
# }
```

Batch share-link responses may omit requested record IDs when the platform does
not expose them to the caller. Check the `record_share_links` map rather than
assuming every requested ID has a link.
