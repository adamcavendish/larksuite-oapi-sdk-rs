# Base v3 application mode

`client.base_v3()` exposes the official CLI-proven Open Platform operations for
Base workspaces and BaseApps. These calls accept an explicit user access token and
automatically add `X-App-Id` from the client configuration.

The API is organized by resource:

| Resource | Operations |
| --- | --- |
| `workspace` | `create`, `list_entities`, `move_in` |
| `app` | `create`, `get` |
| `page` | `list`, `get`, `create`, `rename`, `delete` |
| `block` | `list`, `get`, `create`, `update`, `get_data` |

Workspace, app, page, and block mutation bodies accept any `Serialize` value.
The public protocol currently documents the route and selected outer fields but
not stable Rust schemas for every nested component configuration, so responses
and block payloads intentionally use `JsonResp` / `JsonValue`. This preserves
server compatibility without copying CLI-only validation into the SDK.

```rust,no_run
use larksuite_oapi_sdk_rs::service::base::v3::{
    ListBaseAppPagesQuery, ListWorkspaceEntitiesQuery,
};
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let option = RequestOption {
    user_access_token: Some("USER_ACCESS_TOKEN".into()),
    ..RequestOption::default()
};

let apps = client.base_v3().workspace.list_entities(
    &ListWorkspaceEntitiesQuery::new("workspace_token")
        .entity_type("baseapp")
        .page(PageQuery::new().page_size(20)),
    &option,
).await?;
let pages = client.base_v3().page.list(
    &ListBaseAppPagesQuery::new("app_token").page(PageQuery::new().page_size(20)),
    &option,
).await?;
println!("{:#?} {:#?}", apps.data, pages.data);
# Ok(())
# }
```

## Important boundaries

- `block.get_data` is for chart data. Pass its chart token and the relevant
  `base_token`; text blocks have no `/data` endpoint.
- The current public contract has no BaseApp copy, PageGroup, page layout,
  block deletion, or block type-mutation operation. Do not substitute a
  dashboard operation for these absent BaseApp operations.
- Creation, moves, updates, and deletion have immediate server-side effects.
  Confirm the target tokens and request body before issuing them; the SDK does
  not add the CLI's local uniqueness checks or multi-step orchestration.

See [`examples/base_v3_app_read.rs`](../examples/base_v3_app_read.rs) for a
runnable, read-only workspace/App/Page/Block discovery flow.
