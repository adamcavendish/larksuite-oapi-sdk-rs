# Base v3 workflows

`client.base_v3().workflow` manages Base automation workflows through the
official Base v3 Open Platform routes. These operations accept either a user
access token or a tenant access token through `RequestOption`.

```rust,no_run
use larksuite_oapi_sdk_rs::{
    req::RequestOption,
    service::{base::v3::ListWorkflowQuery, common::PageQuery},
};

# async fn example(client: larksuite_oapi_sdk_rs::Client) -> Result<(), Box<dyn std::error::Error>> {
let option = RequestOption::default();
let workflows = client
    .base_v3()
    .workflow
    .list(
        &ListWorkflowQuery::new("base_token")
            .status("disabled")
            .page(PageQuery::new().page_size(100)),
        &option,
    )
    .await?;
# let _ = workflows;
# Ok(())
# }
```

## Lifecycle

- `create(base_token, body, option)` creates a workflow. Its body must include
  the service-required, unique `client_token`; newly created workflows are
  disabled.
- `get(query, option)` fetches a workflow definition. Set `user_id_type` on
  `GetWorkflowQuery` only when creator/updater identifiers need a specific
  representation.
- `list(query, option)` uses the API's `POST .../workflows/list` body-paginated
  operation. Its optional `status`, `page_size`, and `page_token` are JSON body
  fields, not URL query parameters.
- `update(base_token, workflow_id, body, option)` uses full replacement
  semantics. Fetch first and retain fields you do not intend to clear.
- `enable` and `disable` switch an existing workflow's activation state.

The relevant Open Platform scopes are `base:workflow:read`,
`base:workflow:create`, and `base:workflow:update`.

## Workflow-definition boundary

Workflow `steps` are an evolving nested protocol, including newer
`AIClassificationBranch` and `AIAnalysisAction` nodes. The SDK deliberately
accepts `impl Serialize` for create and update and returns `JsonResp`, so callers
can use current documented JSON without waiting for a release that models every
step variant. The resource owns transport, authentication, route escaping, and
stable list/get fields; it does not reproduce CLI-only flag grammar, local
schema validation, prompting, or output rendering.
