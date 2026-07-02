use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReviewActivity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivityListData {
    #[serde(default)]
    pub items: Vec<ReviewActivity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListActivityResp, ActivityListData);

// -- Query parameter types --

#[derive(Debug, Clone, Copy, Default)]
pub struct ListActivityQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListActivityQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

// ── Resources ──

pub struct ActivityResource<'a> {
    config: &'a Config,
}

impl<'a> ActivityResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListActivityResp, LarkError> {
        let query = ListActivityQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListActivityQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListActivityResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/performance/v1/activities",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_response::<ActivityListData, ListActivityResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub activity: ActivityResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            activity: ActivityResource { config },
        }
    }
}
