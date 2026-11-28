# Spark database audit

Use `client.spark().db_audit` with `RequestOption::user_access_token`.
`get_status` and `list_events` require `spark:app:read`. `set_status`
requires `spark:app:write`.

`DbAuditEnv::Dev` and `DbAuditEnv::Online` select a branch explicitly.
When `env` is omitted, Spark selects dev for a multi-environment app
and online for a single-environment app. The selection applies to all
three methods.

## Read audit status and row changes

`DbAuditStatusQuery::new(app_id)` returns the table configurations
reported by Spark. Use `.table(table_name)` to query one table. An
empty `items` list means Spark returned no matching configuration.

`DbAuditListQuery::new(app_id, &["orders"])` selects one or more tables.
Use RFC 3339 timestamps for `since` and `until`. The page size defaults
to 20. Read `data.page_token` when `data.has_more` is true and pass it to
`.page_token(...)` on the next query, with the same filters.

`operator`, `before`, and `after` can contain JSON encoded as a string.
The SDK decodes those strings and preserves any string it cannot decode.
For example, an insert may omit `before` and a delete may omit `after`.

## Set audit status

Build a `DbAuditSetRequest` with the app ID, table, and desired setting.
`DbAuditSetting::Enable` takes an explicit `DbAuditRetention` value.
`DbAuditSetting::Disable` sends `enabled: false` and omits the retention.
The write is sent once on a fresh connection. A redirect is returned as
an HTTP status error. After an ambiguous failure, read the table status
on the same branch before deciding whether to repeat the write.

Use `DbAuditPrecondition::from_error(&error)` on a failed audit request
to distinguish `AuditDisabled` from `BranchNotInitialized`. The original
error retains the gateway code, message, and diagnostic fields.
