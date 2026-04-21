use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::EmptyResp;
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Application {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_urls: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unaudit_version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_info: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_scene_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_categories: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVersion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_info: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_categories: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_home_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abilities: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activate_regular_users: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activate_users: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activate_suite_users: Option<i32>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CheckUserIsInAppBlacklistReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_list: Option<Vec<String>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<Application>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppListData {
    #[serde(default)]
    pub app_list: Vec<Application>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVersionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_version: Option<AppVersion>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVersionListData {
    #[serde(default)]
    pub items: Vec<AppVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppUsageData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<AppUsage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckBlacklistData {
    #[serde(default)]
    pub user_list: Vec<String>,
    #[serde(default)]
    pub department_list: Vec<String>,
}

impl_resp!(GetAppResp, AppData);
impl_resp!(ListAppResp, AppListData);
impl_resp!(GetAppVersionResp, AppVersionData);
impl_resp!(ListAppVersionResp, AppVersionListData);
impl_resp!(GetAppUsageResp, AppUsageData);
impl_resp!(CheckBlacklistResp, CheckBlacklistData);

// ── Resources ──

pub struct AppResource<'a> {
    config: &'a Config,
}

impl<'a> AppResource<'a> {
    pub async fn get(
        &self,
        app_id: &str,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetAppResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AppData>(self.config, &api_req, option).await?;
        Ok(GetAppResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/application/v6/applications");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AppListData>(self.config, &api_req, option).await?;
        Ok(ListAppResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn check_user_admin(
        &self,
        app_id: &str,
        user_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/application/v6/applications/{app_id}/management/check_admin");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("user_id", user_id);
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

    pub async fn check_user_in_blacklist(
        &self,
        app_id: &str,
        body: &CheckUserIsInAppBlacklistReqBody,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CheckBlacklistResp> {
        let path = format!(
            "/open-apis/application/v6/applications/{app_id}/visibility/check_white_black_list"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CheckBlacklistData>(self.config, &api_req, option).await?;
        Ok(CheckBlacklistResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppVersionResource<'a> {
    config: &'a Config,
}

impl<'a> AppVersionResource<'a> {
    pub async fn get(
        &self,
        app_id: &str,
        version_id: &str,
        lang: &str,
        option: &RequestOption,
    ) -> Result<GetAppVersionResp> {
        let path =
            format!("/open-apis/application/v6/applications/{app_id}/app_versions/{version_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("lang", lang);
        let (api_resp, raw) =
            transport::request_typed::<AppVersionData>(self.config, &api_req, option).await?;
        Ok(GetAppVersionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        app_id: &str,
        lang: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        order: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListAppVersionResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/app_versions");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("lang", lang);
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = order {
            api_req.query_params.set("order", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<AppVersionListData>(self.config, &api_req, option).await?;
        Ok(ListAppVersionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppUsageResource<'a> {
    config: &'a Config,
}

impl<'a> AppUsageResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn department_overview(
        &self,
        app_id: &str,
        date: &str,
        cycle_type: i32,
        department_id: Option<&str>,
        recursion: Option<i32>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetAppUsageResp> {
        let path = format!(
            "/open-apis/application/v6/applications/{app_id}/app_usage/department_overview"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("date", date);
        api_req
            .query_params
            .set("cycle_type", cycle_type.to_string());
        if let Some(v) = department_id {
            api_req.query_params.set("department_id", v);
        }
        if let Some(v) = recursion {
            api_req.query_params.set("recursion", v.to_string());
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<AppUsageData>(self.config, &api_req, option).await?;
        Ok(GetAppUsageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn overview(
        &self,
        app_id: &str,
        date: &str,
        cycle_type: i32,
        option: &RequestOption,
    ) -> Result<GetAppUsageResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/app_usage/overview");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.query_params.set("date", date);
        api_req
            .query_params
            .set("cycle_type", cycle_type.to_string());
        let (api_resp, raw) =
            transport::request_typed::<AppUsageData>(self.config, &api_req, option).await?;
        Ok(GetAppUsageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub app: AppResource<'a>,
    pub app_version: AppVersionResource<'a>,
    pub app_usage: AppUsageResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app: AppResource { config },
            app_version: AppVersionResource { config },
            app_usage: AppUsageResource { config },
        }
    }
}
