# Base v3 record reads

`client.base_v3().record` provides the reusable record-read operations used by
the official Lark CLI's Base shortcuts:

- `list` reads records with offset pagination and optional field, view, filter,
  and sort parameters.
- `search` accepts the upstream JSON request body unchanged. This keeps search
  filters, projections, sorting, and offset pagination forward-compatible while
  the endpoint's schema remains CLI-owned.

Both operations accept either an explicit user access token or tenant access
token in `RequestOption`. The SDK automatically supplies `X-App-Id` from the
client configuration, matching the CLI's protocol behavior.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
use larksuite_oapi_sdk_rs::service::base::v3::ListRecordQuery;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let option = RequestOption {
    user_access_token: Some("USER_ACCESS_TOKEN".into()),
    ..RequestOption::default()
};
let response = client.base_v3().record.list(
    &ListRecordQuery::new("bascn...", "tbl...").offset(0).limit(100),
    &option,
).await?;
println!("{:#?}", response.data);
# Ok(())
# }
```

`filter` and `sort` on `ListRecordQuery` are JSON-encoded query strings. Put
the corresponding structured values directly in the `search` request body.
See [`examples/base_v3_record_read.rs`](../examples/base_v3_record_read.rs) for
a runnable, read-only list example.

## Maintainer live verification

The ignored live test only lists and searches records; it does not create,
update, or delete data. It requires a Base table that the chosen user token can
read. Set:

- `FEISHU_BASE_V3_RECORD_LIVE=1`
- `FEISHU_APP_ID`, `FEISHU_APP_SECRET`, and `FEISHU_BASE_V3_USER_ACCESS_TOKEN`
- `FEISHU_BASE_V3_BASE_TOKEN` and `FEISHU_BASE_V3_TABLE_ID`
- optional `FEISHU_BASE_V3_SEARCH_BODY`, a JSON object. The default is a
  minimal `{"offset":0,"limit":1}` body.

Run:

```bash
cargo test --test base_v3_record_live feishu_base_v3_record_live_read -- --ignored --exact
```
