# Spark application business features

`client.spark().app.features(app_id, operation_ref, option)` queries business
switches and an optional earlier operation. Requires a user token with
`spark:app:read`. `enable_feature(app_id, feature_name, idempotent_key, option)`
requires `spark:app:write`; the current upstream feature is `database`.

The response retains open JSON, including unknown states and absent `retryable`.
The caller supplies a hyphenated UUID and persists it before enabling. POST sends
once. On a timeout or network failure, query with the same key; do not generate a
replacement key while the result is unknown. A failed or conflicted operation
requires caller review; an absent retryable field does not authorize retries.
Only running operations should be polled automatically. The example verifies app
and database feature identity and the actual enabled switch; synchronous success
may omit operation, while succeeded with enabled=false is rejected.

See `examples/spark_features.rs` for a 60-second bounded workflow with a two-second
poll interval. Its deadline includes requests. Errors and timeouts retain the key
in the caller's environment for subsequent inspection. Operation lookup errors
are propagated rather than interpreted as permission to issue another POST.

`enabled=true` is a business switch, not proof that a local database is ready.
Older server deployments may not offer these endpoints. No local environment
files are modified. Contract source: official larksuite/cli main c910e89e,
shortcuts/apps/{apps_features,apps_feature_enable,feature_api}.go.
