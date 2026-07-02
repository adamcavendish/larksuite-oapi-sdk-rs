use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

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

// -- Query parameter types --

#[derive(Debug, Clone, Copy, Default)]
pub struct ListRoomQuery<'a> {
    pub building_id: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListRoomQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn building_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.building_id = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListBuildingQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListBuildingQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

// ── Resources ──

pub struct RoomResource<'a> {
    config: &'a Config,
}

impl<'a> RoomResource<'a> {
    pub async fn get(
        &self,
        room_id: &str,
        option: &RequestOption,
    ) -> Result<GetRoomResp, LarkError> {
        let path = format!("/open-apis/meeting_room/v1/rooms/{room_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<RoomData, GetRoomResp>()
        .await
    }

    pub async fn list(
        &self,
        building_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListRoomResp, LarkError> {
        let query = ListRoomQuery::new()
            .building_id(building_id)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListRoomQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListRoomResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/meeting_room/v1/rooms",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("building_id", query.building_id)
        .page_query(query.page)
        .send_response::<RoomListData, ListRoomResp>()
        .await
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
    ) -> Result<ListBuildingResp, LarkError> {
        let query = ListBuildingQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListBuildingQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListBuildingResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/meeting_room/v1/buildings",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_response::<BuildingListData, ListBuildingResp>()
        .await
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
