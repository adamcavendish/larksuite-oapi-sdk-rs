//! Enable the database feature and follow the same operation key.
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
use std::time::Duration;

// POST uses `feature`; GET uses `features`. Validate identity before accepting
// a switch or operation result. An absent operation is valid synchronous success.
fn finished(data: &serde_json::Value, app: &str, key: &str) -> Result<bool, &'static str> {
    if data["app_id"].as_str() != Some(app) {
        return Err("app identity missing or mismatched");
    }
    let feature = if let Some(features) = data.get("features") {
        features
            .as_array()
            .and_then(|items| items.iter().find(|f| f["name"] == "database"))
            .ok_or("database feature missing")?
    } else {
        data.get("feature").ok_or("feature missing")?
    };
    if feature["name"].as_str() != Some("database") {
        return Err("feature identity mismatched");
    }
    let enabled = feature["enabled"]
        .as_bool()
        .ok_or("enabled missing or invalid")?;
    if let Some(op) = data.get("operation").filter(|v| !v.is_null()) {
        if op["ref"].as_str() != Some(key) {
            return Err("operation reference missing or mismatched");
        }
        match op["state"].as_str() {
            Some("running") => return Ok(false),
            Some("succeeded") => {}
            Some("failed" | "conflicted") => {
                return Err("operation failed; inspect state before retrying");
            }
            _ => return Err("operation outcome unknown; retain the same key for recovery"),
        }
    }
    if enabled {
        Ok(true)
    } else {
        Err("database feature is not enabled")
    }
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
    // Persist a UUID before the first attempt. Reuse it after any ambiguous failure.
    let key = std::env::var("SPARK_FEATURE_KEY")?;
    let spark = client.spark();
    tokio::time::timeout(Duration::from_secs(60), async {
        // Query first. A supplied key may already identify an earlier operation.
        let response = spark.app.features(&app_id, Some(&key), &option).await?;
        let data = response.data.ok_or("missing feature data")?;
        let mut response = if data
            .as_value()
            .get("operation")
            .is_some_and(|v| !v.is_null())
        {
            if finished(data.as_value(), &app_id, &key)? {
                return Ok::<_, Box<dyn std::error::Error>>(());
            }
            spark.app.features(&app_id, Some(&key), &option).await?
        } else {
            spark
                .app
                .enable_feature(&app_id, "database", &key, &option)
                .await?
        };
        loop {
            let data = response.data.ok_or("missing operation data")?;
            if finished(data.as_value(), &app_id, &key)? {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
            response = spark.app.features(&app_id, Some(&key), &option).await?;
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
    fn synchronous_and_polled_success_require_enabled_and_identity() {
        for data in [
            json!({"app_id":"a","feature":{"name":"database","enabled":true}}),
            json!({"app_id":"a","features":[{"name":"database","enabled":true}],"operation":{"ref":"k","state":"succeeded"}}),
        ] {
            assert_eq!(finished(&data, "a", "k"), Ok(true));
            assert!(finished(&data, "other", "k").is_err());
        }
        for enabled in [json!(false), json!(null)] {
            let data = json!({"app_id":"a","feature":{"name":"database","enabled":enabled},"operation":{"ref":"k","state":"succeeded"}});
            assert!(finished(&data, "a", "k").is_err());
        }
    }
    #[test]
    fn unknown_failed_and_mismatched_operations_cannot_succeed() {
        for state in ["unknown", "future", "failed", "conflicted"] {
            let data = json!({"app_id":"a","feature":{"name":"database","enabled":true},"operation":{"ref":"k","state":state}});
            assert!(finished(&data, "a", "k").is_err());
        }
        let data = json!({"app_id":"a","features":[{"name":"database","enabled":false}],"operation":{"ref":"k","state":"running"}});
        assert_eq!(finished(&data, "a", "k"), Ok(false));
        assert!(finished(&data, "a", "other").is_err());
        assert!(finished(&json!({}), "a", "k").is_err());
        assert!(
            finished(
                &json!({"app_id":"a","feature":{"name":"other","enabled":true}}),
                "a",
                "k"
            )
            .is_err()
        );
    }
}
