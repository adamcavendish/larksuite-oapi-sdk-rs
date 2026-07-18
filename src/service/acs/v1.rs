use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Feature {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_uploaded: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Property {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_device_face_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_face_capacity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_clock_in: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceExternal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserExternal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_num: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpeningTimePeriodExternal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_hhmm: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_hhmm: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpeningTimeValidDayExternal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_day: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_day: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpeningTimeExternal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_day: Option<OpeningTimeValidDayExternal>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weekdays: Vec<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub day_times: Vec<OpeningTimePeriodExternal>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Rule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<DeviceExternal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_count: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<UserExternal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visitor_count: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visitors: Vec<UserExternal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remind_face: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opening_time: Option<OpeningTimeExternal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_temp: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<Feature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcsUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_list: Option<Vec<crate::JsonValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<Feature>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_clock_in: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_door_open: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcsDevice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_sn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub door_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub door_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<Property>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserFaceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<crate::JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRuleExternalReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceBindRuleExternalReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateVisitorReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserExternal>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcsUserData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<AcsUser>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcsUserListData {
    #[serde(default)]
    pub items: Vec<AcsUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessRecordListData {
    #[serde(default)]
    pub items: Vec<AccessRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceListData {
    #[serde(default)]
    pub items: Vec<AcsDevice>,
}

impl_resp!(GetAcsUserResp, AcsUserData);
impl_resp!(ListAcsUserResp, AcsUserListData);
impl_resp!(ListAccessRecordResp, AccessRecordListData);
impl_resp!(ListDeviceResp, DeviceListData);

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetUserQuery<'a> {
    pub user_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetUserQuery<'a> {
    pub fn new(user_id: &'a str) -> Self {
        Self {
            user_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListUserQuery<'a> {
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListUserQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListAccessRecordQuery<'a> {
    pub page: PageQuery<'a>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub device_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListAccessRecordQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }

    pub fn from(mut self, value: impl Into<Option<i64>>) -> Self {
        self.from = value.into();
        self
    }

    pub fn to(mut self, value: impl Into<Option<i64>>) -> Self {
        self.to = value.into();
        self
    }

    pub fn device_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.device_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

// ── New data types (v2 pattern) ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateRuleExternalData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRuleExternalData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessRecordAccessPhotoData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFaceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisitorData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visitor_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisitorListData {
    #[serde(default)]
    pub items: Vec<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp_v2!(CreateRuleExternalResp, CreateRuleExternalData);
impl_resp_v2!(DeleteRuleExternalResp, ());
impl_resp_v2!(DeviceBindRuleExternalResp, ());
impl_resp_v2!(GetRuleExternalResp, GetRuleExternalData);
impl_resp_v2!(GetAccessRecordAccessPhotoResp, AccessRecordAccessPhotoData);
impl_resp_v2!(GetUserFaceResp, UserFaceData);
impl_resp_v2!(UpdateUserFaceResp, ());
impl_resp_v2!(CreateVisitorResp, VisitorData);
impl_resp_v2!(DeleteVisitorResp, ());
impl_resp_v2!(ListVisitorResp, VisitorListData);

// ── Resources ──

pub struct AcsUserResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchUserQuery<'a> {
    pub user_id: &'a str,
    pub body: &'a User,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchUserQuery<'a> {
    pub fn new(user_id: &'a str, body: &'a User) -> Self {
        Self {
            user_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> AcsUserResource<'a> {
    pub async fn get(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetAcsUserResp, LarkError> {
        let query = GetUserQuery::new(user_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetAcsUserResp, LarkError> {
        let path = format!("/open-apis/acs/v1/users/{}", query.user_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<AcsUserData, GetAcsUserResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAcsUserResp, LarkError> {
        let query = ListUserQuery::new()
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAcsUserResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/acs/v1/users",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_response::<AcsUserListData, ListAcsUserResp>()
        .await
    }

    pub async fn patch(
        &self,
        user_id: &str,
        body: &User,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = PatchUserQuery::new(user_id, body).user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchUserQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/acs/v1/users/{}", query.user_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_empty()
        .await
    }
}

pub struct AccessRecordResource<'a> {
    config: &'a Config,
}

impl<'a> AccessRecordResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        from: Option<i64>,
        to: Option<i64>,
        device_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAccessRecordResp, LarkError> {
        let query = ListAccessRecordQuery::new()
            .page(PageQuery::from_parts(page_size, page_token))
            .from(from)
            .to(to)
            .device_id(device_id)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAccessRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAccessRecordResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/acs/v1/access_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("from", query.from)
        .query("to", query.to)
        .query("device_id", query.device_id)
        .query("user_id_type", query.user_id_type)
        .send_response::<AccessRecordListData, ListAccessRecordResp>()
        .await
    }
}

pub struct DeviceResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListDeviceQuery;

impl ListDeviceQuery {
    pub fn new() -> Self {
        Self
    }
}

impl<'a> DeviceResource<'a> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListDeviceResp, LarkError> {
        let query = ListDeviceQuery::new();
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        _query: &ListDeviceQuery,
        option: &RequestOption,
    ) -> Result<ListDeviceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/acs/v1/devices",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<DeviceListData, ListDeviceResp>()
        .await
    }
}

// ── RuleExternal resource ──

pub struct RuleExternalResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateRuleExternalQuery<'a> {
    pub body: &'a CreateRuleExternalReqBody,
    pub rule_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateRuleExternalQuery<'a> {
    pub fn new(body: &'a CreateRuleExternalReqBody) -> Self {
        Self {
            body,
            rule_id: None,
            user_id_type: None,
        }
    }

    pub fn rule_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.rule_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteRuleExternalQuery<'a> {
    pub rule_id: &'a str,
}

impl<'a> DeleteRuleExternalQuery<'a> {
    pub fn new(rule_id: &'a str) -> Self {
        Self { rule_id }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeviceBindRuleExternalQuery<'a> {
    pub body: &'a DeviceBindRuleExternalReqBody,
}

impl<'a> DeviceBindRuleExternalQuery<'a> {
    pub fn new(body: &'a DeviceBindRuleExternalReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct GetRuleExternalQuery<'a> {
    pub device_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetRuleExternalQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn device_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.device_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl RuleExternalResource<'_> {
    pub async fn create(
        &self,
        body: &CreateRuleExternalReqBody,
        rule_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateRuleExternalResp, LarkError> {
        let query = CreateRuleExternalQuery::new(body)
            .rule_id(rule_id)
            .user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateRuleExternalQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateRuleExternalResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/acs/v1/rule_external",
            vec![AccessTokenType::User],
            option,
        )
        .query("rule_id", query.rule_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<CreateRuleExternalData, CreateRuleExternalResp>()
        .await
    }

    pub async fn delete(
        &self,
        rule_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteRuleExternalResp, LarkError> {
        let query = DeleteRuleExternalQuery::new(rule_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteRuleExternalQuery<'_>,
        option: &RequestOption,
    ) -> Result<DeleteRuleExternalResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/acs/v1/rule_external",
            vec![AccessTokenType::User],
            option,
        )
        .query("rule_id", query.rule_id)
        .send_v2_response::<(), DeleteRuleExternalResp>()
        .await
    }

    pub async fn device_bind(
        &self,
        body: &DeviceBindRuleExternalReqBody,
        option: &RequestOption,
    ) -> Result<DeviceBindRuleExternalResp, LarkError> {
        let query = DeviceBindRuleExternalQuery::new(body);
        self.device_bind_by_query(&query, option).await
    }

    pub async fn device_bind_by_query(
        &self,
        query: &DeviceBindRuleExternalQuery<'_>,
        option: &RequestOption,
    ) -> Result<DeviceBindRuleExternalResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/acs/v1/rule_external/device_bind",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), DeviceBindRuleExternalResp>()
        .await
    }

    pub async fn get(
        &self,
        device_id: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetRuleExternalResp, LarkError> {
        let query = GetRuleExternalQuery::new()
            .device_id(device_id)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetRuleExternalQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetRuleExternalResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/acs/v1/rule_external",
            vec![AccessTokenType::User],
            option,
        )
        .query("device_id", query.device_id)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetRuleExternalData, GetRuleExternalResp>()
        .await
    }
}

// ── AccessRecordAccessPhoto resource ──

pub struct AccessRecordAccessPhotoResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetAccessRecordAccessPhotoQuery<'a> {
    pub access_record_id: &'a str,
}

impl<'a> GetAccessRecordAccessPhotoQuery<'a> {
    pub fn new(access_record_id: &'a str) -> Self {
        Self { access_record_id }
    }
}

impl AccessRecordAccessPhotoResource<'_> {
    pub async fn get(
        &self,
        access_record_id: &str,
        option: &RequestOption,
    ) -> Result<GetAccessRecordAccessPhotoResp, LarkError> {
        let query = GetAccessRecordAccessPhotoQuery::new(access_record_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetAccessRecordAccessPhotoQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetAccessRecordAccessPhotoResp, LarkError> {
        let path = format!(
            "/open-apis/acs/v1/access_records/{}/access_photo",
            query.access_record_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<AccessRecordAccessPhotoData, GetAccessRecordAccessPhotoResp>()
        .await
    }
}

// ── UserFace resource ──

pub struct UserFaceResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetUserFaceQuery<'a> {
    pub user_id: &'a str,
    pub is_cropped: Option<bool>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetUserFaceQuery<'a> {
    pub fn new(user_id: &'a str) -> Self {
        Self {
            user_id,
            is_cropped: None,
            user_id_type: None,
        }
    }

    pub fn is_cropped(mut self, value: impl Into<Option<bool>>) -> Self {
        self.is_cropped = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateUserFaceQuery<'a> {
    pub user_id: &'a str,
    pub body: &'a UpdateUserFaceReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateUserFaceQuery<'a> {
    pub fn new(user_id: &'a str, body: &'a UpdateUserFaceReqBody) -> Self {
        Self {
            user_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl UserFaceResource<'_> {
    pub async fn get(
        &self,
        user_id: &str,
        is_cropped: Option<bool>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetUserFaceResp, LarkError> {
        let query = GetUserFaceQuery::new(user_id)
            .is_cropped(is_cropped)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetUserFaceQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetUserFaceResp, LarkError> {
        let path = format!("/open-apis/acs/v1/users/{}/face", query.user_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("is_cropped", query.is_cropped)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<UserFaceData, GetUserFaceResp>()
        .await
    }

    pub async fn update(
        &self,
        user_id: &str,
        body: &UpdateUserFaceReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateUserFaceResp, LarkError> {
        let query = UpdateUserFaceQuery::new(user_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateUserFaceQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateUserFaceResp, LarkError> {
        let path = format!("/open-apis/acs/v1/users/{}/face", query.user_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), UpdateUserFaceResp>()
        .await
    }
}

// ── Visitor resource ──

pub struct VisitorResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateVisitorQuery<'a> {
    pub body: &'a CreateVisitorReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateVisitorQuery<'a> {
    pub fn new(body: &'a CreateVisitorReqBody) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteVisitorQuery<'a> {
    pub visitor_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> DeleteVisitorQuery<'a> {
    pub fn new(visitor_id: &'a str) -> Self {
        Self {
            visitor_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl VisitorResource<'_> {
    pub async fn create(
        &self,
        body: &CreateVisitorReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateVisitorResp, LarkError> {
        let query = CreateVisitorQuery::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateVisitorQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateVisitorResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/acs/v1/visitors",
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<VisitorData, CreateVisitorResp>()
        .await
    }

    pub async fn delete(
        &self,
        visitor_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<DeleteVisitorResp, LarkError> {
        let query = DeleteVisitorQuery::new(visitor_id).user_id_type(user_id_type);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteVisitorQuery<'_>,
        option: &RequestOption,
    ) -> Result<DeleteVisitorResp, LarkError> {
        let path = format!("/open-apis/acs/v1/visitors/{}", query.visitor_id);
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<(), DeleteVisitorResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub user: AcsUserResource<'a>,
    pub access_record: AccessRecordResource<'a>,
    pub access_record_access_photo: AccessRecordAccessPhotoResource<'a>,
    pub device: DeviceResource<'a>,
    pub rule_external: RuleExternalResource<'a>,
    pub user_face: UserFaceResource<'a>,
    pub visitor: VisitorResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            user: AcsUserResource { config },
            access_record: AccessRecordResource { config },
            access_record_access_photo: AccessRecordAccessPhotoResource { config },
            device: DeviceResource { config },
            rule_external: RuleExternalResource { config },
            user_face: UserFaceResource { config },
            visitor: VisitorResource { config },
        }
    }
}
