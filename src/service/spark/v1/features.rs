//! Application business features. Contract: official CLI c910e89e.
use super::AppResource;
use crate::service::common::{JsonResp, RestRequest};
use crate::{constants::AccessTokenType, error::LarkError, req::RequestOption};

fn required<'a>(name: &str, value: &'a str) -> Result<&'a str, LarkError> {
    if value.trim().is_empty() {
        return Err(LarkError::IllegalParam(format!("{name} is required")));
    }
    Ok(value)
}

fn key(value: &str) -> Result<&str, LarkError> {
    let valid = value.len() == 36
        && value.bytes().enumerate().all(|(i, c)| {
            if matches!(i, 8 | 13 | 18 | 23) {
                c == b'-'
            } else {
                c.is_ascii_hexdigit()
            }
        });
    if !valid {
        return Err(LarkError::IllegalParam(
            "operation key must be a hyphenated UUID".into(),
        ));
    }
    Ok(value)
}

fn user(option: &RequestOption) -> Result<(), LarkError> {
    if option
        .user_access_token
        .as_deref()
        .is_none_or(|v| v.trim().is_empty())
    {
        return Err(LarkError::IllegalParam(
            "Spark features require a user access token".into(),
        ));
    }
    Ok(())
}

impl AppResource<'_> {
    /// Query business switches and optionally an earlier operation (user token,
    /// `spark:app:read`). Response JSON preserves unknown states and absent
    /// `retryable` fields. Enabled does not imply local database readiness.
    pub async fn features(
        &self,
        app_id: &str,
        operation_ref: Option<&str>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        user(option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/features",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", required("app_id", app_id)?)
        .query("operation_ref", operation_ref.map(key).transpose()?)
        .send_json()
        .await
    }

    /// Enable a business feature (currently `database`) using a user token and
    /// `spark:app:write`. Sends once even when retries are configured.
    /// Keep the caller's UUID key: after an ambiguous failure query that operation
    /// before resuming with the same key. Never replace it automatically.
    pub async fn enable_feature(
        &self,
        app_id: &str,
        feature_name: &str,
        idempotent_key: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        user(option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/features/:feature_name/enable",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", required("app_id", app_id)?)
        .path_param("feature_name", required("feature_name", feature_name)?)
        .json_body(&serde_json::json!({"idempotent_key": key(idempotent_key)?}))?
        .send_json_once()
        .await
    }
}
