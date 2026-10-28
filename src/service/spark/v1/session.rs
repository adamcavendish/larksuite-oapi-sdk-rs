//! Spark cloud-development sessions (official CLI main contract).
use crate::service::common::{JsonResp, PageQuery, RestRequest};
use crate::{config::Config, constants::AccessTokenType, error::LarkError, req::RequestOption};

/// User-token-only Spark sessions. Read operations require `spark:app:read`;
/// create, chat and stop require `spark:app:write`.
/// Responses retain open-ended JSON, including optional polling hints.
/// Create, chat and stop make a single attempt, even when retries are configured.
/// After an ambiguous failure, inspect session state before deciding to resubmit.
pub struct SessionResource<'a> {
    pub(super) config: &'a Config,
}

fn required<'a>(name: &str, value: &'a str) -> Result<&'a str, LarkError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(LarkError::IllegalParam(format!("{name} is required")));
    }
    Ok(value)
}

impl SessionResource<'_> {
    /// Create an empty session. No request body is sent.
    pub async fn create(
        &self,
        app_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/sessions",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", required("app_id", app_id)?)
        .send_json_once()
        .await
    }

    /// List one page of sessions (page size 1–50; omitted uses the server default).
    pub async fn list(
        &self,
        app_id: &str,
        page: &PageQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        if page.page_size.is_some_and(|size| !(1..=50).contains(&size)) {
            return Err(LarkError::IllegalParam(
                "page_size must be between 1 and 50".into(),
            ));
        }
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/sessions",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", required("app_id", app_id)?)
        .query("page_size", page.page_size)
        .query("page_token", page.page_token)
        .send_json()
        .await
    }

    /// Read status once. A queued message can coexist with the previous completed
    /// turn. Follow `next_poll_after_ms` when present; see the session example.
    pub async fn get(
        &self,
        app_id: &str,
        session_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/sessions/:session_id",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", required("app_id", app_id)?)
        .path_param("session_id", required("session_id", session_id)?)
        .send_json()
        .await
    }

    /// Enqueue a message; acceptance does not imply a turn has started or finished.
    pub async fn chat(
        &self,
        app_id: &str,
        session_id: &str,
        message: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let body = crate::JsonValue::from_serializable(
            serde_json::json!({"message": required("message", message)?}),
        )?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/sessions/:session_id/chat",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", required("app_id", app_id)?)
        .path_param("session_id", required("session_id", session_id)?)
        .json_body(&body)?
        .send_json_once()
        .await
    }

    /// Stop the specified latest turn, obtained from `get`.
    pub async fn stop(
        &self,
        app_id: &str,
        session_id: &str,
        turn_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let body = crate::JsonValue::from_serializable(
            serde_json::json!({"turn_id": required("turn_id", turn_id)?}),
        )?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/sessions/:session_id/stop",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", required("app_id", app_id)?)
        .path_param("session_id", required("session_id", session_id)?)
        .json_body(&body)?
        .send_json_once()
        .await
    }

    /// Read one page of replies. Pass the returned `next_page_token` to continue.
    pub async fn messages(
        &self,
        app_id: &str,
        session_id: &str,
        turn_id: &str,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/sessions/:session_id/turns/:turn_id/reply_message",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", required("app_id", app_id)?)
        .path_param("session_id", required("session_id", session_id)?)
        .path_param("turn_id", required("turn_id", turn_id)?)
        .query("page_token", page_token)
        .send_json()
        .await
    }
}
