use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCountryRegionData {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountryRegionListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp_v2!(GetBatchCountryRegionV3Resp, BatchCountryRegionData);
impl_resp_v2!(ListCountryRegionV3Resp, CountryRegionListData);

#[derive(Debug, Clone, Copy, Default)]
pub struct GetBatchCountryRegionV3Query;

impl GetBatchCountryRegionV3Query {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListCountryRegionV3Query<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListCountryRegionV3Query<'a> {
    pub fn new() -> Self {
        Self {
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

impl<'a> Default for ListCountryRegionV3Query<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct V3<'a> {
    pub batch_country_region: BatchCountryRegionV3Resource<'a>,
    pub country_region: CountryRegionV3Resource<'a>,
}

impl<'a> V3<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            batch_country_region: BatchCountryRegionV3Resource { config },
            country_region: CountryRegionV3Resource { config },
        }
    }
}

pub struct BatchCountryRegionV3Resource<'a> {
    config: &'a Config,
}

impl BatchCountryRegionV3Resource<'_> {
    pub async fn get(
        &self,
        option: &RequestOption,
    ) -> Result<GetBatchCountryRegionV3Resp, LarkError> {
        self.get_by_query(&GetBatchCountryRegionV3Query::new(), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        _query: &GetBatchCountryRegionV3Query,
        option: &RequestOption,
    ) -> Result<GetBatchCountryRegionV3Resp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/mdm/v3/batch_country_region",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<BatchCountryRegionData>()
        .await?;
        Ok(GetBatchCountryRegionV3Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CountryRegionV3Resource<'a> {
    config: &'a Config,
}

impl CountryRegionV3Resource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCountryRegionV3Resp, LarkError> {
        let query =
            ListCountryRegionV3Query::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCountryRegionV3Query<'_>,
        option: &RequestOption,
    ) -> Result<ListCountryRegionV3Resp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/mdm/v3/country_regions",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_v2::<CountryRegionListData>()
        .await?;
        Ok(ListCountryRegionV3Resp {
            api_resp,
            code_error,
            data,
        })
    }
}
