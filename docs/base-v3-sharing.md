# Base v3 dashboard and form sharing

`client.base_v3()` exposes sharing status and partial-update operations for
Base dashboards and forms. The client adds `X-App-Id` from its configuration;
pass either an explicit user access token or tenant access token in
`RequestOption` for these calls.

| Resource | Operations |
| --- | --- |
| `dashboard_share` | `get`, `update` |
| `form_share` | `get`, `update` |

Sharing reads return `JsonResp` because the platform controls the response
schema. Updates use typed bodies, so omitting a field leaves it unchanged while
passing `false` explicitly disables that setting.

```rust,no_run
use larksuite_oapi_sdk_rs::service::base::v3::{
    UpdateDashboardShareReqBody, UpdateDashboardShareSettings,
    UpdateFormShareReqBody, UpdateFormShareSettings,
};
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn example() -> Result<(), larksuite_oapi_sdk_rs::LarkError> {
let client = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let option = RequestOption {
    tenant_access_token: Some("TENANT_ACCESS_TOKEN".into()),
    ..RequestOption::default()
};

let dashboard = client
    .base_v3()
    .dashboard_share
    .get("base_token", "dashboard_id", &option)
    .await?;
let dashboard_update = UpdateDashboardShareReqBody::new()
    .access_scope("tenant")
    .settings(UpdateDashboardShareSettings::new().show_source(false));
client
    .base_v3()
    .dashboard_share
    .update("base_token", "dashboard_id", &dashboard_update, &option)
    .await?;

let form_update = UpdateFormShareReqBody::new()
    .enabled(false)
    .settings(UpdateFormShareSettings::new().require_login(false));
client
    .base_v3()
    .form_share
    .update("base_token", "table_id", "form_id", &form_update, &option)
    .await?;
println!("{:#?}", dashboard.data);
# Ok(())
# }
```

`UpdateDashboardShareReqBody` supports `enabled`, `access_scope`, and the
dashboard-specific `show_source` and `enable_auto_analysis` settings.
`UpdateFormShareReqBody` supports `enabled`, `access_scope`, and the
form-specific `allow_anonymous` and `require_login` settings. Build only the
fields the request intends to change: these are PATCH operations with immediate
server-side effects.

Dashboard operations are identified by `base_token` and `dashboard_id`; form
operations also require `table_id` and `form_id`. Confirm those identifiers and
the desired sharing policy before sending an update.
