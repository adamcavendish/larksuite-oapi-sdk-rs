# Spark app listing and deletion

`client.spark().app.list_with_query` filters apps visible to the calling user.
It requires an explicit, nonempty user access token with `spark:app:read`.
`ListAppQuery` combines `owner_id`, `keyword`, `ownership`, `app_type`,
`page_size`, and `page_token` in one server-side query.

```rust,no_run
use larksuite_oapi_sdk_rs::service::spark::v1::ListAppQuery;
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};

# async fn example(client: &LarkClient, user_token: String) -> Result<(), LarkError> {
let option = RequestOption {
    user_access_token: Some(user_token),
    ..Default::default()
};
let query = ListAppQuery::new()
    .owner_id("ou_owner")
    .keyword("Approval")
    .ownership("all")
    .app_type("full_stack")
    .page_size(20);
let response = client.spark().app.list_with_query(&query, &option).await?;
// Inspect response.data for items, has_more, and page_token.
// Keep the same filters when requesting the next page.
# let _ = response;
# Ok(())
# }
```

An omitted owner leaves the server's visibility scope unchanged. An explicit
owner is trimmed, then rejected locally if blank or lacking the `ou_` prefix;
invalid input never silently removes the owner filter. `owner_id` further
narrows the visible apps independently of `ownership`.

Known ownership values are `all`, `mine`, and `shared`; known app types are
`html`, `frontend`, and `full_stack`. These filters use strings so future server
values can pass through. Omitted filters and pagination settings use the server's
defaults. Responses retain all fields, including `icon_url`, `created_at`, and
unknown fields. Empty results are valid. The existing
`list(&SparkPageQuery, option)` remains available with its original behavior.

## Delete a selected app

`client.spark().app.delete` sends
`DELETE /open-apis/spark/v1/apps/{app_id}` once. It requires an explicit,
nonempty user token and a real app ID starting with `app_` (surrounding whitespace
is trimmed). Tenant or app tokens cannot substitute for the user token.

The official CLI declares `spark:app:read` and `spark:app:write` for its deletion
workflow, which includes a GET before DELETE. The minimum scope for a standalone
DELETE has not been independently verified here. This SDK method directly
deletes the supplied ID: callers select the target; the SDK performs no implicit
lookup, name check, or meta-token resolution. A preceding name check cannot
prevent a rename between the check and deletion, because the upstream API has
no name/version precondition.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};

# async fn delete_selected_app(
#     client: &LarkClient, user_token: String, selected_app_id: &str,
# ) -> Result<(), LarkError> {
let option = RequestOption {
    user_access_token: Some(user_token),
    ..Default::default()
};
let deletion = client.spark().app.delete(selected_app_id, &option).await?;
// Inspect deletion.data: only in_trash=true confirms recoverability.
# let _ = deletion;
# Ok(())
# }
```

The open JSON response preserves these distinct outcomes:

- `in_trash=true`: moved to Trash, recoverable within the platform's retention period.
- `in_trash=false`: permanently deleted according to the upstream contract.
- Missing `in_trash`: deletion succeeded without a recovery indication. This is
  a normal success; do not infer recoverability or retry because the field is absent.

Deletion takes effect asynchronously: a read immediately after success may still
find the app. That does not establish failure or justify another DELETE.
Timeouts and connection failures leave the result unknown. The SDK sends once
even if client retries are configured, and preserves platform errors (including
40300/40400 and their gateway-prefixed variants) as `LarkError::Api` rather than
turning them into success or a permission hint.

Contract sources: official larksuite/cli main commits
[`76f7e7eb`](https://github.com/larksuite/cli/commit/76f7e7eb)
(`shortcuts/apps/apps_delete.go` and its reference documentation) and
[`46c434f5`](https://github.com/larksuite/cli/commit/46c434f5)
(`shortcuts/apps/apps_list.go`).
