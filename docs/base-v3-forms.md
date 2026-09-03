# Base v3 forms

`client.base_v3()` provides form lifecycle operations through `form`, and
form-question operations through `form_question`. Both resources accept user
or tenant access tokens in `RequestOption`.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
use larksuite_oapi_sdk_rs::service::base::v3::{
    DeleteFormQuestionsReqBody, ListFormsQuery,
};

# async fn example(client: LarkClient) -> Result<(), Box<dyn std::error::Error>> {
let option = RequestOption::default();
let base_token = "bascn_example";
let table_id = "tbl_example";

let forms = client.base_v3().form.list(
    &ListFormsQuery::new(base_token, table_id),
    &option,
).await?;

let created = client.base_v3().form.create(
    base_token,
    table_id,
    serde_json::json!({"name": "Customer survey"}),
    &option,
).await?;

client.base_v3().form_question.create(
    base_token,
    table_id,
    "form_id_from_created",
    serde_json::json!({"questions": [{"title": "Name", "type": "text"}]}),
    &option,
).await?;
# let _ = (forms, created);
# Ok(())
# }
```

Create and update bodies are deliberately `Serialize`-generic. This keeps the
SDK compatible with the platform's evolving question options while preserving
the endpoint and credential contract.

## Removing questions

`form_question.delete` requires `DeleteFormQuestionsReqBody`. The platform
deletes the backing fields and their record data when `keep_field` is omitted
or false. Set `keep_field(true)` only when the intent is to remove the question
from the form while retaining its field and data.

```rust,no_run
# use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
# use larksuite_oapi_sdk_rs::service::base::v3::DeleteFormQuestionsReqBody;
# async fn example(client: LarkClient) -> Result<(), Box<dyn std::error::Error>> {
let option = RequestOption::default();
let remove_only_from_form = DeleteFormQuestionsReqBody::new(["fld_name"])
    .keep_field(true);
client.base_v3().form_question.delete(
    "bascn_example", "tbl_example", "form_id", &remove_only_from_form, &option,
).await?;
# Ok(())
# }
```
