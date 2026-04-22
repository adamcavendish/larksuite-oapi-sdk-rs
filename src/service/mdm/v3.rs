use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, RequestOption};
use crate::service::common::parse_v2;
use crate::transport;

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
    pub async fn get(&self, option: &RequestOption) -> Result<GetBatchCountryRegionV3Resp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/mdm/v3/batch_country_region");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<BatchCountryRegionData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
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
    ) -> Result<ListCountryRegionV3Resp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/mdm/v3/country_regions");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<CountryRegionListData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListCountryRegionV3Resp {
            api_resp,
            code_error,
            data,
        })
    }
}
