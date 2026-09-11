# Spark app source export

`client.spark().app.export` streams `POST /open-apis/spark/v1/apps/export`.
It requires a user access token with `spark:app:read`. This is app source
export, not Spark storage-file download.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
use larksuite_oapi_sdk_rs::service::spark::v1::{ExportAppError, ExportAppLocator};

# async fn example(client: &LarkClient, user_token: String) -> Result<(), ExportAppError> {
let option = RequestOption {
    user_access_token: Some(user_token),
    ..Default::default()
};
let mut export = client.spark().app
    .export(ExportAppLocator::AppId("app-id"), &option)
    .await?;
// Alternatively: ExportAppLocator::MetaToken("meta-token").
// The enum sends exactly one locator in the JSON body.
while let Some(chunk) = export.body.next_chunk().await? {
    // Forward each chunk to your own file, object store or HTTP response.
    // Do not collect all chunks if bounded archive memory is important.
    let _ = chunk;
}
# Ok(())
# }
```

The response exposes HTTP metadata, an optional server-supplied filename and
content length, and a `DownloadBody`. Filename metadata is untrusted: the SDK
does not create paths or extract archives. Only 2xx responses explicitly marked
`application/zip` or `application/octet-stream` are accepted. This is a
content-type check, not ZIP integrity validation.

Non-archive responses, including HTTP 200 JSON/text failures, return
`ExportAppError::InvalidResponse` with HTTP status/headers, up to 4 KiB of body
prefix in `api_resp.raw_body`, and a `code_error` when a complete nonzero JSON
error envelope can be decoded. Oversized/truncated or malformed envelopes may
have no decoded code; the request still fails. Code `40901` means the app has
no published build to export: publish the appropriate build before retrying.
Transport/auth failures use `ExportAppError::Request`; stream read failures
after export starts return `LarkError` from `next_chunk`.

The source snapshot depends on app type. The official CLI describes the
default branch's last commit (not uncommitted sandbox work) for source-backed
apps; artifact-hosted apps require a successfully published build. The SDK
does not publish, poll for builds, or retry business-level export failures.
Existing configured transport retries still apply; `.max_retries(1)` on the
client selects one transport attempt.

Contract source: official Lark CLI commit `0493db0c`, in
`shortcuts/apps/apps_export.go` (request, archive/error handling and source
semantics). This CLI-derived endpoint is not in the pinned Go v3.12.0 catalog.
Mock tests verify the request and streaming contracts; live authorization,
app-type availability and exported content require separate platform testing.
