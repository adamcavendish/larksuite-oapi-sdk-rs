# Drive media preview downloads

`client.drive().media.preview_download(...)` downloads one selected binary
preview artifact from `GET /open-apis/drive/v1/medias/{file_token}/preview_download`.
It accepts either user or tenant request credentials and returns
[`DownloadResp`](../src/service/common.rs), including the response bytes and an
optional file name from `Content-Disposition`.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = LarkClient::builder("app_id", "app_secret").build()?;
let preview = client
    .drive()
    .media
    .preview_download("file_token", "16", Some("file_version"), &RequestOption::default())
    .await?;
println!("downloaded {} bytes", preview.data.len());
# Ok(())
# }
```

`preview_type` stays an open string because available artifact codes are
platform-defined. The official CLI uses `"16"` for a source-file artifact; use
the Drive preview-result endpoint when the available artifacts must be selected
dynamically. The method sends one download request only: it does not implement
the CLI's preview selection, permission fallback, or local-file handling.

The platform controls access to preview artifacts. Configure the relevant Drive
permissions (commonly `drive:file:download`) for the selected resource before
calling the method.
