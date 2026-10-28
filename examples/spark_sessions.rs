//! Read an existing Spark session until terminal or the caller's deadline.
//! Requires spark:app:read. Use session.chat/create/stop with spark:app:write.
//! CLI contract: larksuite/cli a92db877, shortcuts/apps/apps_session_*.go.
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
use std::time::Duration;

fn terminal(data: &serde_json::Value) -> Option<&str> {
    // Missing fields are unknown, not evidence that work has completed.
    if data["is_streaming"].as_bool() != Some(false) || data["queued_count"].as_u64() != Some(0) {
        return None;
    }
    match data["latest_turn"]["status"].as_str()? {
        status @ ("completed" | "failed" | "cancelled") => Some(status),
        _ => None,
    }
}

fn interval(data: &serde_json::Value, fallback: Duration) -> Duration {
    data["next_poll_after_ms"]
        .as_u64()
        .map(Duration::from_millis)
        .unwrap_or(fallback)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client =
        LarkClient::builder(std::env::var("APP_ID")?, std::env::var("APP_SECRET")?).build()?;
    let option = RequestOption {
        user_access_token: Some(std::env::var("USER_ACCESS_TOKEN")?),
        ..Default::default()
    };
    let app_id = std::env::var("SPARK_APP_ID")?;
    let session_id = std::env::var("SPARK_SESSION_ID")?;
    // Caller must select the fallback using actual app initialization state.
    // CLI guidance: incremental 5–10s; initialization 60–120s when hint absent.
    let fallback = Duration::from_millis(std::env::var("SPARK_POLL_FALLBACK_MS")?.parse()?);
    let deadline = Duration::from_secs(std::env::var("SPARK_POLL_TIMEOUT_SECS")?.parse()?);
    tokio::time::timeout(deadline, async {
        loop {
            let response = client
                .spark()
                .session
                .get(&app_id, &session_id, &option)
                .await?;
            let data = response.data.ok_or("session response has no data")?;
            let data = data.as_value();
            if let Some(status) = terminal(data) {
                if status != "completed" {
                    return Err(format!("session ended: {status}").into());
                }
                // Completion of a cloud turn does not mean the app is published.
                // Read replies using session.messages and its next_page_token.
                return Ok::<_, Box<dyn std::error::Error>>(());
            }
            tokio::time::sleep(interval(data, fallback)).await;
        }
    })
    .await??;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn queued_and_streaming_work_cannot_terminate() {
        for status in ["completed", "failed", "cancelled"] {
            let mut data =
                json!({"is_streaming":false,"queued_count":1,"latest_turn":{"status":status}});
            assert_eq!(terminal(&data), None);
            data["queued_count"] = json!(0);
            assert_eq!(terminal(&data), Some(status));
            data["is_streaming"] = json!(true);
            assert_eq!(terminal(&data), None);
        }
        assert_eq!(
            terminal(&json!({"latest_turn":{"status":"completed"}})),
            None
        );
    }

    #[test]
    fn optional_interval_uses_caller_fallback() {
        let fallback = Duration::from_secs(60);
        assert_eq!(interval(&json!({}), fallback), fallback);
        assert_eq!(
            interval(&json!({"next_poll_after_ms":1234}), fallback),
            Duration::from_millis(1234)
        );
    }
}
