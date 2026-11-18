# Spark app source browsing

The Spark source endpoints let a user with view permission inspect a shared
creative app's project tree and read one source file. Both calls use a user
access token with `spark:app:read`. The `meta_token` value comes from the
creative page's `/page/<token>` link or app metadata. Pass the bare token.

## List a directory

`client.spark().app.list_page_source` calls
`GET /open-apis/spark/v1/apps/source/files`. A missing `path` lists the project
root. The response contains file or directory entries and an opaque continuation
token.

```rust,no_run
use larksuite_oapi_sdk_rs::service::spark::v1::PageSourceListQuery;
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};

# async fn example(client: &LarkClient, user_token: String) -> Result<(), LarkError> {
let option = RequestOption {
    user_access_token: Some(user_token),
    ..Default::default()
};
let query = PageSourceListQuery::new("shared-page-token")
    .path("src")
    .page_size(50);
let page = client.spark().app.list_page_source(&query, &option).await?;
if let Some(data) = page.data {
    for item in data.items {
        println!("{} {} {}", item.r#type, item.path, item.size);
    }
    if data.has_more {
        // Reuse the same path and pass data.next_page_token to page_token.
    }
}
# Ok(())
# }
```

The SDK preserves file sizes as `i64` and passes page tokens without changing
their contents. A page size outside 1–100 is rejected locally.

## Read a source file

`client.spark().app.get_page_source` calls
`GET /open-apis/spark/v1/apps/source/file` and returns the existing
`DownloadStreamResp`. Consume `body.next_chunk()` to process the raw bytes.
The successful body is kept as source content even when it looks like a JSON
API response. The platform limits source files to 10 MiB.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};

# async fn example(client: &LarkClient, user_token: String) -> Result<(), LarkError> {
let option = RequestOption {
    user_access_token: Some(user_token),
    ..Default::default()
};
let mut source = client
    .spark()
    .app
    .get_page_source("shared-page-token", "src/main.ts", &option)
    .await?;
while let Some(chunk) = source.body.next_chunk().await? {
    // Write or inspect chunk without loading the whole file.
    let _ = chunk;
}
# Ok(())
# }
```

`path` must be a relative project path. The SDK does not access the local
filesystem and does not decode the content for the caller.

Contract source: official `larksuite/cli` main commit `c5fbf1a5`, including
`shortcuts/apps/apps_page_source_list.go` and
`shortcuts/apps/apps_page_source_get.go`.
