use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkplaceAccessData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_workplace_access_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_workplace_access: Option<Vec<PerWorkplaceAccess>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerWorkplaceAccess {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workplace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workplace_name: Option<Vec<crate::JsonValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_user_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockAccessData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_user_count: Option<i64>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkplaceAccessListData {
    #[serde(default)]
    pub items: Vec<WorkplaceAccessData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockAccessListData {
    #[serde(default)]
    pub items: Vec<BlockAccessData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(GetWorkplaceAccessResp, WorkplaceAccessListData);
impl_resp!(GetBlockAccessResp, BlockAccessListData);

#[derive(Debug, Clone, Copy)]
pub struct WorkplaceAccessDataQuery<'a> {
    pub from_date: &'a str,
    pub to_date: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> WorkplaceAccessDataQuery<'a> {
    pub fn new(from_date: &'a str, to_date: &'a str) -> Self {
        Self {
            from_date,
            to_date,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CustomWorkplaceAccessDataQuery<'a> {
    pub from_date: &'a str,
    pub to_date: &'a str,
    pub workplace_id: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> CustomWorkplaceAccessDataQuery<'a> {
    pub fn new(from_date: &'a str, to_date: &'a str) -> Self {
        Self {
            from_date,
            to_date,
            workplace_id: None,
            page: PageQuery::default(),
        }
    }

    pub fn workplace_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.workplace_id = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WorkplaceBlockAccessDataQuery<'a> {
    pub from_date: &'a str,
    pub to_date: &'a str,
    pub block_id: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> WorkplaceBlockAccessDataQuery<'a> {
    pub fn new(from_date: &'a str, to_date: &'a str) -> Self {
        Self {
            from_date,
            to_date,
            block_id: None,
            page: PageQuery::default(),
        }
    }

    pub fn block_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.block_id = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

// ── Resources ──

pub struct WorkplaceAccessDataResource<'a> {
    config: &'a Config,
}

impl<'a> WorkplaceAccessDataResource<'a> {
    pub async fn search(
        &self,
        from_date: &str,
        to_date: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetWorkplaceAccessResp, LarkError> {
        let query = WorkplaceAccessDataQuery::new(from_date, to_date)
            .page(PageQuery::from_parts(page_size, page_token));
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &WorkplaceAccessDataQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetWorkplaceAccessResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/workplace/v1/workplace_access_data/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("from_date", query.from_date)
        .query("to_date", query.to_date)
        .page_query(query.page)
        .send_response::<WorkplaceAccessListData, GetWorkplaceAccessResp>()
        .await
    }

    pub async fn get(
        &self,
        from_date: &str,
        to_date: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetWorkplaceAccessResp, LarkError> {
        let query = WorkplaceAccessDataQuery::new(from_date, to_date)
            .page(PageQuery::from_parts(page_size, page_token));
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &WorkplaceAccessDataQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetWorkplaceAccessResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/workplace/v1/workplace_access_data/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("from_date", query.from_date)
        .query("to_date", query.to_date)
        .page_query(query.page)
        .send_response::<WorkplaceAccessListData, GetWorkplaceAccessResp>()
        .await
    }
}

pub struct CustomWorkplaceAccessDataResource<'a> {
    config: &'a Config,
}

impl<'a> CustomWorkplaceAccessDataResource<'a> {
    pub async fn search(
        &self,
        from_date: &str,
        to_date: &str,
        workplace_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetBlockAccessResp, LarkError> {
        let query = CustomWorkplaceAccessDataQuery::new(from_date, to_date)
            .workplace_id(workplace_id)
            .page(PageQuery::from_parts(page_size, page_token));
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &CustomWorkplaceAccessDataQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetBlockAccessResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/workplace/v1/custom_workplace_access_data/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("from_date", query.from_date)
        .query("to_date", query.to_date)
        .query("workplace_id", query.workplace_id)
        .page_query(query.page)
        .send_response::<BlockAccessListData, GetBlockAccessResp>()
        .await
    }

    pub async fn get(
        &self,
        from_date: &str,
        to_date: &str,
        workplace_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetBlockAccessResp, LarkError> {
        let query = CustomWorkplaceAccessDataQuery::new(from_date, to_date)
            .workplace_id(workplace_id)
            .page(PageQuery::from_parts(page_size, page_token));
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &CustomWorkplaceAccessDataQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetBlockAccessResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/workplace/v1/custom_workplace_access_data/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("from_date", query.from_date)
        .query("to_date", query.to_date)
        .query("workplace_id", query.workplace_id)
        .page_query(query.page)
        .send_response::<BlockAccessListData, GetBlockAccessResp>()
        .await
    }
}

// ── Version struct ──

pub struct WorkplaceBlockAccessDataResource<'a> {
    config: &'a Config,
}

impl<'a> WorkplaceBlockAccessDataResource<'a> {
    pub async fn search(
        &self,
        from_date: &str,
        to_date: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        block_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetBlockAccessResp, LarkError> {
        let query = WorkplaceBlockAccessDataQuery::new(from_date, to_date)
            .page(PageQuery::from_parts(page_size, page_token))
            .block_id(block_id);
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &WorkplaceBlockAccessDataQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetBlockAccessResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/workplace/v1/workplace_block_access_data/search",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("from_date", query.from_date)
        .query("to_date", query.to_date)
        .query("block_id", query.block_id)
        .page_query(query.page)
        .send_response::<BlockAccessListData, GetBlockAccessResp>()
        .await
    }
}

pub struct V1<'a> {
    pub workplace_access_data: WorkplaceAccessDataResource<'a>,
    pub custom_workplace_access_data: CustomWorkplaceAccessDataResource<'a>,
    pub workplace_block_access_data: WorkplaceBlockAccessDataResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            workplace_access_data: WorkplaceAccessDataResource { config },
            custom_workplace_access_data: CustomWorkplaceAccessDataResource { config },
            workplace_block_access_data: WorkplaceBlockAccessDataResource { config },
        }
    }
}
