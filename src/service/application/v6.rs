use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

// ── Response helper ──

// ── Response types ──

impl_resp_v2!(SetAppBadgeResp, ());

impl_resp_v2!(ListAppRecommendRuleResp, ListAppRecommendRuleRespData);

impl_resp_v2!(
    ContactsRangeConfigurationApplicationResp,
    ContactsRangeConfigurationApplicationRespData
);

impl_resp_v2!(GetApplicationResp, GetApplicationRespData);

impl_resp_v2!(ListApplicationResp, ListApplicationRespData);

impl_resp_v2!(PatchApplicationResp, ());

impl_resp_v2!(
    UnderauditlistApplicationResp,
    UnderauditlistApplicationRespData
);

impl_resp_v2!(
    DepartmentOverviewApplicationAppUsageResp,
    DepartmentOverviewApplicationAppUsageRespData
);

impl_resp_v2!(
    MessagePushOverviewApplicationAppUsageResp,
    MessagePushOverviewApplicationAppUsageRespData
);

impl_resp_v2!(
    OverviewApplicationAppUsageResp,
    OverviewApplicationAppUsageRespData
);

impl_resp_v2!(
    ContactsRangeSuggestApplicationAppVersionResp,
    ContactsRangeSuggestApplicationAppVersionRespData
);

impl_resp_v2!(
    GetApplicationAppVersionResp,
    GetApplicationAppVersionRespData
);

impl_resp_v2!(
    ListApplicationAppVersionResp,
    ListApplicationAppVersionRespData
);

impl_resp_v2!(PatchApplicationAppVersionResp, ());

impl_resp_v2!(
    GetApplicationCollaboratorsResp,
    GetApplicationCollaboratorsRespData
);

impl_resp_v2!(UpdateApplicationCollaboratorsResp, ());

impl_resp_v2!(PatchApplicationContactsRangeResp, ());

impl_resp_v2!(ListApplicationFeedbackResp, ListApplicationFeedbackRespData);

impl_resp_v2!(PatchApplicationFeedbackResp, ());

impl_resp_v2!(UpdateApplicationManagementResp, ());

impl_resp_v2!(UpdateApplicationOwnerResp, ());

impl_resp_v2!(
    CheckWhiteBlackListApplicationVisibilityResp,
    CheckWhiteBlackListApplicationVisibilityRespData
);

impl_resp_v2!(PatchApplicationVisibilityResp, ());

impl_resp_v2!(ApplyScopeResp, ());

impl_resp_v2!(ListScopeResp, ListScopeRespData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListAppRecommendRuleRespData {
    #[serde(default)]
    pub rules: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactsRangeConfigurationApplicationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts_range: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListApplicationRespData {
    #[serde(default)]
    pub app_list: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnderauditlistApplicationRespData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentOverviewApplicationAppUsageRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePushOverviewApplicationAppUsageRespData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OverviewApplicationAppUsageRespData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactsRangeSuggestApplicationAppVersionRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts_range: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationAppVersionRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_version: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListApplicationAppVersionRespData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetApplicationCollaboratorsRespData {
    #[serde(default)]
    pub collaborators: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListApplicationFeedbackRespData {
    #[serde(default)]
    pub feedback_list: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckWhiteBlackListApplicationVisibilityRespData {
    #[serde(default)]
    pub user_visibility_list: Vec<serde_json::Value>,
    #[serde(default)]
    pub department_visibility_list: Vec<serde_json::Value>,
    #[serde(default)]
    pub group_visibility_list: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListScopeRespData {
    #[serde(default)]
    pub scopes: Vec<serde_json::Value>,
}
// ── Resources ──

pub struct AppBadgeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SetAppBadgeQuery<'a> {
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SetAppBadgeQuery<'a> {
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

impl<'a> AppBadgeResource<'a> {
    /// Set app badge — POST /open-apis/application/v6/app_badge/set
    pub async fn set(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SetAppBadgeResp, LarkError> {
        let query = SetAppBadgeQuery::new(body).user_id_type(user_id_type);
        self.set_by_query(&query, option).await
    }

    pub async fn set_by_query(
        &self,
        query: &SetAppBadgeQuery<'_>,
        option: &RequestOption,
    ) -> Result<SetAppBadgeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/application/v6/app_badge/set",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), SetAppBadgeResp>()
        .await
    }
}

pub struct AppRecommendRuleResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListAppRecommendRuleQuery<'a> {
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListAppRecommendRuleQuery<'a> {
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> AppRecommendRuleResource<'a> {
    /// List recommend rules — GET /open-apis/application/v6/app_recommend_rules
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppRecommendRuleResp, LarkError> {
        let query = ListAppRecommendRuleQuery {
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAppRecommendRuleQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAppRecommendRuleResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v6/app_recommend_rules",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListAppRecommendRuleRespData, ListAppRecommendRuleResp>()
        .await
    }
}

pub struct ApplicationResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ApplicationContactsRangeConfigurationQuery<'a> {
    pub app_id: &'a str,
    pub page: PageQuery<'a>,
    pub department_id_type: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ApplicationContactsRangeConfigurationQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            page: PageQuery::new(),
            department_id_type: None,
            user_id_type: None,
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

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetApplicationQuery<'a> {
    pub app_id: &'a str,
    pub lang: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetApplicationQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            lang: None,
            user_id_type: None,
        }
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub lang: Option<&'a str>,
}

impl<'a> PatchApplicationQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            lang: None,
        }
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListApplicationQuery<'a> {
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
    pub lang: Option<&'a str>,
    pub status: Option<i32>,
    pub payment_type: Option<i32>,
    pub owner_type: Option<i32>,
}

impl<'a> ListApplicationQuery<'a> {
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }

    pub fn status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn payment_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.payment_type = value.into();
        self
    }

    pub fn owner_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.owner_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct UnderauditlistApplicationQuery<'a> {
    pub page: PageQuery<'a>,
    pub lang: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UnderauditlistApplicationQuery<'a> {
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

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ApplicationResource<'a> {
    /// ContactsRangeConfiguration — GET /open-apis/application/v6/applications/:app_id/contacts_range_configuration
    #[allow(clippy::too_many_arguments)]
    pub async fn contacts_range_configuration(
        &self,
        app_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ContactsRangeConfigurationApplicationResp, LarkError> {
        let query = ApplicationContactsRangeConfigurationQuery {
            app_id,
            page: PageQuery::from_parts(page_size, page_token),
            department_id_type,
            user_id_type,
        };
        self.contacts_range_configuration_by_query(&query, option)
            .await
    }

    pub async fn contacts_range_configuration_by_query(
        &self,
        query: &ApplicationContactsRangeConfigurationQuery<'_>,
        option: &RequestOption,
    ) -> Result<ContactsRangeConfigurationApplicationResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/contacts_range_configuration",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("department_id_type", query.department_id_type)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ContactsRangeConfigurationApplicationRespData, ContactsRangeConfigurationApplicationResp>()
        .await
    }

    /// Get application info — GET /open-apis/application/v6/applications/:app_id
    pub async fn get(
        &self,
        app_id: &str,
        lang: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationResp, LarkError> {
        let query = GetApplicationQuery::new(app_id)
            .lang(lang)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetApplicationResp, LarkError> {
        let path = format!("/open-apis/application/v6/applications/{}", query.app_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetApplicationRespData, GetApplicationResp>()
        .await
    }

    /// List installed applications — GET /open-apis/application/v6/applications
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        lang: Option<&str>,
        status: Option<i32>,
        payment_type: Option<i32>,
        owner_type: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListApplicationResp, LarkError> {
        let query = ListApplicationQuery {
            page: PageQuery::from_parts(page_size, page_token),
            user_id_type,
            lang,
            status,
            payment_type,
            owner_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListApplicationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v6/applications",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .query("lang", query.lang)
        .query("status", query.status)
        .query("payment_type", query.payment_type)
        .query("owner_type", query.owner_type)
        .send_v2_response::<ListApplicationRespData, ListApplicationResp>()
        .await
    }

    /// Patch application group info — PATCH /open-apis/application/v6/applications/:app_id
    pub async fn patch(
        &self,
        app_id: &str,
        lang: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationResp, LarkError> {
        let query = PatchApplicationQuery::new(app_id, body).lang(lang);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationResp, LarkError> {
        let path = format!("/open-apis/application/v6/applications/{}", query.app_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationResp>()
        .await
    }

    /// Underauditlist — GET /open-apis/application/v6/applications/underauditlist
    #[allow(clippy::too_many_arguments)]
    pub async fn underauditlist(
        &self,
        lang: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UnderauditlistApplicationResp, LarkError> {
        let query = UnderauditlistApplicationQuery {
            page: PageQuery::from_parts(page_size, page_token),
            lang,
            user_id_type,
        };
        self.underauditlist_by_query(&query, option).await
    }

    pub async fn underauditlist_by_query(
        &self,
        query: &UnderauditlistApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<UnderauditlistApplicationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v6/applications/underauditlist",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("lang", query.lang)
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<UnderauditlistApplicationRespData, UnderauditlistApplicationResp>()
        .await
    }
}

pub struct ApplicationAppUsageResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ApplicationAppUsageQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub department_id_type: Option<&'a str>,
}

impl<'a> ApplicationAppUsageQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            department_id_type: None,
        }
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }
}

pub type DepartmentOverviewApplicationAppUsageQuery<'a> = ApplicationAppUsageQuery<'a>;
pub type MessagePushOverviewApplicationAppUsageQuery<'a> = ApplicationAppUsageQuery<'a>;
pub type OverviewApplicationAppUsageQuery<'a> = ApplicationAppUsageQuery<'a>;

impl<'a> ApplicationAppUsageResource<'a> {
    /// DepartmentOverview — POST /open-apis/application/v6/applications/:app_id/app_usage/department_overview
    #[allow(clippy::too_many_arguments)]
    pub async fn department_overview(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<DepartmentOverviewApplicationAppUsageResp, LarkError> {
        let query = DepartmentOverviewApplicationAppUsageQuery::new(app_id, body)
            .department_id_type(department_id_type);
        self.department_overview_by_query(&query, option).await
    }

    pub async fn department_overview_by_query(
        &self,
        query: &DepartmentOverviewApplicationAppUsageQuery<'_>,
        option: &RequestOption,
    ) -> Result<DepartmentOverviewApplicationAppUsageResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/department_overview",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<DepartmentOverviewApplicationAppUsageRespData, DepartmentOverviewApplicationAppUsageResp>()
        .await
    }

    /// MessagePushOverview — POST /open-apis/application/v6/applications/:app_id/app_usage/message_push_overview
    pub async fn message_push_overview(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<MessagePushOverviewApplicationAppUsageResp, LarkError> {
        let query = MessagePushOverviewApplicationAppUsageQuery::new(app_id, body)
            .department_id_type(department_id_type);
        self.message_push_overview_by_query(&query, option).await
    }

    pub async fn message_push_overview_by_query(
        &self,
        query: &MessagePushOverviewApplicationAppUsageQuery<'_>,
        option: &RequestOption,
    ) -> Result<MessagePushOverviewApplicationAppUsageResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/message_push_overview",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<MessagePushOverviewApplicationAppUsageRespData, MessagePushOverviewApplicationAppUsageResp>()
        .await
    }

    /// Overview — POST /open-apis/application/v6/applications/:app_id/app_usage/overview
    pub async fn overview(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<OverviewApplicationAppUsageResp, LarkError> {
        let query = OverviewApplicationAppUsageQuery::new(app_id, body)
            .department_id_type(department_id_type);
        self.overview_by_query(&query, option).await
    }

    pub async fn overview_by_query(
        &self,
        query: &OverviewApplicationAppUsageQuery<'_>,
        option: &RequestOption,
    ) -> Result<OverviewApplicationAppUsageResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/overview",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<OverviewApplicationAppUsageRespData, OverviewApplicationAppUsageResp>()
        .await
    }
}

pub struct ApplicationAppVersionResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ContactsRangeSuggestApplicationAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub version_id: &'a str,
    pub department_id_type: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ContactsRangeSuggestApplicationAppVersionQuery<'a> {
    pub fn new(app_id: &'a str, version_id: &'a str) -> Self {
        Self {
            app_id,
            version_id,
            department_id_type: None,
            user_id_type: None,
        }
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetApplicationAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub version_id: &'a str,
    pub lang: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetApplicationAppVersionQuery<'a> {
    pub fn new(app_id: &'a str, version_id: &'a str) -> Self {
        Self {
            app_id,
            version_id,
            lang: None,
            user_id_type: None,
        }
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListApplicationAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub page: PageQuery<'a>,
    pub lang: Option<&'a str>,
    pub order: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListApplicationAppVersionQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            page: PageQuery::new(),
            lang: None,
            order: None,
            user_id_type: None,
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

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }

    pub fn order(mut self, value: impl Into<Option<i32>>) -> Self {
        self.order = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationAppVersionQuery<'a> {
    pub app_id: &'a str,
    pub version_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
    pub operator_id: Option<&'a str>,
    pub reject_reason: Option<&'a str>,
}

impl<'a> PatchApplicationAppVersionQuery<'a> {
    pub fn new(app_id: &'a str, version_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            version_id,
            body,
            user_id_type: None,
            operator_id: None,
            reject_reason: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn operator_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.operator_id = value.into();
        self
    }

    pub fn reject_reason(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.reject_reason = value.into();
        self
    }
}

impl<'a> ApplicationAppVersionResource<'a> {
    /// ContactsRangeSuggest — GET /open-apis/application/v6/applications/:app_id/app_versions/:version_id/contacts_range_suggest
    pub async fn contacts_range_suggest(
        &self,
        app_id: &str,
        version_id: &str,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ContactsRangeSuggestApplicationAppVersionResp, LarkError> {
        let query = ContactsRangeSuggestApplicationAppVersionQuery::new(app_id, version_id)
            .department_id_type(department_id_type)
            .user_id_type(user_id_type);
        self.contacts_range_suggest_by_query(&query, option).await
    }

    pub async fn contacts_range_suggest_by_query(
        &self,
        query: &ContactsRangeSuggestApplicationAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ContactsRangeSuggestApplicationAppVersionResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions/{}/contacts_range_suggest",
            query.app_id, query.version_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ContactsRangeSuggestApplicationAppVersionRespData, ContactsRangeSuggestApplicationAppVersionResp>()
        .await
    }

    /// Get app version — GET /open-apis/application/v6/applications/:app_id/app_versions/:version_id
    pub async fn get(
        &self,
        app_id: &str,
        version_id: &str,
        lang: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationAppVersionResp, LarkError> {
        let query = GetApplicationAppVersionQuery::new(app_id, version_id)
            .lang(lang)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetApplicationAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetApplicationAppVersionResp, LarkError> {
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
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetApplicationAppVersionRespData, GetApplicationAppVersionResp>()
        .await
    }

    /// List app versions — GET /open-apis/application/v6/applications/:app_id/app_versions
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        app_id: &str,
        lang: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        order: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListApplicationAppVersionResp, LarkError> {
        let query = ListApplicationAppVersionQuery {
            app_id,
            page: PageQuery::from_parts(page_size, page_token),
            lang,
            order,
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListApplicationAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListApplicationAppVersionResp, LarkError> {
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
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListApplicationAppVersionRespData, ListApplicationAppVersionResp>()
        .await
    }

    /// Patch app version audit status — PATCH /open-apis/application/v6/applications/:app_id/app_versions/:version_id
    #[allow(clippy::too_many_arguments)]
    pub async fn patch(
        &self,
        app_id: &str,
        version_id: &str,
        user_id_type: Option<&str>,
        operator_id: Option<&str>,
        reject_reason: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationAppVersionResp, LarkError> {
        let query = PatchApplicationAppVersionQuery::new(app_id, version_id, body)
            .user_id_type(user_id_type)
            .operator_id(operator_id)
            .reject_reason(reject_reason);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationAppVersionQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationAppVersionResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_versions/{}",
            query.app_id, query.version_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("operator_id", query.operator_id)
        .query("reject_reason", query.reject_reason)
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationAppVersionResp>()
        .await
    }
}

pub struct ApplicationCollaboratorsResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetApplicationCollaboratorsQuery<'a> {
    pub app_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetApplicationCollaboratorsQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
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
pub struct UpdateApplicationCollaboratorsQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateApplicationCollaboratorsQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ApplicationCollaboratorsResource<'a> {
    /// Get collaborators — GET /open-apis/application/v6/applications/:app_id/collaborators
    pub async fn get(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationCollaboratorsResp, LarkError> {
        let query = GetApplicationCollaboratorsQuery::new(app_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetApplicationCollaboratorsQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetApplicationCollaboratorsResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/collaborators",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetApplicationCollaboratorsRespData, GetApplicationCollaboratorsResp>()
        .await
    }

    /// Update collaborators — PUT /open-apis/application/v6/applications/:app_id/collaborators
    pub async fn update(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateApplicationCollaboratorsResp, LarkError> {
        let query =
            UpdateApplicationCollaboratorsQuery::new(app_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateApplicationCollaboratorsQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateApplicationCollaboratorsResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/collaborators",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), UpdateApplicationCollaboratorsResp>()
        .await
    }
}

pub struct ApplicationContactsRangeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationContactsRangeQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> PatchApplicationContactsRangeQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
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

impl<'a> ApplicationContactsRangeResource<'a> {
    /// Patch contacts range — PATCH /open-apis/application/v6/applications/:app_id/contacts_range
    pub async fn patch(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationContactsRangeResp, LarkError> {
        let query = PatchApplicationContactsRangeQuery::new(app_id, body)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationContactsRangeQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationContactsRangeResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/contacts_range",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("department_id_type", query.department_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationContactsRangeResp>()
        .await
    }
}

pub struct ApplicationFeedbackResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListApplicationFeedbackQuery<'a> {
    pub app_id: &'a str,
    pub page: PageQuery<'a>,
    pub from_date: Option<&'a str>,
    pub to_date: Option<&'a str>,
    pub feedback_type: Option<i32>,
    pub status: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListApplicationFeedbackQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            page: PageQuery::new(),
            from_date: None,
            to_date: None,
            feedback_type: None,
            status: None,
            user_id_type: None,
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

    pub fn from_date(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.from_date = value.into();
        self
    }

    pub fn to_date(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.to_date = value.into();
        self
    }

    pub fn feedback_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.feedback_type = value.into();
        self
    }

    pub fn status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationFeedbackQuery<'a> {
    pub app_id: &'a str,
    pub feedback_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub status: Option<i32>,
    pub operator_id: Option<&'a str>,
}

impl<'a> PatchApplicationFeedbackQuery<'a> {
    pub fn new(app_id: &'a str, feedback_id: &'a str) -> Self {
        Self {
            app_id,
            feedback_id,
            user_id_type: None,
            status: None,
            operator_id: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn operator_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.operator_id = value.into();
        self
    }
}

impl<'a> ApplicationFeedbackResource<'a> {
    /// List feedbacks — GET /open-apis/application/v6/applications/:app_id/feedbacks
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        app_id: &str,
        from_date: Option<&str>,
        to_date: Option<&str>,
        feedback_type: Option<i32>,
        status: Option<i32>,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListApplicationFeedbackResp, LarkError> {
        let query = ListApplicationFeedbackQuery {
            app_id,
            page: PageQuery::from_parts(page_size, page_token),
            from_date,
            to_date,
            feedback_type,
            status,
            user_id_type,
        };
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListApplicationFeedbackQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListApplicationFeedbackResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/feedbacks",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("from_date", query.from_date)
        .query("to_date", query.to_date)
        .query("feedback_type", query.feedback_type)
        .query("status", query.status)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2_response::<ListApplicationFeedbackRespData, ListApplicationFeedbackResp>()
        .await
    }

    /// Patch feedback — PATCH /open-apis/application/v6/applications/:app_id/feedbacks/:feedback_id
    #[allow(clippy::too_many_arguments)]
    pub async fn patch(
        &self,
        app_id: &str,
        feedback_id: &str,
        user_id_type: Option<&str>,
        status: Option<i32>,
        operator_id: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchApplicationFeedbackResp, LarkError> {
        let query = PatchApplicationFeedbackQuery::new(app_id, feedback_id)
            .user_id_type(user_id_type)
            .status(status)
            .operator_id(operator_id);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationFeedbackQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationFeedbackResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/feedbacks/{}",
            query.app_id, query.feedback_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query("status", query.status)
        .query("operator_id", query.operator_id)
        .send_v2_response::<(), PatchApplicationFeedbackResp>()
        .await
    }
}

pub struct ApplicationManagementResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateApplicationManagementQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> UpdateApplicationManagementQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { app_id, body }
    }
}

impl<'a> ApplicationManagementResource<'a> {
    /// Update management — PUT /open-apis/application/v6/applications/:app_id/management
    pub async fn update(
        &self,
        app_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateApplicationManagementResp, LarkError> {
        self.update_by_query(&UpdateApplicationManagementQuery::new(app_id, body), option)
            .await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateApplicationManagementQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateApplicationManagementResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/management",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), UpdateApplicationManagementResp>()
        .await
    }
}

pub struct ApplicationOwnerResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateApplicationOwnerQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateApplicationOwnerQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ApplicationOwnerResource<'a> {
    /// Update owner — PUT /open-apis/application/v6/applications/:app_id/owner
    pub async fn update(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateApplicationOwnerResp, LarkError> {
        let query = UpdateApplicationOwnerQuery::new(app_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateApplicationOwnerQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateApplicationOwnerResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/owner",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), UpdateApplicationOwnerResp>()
        .await
    }
}

pub struct ApplicationVisibilityResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CheckWhiteBlackListApplicationVisibilityQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
    pub department_id_type: Option<&'a str>,
}

impl<'a> CheckWhiteBlackListApplicationVisibilityQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
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

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationVisibilityQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a serde_json::Value,
    pub department_id_type: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchApplicationVisibilityQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            body,
            department_id_type: None,
            user_id_type: None,
        }
    }

    pub fn department_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.department_id_type = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> ApplicationVisibilityResource<'a> {
    /// CheckWhiteBlackList — POST /open-apis/application/v6/applications/:app_id/visibility/check_white_black_list
    pub async fn check_white_black_list(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CheckWhiteBlackListApplicationVisibilityResp, LarkError> {
        let query = CheckWhiteBlackListApplicationVisibilityQuery::new(app_id, body)
            .user_id_type(user_id_type)
            .department_id_type(department_id_type);
        self.check_white_black_list_by_query(&query, option).await
    }

    pub async fn check_white_black_list_by_query(
        &self,
        query: &CheckWhiteBlackListApplicationVisibilityQuery<'_>,
        option: &RequestOption,
    ) -> Result<CheckWhiteBlackListApplicationVisibilityResp, LarkError> {
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
        .send_v2_response::<CheckWhiteBlackListApplicationVisibilityRespData, CheckWhiteBlackListApplicationVisibilityResp>()
        .await
    }

    /// Patch visibility — PATCH /open-apis/application/v6/applications/:app_id/visibility
    pub async fn patch(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationVisibilityResp, LarkError> {
        let query = PatchApplicationVisibilityQuery::new(app_id, body)
            .department_id_type(department_id_type)
            .user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationVisibilityQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationVisibilityResp, LarkError> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/visibility",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationVisibilityResp>()
        .await
    }
}

pub struct ScopeResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ApplyScopeQuery;

impl ApplyScopeQuery {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListScopeQuery;

impl ListScopeQuery {
    pub fn new() -> Self {
        Self
    }
}

impl<'a> ScopeResource<'a> {
    /// Apply scope — POST /open-apis/application/v6/scopes/apply
    pub async fn apply(&self, option: &RequestOption) -> Result<ApplyScopeResp, LarkError> {
        self.apply_by_query(&ApplyScopeQuery::new(), option).await
    }

    pub async fn apply_by_query(
        &self,
        _query: &ApplyScopeQuery,
        option: &RequestOption,
    ) -> Result<ApplyScopeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/application/v6/scopes/apply",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), ApplyScopeResp>()
        .await
    }

    /// List scopes — GET /open-apis/application/v6/scopes
    pub async fn list(&self, option: &RequestOption) -> Result<ListScopeResp, LarkError> {
        self.list_by_query(&ListScopeQuery::new(), option).await
    }

    pub async fn list_by_query(
        &self,
        _query: &ListScopeQuery,
        option: &RequestOption,
    ) -> Result<ListScopeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v6/scopes",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<ListScopeRespData, ListScopeResp>()
        .await
    }
}

// ── Version struct ──

pub struct V6<'a> {
    pub app_badge: AppBadgeResource<'a>,
    pub app_recommend_rule: AppRecommendRuleResource<'a>,
    pub application: ApplicationResource<'a>,
    pub application_app_usage: ApplicationAppUsageResource<'a>,
    pub application_app_version: ApplicationAppVersionResource<'a>,
    pub application_collaborators: ApplicationCollaboratorsResource<'a>,
    pub application_contacts_range: ApplicationContactsRangeResource<'a>,
    pub application_feedback: ApplicationFeedbackResource<'a>,
    pub application_management: ApplicationManagementResource<'a>,
    pub application_owner: ApplicationOwnerResource<'a>,
    pub application_visibility: ApplicationVisibilityResource<'a>,
    pub scope: ScopeResource<'a>,
}

impl<'a> V6<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app_badge: AppBadgeResource { config },
            app_recommend_rule: AppRecommendRuleResource { config },
            application: ApplicationResource { config },
            application_app_usage: ApplicationAppUsageResource { config },
            application_app_version: ApplicationAppVersionResource { config },
            application_collaborators: ApplicationCollaboratorsResource { config },
            application_contacts_range: ApplicationContactsRangeResource { config },
            application_feedback: ApplicationFeedbackResource { config },
            application_management: ApplicationManagementResource { config },
            application_owner: ApplicationOwnerResource { config },
            application_visibility: ApplicationVisibilityResource { config },
            scope: ScopeResource { config },
        }
    }
}
