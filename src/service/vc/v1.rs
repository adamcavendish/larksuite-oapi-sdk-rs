use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{EmptyResp, parse_v2};
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
    ) -> Result<CreateRoomResp, LarkError> {
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
    ) -> Result<GetRoomResp, LarkError> {
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
    ) -> Result<EmptyResp, LarkError> {
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

    pub async fn delete(
        &self,
        room_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
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
    ) -> Result<ListRoomResp, LarkError> {
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

    pub async fn mget(
        &self,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<MgetRoomResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/rooms/mget");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(MgetRoomResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn search(
        &self,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchRoomResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/rooms/search");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SearchRoomResp {
            api_resp,
            code_error,
            data,
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
    ) -> Result<GetRoomConfigResp, LarkError> {
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
    ) -> Result<EmptyResp, LarkError> {
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

    pub async fn set_checkboard_access_code(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SetCheckboardAccessCodeRoomConfigResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/vc/v1/room_configs/set_checkboard_access_code",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SetCheckboardAccessCodeRoomConfigResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn set_room_access_code(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SetRoomAccessCodeRoomConfigResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/vc/v1/room_configs/set_room_access_code",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SetRoomAccessCodeRoomConfigResp {
            api_resp,
            code_error,
            data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn query(
        &self,
        scope: i32,
        country_id: Option<&str>,
        district_id: Option<&str>,
        building_id: Option<&str>,
        floor_name: Option<&str>,
        room_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<QueryRoomConfigResp, LarkError> {
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
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryRoomConfigResp {
            api_resp,
            code_error,
            data,
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
    ) -> Result<GetMeetingResp, LarkError> {
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
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/invite");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
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
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/kickout");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
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
    ) -> Result<EmptyResp, LarkError> {
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

    pub async fn end(
        &self,
        meeting_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/end");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
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
    ) -> Result<ListMeetingResp, LarkError> {
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
    ) -> Result<ListParticipantResp, LarkError> {
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
    ) -> Result<GetMeetingReportResp, LarkError> {
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
    ) -> Result<EmptyResp, LarkError> {
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

// ── impl_resp_v2! macro ──

impl_resp_v2!(MgetRoomResp, serde_json::Value);
impl_resp_v2!(SearchRoomResp, serde_json::Value);
impl_resp_v2!(SetCheckboardAccessCodeRoomConfigResp, serde_json::Value);
impl_resp_v2!(SetRoomAccessCodeRoomConfigResp, serde_json::Value);
impl_resp_v2!(QueryRoomConfigResp, serde_json::Value);

// ── Alert response types ──

impl_resp_v2!(ListAlertResp, serde_json::Value);

// ── Export response types ──

impl_resp_v2!(ExportMeetingListResp, serde_json::Value);
impl_resp_v2!(ExportParticipantListResp, serde_json::Value);
impl_resp_v2!(ExportParticipantQualityListResp, serde_json::Value);
impl_resp_v2!(ExportResourceReservationListResp, serde_json::Value);
impl_resp_v2!(GetExportResp, serde_json::Value);
impl_resp_v2!(DownloadExportResp, serde_json::Value);

// ── MeetingRecording response types ──

impl_resp_v2!(GetMeetingRecordingResp, serde_json::Value);
impl_resp_v2!(SetPermissionMeetingRecordingResp, serde_json::Value);
impl_resp_v2!(StartMeetingRecordingResp, serde_json::Value);
impl_resp_v2!(StopMeetingRecordingResp, serde_json::Value);

// ── Reserve response types ──

impl_resp_v2!(ApplyReserveResp, serde_json::Value);
impl_resp_v2!(DeleteReserveResp, ());
impl_resp_v2!(GetReserveResp, serde_json::Value);
impl_resp_v2!(GetActiveMeetingReserveResp, serde_json::Value);
impl_resp_v2!(UpdateReserveResp, serde_json::Value);

// ── ReserveConfig response types ──

impl_resp_v2!(PatchReserveConfigResp, serde_json::Value);
impl_resp_v2!(ReserveScopeReserveConfigResp, serde_json::Value);
impl_resp_v2!(GetReserveConfigAdminResp, serde_json::Value);
impl_resp_v2!(PatchReserveConfigAdminResp, serde_json::Value);
impl_resp_v2!(GetReserveConfigDisableInformResp, serde_json::Value);
impl_resp_v2!(PatchReserveConfigDisableInformResp, serde_json::Value);
impl_resp_v2!(GetReserveConfigFormResp, serde_json::Value);
impl_resp_v2!(PatchReserveConfigFormResp, serde_json::Value);

// ── RoomLevel response types ──

impl_resp_v2!(CreateRoomLevelResp, serde_json::Value);
impl_resp_v2!(DelRoomLevelResp, serde_json::Value);
impl_resp_v2!(GetRoomLevelResp, serde_json::Value);
impl_resp_v2!(ListRoomLevelResp, serde_json::Value);
impl_resp_v2!(MgetRoomLevelResp, serde_json::Value);
impl_resp_v2!(PatchRoomLevelResp, serde_json::Value);
impl_resp_v2!(SearchRoomLevelResp, serde_json::Value);

// ── ScopeConfig response types ──

impl_resp_v2!(CreateScopeConfigResp, serde_json::Value);
impl_resp_v2!(GetScopeConfigResp, serde_json::Value);

// ── MeetingList / ParticipantList / ParticipantQualityList / ResourceReservationList response types ──

impl_resp_v2!(GetMeetingListResp, serde_json::Value);
impl_resp_v2!(GetParticipantListResp, serde_json::Value);
impl_resp_v2!(GetParticipantQualityListResp, serde_json::Value);
impl_resp_v2!(GetResourceReservationListResp, serde_json::Value);

// ── AlertResource ──

pub struct AlertResource<'a> {
    config: &'a Config,
}

impl<'a> AlertResource<'a> {
    /// GET /open-apis/vc/v1/alerts — 获取告警记录
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
        query_type: Option<i32>,
        query_value: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAlertResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/alerts");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = start_time {
            api_req.query_params.set("start_time", v);
        }
        if let Some(v) = end_time {
            api_req.query_params.set("end_time", v);
        }
        if let Some(v) = query_type {
            api_req.query_params.set("query_type", v.to_string());
        }
        if let Some(v) = query_value {
            api_req.query_params.set("query_value", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListAlertResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ExportResource ──

pub struct ExportResource<'a> {
    config: &'a Config,
}

impl<'a> ExportResource<'a> {
    /// POST /open-apis/vc/v1/exports/meeting_list — 导出会议明细
    pub async fn meeting_list(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ExportMeetingListResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/exports/meeting_list");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ExportMeetingListResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/vc/v1/exports/participant_list — 导出参会人明细
    pub async fn participant_list(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ExportParticipantListResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/vc/v1/exports/participant_list",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ExportParticipantListResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/vc/v1/exports/participant_quality_list — 导出参会人会议质量数据
    pub async fn participant_quality_list(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ExportParticipantQualityListResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/vc/v1/exports/participant_quality_list",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ExportParticipantQualityListResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/vc/v1/exports/resource_reservation_list — 导出会议室预定数据
    pub async fn resource_reservation_list(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ExportResourceReservationListResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/vc/v1/exports/resource_reservation_list",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ExportResourceReservationListResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/vc/v1/exports/:task_id — 查询导出任务结果
    pub async fn get(
        &self,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<GetExportResp, LarkError> {
        let path = format!("/open-apis/vc/v1/exports/{task_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetExportResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/vc/v1/exports/download — 下载导出文件
    pub async fn download(
        &self,
        file_token: &str,
        option: &RequestOption,
    ) -> Result<DownloadExportResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/exports/download");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("file_token", file_token);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DownloadExportResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── MeetingRecordingResource ──

pub struct MeetingRecordingResource<'a> {
    config: &'a Config,
}

impl<'a> MeetingRecordingResource<'a> {
    /// GET /open-apis/vc/v1/meetings/:meeting_id/recording — 获取录制文件
    pub async fn get(
        &self,
        meeting_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMeetingRecordingResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/recording");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetMeetingRecordingResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/recording/set_permission — 授权录制文件
    pub async fn set_permission(
        &self,
        meeting_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SetPermissionMeetingRecordingResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/set_permission");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SetPermissionMeetingRecordingResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/recording/start — 开始录制
    pub async fn start(
        &self,
        meeting_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<StartMeetingRecordingResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/start");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(StartMeetingRecordingResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/vc/v1/meetings/:meeting_id/recording/stop — 停止录制
    pub async fn stop(
        &self,
        meeting_id: &str,
        option: &RequestOption,
    ) -> Result<StopMeetingRecordingResp, LarkError> {
        let path = format!("/open-apis/vc/v1/meetings/{meeting_id}/recording/stop");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(StopMeetingRecordingResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ReserveResource ──

pub struct ReserveResource<'a> {
    config: &'a Config,
}

impl<'a> ReserveResource<'a> {
    /// POST /open-apis/vc/v1/reserves/apply — 预约会议
    pub async fn apply(
        &self,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ApplyReserveResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/reserves/apply");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ApplyReserveResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// DELETE /open-apis/vc/v1/reserves/:reserve_id — 删除预约
    pub async fn delete(
        &self,
        reserve_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteReserveResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserves/{reserve_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteReserveResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/vc/v1/reserves/:reserve_id — 获取预约
    pub async fn get(
        &self,
        reserve_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetReserveResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserves/{reserve_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetReserveResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/vc/v1/reserves/:reserve_id/get_active_meeting — 获取活跃会议
    pub async fn get_active_meeting(
        &self,
        reserve_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetActiveMeetingReserveResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserves/{reserve_id}/get_active_meeting");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetActiveMeetingReserveResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PUT /open-apis/vc/v1/reserves/:reserve_id — 更新预约
    pub async fn update(
        &self,
        reserve_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateReserveResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserves/{reserve_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateReserveResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ReserveConfigResource ──

pub struct ReserveConfigResource<'a> {
    config: &'a Config,
}

impl<'a> ReserveConfigResource<'a> {
    /// GET /open-apis/vc/v1/reserve_configs/reserve_scope — 获取会议室预定范围
    pub async fn reserve_scope(
        &self,
        scope_type: i32,
        scope_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ReserveScopeReserveConfigResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/vc/v1/reserve_configs/reserve_scope",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("scope_type", scope_type.to_string());
        api_req.query_params.set("scope_id", scope_id);
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ReserveScopeReserveConfigResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id — 更新会议室预定范围
    pub async fn patch(
        &self,
        reserve_config_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchReserveConfigResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchReserveConfigResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ReserveConfigAdminResource ──

pub struct ReserveConfigAdminResource<'a> {
    config: &'a Config,
}

impl<'a> ReserveConfigAdminResource<'a> {
    /// GET /open-apis/vc/v1/reserve_configs/:reserve_config_id/admin
    pub async fn get(
        &self,
        reserve_config_id: &str,
        scope_type: i32,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetReserveConfigAdminResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/admin");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("scope_type", scope_type.to_string());
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetReserveConfigAdminResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id/admin
    pub async fn patch(
        &self,
        reserve_config_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchReserveConfigAdminResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/admin");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchReserveConfigAdminResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ReserveConfigDisableInformResource ──

pub struct ReserveConfigDisableInformResource<'a> {
    config: &'a Config,
}

impl<'a> ReserveConfigDisableInformResource<'a> {
    /// GET /open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform
    pub async fn get(
        &self,
        reserve_config_id: &str,
        scope_type: i32,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetReserveConfigDisableInformResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/disable_inform");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("scope_type", scope_type.to_string());
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetReserveConfigDisableInformResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id/disable_inform
    pub async fn patch(
        &self,
        reserve_config_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchReserveConfigDisableInformResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/disable_inform");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchReserveConfigDisableInformResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ReserveConfigFormResource ──

pub struct ReserveConfigFormResource<'a> {
    config: &'a Config,
}

impl<'a> ReserveConfigFormResource<'a> {
    /// GET /open-apis/vc/v1/reserve_configs/:reserve_config_id/form
    pub async fn get(
        &self,
        reserve_config_id: &str,
        scope_type: i32,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetReserveConfigFormResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/form");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("scope_type", scope_type.to_string());
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetReserveConfigFormResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/vc/v1/reserve_configs/:reserve_config_id/form
    pub async fn patch(
        &self,
        reserve_config_id: &str,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchReserveConfigFormResp, LarkError> {
        let path = format!("/open-apis/vc/v1/reserve_configs/{reserve_config_id}/form");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchReserveConfigFormResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── RoomLevelResource ──

pub struct RoomLevelResource<'a> {
    config: &'a Config,
}

impl<'a> RoomLevelResource<'a> {
    /// POST /open-apis/vc/v1/room_levels — 创建会议室层级
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateRoomLevelResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/room_levels");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateRoomLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/vc/v1/room_levels/del — 删除会议室层级
    pub async fn del(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<DelRoomLevelResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/room_levels/del");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DelRoomLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/vc/v1/room_levels/:room_level_id — 查询会议室层级详情
    pub async fn get(
        &self,
        room_level_id: &str,
        option: &RequestOption,
    ) -> Result<GetRoomLevelResp, LarkError> {
        let path = format!("/open-apis/vc/v1/room_levels/{room_level_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetRoomLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/vc/v1/room_levels — 查询会议室层级列表
    pub async fn list(
        &self,
        room_level_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListRoomLevelResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/room_levels");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = room_level_id {
            api_req.query_params.set("room_level_id", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListRoomLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/vc/v1/room_levels/mget — 批量查询会议室层级详情
    pub async fn mget(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<MgetRoomLevelResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/room_levels/mget");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(MgetRoomLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// PATCH /open-apis/vc/v1/room_levels/:room_level_id — 更新会议室层级
    pub async fn patch(
        &self,
        room_level_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchRoomLevelResp, LarkError> {
        let path = format!("/open-apis/vc/v1/room_levels/{room_level_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchRoomLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/vc/v1/room_levels/search — 搜索会议室层级
    pub async fn search(
        &self,
        custom_level_ids: &str,
        option: &RequestOption,
    ) -> Result<SearchRoomLevelResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/room_levels/search");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("custom_level_ids", custom_level_ids);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SearchRoomLevelResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ScopeConfigResource ──

pub struct ScopeConfigResource<'a> {
    config: &'a Config,
}

impl<'a> ScopeConfigResource<'a> {
    /// POST /open-apis/vc/v1/scope_config — 设置会议室配置
    pub async fn create(
        &self,
        body: serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateScopeConfigResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/vc/v1/scope_config");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateScopeConfigResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/vc/v1/scope_config — 查询会议室配置
    pub async fn get(
        &self,
        scope_type: i32,
        scope_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetScopeConfigResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/scope_config");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req
            .query_params
            .set("scope_type", scope_type.to_string());
        api_req.query_params.set("scope_id", scope_id);
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetScopeConfigResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── MeetingListResource ──

pub struct MeetingListResource<'a> {
    config: &'a Config,
}

impl<'a> MeetingListResource<'a> {
    /// GET /open-apis/vc/v1/meeting_list
    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        start_time: &str,
        end_time: &str,
        meeting_status: Option<i32>,
        meeting_no: Option<&str>,
        user_id: Option<&str>,
        room_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetMeetingListResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/meeting_list");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("start_time", start_time);
        api_req.query_params.set("end_time", end_time);
        if let Some(v) = meeting_status {
            api_req.query_params.set("meeting_status", v.to_string());
        }
        if let Some(v) = meeting_no {
            api_req.query_params.set("meeting_no", v);
        }
        if let Some(v) = user_id {
            api_req.query_params.set("user_id", v);
        }
        if let Some(v) = room_id {
            api_req.query_params.set("room_id", v);
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
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetMeetingListResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ParticipantListResource ──

pub struct ParticipantListResource<'a> {
    config: &'a Config,
}

impl<'a> ParticipantListResource<'a> {
    /// GET /open-apis/vc/v1/participant_list
    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        meeting_start_time: &str,
        meeting_end_time: &str,
        meeting_status: Option<i32>,
        meeting_no: &str,
        user_id: Option<&str>,
        room_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetParticipantListResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/vc/v1/participant_list");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req
            .query_params
            .set("meeting_start_time", meeting_start_time);
        api_req
            .query_params
            .set("meeting_end_time", meeting_end_time);
        api_req.query_params.set("meeting_no", meeting_no);
        if let Some(v) = meeting_status {
            api_req.query_params.set("meeting_status", v.to_string());
        }
        if let Some(v) = user_id {
            api_req.query_params.set("user_id", v);
        }
        if let Some(v) = room_id {
            api_req.query_params.set("room_id", v);
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
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetParticipantListResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ParticipantQualityListResource ──

pub struct ParticipantQualityListResource<'a> {
    config: &'a Config,
}

impl<'a> ParticipantQualityListResource<'a> {
    /// GET /open-apis/vc/v1/participant_quality_list
    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        meeting_start_time: &str,
        meeting_end_time: &str,
        meeting_no: &str,
        join_time: &str,
        user_id: Option<&str>,
        room_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetParticipantQualityListResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/vc/v1/participant_quality_list",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req
            .query_params
            .set("meeting_start_time", meeting_start_time);
        api_req
            .query_params
            .set("meeting_end_time", meeting_end_time);
        api_req.query_params.set("meeting_no", meeting_no);
        api_req.query_params.set("join_time", join_time);
        if let Some(v) = user_id {
            api_req.query_params.set("user_id", v);
        }
        if let Some(v) = room_id {
            api_req.query_params.set("room_id", v);
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
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetParticipantQualityListResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ResourceReservationListResource ──

pub struct ResourceReservationListResource<'a> {
    config: &'a Config,
}

impl<'a> ResourceReservationListResource<'a> {
    /// GET /open-apis/vc/v1/resource_reservation_list
    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        room_level_id: &str,
        has_video: Option<bool>,
        start_time: &str,
        end_time: &str,
        room_ids: Option<&str>,
        is_exclude: Option<bool>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetResourceReservationListResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/vc/v1/resource_reservation_list",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("room_level_id", room_level_id);
        api_req.query_params.set("start_time", start_time);
        api_req.query_params.set("end_time", end_time);
        if let Some(v) = has_video {
            api_req.query_params.set("has_video", v.to_string());
        }
        if let Some(v) = room_ids {
            api_req.query_params.set("room_ids", v);
        }
        if let Some(v) = is_exclude {
            api_req.query_params.set("is_exclude", v.to_string());
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetResourceReservationListResp {
            api_resp,
            code_error,
            data,
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
    pub alert: AlertResource<'a>,
    pub export: ExportResource<'a>,
    pub meeting_recording: MeetingRecordingResource<'a>,
    pub reserve: ReserveResource<'a>,
    pub reserve_config: ReserveConfigResource<'a>,
    pub reserve_config_admin: ReserveConfigAdminResource<'a>,
    pub reserve_config_disable_inform: ReserveConfigDisableInformResource<'a>,
    pub reserve_config_form: ReserveConfigFormResource<'a>,
    pub room_level: RoomLevelResource<'a>,
    pub scope_config: ScopeConfigResource<'a>,
    pub meeting_list: MeetingListResource<'a>,
    pub participant_list: ParticipantListResource<'a>,
    pub participant_quality_list: ParticipantQualityListResource<'a>,
    pub resource_reservation_list: ResourceReservationListResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            room: RoomResource { config },
            room_config: RoomConfigResource { config },
            meeting: MeetingResource { config },
            participant: ParticipantResource { config },
            report: ReportResource { config },
            alert: AlertResource { config },
            export: ExportResource { config },
            meeting_recording: MeetingRecordingResource { config },
            reserve: ReserveResource { config },
            reserve_config: ReserveConfigResource { config },
            reserve_config_admin: ReserveConfigAdminResource { config },
            reserve_config_disable_inform: ReserveConfigDisableInformResource { config },
            reserve_config_form: ReserveConfigFormResource { config },
            room_level: RoomLevelResource { config },
            scope_config: ScopeConfigResource { config },
            meeting_list: MeetingListResource { config },
            participant_list: ParticipantListResource { config },
            participant_quality_list: ParticipantQualityListResource { config },
            resource_reservation_list: ResourceReservationListResource { config },
        }
    }
}
