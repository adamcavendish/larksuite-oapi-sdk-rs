# Spark Base-to-database sync

`client.spark().db_sync` exposes the user-token-only Base-to-database sync
endpoints currently shipped by the official Lark CLI. The configuration and
response payloads are intentionally `JsonValue`: no authoritative SDK schema
has been published yet.

Use `create` with `"preview": true` before creating a task. The API has
non-obvious bindings that the SDK preserves:

- `sync_create` and `sync_update` receive `env` in the JSON body.
- `sync_update`, `sync_enable`, `sync_disable`, and `sync_del` receive
  `task_id` in the JSON body.
- `sync_list` and `sync_task` use query parameters.
- All seven operations require `RequestOption::user_access_token`.

See [`examples/spark_db_sync.rs`](../examples/spark_db_sync.rs) for a
preview-only workflow.

## Maintainer live verification

The ignored lifecycle test validates preview, create, get, list, enable,
disable, update, and delete against a real Feishu dev target. It is excluded
from CI because it creates and deletes a task.

Use only a dedicated disposable Spark app and target. Set:

- `FEISHU_SPARK_DB_SYNC_LIVE=1`
- `FEISHU_APP_ID`, `FEISHU_APP_SECRET`, and `FEISHU_SPARK_USER_ACCESS_TOKEN`
- `FEISHU_SPARK_DB_SYNC_APP_ID`
- `FEISHU_SPARK_DB_SYNC_CREATE_BODY`: a streaming-task JSON body without a
  `preview` value (the test sets it)
- `FEISHU_SPARK_DB_SYNC_UPDATE_BODY`: an update JSON body without `task_id`
  (the test inserts the created task ID)

The user token must have `spark:app:read` and `spark:app:write`. Run:

```bash
cargo test --test spark_db_sync_live feishu_spark_db_sync_live_lifecycle -- --ignored --exact
```

The test always attempts `sync_del` after a successful create. Treat any
cleanup failure as an environment incident and remove the task before rerunning.
