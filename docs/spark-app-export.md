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
with one of these media types are accepted:

- `application/zip`
- `application/octet-stream`
- `application/x-zip-compressed`
- `binary/octet-stream`
- `application/force-download`

Matching is case-insensitive and ignores media-type parameters. The binary
aliases allow exports relabelled by gateways without accepting every non-JSON
response. Missing or unrecognized Content-Type values and text/JSON responses
(including structured `+json` types) are still rejected. This is a content-type
check, not ZIP integrity validation; a mislabeled body can still pass.

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
The additional binary aliases follow the compatibility cases in CLI commit
`2ca601ce`. Unlike that CLI revision, the SDK retains rejection of missing
Content-Type and HTTP 200 text errors instead of accepting all non-JSON bodies.
Mock tests verify the request and streaming contracts; live authorization,
app-type availability and exported content require separate platform testing.

## Spark storage management

`client.spark().app_storage` supports user-token access to Spark app storage:

- `list_files` and `get_file` read file metadata.
- `sign_file` creates a temporary download URL.
- `get_file_quota` returns the platform-reported usage and quota fields without
  SDK rounding or field projection.
- `pre_upload_file` obtains the platform upload URL and upload ID;
  `upload_file_callback` registers that upload using the ETag returned by the
  external upload.
- `batch_remove_files` sends a batch of remote paths. Its successful API
  envelope can still contain individual failures, so inspect `data.results`.

Storage management requires a real Spark app ID. This differs from source
export, which can also use a meta token.

The SDK deliberately leaves the external upload transfer under caller control:
perform the PUT only to the returned presigned URL, capture its ETag, and do not
forward Lark authorization headers to that URL. It does not read local files,
choose a file name, retry destructive operations, or flatten batch results.

## Spark database quota

`client.spark().app.get_db_quota` reads
`GET /open-apis/spark/v1/apps/{app_id}/db/quota`. It requires an explicit,
nonempty user access token with `spark:app:read`; tenant credentials are not a
substitute. This reports database usage and table/view counts, not the
file-storage quota returned by `app_storage.get_file_quota`.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};
use larksuite_oapi_sdk_rs::service::spark::v1::GetDbQuotaQuery;

# async fn example(client: &LarkClient, user_token: String) -> Result<(), LarkError> {
let option = RequestOption {
    user_access_token: Some(user_token),
    ..Default::default()
};
let quota = client.spark().app
    .get_db_quota(&GetDbQuotaQuery::new("app_id").env("online"), &option)
    .await?;
// Inspect quota.data for the platform's unmodified quota and usage fields.
let _ = quota;
# Ok(())
# }
```

Use `.env("dev")` or `.env("online")` to select an environment explicitly.
Without `.env(...)`, the query parameter is omitted: the platform selects
`dev` for multi-environment apps and `online` for single-environment apps.
The SDK does not choose an environment locally. Responses retain zero or
missing quota values, unrounded usage percentages, and unknown fields rather
than applying the CLI's display projection.

Contract source: official Lark CLI commit `32d19889`, in
`shortcuts/apps/apps_db_quota_get.go` and `shortcuts/apps/db_common.go`.
The endpoint was introduced in `75926f97`; `1a9f6378` made environment
selection optional. Mock tests cover the wire contract, not live platform
authorization or availability. No database migration, recovery, polling, or
automatic environment policy is added.
