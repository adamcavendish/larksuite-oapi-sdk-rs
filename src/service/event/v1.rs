use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, RequestOption};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OutboundIp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_list: Option<Vec<String>>,
}

// ── Response wrappers ──

impl_resp!(GetOutboundIpResp, OutboundIp);

// ── Resources ──

pub struct OutboundIpResource<'a> {
    config: &'a Config,
}

impl<'a> OutboundIpResource<'a> {
    pub async fn list(&self, option: &RequestOption) -> Result<GetOutboundIpResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/event/v1/outbound_ip");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<OutboundIp>(self.config, &api_req, option).await?;
        Ok(GetOutboundIpResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub outbound_ip: OutboundIpResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            outbound_ip: OutboundIpResource { config },
        }
    }
}
