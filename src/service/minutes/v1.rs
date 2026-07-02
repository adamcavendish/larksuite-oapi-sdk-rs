use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

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

// -- Query parameter types --

#[derive(Debug, Clone, Copy)]
pub struct GetMinutesQuery<'a> {
    pub minutes_token: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetMinutesQuery<'a> {
    pub fn new(minutes_token: &'a str) -> Self {
        Self {
            minutes_token,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListParticipantQuery<'a> {
    pub minutes_token: &'a str,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListParticipantQuery<'a> {
    pub fn new(minutes_token: &'a str) -> Self {
        Self {
            minutes_token,
            user_id_type: None,
            page: PageQuery::new(),
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetTranscriptQuery<'a> {
    pub minutes_token: &'a str,
    pub need_speaker: Option<bool>,
    pub need_timestamp: Option<bool>,
    pub file_format: Option<&'a str>,
}

impl<'a> GetTranscriptQuery<'a> {
    pub fn new(minutes_token: &'a str) -> Self {
        Self {
            minutes_token,
            need_speaker: None,
            need_timestamp: None,
            file_format: None,
        }
    }

    pub fn need_speaker(mut self, value: impl Into<Option<bool>>) -> Self {
        self.need_speaker = value.into();
        self
    }

    pub fn need_timestamp(mut self, value: impl Into<Option<bool>>) -> Self {
        self.need_timestamp = value.into();
        self
    }

    pub fn file_format(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.file_format = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetMinuteStatisticsQuery<'a> {
    pub minutes_token: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetMinuteStatisticsQuery<'a> {
    pub fn new(minutes_token: &'a str) -> Self {
        Self {
            minutes_token,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

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
    ) -> Result<GetMinutesResp, LarkError> {
        self.get_by_query(
            &GetMinutesQuery::new(minutes_token).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetMinutesQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetMinutesResp, LarkError> {
        let path = format!("/open-apis/minutes/v1/minutes/{}", query.minutes_token);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<MinutesInfoData, GetMinutesResp>()
        .await
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
    ) -> Result<ListParticipantResp, LarkError> {
        let query = ListParticipantQuery::new(minutes_token)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListParticipantQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListParticipantResp, LarkError> {
        let path = format!(
            "/open-apis/minutes/v1/minutes/{}/participants",
            query.minutes_token
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_response::<ParticipantListData, ListParticipantResp>()
        .await
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
    ) -> Result<GetTranscriptResp, LarkError> {
        let path = format!("/open-apis/minutes/v1/minutes/{minutes_token}/transcript");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_response::<TranscriptData, GetTranscriptResp>()
        .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetTranscriptQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetTranscriptResp, LarkError> {
        let path = format!(
            "/open-apis/minutes/v1/minutes/{}/transcript",
            query.minutes_token
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("need_speaker", query.need_speaker)
        .query("need_timestamp", query.need_timestamp)
        .query("file_format", query.file_format)
        .send_response::<TranscriptData, GetTranscriptResp>()
        .await
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
    ) -> Result<GetMinuteMediaResp, LarkError> {
        let path = format!("/open-apis/minutes/v1/minutes/{minutes_token}/media");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<MinuteMediaData, GetMinuteMediaResp>()
        .await
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
    ) -> Result<GetMinuteStatisticsResp, LarkError> {
        self.get_by_query(
            &GetMinuteStatisticsQuery::new(minutes_token).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetMinuteStatisticsQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetMinuteStatisticsResp, LarkError> {
        let path = format!(
            "/open-apis/minutes/v1/minutes/{}/statistics",
            query.minutes_token
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<MinuteStatisticsData, GetMinuteStatisticsResp>()
        .await
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
