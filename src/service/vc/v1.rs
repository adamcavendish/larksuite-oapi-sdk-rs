use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Room {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<Vec<Device>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_status: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_waiting_room: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_allow_call_in: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Device {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enrolled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital_signage: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_box_digital_signage: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Meeting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_user: Option<MeetingUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participant_count_accumulated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<Participant>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Participant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_host: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cohost: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_avg: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_meeting: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_meeting_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_meeting_duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_participant_count: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_meeting_per_day: Option<Vec<serde_json::Value>>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRoomReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_room_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_level_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRoomReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_room_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Vec<Device>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SetRoomConfigReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_room_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_display_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_digital_signage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_signage: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_room_box_digital_signage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_box_digital_signage: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_room_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
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

#[derive(Debug, Clone)]
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room: Option<Room>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomListData {
    #[serde(default)]
    pub rooms: Vec<Room>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomConfigData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_config: Option<RoomConfig>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting: Option<Meeting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingListData {
    #[serde(default)]
    pub meeting_briefs: Vec<Meeting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParticipantListData {
    #[serde(default)]
    pub participants: Vec<Participant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingReportData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meeting_report: Option<MeetingReport>,
}

impl_resp!(CreateRoomResp, RoomData);
impl_resp!(GetRoomResp, RoomData);
impl_resp!(ListRoomResp, RoomListData);
impl_resp!(GetRoomConfigResp, RoomConfigData);
impl_resp!(GetMeetingResp, MeetingData);
impl_resp!(ListMeetingResp, MeetingListData);
impl_resp!(ListParticipantResp, ParticipantListData);
impl_resp!(GetMeetingReportResp, MeetingReportData);

// ── Resources ──

pub struct RoomResource<'a> {
    config: &'a Config,
}

impl<'a> RoomResource<'a> {
    pub async fn create(
        &self,
        body: &CreateRoomReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateRoomResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/rooms");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<RoomData>(self.config, &api_req, option).await?;
        Ok(CreateRoomResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        room_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetRoomResp> {
        let path = format!("/open-apis/vc/v1/rooms/{room_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<RoomData>(self.config, &api_req, option).await?;
        Ok(GetRoomResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        room_id: &str,
        body: &UpdateRoomReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/vc/v1/rooms/{room_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete(&self, room_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/vc/v1/rooms/{room_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        room_level_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListRoomResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/rooms");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = room_level_id {
            api_req.query_params.set("room_level_id", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<RoomListData>(self.config, &api_req, option).await?;
        Ok(ListRoomResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct RoomConfigResource<'a> {
    config: &'a Config,
}

impl<'a> RoomConfigResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        scope: i32,
        country_id: Option<&str>,
        district_id: Option<&str>,
        building_id: Option<&str>,
        floor_name: Option<&str>,
        room_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetRoomConfigResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/room_configs/query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("scope", scope.to_string());
        if let Some(v) = country_id {
            api_req.query_params.set("country_id", v);
        }
        if let Some(v) = district_id {
            api_req.query_params.set("district_id", v);
        }
        if let Some(v) = building_id {
            api_req.query_params.set("building_id", v);
        }
        if let Some(v) = floor_name {
            api_req.query_params.set("floor_name", v);
        }
        if let Some(v) = room_id {
            api_req.query_params.set("room_id", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<RoomConfigData>(self.config, &api_req, option).await?;
        Ok(GetRoomConfigResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn set(
        &self,
        body: &SetRoomConfigReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/room_configs/set");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct MeetingResource<'a> {
    config: &'a Config,
}

impl<'a> MeetingResource<'a> {
    pub async fn get(
        &self,
        meeting_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMeetingResp> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<MeetingData>(self.config, &api_req, option).await?;
        Ok(GetMeetingResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn invite(
        &self,
        meeting_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/invite");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn kickout(
        &self,
        meeting_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/kickout");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn set_host(
        &self,
        meeting_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/set_host");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn end(&self, meeting_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/end");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list_by_no(
        &self,
        meeting_no: &str,
        start_time: &str,
        end_time: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMeetingResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/meetings/list_by_no");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("meeting_no", meeting_no);
        api_req.query_params.set("start_time", start_time);
        api_req.query_params.set("end_time", end_time);
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
            transport::request_typed::<MeetingListData>(self.config, &api_req, option).await?;
        Ok(ListMeetingResp {
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
        meeting_id: &str,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListParticipantResp> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/participants");
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

pub struct ReportResource<'a> {
    config: &'a Config,
}

impl<'a> ReportResource<'a> {
    pub async fn get_daily(
        &self,
        start_time: &str,
        end_time: &str,
        meeting_type: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMeetingReportResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/reports/get_daily");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("start_time", start_time);
        api_req.query_params.set("end_time", end_time);
        if let Some(v) = meeting_type {
            api_req.query_params.set("meeting_type", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<MeetingReportData>(self.config, &api_req, option).await?;
        Ok(GetMeetingReportResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_top_user(
        &self,
        start_time: &str,
        end_time: &str,
        limit: i32,
        order_by: i32,
        meeting_type: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/reports/get_top_user");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("start_time", start_time);
        api_req.query_params.set("end_time", end_time);
        api_req.query_params.set("limit", limit.to_string());
        api_req.query_params.set("order_by", order_by.to_string());
        if let Some(v) = meeting_type {
            api_req.query_params.set("meeting_type", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub room: RoomResource<'a>,
    pub room_config: RoomConfigResource<'a>,
    pub meeting: MeetingResource<'a>,
    pub participant: ParticipantResource<'a>,
    pub report: ReportResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            room: RoomResource { config },
            room_config: RoomConfigResource { config },
            meeting: MeetingResource { config },
            participant: ParticipantResource { config },
            report: ReportResource { config },
        }
    }
}
