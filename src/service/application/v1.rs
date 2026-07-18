use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

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
    pub avatar_info: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_scene_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n: Option<Vec<crate::JsonValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_categories: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<crate::JsonValue>,
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
    pub avatar_info: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n: Option<Vec<crate::JsonValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_categories: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<crate::JsonValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub back_home_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abilities: Option<crate::JsonValue>,
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetAppQuery<'a> {
    pub app_id: &'a str,
    pub lang: Option<&'a str>,
}

impl<'a> GetAppQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self { app_id, lang: None }
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListAppQuery<'a> {
    pub page: PageQuery<'a>,
    pub lang: Option<&'a str>,
}

impl<'a> ListAppQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CheckUserAdminQuery<'a> {
    pub app_id: &'a str,
    pub user_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CheckUserAdminQuery<'a> {
    pub fn new(app_id: &'a str, user_id: &'a str) -> Self {
        Self {
            app_id,
            user_id,
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
pub struct CheckUserInAppBlacklistQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a CheckUserIsInAppBlacklistReqBody,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> CheckUserInAppBlacklistQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a CheckUserIsInAppBlacklistReqBody) -> Self {
        Self {
            app_id,
            body,
            user_id_type: None,
            department_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }
}

impl<'a> AppResource<'a> {
    pub async fn get(
        &self,
        app_id: &str,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetAppResp, LarkError> {
        let query = GetAppQuery::new(app_id).lang(lang);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetAppQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetAppResp, LarkError> {
        let path = format!("/open-apis/application/v6/applications/{}", query.app_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .send_response::<AppData, GetAppResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppResp, LarkError> {
        let query = ListAppQuery {
            page: PageQuery::from_parts(page_size, page_token),
            lang,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAppQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAppResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v6/applications",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("lang", query.lang)
        .send_response::<AppListData, ListAppResp>()
        .await
    }

    pub async fn check_user_admin(
        &self,
        app_id: &str,
        user_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = CheckUserAdminQuery::new(app_id, user_id).user_id_type(user_id_type);
        self.check_user_admin_by_query(&query, option).await
    }

    pub async fn check_user_admin_by_query(
        &self,
        query: &CheckUserAdminQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/management/check_admin",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id", query.user_id)
        .query("user_id_type", query.user_id_type)
        .send_empty()
        .await
    }

    pub async fn check_user_in_blacklist(
        &self,
        app_id: &str,
        body: &CheckUserIsInAppBlacklistReqBody,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CheckBlacklistResp, LarkError> {
        let query = CheckUserInAppBlacklistQuery::new(app_id, body)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.check_user_in_blacklist_by_query(&query, option).await
    }

    pub async fn check_user_in_blacklist_by_query(
        &self,
        query: &CheckUserInAppBlacklistQuery<'_>,
        option: &RequestOption,
    ) -> Result<CheckBlacklistResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/visibility/check_white_black_list",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_response::<CheckBlacklistData, CheckBlacklistResp>()
        .await
    }
}

pub struct AppVersionResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub version_id: &'a str,
    pub lang: &'a str,
}

impl<'a> GetAppVersionQuery<'a> {
    pub fn new(app_id: &'a str, version_id: &'a str, lang: &'a str) -> Self {
        Self {
            app_id,
            version_id,
            lang,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub lang: &'a str,
    pub page: PageQuery<'a>,
    pub order: Option<i32>,
}

impl<'a> ListAppVersionQuery<'a> {
    pub fn new(app_id: &'a str, lang: &'a str) -> Self {
        Self {
            app_id,
            lang,
            page: PageQuery::new(),
            order: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn order(mut self, value: impl Into<Option<i32>>) -> Self {
        self.order = value.into();
        self
    }
}

impl<'a> AppVersionResource<'a> {
    pub async fn get(
        &self,
        app_id: &str,
        version_id: &str,
        lang: &str,
        option: &RequestOption,
    ) -> Result<GetAppVersionResp, LarkError> {
        let query = GetAppVersionQuery::new(app_id, version_id, lang);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetAppVersionResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions/{}",
            query.app_id, query.version_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .send_response::<AppVersionData, GetAppVersionResp>()
        .await
    }

    pub async fn list(
        &self,
        app_id: &str,
        lang: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        order: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListAppVersionResp, LarkError> {
        let query = ListAppVersionQuery {
            app_id,
            lang,
            page: PageQuery::from_parts(page_size, page_token),
            order,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAppVersionResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .page_query(query.page)
        .query("order", query.order)
        .send_response::<AppVersionListData, ListAppVersionResp>()
        .await
    }
}

pub struct AppUsageResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DepartmentOverviewAppUsageQuery<'a> {
    pub app_id: &'a str,
    pub date: &'a str,
    pub cycle_type: i32,
    pub department_id: Option<&'a str>,
    pub recursion: Option<i32>,
    pub page: PageQuery<'a>,
}

impl<'a> DepartmentOverviewAppUsageQuery<'a> {
    pub fn new(app_id: &'a str, date: &'a str, cycle_type: i32) -> Self {
        Self {
            app_id,
            date,
            cycle_type,
            department_id: None,
            recursion: None,
            page: PageQuery::new(),
        }
    }

    pub fn department_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id = value.into();
        self
    }

    pub fn recursion(mut self, value: impl Into<Option<i32>>) -> Self {
        self.recursion = value.into();
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
pub struct OverviewAppUsageQuery<'a> {
    pub app_id: &'a str,
    pub date: &'a str,
    pub cycle_type: i32,
}

impl<'a> OverviewAppUsageQuery<'a> {
    pub fn new(app_id: &'a str, date: &'a str, cycle_type: i32) -> Self {
        Self {
            app_id,
            date,
            cycle_type,
        }
    }
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
    ) -> Result<GetAppUsageResp, LarkError> {
        let query = DepartmentOverviewAppUsageQuery {
            app_id,
            date,
            cycle_type,
            department_id,
            recursion,
            page: PageQuery::from_parts(page_size, page_token),
        };
        self.department_overview_by_query(&query, option).await
    }

    pub async fn department_overview_by_query(
        &self,
        query: &DepartmentOverviewAppUsageQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetAppUsageResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/department_overview",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("date", query.date)
        .query("cycle_type", query.cycle_type)
        .query("department_id", query.department_id)
        .query("recursion", query.recursion)
        .page_query(query.page)
        .send_response::<AppUsageData, GetAppUsageResp>()
        .await
    }

    pub async fn overview(
        &self,
        app_id: &str,
        date: &str,
        cycle_type: i32,
        option: &RequestOption,
    ) -> Result<GetAppUsageResp, LarkError> {
        let query = OverviewAppUsageQuery::new(app_id, date, cycle_type);
        self.overview_by_query(&query, option).await
    }

    pub async fn overview_by_query(
        &self,
        query: &OverviewAppUsageQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetAppUsageResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/overview",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("date", query.date)
        .query("cycle_type", query.cycle_type)
        .send_response::<AppUsageData, GetAppUsageResp>()
        .await
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
