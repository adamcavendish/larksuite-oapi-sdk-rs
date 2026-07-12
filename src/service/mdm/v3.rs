use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nString {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multilingual_value: Option<std::collections::HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_language: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Enum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multilingual_name: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Language {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ietf_language_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdm_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeZone {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Common>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utc_offset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdm_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Common {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_executors: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountryRegion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_3_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_2_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub western_script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdm_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overseas: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<I18nString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continents: Option<Enum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub md_local_script: Option<Language>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub md_western_script: Option<Language>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub md_time_zone: Option<Vec<TimeZone>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCountryRegionData {
    #[serde(default)]
    pub data: Vec<CountryRegion>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountryRegionListData {
    #[serde(default)]
    pub data: Vec<CountryRegion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/mdm/v3/batch_country_region",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<BatchCountryRegionData, GetBatchCountryRegionV3Resp>()
        .await
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
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/mdm/v3/country_regions",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_v2_response::<CountryRegionListData, ListCountryRegionV3Resp>()
        .await
    }
}
