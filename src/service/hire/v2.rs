use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Response data types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewRecordData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_record: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewRecordListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentV2Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent: Option<serde_json::Value>,
}

// ── Response types ────────────────────────────────────────────────────────────

impl_resp_v2!(GetInterviewRecordV2Resp, InterviewRecordData);
impl_resp_v2!(ListInterviewRecordV2Resp, InterviewRecordListData);
impl_resp_v2!(GetTalentV2Resp, TalentV2Data);

// ── Resource helper ───────────────────────────────────────────────────────────

fn parse<T: for<'de> serde::Deserialize<'de>>(
    api_resp: ApiResp,
    raw: crate::resp::RawResponse<T>,
) -> impl FnOnce() -> (ApiResp, Option<CodeError>, Option<T>) {
    move || {
        let code_error = if raw.code_error.code != 0 {
            Some(raw.code_error)
        } else {
            None
        };
        (api_resp, code_error, raw.data)
    }
}

// ── V2 service entry ──────────────────────────────────────────────────────────

pub struct V2<'a> {
    pub interview_record: InterviewRecordV2Resource<'a>,
    pub talent: TalentV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            interview_record: InterviewRecordV2Resource { config },
            talent: TalentV2Resource { config },
        }
    }
}

// ── InterviewRecord resource ──────────────────────────────────────────────────

pub struct InterviewRecordV2Resource<'a> {
    config: &'a Config,
}

impl InterviewRecordV2Resource<'_> {
    pub async fn get(
        &self,
        interview_record_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetInterviewRecordV2Resp> {
        let path = format!("/open-apis/hire/v2/interview_records/{interview_record_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<InterviewRecordData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(GetInterviewRecordV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        interview_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewRecordV2Resp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/hire/v2/interview_records");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = interview_id {
            api_req.query_params.set("interview_id", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<InterviewRecordListData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(ListInterviewRecordV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Talent resource ───────────────────────────────────────────────────────────

pub struct TalentV2Resource<'a> {
    config: &'a Config,
}

impl TalentV2Resource<'_> {
    pub async fn get(
        &self,
        talent_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetTalentV2Resp> {
        let path = format!("/open-apis/hire/v2/talents/{talent_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TalentV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse(api_resp, raw)();
        Ok(GetTalentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}
