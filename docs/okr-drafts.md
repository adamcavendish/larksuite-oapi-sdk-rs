# OKR draft reads

OKR v2 supports explicit draft selection on individual alignment, key-result and
objective reads, and on these five list resources:

- `okr_cycle_objective`: objectives in a cycle (paginated).
- `okr_objective_key_result`: key results in an objective (paginated).
- `okr_objective_alignment`: alignments in an objective (paginated).
- `okr_key_result_indicator`: a key result's indicator (single indicator response).
- `okr_objective_indicator`: an objective's indicator (single indicator response).

Use `list_with_draft` with `None` to omit `prefer_draft`, `Some(false)` to request
the published view, or `Some(true)` to prefer the draft view. Existing `list`
methods omit the parameter. A read does not create a draft. Draft visibility
depends on ownership and the user or application's data permissions; a missing
draft in the response does not prove that no draft exists.

The three paginated resources expose `list_by_iterator_with_draft`. The iterator
keeps the selection on every page along with the original user and page-size
parameters. Existing iterators omit the draft parameter on every page.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, error::LarkError, req::RequestOption};
use larksuite_oapi_sdk_rs::service::okr::v2::OkrPageQuery;

async fn read_drafts(client: &LarkClient, option: &RequestOption) -> Result<(), LarkError> {
    let okr = client.okr_v2();
    let query = OkrPageQuery::new().page_size(Some(20));
    let mut objectives = okr.okr_cycle_objective
        .list_by_iterator_with_draft("cycle-id", &query, Some(true));
    while let Some(objective) = objectives.next(option).await? {
        println!("{:?}: {:?}", objective.objective_id, objective.is_draft);
    }
    let indicator = okr.okr_objective_indicator
        .list_with_draft("objective-id", Some(true), option).await?;
    println!("{:?}", indicator.data);
    Ok(())
}
```

A runnable [draft-read example](../examples/okr_draft_reads.rs) uses environment
variables for credentials and the cycle ID. Compile it with
`cargo check --example okr_draft_reads`.

For a raw request, the existing Go compatibility endpoint also accepts
`prefer_draft` in its query parameters; preserve the same optional boolean
semantics when constructing that request.

Draft writes use `apply_to_draft` on supported operations. Publishing or dropping
a draft affects the entire objective aggregate; a later publication can overwrite
published-state edits made while that draft existed. The read example above does
not publish or drop a draft. These contracts come from official CLI commit
`3d82c854` (OKR catalog and cycle-detail calls); live service behavior has not been
independently verified here.
