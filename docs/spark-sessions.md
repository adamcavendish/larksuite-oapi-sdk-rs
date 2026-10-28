# Spark cloud sessions

Use `client.spark().session` with `RequestOption.user_access_token`.
Reads require `spark:app:read`; create, chat and stop require `spark:app:write`.
These Open Platform routes are sourced from official CLI main at `a92db877`
(`shortcuts/apps/apps_session_*.go` and `apps_chat.go`), rather than the Go catalog.
Responses use the SDK's `JsonResp`, preserving optional and future data fields.

```rust,no_run
# use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
# use larksuite_oapi_sdk_rs::service::common::PageQuery;
# async fn demo(client: &LarkClient, option: &RequestOption) -> Result<(), Box<dyn std::error::Error>> {
let spark = client.spark();
let created = spark.session.create("app_id", option).await?;
let data = created.data.ok_or("missing data")?;
let session_id = data.as_value()["session_id"].as_str().ok_or("missing session_id")?;
let page = spark.session.list("app_id", &PageQuery::new().page_size(20), option).await?;
let accepted = spark.session.chat("app_id", session_id, "Describe this app", option).await?;
let status = spark.session.get("app_id", session_id, option).await?;
// Once latest_turn.turn_id is available, read replies one page at a time.
let mut cursor: Option<String> = None;
loop {
    let response = spark.session.messages("app_id", session_id, "turn_id", cursor.as_deref(), option).await?;
    let data = response.data.ok_or("missing reply data")?;
    let data = data.as_value();
    // Consume data["messages"] here.
    if data["has_more"].as_bool() != Some(true) { break; }
    cursor = Some(data["next_page_token"].as_str().filter(|s| !s.is_empty())
        .ok_or("missing next_page_token")?.to_owned());
}
// To cancel, use the actual latest_turn.turn_id returned by get:
// spark.session.stop("app_id", session_id, turn_id, option).await?;
# Ok(())
# }
```

Message submission only enqueues work. A prior turn may remain `completed` while
`queued_count > 0`. Only treat `latest_turn.status` as terminal when
`is_streaming == false` and `queued_count == 0`; terminal statuses are `completed`,
`failed` and `cancelled`. Missing fields mean unknown, not completed. Surface
failures rather than silently resending messages.

Follow `next_poll_after_ms` when returned. When absent, the caller chooses a
fallback based on actual app initialization state: official CLI guidance uses
5–10 seconds for incremental work and 60–120 seconds for initialization.
A new session alone does not establish the app's initialization state.
The SDK does not start background polling. Reads follow the configured retry
policy. Create, chat and stop make one attempt without automatic replay, even
when retries are enabled. A gateway timeout can leave acceptance unknown; inspect
session state before deciding to resubmit a message or create another session.

`examples/spark_sessions.rs` demonstrates status polling with a caller deadline.
It only reads an existing session, and requires `APP_ID`, `APP_SECRET`,
`USER_ACCESS_TOKEN`, `SPARK_APP_ID`, `SPARK_SESSION_ID`,
`SPARK_POLL_FALLBACK_MS` and `SPARK_POLL_TIMEOUT_SECS`.
A completed cloud turn is not evidence that the app has been published.
