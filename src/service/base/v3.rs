use serde::Serialize;

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{JsonResp, RestRequest};

pub type ListRecordResp = JsonResp;
pub type SearchRecordResp = JsonResp;

/// Query parameters for reading Base v3 records.
///
/// `filter` and `sort` are JSON-encoded strings, as required by the Base v3
/// list endpoint. Search-specific filters belong in the JSON body passed to
/// [`RecordResource::search`].
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListRecordQuery<'a> {
    pub base_token: &'a str,
    pub table_id: &'a str,
    pub field_ids: Option<&'a [&'a str]>,
    pub view_id: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub sort: Option<&'a str>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

impl<'a> ListRecordQuery<'a> {
    pub fn new(base_token: &'a str, table_id: &'a str) -> Self {
        Self {
            base_token,
            table_id,
            ..Self::default()
        }
    }

    pub fn field_ids(mut self, value: &'a [&'a str]) -> Self {
        self.field_ids = Some(value);
        self
    }

    pub fn view_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.view_id = value.into();
        self
    }

    pub fn filter(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.filter = value.into();
        self
    }

    pub fn sort(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.sort = value.into();
        self
    }

    pub fn offset(mut self, value: impl Into<Option<i32>>) -> Self {
        self.offset = value.into();
        self
    }

    pub fn limit(mut self, value: impl Into<Option<i32>>) -> Self {
        self.limit = value.into();
        self
    }
}

pub struct V3<'a> {
    pub record: RecordResource<'a>,
}

impl<'a> V3<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            record: RecordResource { config },
        }
    }
}

pub struct RecordResource<'a> {
    config: &'a Config,
}

impl RecordResource<'_> {
    /// Lists records from a Base v3 table.
    pub async fn list(
        &self,
        query: &ListRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListRecordResp, LarkError> {
        let option = self.with_app_id(option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/records",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", query.base_token)
        .path_param("table_id", query.table_id)
        .query_values("field_id", query.field_ids)
        .query("view_id", query.view_id)
        .query("filter", query.filter)
        .query("sort", query.sort)
        .query("offset", query.offset)
        .query("limit", query.limit)
        .send_json()
        .await
    }

    /// Searches records from a Base v3 table with the upstream JSON request body.
    pub async fn search(
        &self,
        base_token: &str,
        table_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<SearchRecordResp, LarkError> {
        let option = self.with_app_id(option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/records/search",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    fn with_app_id(&self, option: &RequestOption) -> Result<RequestOption, LarkError> {
        let mut option = option.clone();
        option
            .headers
            .get_or_insert_with(http::HeaderMap::new)
            .insert(
                http::HeaderName::from_static("x-app-id"),
                http::HeaderValue::from_str(self.config.app_id())
                    .map_err(|err| LarkError::IllegalParam(err.to_string()))?,
            );
        Ok(option)
    }
}
