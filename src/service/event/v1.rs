use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OutboundIp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_list: Option<Vec<String>>,
}

// ── Response wrappers ──

impl_resp!(GetOutboundIpResp, OutboundIp);

#[derive(Debug, Clone, Copy, Default)]
pub struct ListOutboundIpQuery;

impl ListOutboundIpQuery {
    pub fn new() -> Self {
        Self
    }
}

// ── Resources ──

pub struct OutboundIpResource<'a> {
    config: &'a Config,
}

impl<'a> OutboundIpResource<'a> {
    pub async fn list(&self, option: &RequestOption) -> Result<GetOutboundIpResp, LarkError> {
        self.list_by_query(&ListOutboundIpQuery::new(), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        _query: &ListOutboundIpQuery,
        option: &RequestOption,
    ) -> Result<GetOutboundIpResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/event/v1/outbound_ip",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<OutboundIp, GetOutboundIpResp>()
        .await
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
