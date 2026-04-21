use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError, RawResponse};
use crate::transport;

// ── Response helper ──

fn parse_v2<T>(api_resp: ApiResp, raw: RawResponse<T>) -> (ApiResp, Option<CodeError>, Option<T>) {
    if raw.code_error.code != 0 {
        (api_resp, Some(raw.code_error), None)
    } else {
        (api_resp, None, raw.data)
    }
}

macro_rules! impl_resp_v2 {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: Option<CodeError>,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.as_ref().is_none_or(|e| e.code == 0)
            }
        }
    };
}

// ── Response types ──

impl_resp_v2!(SetAppBadgeResp, serde_json::Value);

impl_resp_v2!(ListAppRecommendRuleResp, serde_json::Value);

impl_resp_v2!(ContactsRangeConfigurationApplicationResp, serde_json::Value);

impl_resp_v2!(GetApplicationResp, serde_json::Value);

impl_resp_v2!(ListApplicationResp, serde_json::Value);

impl_resp_v2!(PatchApplicationResp, serde_json::Value);

impl_resp_v2!(UnderauditlistApplicationResp, serde_json::Value);

impl_resp_v2!(DepartmentOverviewApplicationAppUsageResp, serde_json::Value);

impl_resp_v2!(
    MessagePushOverviewApplicationAppUsageResp,
    serde_json::Value
);

impl_resp_v2!(OverviewApplicationAppUsageResp, serde_json::Value);

impl_resp_v2!(
    ContactsRangeSuggestApplicationAppVersionResp,
    serde_json::Value
);

impl_resp_v2!(GetApplicationAppVersionResp, serde_json::Value);

impl_resp_v2!(ListApplicationAppVersionResp, serde_json::Value);

impl_resp_v2!(PatchApplicationAppVersionResp, serde_json::Value);

impl_resp_v2!(GetApplicationCollaboratorsResp, serde_json::Value);

impl_resp_v2!(UpdateApplicationCollaboratorsResp, serde_json::Value);

impl_resp_v2!(PatchApplicationContactsRangeResp, serde_json::Value);

impl_resp_v2!(ListApplicationFeedbackResp, serde_json::Value);

impl_resp_v2!(PatchApplicationFeedbackResp, serde_json::Value);

impl_resp_v2!(UpdateApplicationManagementResp, serde_json::Value);

impl_resp_v2!(UpdateApplicationOwnerResp, serde_json::Value);

impl_resp_v2!(
    CheckWhiteBlackListApplicationVisibilityResp,
    serde_json::Value
);

impl_resp_v2!(PatchApplicationVisibilityResp, serde_json::Value);

impl_resp_v2!(ApplyScopeResp, serde_json::Value);

impl_resp_v2!(ListScopeResp, serde_json::Value);

// ── Resources ──

pub struct AppBadgeResource<'a> {
    config: &'a Config,
}

impl<'a> AppBadgeResource<'a> {
    /// Set app badge — POST /open-apis/application/v6/app_badge/set
    pub async fn set(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SetAppBadgeResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/application/v6/app_badge/set",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SetAppBadgeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct AppRecommendRuleResource<'a> {
    config: &'a Config,
}

impl<'a> AppRecommendRuleResource<'a> {
    /// List recommend rules — GET /open-apis/application/v6/app_recommend_rules
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppRecommendRuleResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/application/v6/app_recommend_rules",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
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
        Ok(ListAppRecommendRuleResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApplicationResource<'a> {
    config: &'a Config,
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
    ) -> Result<ContactsRangeConfigurationApplicationResp> {
        let path =
            format!("/open-apis/application/v6/applications/{app_id}/contacts_range_configuration");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ContactsRangeConfigurationApplicationResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Get application info — GET /open-apis/application/v6/applications/:app_id
    pub async fn get(
        &self,
        app_id: &str,
        lang: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetApplicationResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<ListApplicationResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/application/v6/applications");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        if let Some(v) = status {
            api_req.query_params.set("status", v.to_string());
        }
        if let Some(v) = payment_type {
            api_req.query_params.set("payment_type", v.to_string());
        }
        if let Some(v) = owner_type {
            api_req.query_params.set("owner_type", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListApplicationResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Patch application group info — PATCH /open-apis/application/v6/applications/:app_id
    pub async fn patch(
        &self,
        app_id: &str,
        lang: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchApplicationResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<UnderauditlistApplicationResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/application/v6/applications/underauditlist",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UnderauditlistApplicationResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApplicationAppUsageResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationAppUsageResource<'a> {
    /// DepartmentOverview — POST /open-apis/application/v6/applications/:app_id/app_usage/department_overview
    #[allow(clippy::too_many_arguments)]
    pub async fn department_overview(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<DepartmentOverviewApplicationAppUsageResp> {
        let path = format!(
            "/open-apis/application/v6/applications/{app_id}/app_usage/department_overview"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DepartmentOverviewApplicationAppUsageResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// MessagePushOverview — POST /open-apis/application/v6/applications/:app_id/app_usage/message_push_overview
    pub async fn message_push_overview(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<MessagePushOverviewApplicationAppUsageResp> {
        let path = format!(
            "/open-apis/application/v6/applications/{app_id}/app_usage/message_push_overview"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(MessagePushOverviewApplicationAppUsageResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Overview — POST /open-apis/application/v6/applications/:app_id/app_usage/overview
    pub async fn overview(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<OverviewApplicationAppUsageResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/app_usage/overview");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(OverviewApplicationAppUsageResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApplicationAppVersionResource<'a> {
    config: &'a Config,
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
    ) -> Result<ContactsRangeSuggestApplicationAppVersionResp> {
        let path = format!(
            "/open-apis/application/v6/applications/{app_id}/app_versions/{version_id}/contacts_range_suggest"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ContactsRangeSuggestApplicationAppVersionResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Get app version — GET /open-apis/application/v6/applications/:app_id/app_versions/:version_id
    pub async fn get(
        &self,
        app_id: &str,
        version_id: &str,
        lang: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationAppVersionResp> {
        let path =
            format!("/open-apis/application/v6/applications/{app_id}/app_versions/{version_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetApplicationAppVersionResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<ListApplicationAppVersionResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/app_versions");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = order {
            api_req.query_params.set("order", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListApplicationAppVersionResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<PatchApplicationAppVersionResp> {
        let path =
            format!("/open-apis/application/v6/applications/{app_id}/app_versions/{version_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = operator_id {
            api_req.query_params.set("operator_id", v);
        }
        if let Some(v) = reject_reason {
            api_req.query_params.set("reject_reason", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchApplicationAppVersionResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApplicationCollaboratorsResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationCollaboratorsResource<'a> {
    /// Get collaborators — GET /open-apis/application/v6/applications/:app_id/collaborators
    pub async fn get(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetApplicationCollaboratorsResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/collaborators");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetApplicationCollaboratorsResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Update collaborators — PUT /open-apis/application/v6/applications/:app_id/collaborators
    pub async fn update(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateApplicationCollaboratorsResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/collaborators");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateApplicationCollaboratorsResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApplicationContactsRangeResource<'a> {
    config: &'a Config,
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
    ) -> Result<PatchApplicationContactsRangeResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/contacts_range");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchApplicationContactsRangeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApplicationFeedbackResource<'a> {
    config: &'a Config,
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
    ) -> Result<ListApplicationFeedbackResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/feedbacks");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = from_date {
            api_req.query_params.set("from_date", v);
        }
        if let Some(v) = to_date {
            api_req.query_params.set("to_date", v);
        }
        if let Some(v) = feedback_type {
            api_req.query_params.set("feedback_type", v.to_string());
        }
        if let Some(v) = status {
            api_req.query_params.set("status", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListApplicationFeedbackResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<PatchApplicationFeedbackResp> {
        let path =
            format!("/open-apis/application/v6/applications/{app_id}/feedbacks/{feedback_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = status {
            api_req.query_params.set("status", v.to_string());
        }
        if let Some(v) = operator_id {
            api_req.query_params.set("operator_id", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchApplicationFeedbackResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApplicationManagementResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationManagementResource<'a> {
    /// Update management — PUT /open-apis/application/v6/applications/:app_id/management
    pub async fn update(
        &self,
        app_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateApplicationManagementResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/management");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateApplicationManagementResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApplicationOwnerResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationOwnerResource<'a> {
    /// Update owner — PUT /open-apis/application/v6/applications/:app_id/owner
    pub async fn update(
        &self,
        app_id: &str,
        user_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateApplicationOwnerResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/owner");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UpdateApplicationOwnerResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ApplicationVisibilityResource<'a> {
    config: &'a Config,
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
    ) -> Result<CheckWhiteBlackListApplicationVisibilityResp> {
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
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CheckWhiteBlackListApplicationVisibilityResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// Patch visibility — PATCH /open-apis/application/v6/applications/:app_id/visibility
    pub async fn patch(
        &self,
        app_id: &str,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchApplicationVisibilityResp> {
        let path = format!("/open-apis/application/v6/applications/{app_id}/visibility");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = department_id_type {
            api_req.query_params.set("department_id_type", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchApplicationVisibilityResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ScopeResource<'a> {
    config: &'a Config,
}

impl<'a> ScopeResource<'a> {
    /// Apply scope — POST /open-apis/application/v6/scopes/apply
    pub async fn apply(&self, option: &RequestOption) -> Result<ApplyScopeResp> {
        let api_req = ApiReq::new(http::Method::POST, "/open-apis/application/v6/scopes/apply");
        let mut api_req = api_req;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ApplyScopeResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// List scopes — GET /open-apis/application/v6/scopes
    pub async fn list(&self, option: &RequestOption) -> Result<ListScopeResp> {
        let api_req = ApiReq::new(http::Method::GET, "/open-apis/application/v6/scopes");
        let mut api_req = api_req;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListScopeResp {
            api_resp,
            code_error,
            data,
        })
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
