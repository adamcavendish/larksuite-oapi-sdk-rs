# Base v3 dashboards

`client.base_v3()` exposes the reusable Base dashboard management endpoints
proven by the official CLI. The client adds `X-App-Id`; provide a user or tenant
access token in `RequestOption` for every call.

| Resource | Operations |
| --- | --- |
| `dashboard` | `list`, `get`, `create`, `update`, `delete`, `arrange` |
| `dashboard_block` | `list`, `get`, `create`, `update`, `delete`, `get_data` |

```rust,no_run
use larksuite_oapi_sdk_rs::service::base::v3::{
    DashboardUserIdTypeQuery, ListDashboardBlocksQuery, ListDashboardsQuery,
};
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::{JsonValue, LarkClient};
use larksuite_oapi_sdk_rs::req::RequestOption;

# async fn example(client: LarkClient) -> Result<(), Box<dyn std::error::Error>> {
let option = RequestOption {
    user_access_token: Some("user_access_token".into()),
    ..RequestOption::default()
};
let base_token = "base_token";

let dashboards = client.base_v3().dashboard.list(
    &ListDashboardsQuery::new(base_token).page(PageQuery::new().page_size(20)),
    &option,
).await?;

let dashboard = client.base_v3().dashboard.create(
    base_token,
    JsonValue::from(serde_json::json!({"name": "Sales"})),
    &option,
).await?;

let dashboard_id = "dashboard_id"; // Read this from the create response in application code.
let user_ids = DashboardUserIdTypeQuery::new().user_id_type("open_id");
let blocks = client.base_v3().dashboard_block.list(
    &ListDashboardBlocksQuery::new(base_token, dashboard_id)
        .page(PageQuery::new().page_size(20)),
    &option,
).await?;
client.base_v3().dashboard.arrange(base_token, dashboard_id, &user_ids, &option).await?;
println!("{dashboards:?} {dashboard:?} {blocks:?}");
# Ok(())
# }
```

Dashboard and dashboard-block creation/update bodies deliberately accept any
`Serialize` value. This preserves the documented protocol without embedding the
CLI's terminal-only semantic validation or JSON normalization in the SDK.
Confirm identifiers and the exact body before write or arrange calls; those
operations have immediate server-side effects. `get_data` reads chart data and
requires only `base_token` and `block_id`; it does not take a dashboard ID.
