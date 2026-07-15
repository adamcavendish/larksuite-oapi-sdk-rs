use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{FormDataBuilder, FormDataField, RequestOption};
use crate::service::common::RestRequest;
use crate::service::go_v397::GoV397;

impl_resp_v2!(CreateAppAvatarUploadResp, CreateAppAvatarUploadRespData);

impl_resp_v2!(PatchApplicationAbilityResp, ());

impl_resp_v2!(PatchApplicationBaseResp, ());

impl_resp_v2!(PatchApplicationConfigResp, ());

impl_resp_v2!(
    CreateApplicationPublishResp,
    CreateApplicationPublishRespData
);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAppAvatarUploadRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateApplicationPublishRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppAbilityWeb {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_new_page_open_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppAbilityBot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_card_callback_url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub i18ns: Vec<AppAbilityBotI18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppAbilityBotI18n {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_started_desc: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppI18nInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_use: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigScope {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_scopes: Vec<AppConfigScopeItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_scopes: Vec<AppConfigScopeItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigScopeItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_events: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_events: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigSecurity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<AppConfigSecurityItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<AppConfigSecurityItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_refresh_token: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigSecurityItem {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub redirect_urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_ips: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub h5_trusted_domains: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub web_view_trusted_domains: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_schemas: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_server_domains: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigVisibility {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible_to_all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_list: Option<AppVisibilityIdList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppVisibilityIdList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub department_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigContactsRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts_range_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_list: Option<AppContactsRangeIdList>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppContactsRangeIdList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub department_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventAndCallbackEncryptStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppConfigCallback {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_callbacks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove_callbacks: Vec<String>,
}
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreateAppAvatarUploadQuery {
    pub body: Vec<FormDataField>,
}

impl CreateAppAvatarUploadQuery {
    pub fn new(body: Vec<FormDataField>) -> Self {
        Self { body }
    }

    pub fn avatar(
        filename: impl Into<String>,
        data: Vec<u8>,
        content_type: Option<impl Into<String>>,
    ) -> Self {
        Self {
            body: FormDataBuilder::new()
                .file("avatar", filename, data, content_type)
                .build(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct PatchApplicationAbilityReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app: Option<AppAbilityWeb>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<AppAbilityBot>,
}

impl PatchApplicationAbilityReqBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn web_app(mut self, value: AppAbilityWeb) -> Self {
        self.web_app = Some(value);
        self
    }

    pub fn bot(mut self, value: AppAbilityBot) -> Self {
        self.bot = Some(value);
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationAbilityQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a PatchApplicationAbilityReqBody,
}

impl<'a> PatchApplicationAbilityQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a PatchApplicationAbilityReqBody) -> Self {
        Self { app_id, body }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct PatchApplicationBaseReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18ns: Option<Vec<AppI18nInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage_url: Option<String>,
}

impl PatchApplicationBaseReqBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn i18ns(mut self, value: Vec<AppI18nInfo>) -> Self {
        self.i18ns = Some(value);
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    pub fn homepage_url(mut self, value: impl Into<String>) -> Self {
        self.homepage_url = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationBaseQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a PatchApplicationBaseReqBody,
}

impl<'a> PatchApplicationBaseQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a PatchApplicationBaseReqBody) -> Self {
        Self { app_id, body }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct PatchApplicationConfigReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<AppConfigScope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<AppConfigEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<AppConfigSecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AppConfigVisibility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<AppConfigContactsRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_and_callback_encrypt_strategy: Option<EventAndCallbackEncryptStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback: Option<AppConfigCallback>,
}

impl PatchApplicationConfigReqBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scope(mut self, value: AppConfigScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn event(mut self, value: AppConfigEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn security(mut self, value: AppConfigSecurity) -> Self {
        self.security = Some(value);
        self
    }

    pub fn visibility(mut self, value: AppConfigVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn contacts(mut self, value: AppConfigContactsRange) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn event_and_callback_encrypt_strategy(
        mut self,
        value: EventAndCallbackEncryptStrategy,
    ) -> Self {
        self.event_and_callback_encrypt_strategy = Some(value);
        self
    }

    pub fn callback(mut self, value: AppConfigCallback) -> Self {
        self.callback = Some(value);
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchApplicationConfigQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a PatchApplicationConfigReqBody,
    pub department_id_type: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchApplicationConfigQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a PatchApplicationConfigReqBody) -> Self {
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

#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct CreateApplicationPublishReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_default_ability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_default_ability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl CreateApplicationPublishReqBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mobile_default_ability(mut self, value: impl Into<String>) -> Self {
        self.mobile_default_ability = Some(value.into());
        self
    }

    pub fn pc_default_ability(mut self, value: impl Into<String>) -> Self {
        self.pc_default_ability = Some(value.into());
        self
    }

    pub fn remark(mut self, value: impl Into<String>) -> Self {
        self.remark = Some(value.into());
        self
    }

    pub fn changelog(mut self, value: impl Into<String>) -> Self {
        self.changelog = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateApplicationPublishQuery<'a> {
    pub app_id: &'a str,
    pub body: &'a CreateApplicationPublishReqBody,
}

impl<'a> CreateApplicationPublishQuery<'a> {
    pub fn new(app_id: &'a str, body: &'a CreateApplicationPublishReqBody) -> Self {
        Self { app_id, body }
    }
}

pub struct AppAvatarUploadResource<'a> {
    config: &'a Config,
}

impl<'a> AppAvatarUploadResource<'a> {
    /// Upload app avatar — POST /open-apis/application/v7/app_avatar/upload
    pub async fn create(
        &self,
        body: Vec<FormDataField>,
        option: &RequestOption,
    ) -> Result<CreateAppAvatarUploadResp, LarkError> {
        self.create_by_query(&CreateAppAvatarUploadQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateAppAvatarUploadQuery,
        option: &RequestOption,
    ) -> Result<CreateAppAvatarUploadResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/application/v7/app_avatar/upload",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .form_body(query.body.clone())
        .send_v2_response::<CreateAppAvatarUploadRespData, CreateAppAvatarUploadResp>()
        .await
    }
}

pub struct ApplicationAbilityResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationAbilityResource<'a> {
    /// Patch application ability — PATCH /open-apis/application/v7/applications/{app_id}/ability
    pub async fn patch(
        &self,
        app_id: &str,
        body: &PatchApplicationAbilityReqBody,
        option: &RequestOption,
    ) -> Result<PatchApplicationAbilityResp, LarkError> {
        self.patch_by_query(&PatchApplicationAbilityQuery::new(app_id, body), option)
            .await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationAbilityQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationAbilityResp, LarkError> {
        let path = format!(
            "/open-apis/application/v7/applications/{}/ability",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationAbilityResp>()
        .await
    }
}

pub struct ApplicationBaseResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationBaseResource<'a> {
    /// Patch application base — PATCH /open-apis/application/v7/applications/{app_id}/base
    pub async fn patch(
        &self,
        app_id: &str,
        body: &PatchApplicationBaseReqBody,
        option: &RequestOption,
    ) -> Result<PatchApplicationBaseResp, LarkError> {
        self.patch_by_query(&PatchApplicationBaseQuery::new(app_id, body), option)
            .await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationBaseQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationBaseResp, LarkError> {
        let path = format!(
            "/open-apis/application/v7/applications/{}/base",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationBaseResp>()
        .await
    }
}

pub struct ApplicationConfigResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationConfigResource<'a> {
    /// Patch application config — PATCH /open-apis/application/v7/applications/{app_id}/config
    pub async fn patch(
        &self,
        app_id: &str,
        body: &PatchApplicationConfigReqBody,
        department_id_type: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchApplicationConfigResp, LarkError> {
        let query = PatchApplicationConfigQuery::new(app_id, body)
            .department_id_type(department_id_type)
            .user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchApplicationConfigQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchApplicationConfigResp, LarkError> {
        let path = format!(
            "/open-apis/application/v7/applications/{}/config",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("department_id_type", query.department_id_type)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<(), PatchApplicationConfigResp>()
        .await
    }
}

pub struct ApplicationPublishResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationPublishResource<'a> {
    /// Create application publish — POST /open-apis/application/v7/applications/{app_id}/publish
    pub async fn create(
        &self,
        app_id: &str,
        body: &CreateApplicationPublishReqBody,
        option: &RequestOption,
    ) -> Result<CreateApplicationPublishResp, LarkError> {
        self.create_by_query(&CreateApplicationPublishQuery::new(app_id, body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateApplicationPublishQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateApplicationPublishResp, LarkError> {
        let path = format!(
            "/open-apis/application/v7/applications/{}/publish",
            query.app_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<CreateApplicationPublishRespData, CreateApplicationPublishResp>()
        .await
    }
}

pub struct V7<'a> {
    pub app_avatar_upload: AppAvatarUploadResource<'a>,
    pub application_ability: ApplicationAbilityResource<'a>,
    pub application_base: ApplicationBaseResource<'a>,
    pub application_config: ApplicationConfigResource<'a>,
    pub application_publish: ApplicationPublishResource<'a>,
    config: &'a Config,
}

impl<'a> V7<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app_avatar_upload: AppAvatarUploadResource { config },
            application_ability: ApplicationAbilityResource { config },
            application_base: ApplicationBaseResource { config },
            application_config: ApplicationConfigResource { config },
            application_publish: ApplicationPublishResource { config },
            config,
        }
    }

    pub fn go_v397(&self) -> GoV397<'a> {
        GoV397::new(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_patch_builders_assign_every_supported_branch() {
        let ability = PatchApplicationAbilityReqBody::new()
            .web_app(AppAbilityWeb {
                enable: Some(true),
                ..Default::default()
            })
            .bot(AppAbilityBot {
                enable: Some(true),
                ..Default::default()
            });
        let base = PatchApplicationBaseReqBody::new().i18ns(vec![AppI18nInfo {
            i18n_key: Some("en_us".into()),
            ..Default::default()
        }]);
        let config = PatchApplicationConfigReqBody::new()
            .scope(AppConfigScope::default())
            .visibility(AppConfigVisibility::default())
            .callback(AppConfigCallback::default());

        assert!(ability.web_app.is_some());
        assert!(ability.bot.is_some());
        assert_eq!(base.i18ns.unwrap()[0].i18n_key.as_deref(), Some("en_us"));
        assert!(config.scope.is_some());
        assert!(config.visibility.is_some());
        assert!(config.callback.is_some());
    }
}
