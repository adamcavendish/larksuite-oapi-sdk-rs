use super::require_app_user;
use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::req::RequestOption;
use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::service::common::{FromRawResponse, PageQuery, RestRequest};
use crate::{JsonValue, LarkError};
use serde::{Deserialize, Deserializer, Serialize};

/// Database branch for an audit request. Omit it to let Spark select the branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbAuditEnv {
    Dev,
    Online,
}

impl DbAuditEnv {
    fn as_str(self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Online => "online",
        }
    }
}

/// Select an app and optionally a table for an audit status query.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DbAuditStatusQuery<'a> {
    pub app_id: &'a str,
    pub table: Option<&'a str>,
    pub env: Option<DbAuditEnv>,
}

impl<'a> DbAuditStatusQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            table: None,
            env: None,
        }
    }

    pub fn table(mut self, table: impl Into<Option<&'a str>>) -> Self {
        self.table = table.into();
        self
    }

    pub fn env(mut self, env: impl Into<Option<DbAuditEnv>>) -> Self {
        self.env = env.into();
        self
    }
}

/// Select one or more tables and a page of row-change events.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DbAuditListQuery<'a> {
    pub app_id: &'a str,
    pub tables: &'a [&'a str],
    pub env: Option<DbAuditEnv>,
    /// Inclusive lower time bound in RFC 3339 format.
    pub since: Option<&'a str>,
    pub until: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> DbAuditListQuery<'a> {
    pub fn new(app_id: &'a str, tables: &'a [&'a str]) -> Self {
        Self {
            app_id,
            tables,
            env: None,
            since: None,
            until: None,
            page: PageQuery::new(),
        }
    }

    pub fn env(mut self, env: impl Into<Option<DbAuditEnv>>) -> Self {
        self.env = env.into();
        self
    }

    pub fn since(mut self, since: impl Into<Option<&'a str>>) -> Self {
        self.since = since.into();
        self
    }

    /// Set the upper time bound in RFC 3339 format.
    pub fn until(mut self, until: impl Into<Option<&'a str>>) -> Self {
        self.until = until.into();
        self
    }

    pub fn page_size(mut self, page_size: impl Into<Option<i32>>) -> Self {
        self.page.page_size = page_size.into();
        self
    }

    pub fn page_token(mut self, page_token: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = page_token.into();
        self
    }
}

/// Retention period to send when enabling audit logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DbAuditRetention {
    #[serde(rename = "7d")]
    Days7,
    #[serde(rename = "30d")]
    Days30,
    #[serde(rename = "180d")]
    Days180,
    #[serde(rename = "360d")]
    Days360,
    #[serde(rename = "forever")]
    Forever,
}

/// Change to make to one table's audit configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbAuditSetting {
    Enable { retention: DbAuditRetention },
    Disable,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DbAuditSetRequest<'a> {
    pub app_id: &'a str,
    pub table: &'a str,
    pub env: Option<DbAuditEnv>,
    pub setting: DbAuditSetting,
}

impl<'a> DbAuditSetRequest<'a> {
    pub fn new(app_id: &'a str, table: &'a str, setting: DbAuditSetting) -> Self {
        Self {
            app_id,
            table,
            env: None,
            setting,
        }
    }

    pub fn env(mut self, env: impl Into<Option<DbAuditEnv>>) -> Self {
        self.env = env.into();
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct DbAuditStatus {
    pub table: String,
    pub enabled: bool,
    pub enabled_at: Option<String>,
    pub retention: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct DbAuditStatusData {
    /// The table configurations returned by Spark. An empty list means there was no matching record.
    #[serde(default)]
    pub items: Vec<DbAuditStatus>,
}

/// Row change recorded by Spark. JSON fields tolerate both native and string-encoded JSON.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct DbAuditEvent {
    pub event_id: String,
    pub event_time: String,
    pub target_table: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub summary: Option<String>,
    #[serde(default, deserialize_with = "audit_json")]
    pub operator: Option<JsonValue>,
    #[serde(default, deserialize_with = "audit_json")]
    pub before: Option<JsonValue>,
    #[serde(default, deserialize_with = "audit_json")]
    pub after: Option<JsonValue>,
}

fn audit_json<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<JsonValue>, D::Error> {
    let value = Option::<JsonValue>::deserialize(deserializer)?;
    Ok(value.map(|value| {
        if let Some(encoded) = value.as_str()
            && let Ok(decoded) = serde_json::from_str(encoded)
        {
            return decoded;
        }
        value
    }))
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct DbAuditListData {
    #[serde(default)]
    pub items: Vec<DbAuditEvent>,
    #[serde(default)]
    pub has_more: bool,
    /// Pass the cursor to the next query when `has_more` is true.
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct DbAuditSetData {
    pub status: Option<DbAuditSetStatus>,
}

/// Audit settings reported by the write response. Spark can return a partial status.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct DbAuditSetStatus {
    pub table: Option<String>,
    pub enabled: Option<bool>,
    pub enabled_at: Option<String>,
    pub retention: Option<String>,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DbAuditResp<T> {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
    pub data: Option<T>,
}

impl<T> DbAuditResp<T> {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

impl<T> FromRawResponse<T> for DbAuditResp<T> {
    fn from_raw_response(api_resp: ApiResp, raw: RawResponse<T>) -> Self {
        Self {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        }
    }
}

/// Recognized business preconditions returned by Spark database audit endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DbAuditPrecondition {
    AuditDisabled,
    BranchNotInitialized,
}

impl DbAuditPrecondition {
    /// Classify an audit request failure while leaving its original diagnostics available.
    pub fn from_error(error: &LarkError) -> Option<Self> {
        let LarkError::Api(error) = error else {
            return None;
        };
        if error.code != 400_002_476 {
            return None;
        }
        let (subcode, _) = error.effective_message().split_once([':', '：'])?;
        match subcode {
            "k_dl_4000028" => Some(Self::AuditDisabled),
            "k_dl_1600000" => Some(Self::BranchNotInitialized),
            _ => None,
        }
    }
}

fn required<'a>(field: &str, value: &'a str) -> Result<&'a str, LarkError> {
    if value.trim().is_empty() {
        return Err(LarkError::IllegalParam(format!("{field} is required")));
    }
    Ok(value)
}

/// Read and configure row-change audit logging for Spark databases.
pub struct DbAuditResource<'a> {
    pub(super) config: &'a Config,
}

impl DbAuditResource<'_> {
    /// Read table audit configuration with a user token and `spark:app:read`.
    pub async fn get_status(
        &self,
        query: &DbAuditStatusQuery<'_>,
        option: &RequestOption,
    ) -> Result<DbAuditResp<DbAuditStatusData>, LarkError> {
        require_app_user(option)?;
        let app_id = required("app_id", query.app_id)?;
        let table = query
            .table
            .map(|name| required("table", name))
            .transpose()?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/db/audit_status",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .query("table", table)
        .query("env", query.env.map(DbAuditEnv::as_str))
        .send_response()
        .await
    }

    /// List one page of row changes with a user token and `spark:app:read`.
    ///
    /// Each call asks Spark for the supplied tables directly. Page size defaults to 20.
    pub async fn list_events(
        &self,
        query: &DbAuditListQuery<'_>,
        option: &RequestOption,
    ) -> Result<DbAuditResp<DbAuditListData>, LarkError> {
        require_app_user(option)?;
        let app_id = required("app_id", query.app_id)?;
        if query.tables.is_empty()
            || query
                .tables
                .iter()
                .any(|table| table.trim().is_empty() || table.contains(','))
        {
            return Err(LarkError::IllegalParam(
                "tables must contain one or more nonempty names without commas".into(),
            ));
        }
        let page_size = query.page.page_size.unwrap_or(20);
        if page_size <= 0 {
            return Err(LarkError::IllegalParam("page_size must be positive".into()));
        }
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/spark/v1/apps/:app_id/db/audit_list",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .query("tables", query.tables.join(","))
        .query("env", query.env.map(DbAuditEnv::as_str))
        .query("since", query.since)
        .query("until", query.until)
        .query("page_size", page_size)
        .query("page_token", query.page.page_token)
        .send_response()
        .await
    }

    /// Set audit logging for a table with a user token and `spark:app:write`.
    ///
    /// The SDK sends the write once. After an ambiguous failure, check the
    /// table's status on the same branch before deciding whether to repeat it.
    pub async fn set_status(
        &self,
        request: &DbAuditSetRequest<'_>,
        option: &RequestOption,
    ) -> Result<DbAuditResp<DbAuditSetData>, LarkError> {
        require_app_user(option)?;
        let app_id = required("app_id", request.app_id)?;
        let table = required("table", request.table)?;
        let body = match request.setting {
            DbAuditSetting::Enable { retention } => {
                serde_json::json!({"table": table, "enabled": true, "retention": retention})
            }
            DbAuditSetting::Disable => serde_json::json!({"table": table, "enabled": false}),
        };
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/spark/v1/apps/:app_id/db/audit_set",
            vec![AccessTokenType::User],
            option,
        )
        .path_param("app_id", app_id)
        .query("env", request.env.map(DbAuditEnv::as_str))
        .json_body(&body)?
        .send_without_replay()
        .await?;
        Ok(DbAuditResp::from_raw_response(api_resp, raw))
    }
}
