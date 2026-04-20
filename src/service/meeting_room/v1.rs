use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MeetingRoom {
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
    pub floor_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub building_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_check: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_highlight: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Building {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub building_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floors: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district_id: Option<String>,
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
pub struct RoomData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room: Option<MeetingRoom>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoomListData {
    #[serde(default)]
    pub rooms: Vec<MeetingRoom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildingListData {
    #[serde(default)]
    pub buildings: Vec<Building>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(GetRoomResp, RoomData);
impl_resp!(ListRoomResp, RoomListData);
impl_resp!(ListBuildingResp, BuildingListData);

// ── Resources ──

pub struct RoomResource<'a> {
    config: &'a Config,
}

impl<'a> RoomResource<'a> {
    pub async fn get(&self, room_id: &str, option: &RequestOption) -> Result<GetRoomResp> {
        let path = format!("/open-apis/meeting_room/v1/rooms/{room_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<RoomData>(self.config, &api_req, option).await?;
        Ok(GetRoomResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        building_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListRoomResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/meeting_room/v1/rooms");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = building_id {
            api_req.query_params.set("building_id", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
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

pub struct BuildingResource<'a> {
    config: &'a Config,
}

impl<'a> BuildingResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListBuildingResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/meeting_room/v1/buildings");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<BuildingListData>(self.config, &api_req, option).await?;
        Ok(ListBuildingResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub room: RoomResource<'a>,
    pub building: BuildingResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            room: RoomResource { config },
            building: BuildingResource { config },
        }
    }
}
