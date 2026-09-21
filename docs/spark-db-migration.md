# Spark database schema migration

`client.spark().app` exposes the user-token-only dev-to-online schema migration
contract:

- `preview_db_migration` sends `dry_run: true` and returns pending changes
  without applying them.
- `apply_db_migration` sends `dry_run: false`. Applying pending changes is
  irreversible.
- `get_db_migration_status` reads one asynchronous migration task.

All three methods require an explicit, nonempty user access token with
`spark:app:write`. The platform requires write scope for preview as well as
apply; tenant credentials are not a substitute.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, LarkError, RequestOption};

# async fn example(client: &LarkClient, user_token: String) -> Result<(), LarkError> {
let option = RequestOption {
    user_access_token: Some(user_token),
    ..Default::default()
};

let preview = client.spark().app
    .preview_db_migration("app_id", &option)
    .await?;
// Review preview.data before authorizing the irreversible apply operation.
let _ = preview;

let submission = client.spark().app
    .apply_db_migration("app_id", &option)
    .await?;
// A submission may complete synchronously or contain a task_id. If it contains
// a task_id, choose application-specific polling and timeout behavior.
let _ = submission;

let status = client.spark().app
    .get_db_migration_status("app_id", "task_id", &option)
    .await?;
let _ = status;
# Ok(())
# }
```

Because apply is irreversible, `apply_db_migration` makes exactly one network
attempt even when the client is configured for retries. An ambiguous transport
failure is returned to the caller rather than automatically submitting the
migration again. Preview and status reads continue to use the client's normal
retry policy.

The SDK does not automatically preview before apply, ask for confirmation,
poll, choose a timeout, rename statuses, synthesize change counts, or project
response fields. Synchronous and asynchronous submission shapes, platform
errors, and future fields remain available through the generic JSON response.
