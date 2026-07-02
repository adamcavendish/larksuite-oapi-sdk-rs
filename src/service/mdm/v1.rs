use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MdmDevice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceListData {
    #[serde(default)]
    pub items: Vec<MdmDevice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListDeviceResp, DeviceListData);

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDeviceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListUserDeviceQuery<'a> {
    pub user_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListUserDeviceQuery<'a> {
    pub fn new(user_id: &'a str) -> Self {
        Self {
            user_id,
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

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateUserDeviceQuery<'a> {
    pub device_id: &'a str,
    pub body: &'a UpdateDeviceReqBody,
}

impl<'a> UpdateUserDeviceQuery<'a> {
    pub fn new(device_id: &'a str, body: &'a UpdateDeviceReqBody) -> Self {
        Self { device_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BindUserAuthDataRelationQuery<'a> {
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BindUserAuthDataRelationQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
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
pub struct UnbindUserAuthDataRelationQuery<'a> {
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UnbindUserAuthDataRelationQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
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

// ── Resources ──

pub struct UserDeviceResource<'a> {
    config: &'a Config,
}

impl<'a> UserDeviceResource<'a> {
    pub async fn list(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDeviceResp, LarkError> {
        let query = ListUserDeviceQuery::new(user_id)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListUserDeviceQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListDeviceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/mdm/v1/user_devices",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id", query.user_id)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_response::<DeviceListData, ListDeviceResp>()
        .await
    }

    pub async fn update(
        &self,
        device_id: &str,
        body: &UpdateDeviceReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = UpdateUserDeviceQuery::new(device_id, body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateUserDeviceQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/mdm/v1/user_devices/{}", query.device_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_empty()
        .await
    }
}

pub struct UserAuthDataRelationResource<'a> {
    config: &'a Config,
}

impl<'a> UserAuthDataRelationResource<'a> {
    pub async fn bind(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = BindUserAuthDataRelationQuery::new(body).user_id_type(user_id_type);
        self.bind_by_query(&query, option).await
    }

    pub async fn bind_by_query(
        &self,
        query: &BindUserAuthDataRelationQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/mdm/v1/user_auth_data_relations/bind",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_empty()
        .await
    }

    pub async fn unbind(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = UnbindUserAuthDataRelationQuery::new(body).user_id_type(user_id_type);
        self.unbind_by_query(&query, option).await
    }

    pub async fn unbind_by_query(
        &self,
        query: &UnbindUserAuthDataRelationQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/mdm/v1/user_auth_data_relations/unbind",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_empty()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub user_device: UserDeviceResource<'a>,
    pub user_auth_data_relation: UserAuthDataRelationResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            user_device: UserDeviceResource { config },
            user_auth_data_relation: UserAuthDataRelationResource { config },
        }
    }
}
