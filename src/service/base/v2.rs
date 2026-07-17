use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_perm: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rec_rule: Option<RecRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_rec_rule: Option<OtherRecRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_perm: Option<std::collections::HashMap<String, i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_add_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_delete_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_perm: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_rules: Option<std::collections::HashMap<String, i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_action_rules:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, i32>>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockRole {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_perm: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RecRuleCondition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perm: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_perm: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_groups: Option<Vec<ConditionGroup>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_rec_rule_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OtherRecRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RecRuleCondition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perm: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecRuleCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConditionGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RecRuleCondition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_roles: Option<Vec<TableRole>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<BlockRole>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_rule: Option<std::collections::HashMap<String, i32>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<AppRoleV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppRoleListData {
    #[serde(default)]
    pub items: Vec<AppRoleV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl_resp_v2!(CreateAppRoleV2Resp, AppRoleData);
impl_resp_v2!(ListAppRoleV2Resp, AppRoleListData);
impl_resp_v2!(UpdateAppRoleV2Resp, AppRoleData);

#[derive(Debug, Clone, Copy)]
pub struct CreateAppRoleV2Query<'a> {
    pub app_token: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> CreateAppRoleV2Query<'a> {
    pub fn new(app_token: &'a str, body: &'a serde_json::Value) -> Self {
        Self { app_token, body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListAppRoleV2Query<'a> {
    pub app_token: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListAppRoleV2Query<'a> {
    pub fn new(app_token: &'a str) -> Self {
        Self {
            app_token,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateAppRoleV2Query<'a> {
    pub app_token: &'a str,
    pub role_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> UpdateAppRoleV2Query<'a> {
    pub fn new(app_token: &'a str, role_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_token,
            role_id,
            body,
        }
    }
}

pub struct V2<'a> {
    pub app_role: AppRoleV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app_role: AppRoleV2Resource { config },
        }
    }
}

pub struct AppRoleV2Resource<'a> {
    config: &'a Config,
}

impl AppRoleV2Resource<'_> {
    pub async fn create(
        &self,
        app_token: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateAppRoleV2Resp, LarkError> {
        let body = serde_json::to_value(body)?;
        self.create_by_query(&CreateAppRoleV2Query::new(app_token, &body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateAppRoleV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateAppRoleV2Resp, LarkError> {
        let path = format!("/open-apis/base/v2/apps/{}/roles", query.app_token);
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<AppRoleData, CreateAppRoleV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        app_token: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppRoleV2Resp, LarkError> {
        let query =
            ListAppRoleV2Query::new(app_token).page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAppRoleV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListAppRoleV2Resp, LarkError> {
        let path = format!("/open-apis/base/v2/apps/{}/roles", query.app_token);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_v2_response::<AppRoleListData, ListAppRoleV2Resp>()
        .await
    }

    pub async fn update(
        &self,
        app_token: &str,
        role_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateAppRoleV2Resp, LarkError> {
        let body = serde_json::to_value(body)?;
        self.update_by_query(
            &UpdateAppRoleV2Query::new(app_token, role_id, &body),
            option,
        )
        .await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateAppRoleV2Query<'_>,
        option: &RequestOption,
    ) -> Result<UpdateAppRoleV2Resp, LarkError> {
        let path = format!(
            "/open-apis/base/v2/apps/{}/roles/{}",
            query.app_token, query.role_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<AppRoleData, UpdateAppRoleV2Resp>()
        .await
    }
}
