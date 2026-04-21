use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinutesInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinutesParticipant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinutesTranscript {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<TranscriptPhrase>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TranscriptPhrase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaker: Option<MinutesParticipant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<serde_json::Value>>,
}

// ── Response wrappers ──

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinutesInfoData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes: Option<MinutesInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParticipantListData {
    #[serde(default)]
    pub participants: Vec<MinutesParticipant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TranscriptData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<MinutesTranscript>,
}

impl_resp!(GetMinutesResp, MinutesInfoData);
impl_resp!(ListParticipantResp, ParticipantListData);
impl_resp!(GetTranscriptResp, TranscriptData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinuteMediaData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinuteStatisticsData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<serde_json::Value>,
}

impl_resp!(GetMinuteMediaResp, MinuteMediaData);
impl_resp!(GetMinuteStatisticsResp, MinuteStatisticsData);

// ── Resources ──

pub struct MinutesResource<'a> {
    config: &'a Config,
}

impl<'a> MinutesResource<'a> {
    pub async fn get(
        &self,
        minutes_token: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMinutesResp> {
        let path = format!("/open-apis/minutes/v1/minutes/{minutes_token}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<MinutesInfoData>(self.config, &api_req, option).await?;
        Ok(GetMinutesResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct ParticipantResource<'a> {
    config: &'a Config,
}

impl<'a> ParticipantResource<'a> {
    pub async fn list(
        &self,
        minutes_token: &str,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListParticipantResp> {
        let path = format!("/open-apis/minutes/v1/minutes/{minutes_token}/participants");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ParticipantListData>(self.config, &api_req, option).await?;
        Ok(ListParticipantResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct TranscriptResource<'a> {
    config: &'a Config,
}

impl<'a> TranscriptResource<'a> {
    pub async fn get(
        &self,
        minutes_token: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetTranscriptResp> {
        let path = format!("/open-apis/minutes/v1/minutes/{minutes_token}/transcript");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TranscriptData>(self.config, &api_req, option).await?;
        Ok(GetTranscriptResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct MinuteMediaResource<'a> {
    config: &'a Config,
}

impl<'a> MinuteMediaResource<'a> {
    pub async fn get(
        &self,
        minutes_token: &str,
        option: &RequestOption,
    ) -> Result<GetMinuteMediaResp> {
        let path = format!("/open-apis/minutes/v1/minutes/{minutes_token}/media");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<MinuteMediaData>(self.config, &api_req, option).await?;
        Ok(GetMinuteMediaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct MinuteStatisticsResource<'a> {
    config: &'a Config,
}

impl<'a> MinuteStatisticsResource<'a> {
    pub async fn get(
        &self,
        minutes_token: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMinuteStatisticsResp> {
        let path = format!("/open-apis/minutes/v1/minutes/{minutes_token}/statistics");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<MinuteStatisticsData>(self.config, &api_req, option).await?;
        Ok(GetMinuteStatisticsResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub minutes: MinutesResource<'a>,
    pub participant: ParticipantResource<'a>,
    pub transcript: TranscriptResource<'a>,
    pub minute_media: MinuteMediaResource<'a>,
    pub minute_statistics: MinuteStatisticsResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            minutes: MinutesResource { config },
            participant: ParticipantResource { config },
            transcript: TranscriptResource { config },
            minute_media: MinuteMediaResource { config },
            minute_statistics: MinuteStatisticsResource { config },
        }
    }
}
