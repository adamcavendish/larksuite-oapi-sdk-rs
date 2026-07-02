use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

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

#[derive(Debug, Clone, Copy)]
pub struct GetInterviewRecordV2Query<'a> {
    pub interview_record_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetInterviewRecordV2Query<'a> {
    pub fn new(interview_record_id: &'a str) -> Self {
        Self {
            interview_record_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListInterviewRecordV2Query<'a> {
    pub interview_id: Option<&'a str>,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListInterviewRecordV2Query<'a> {
    pub fn new() -> Self {
        Self {
            interview_id: None,
            page: PageQuery::default(),
            user_id_type: None,
        }
    }

    pub fn interview_id(mut self, interview_id: impl Into<Option<&'a str>>) -> Self {
        self.interview_id = interview_id.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

impl<'a> Default for ListInterviewRecordV2Query<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetTalentV2Query<'a> {
    pub talent_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetTalentV2Query<'a> {
    pub fn new(talent_id: &'a str) -> Self {
        Self {
            talent_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
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
    ) -> Result<GetInterviewRecordV2Resp, LarkError> {
        self.get_by_query(
            &GetInterviewRecordV2Query::new(interview_record_id).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetInterviewRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetInterviewRecordV2Resp, LarkError> {
        let path = format!(
            "/open-apis/hire/v2/interview_records/{}",
            query.interview_record_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<InterviewRecordData, GetInterviewRecordV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        interview_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewRecordV2Resp, LarkError> {
        let query = ListInterviewRecordV2Query::new()
            .interview_id(interview_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInterviewRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListInterviewRecordV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v2/interview_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("interview_id", query.interview_id)
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<InterviewRecordListData, ListInterviewRecordV2Resp>()
        .await
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
    ) -> Result<GetTalentV2Resp, LarkError> {
        self.get_by_query(
            &GetTalentV2Query::new(talent_id).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetTalentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetTalentV2Resp, LarkError> {
        let path = format!("/open-apis/hire/v2/talents/{}", query.talent_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<TalentV2Data, GetTalentV2Resp>()
        .await
    }
}
