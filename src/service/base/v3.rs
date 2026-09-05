use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{JsonResp, PageQuery, RestRequest};

pub type ListRecordResp = JsonResp;
pub type SearchRecordResp = JsonResp;
pub type GetFieldExtensionResp = JsonResp;
pub type UpdateFieldExtensionResp = JsonResp;
pub type UpdateFieldExtensionCellsResp = JsonResp;

/// The default number of Base templates returned per request.
pub const DEFAULT_BASE_TEMPLATE_LIMIT: i32 = 10;
/// The maximum number of Base templates accepted by the template-center API.
pub const MAX_BASE_TEMPLATE_LIMIT: i32 = 100;

/// A category in the Base template center.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct TemplateCategory {
    pub key: Option<String>,
    pub name: Option<String>,
}

/// A Base template that can be copied into a new Base.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct BaseTemplate {
    pub token: Option<String>,
    pub name: Option<String>,
    pub introduction: Option<String>,
    pub scenarios: Option<Vec<String>>,
    pub developer: Option<String>,
    pub link: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Response data for template-center category discovery.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct ListTemplateCategoryRespData {
    pub categories: Option<Vec<TemplateCategory>>,
}

/// Response data for template-center list and search operations.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct ListBaseTemplateRespData {
    pub templates: Option<Vec<BaseTemplate>>,
    pub has_more: Option<bool>,
    pub offset: Option<String>,
}

/// PATCH body for dashboard sharing settings.
///
/// Every field is optional because the endpoint applies a partial update. In
/// particular, `Some(false)` is serialized so callers can explicitly disable
/// a setting.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UpdateDashboardShareReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateDashboardShareSettings>,
}

impl UpdateDashboardShareReqBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn access_scope(mut self, value: impl Into<String>) -> Self {
        self.access_scope = Some(value.into());
        self
    }

    pub fn settings(mut self, value: UpdateDashboardShareSettings) -> Self {
        self.settings = Some(value);
        self
    }
}

/// Dashboard-specific fields for [`UpdateDashboardShareReqBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UpdateDashboardShareSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_source: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_auto_analysis: Option<bool>,
}

impl UpdateDashboardShareSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_source(mut self, value: bool) -> Self {
        self.show_source = Some(value);
        self
    }

    pub fn enable_auto_analysis(mut self, value: bool) -> Self {
        self.enable_auto_analysis = Some(value);
        self
    }
}

/// PATCH body for form sharing settings.
///
/// Every field is optional because the endpoint applies a partial update. In
/// particular, `Some(false)` is serialized so callers can explicitly disable
/// a setting.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UpdateFormShareReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<UpdateFormShareSettings>,
}

impl UpdateFormShareReqBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn access_scope(mut self, value: impl Into<String>) -> Self {
        self.access_scope = Some(value.into());
        self
    }

    pub fn settings(mut self, value: UpdateFormShareSettings) -> Self {
        self.settings = Some(value);
        self
    }
}

/// Form-specific fields for [`UpdateFormShareReqBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UpdateFormShareSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_login: Option<bool>,
}

/// One segment in a field-extension completion prompt.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct FieldExtensionPromptSegment {
    #[serde(rename = "type")]
    pub kind: FieldExtensionPromptSegmentKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl FieldExtensionPromptSegment {
    pub fn text(value: impl Into<String>) -> Self {
        Self {
            kind: FieldExtensionPromptSegmentKind::Text,
            text: Some(value.into()),
            field: None,
        }
    }

    pub fn field_ref(value: impl Into<String>) -> Self {
        Self {
            kind: FieldExtensionPromptSegmentKind::FieldRef,
            text: None,
            field: Some(value.into()),
        }
    }
}

/// Supported kinds of field-extension completion prompt segments.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FieldExtensionPromptSegmentKind {
    Text,
    FieldRef,
}

/// Input for the built-in field-extension completion configuration.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct FieldExtensionCompletionInput {
    pub prompt: Vec<FieldExtensionPromptSegment>,
}

impl FieldExtensionCompletionInput {
    pub fn new(prompt: impl IntoIterator<Item = FieldExtensionPromptSegment>) -> Self {
        Self {
            prompt: prompt.into_iter().collect(),
        }
    }
}

/// PUT body for a Base field extension. The default value serializes as `{}`
/// and clears the extension.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct UpdateFieldExtensionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<FieldExtensionCompletionInput>,
}

impl UpdateFieldExtensionReqBody {
    /// Clears the extension configuration.
    pub fn clear() -> Self {
        Self::default()
    }

    /// Configures the official built-in LLM completion extension.
    pub fn builtin_llm_completion(inputs: FieldExtensionCompletionInput) -> Self {
        Self {
            extension_id: Some("builtin_llm_completion".to_string()),
            inputs: Some(inputs),
        }
    }
}

/// Scope for a field-extension cell update task.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FieldExtensionUpdateCellsType {
    Column,
    Row,
}

/// POST body for a field-extension cell update task.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct UpdateFieldExtensionCellsReqBody {
    #[serde(rename = "type")]
    pub update_type: FieldExtensionUpdateCellsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_ids: Option<Vec<String>>,
}

impl UpdateFieldExtensionCellsReqBody {
    pub fn column(view_id: Option<impl Into<String>>) -> Self {
        Self {
            update_type: FieldExtensionUpdateCellsType::Column,
            view_id: view_id.map(Into::into),
            record_ids: None,
        }
    }

    pub fn rows(record_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            update_type: FieldExtensionUpdateCellsType::Row,
            view_id: None,
            record_ids: Some(record_ids.into_iter().map(Into::into).collect()),
        }
    }
}

/// Request body for batch-generating Base record share links.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct CreateRecordShareLinksReqBody {
    pub record_ids: Vec<String>,
}

impl CreateRecordShareLinksReqBody {
    pub fn new(record_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            record_ids: record_ids.into_iter().map(Into::into).collect(),
        }
    }
}

/// Response data for batch record-share-link generation. Missing request IDs
/// represent records the service did not expose to the caller.
#[derive(Debug, Clone, Default, Deserialize)]
#[non_exhaustive]
pub struct CreateRecordShareLinksRespData {
    pub record_share_links: Option<std::collections::HashMap<String, String>>,
}

impl_resp!(CreateRecordShareLinksResp, CreateRecordShareLinksRespData);

impl UpdateFormShareSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_anonymous(mut self, value: bool) -> Self {
        self.allow_anonymous = Some(value);
        self
    }

    pub fn require_login(mut self, value: bool) -> Self {
        self.require_login = Some(value);
        self
    }
}

/// Query parameters for reading Base v3 records.
///
/// `filter` and `sort` are JSON-encoded strings, as required by the Base v3
/// list endpoint. Search-specific filters belong in the JSON body passed to
/// [`RecordResource::search`].
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListRecordQuery<'a> {
    pub base_token: &'a str,
    pub table_id: &'a str,
    pub field_ids: Option<&'a [&'a str]>,
    pub view_id: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub sort: Option<&'a str>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

impl<'a> ListRecordQuery<'a> {
    pub fn new(base_token: &'a str, table_id: &'a str) -> Self {
        Self {
            base_token,
            table_id,
            ..Self::default()
        }
    }

    pub fn field_ids(mut self, value: &'a [&'a str]) -> Self {
        self.field_ids = Some(value);
        self
    }

    pub fn view_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.view_id = value.into();
        self
    }

    pub fn filter(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.filter = value.into();
        self
    }

    pub fn sort(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.sort = value.into();
        self
    }

    pub fn offset(mut self, value: impl Into<Option<i32>>) -> Self {
        self.offset = value.into();
        self
    }

    pub fn limit(mut self, value: impl Into<Option<i32>>) -> Self {
        self.limit = value.into();
        self
    }
}

/// Query parameters for listing entities in a Base workspace.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListWorkspaceEntitiesQuery<'a> {
    pub workspace_token: &'a str,
    pub entity_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListWorkspaceEntitiesQuery<'a> {
    pub fn new(workspace_token: &'a str) -> Self {
        Self {
            workspace_token,
            entity_type: None,
            page: PageQuery::default(),
        }
    }

    pub fn entity_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.entity_type = value.into();
        self
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Query parameters for listing pages in a BaseApp.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListBaseAppPagesQuery<'a> {
    pub app_token: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListBaseAppPagesQuery<'a> {
    pub fn new(app_token: &'a str) -> Self {
        Self {
            app_token,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Query parameters for listing blocks on a BaseApp page.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListBaseAppBlocksQuery<'a> {
    pub app_token: &'a str,
    pub page_id: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListBaseAppBlocksQuery<'a> {
    pub fn new(app_token: &'a str, page_id: &'a str) -> Self {
        Self {
            app_token,
            page_id,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Query parameters for listing Base templates by category.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListBaseTemplateQuery<'a> {
    pub category_key: Option<&'a str>,
    pub limit: Option<i32>,
    pub offset: Option<&'a str>,
}

impl<'a> Default for ListBaseTemplateQuery<'a> {
    fn default() -> Self {
        Self {
            category_key: None,
            limit: Some(DEFAULT_BASE_TEMPLATE_LIMIT),
            offset: None,
        }
    }
}

impl<'a> ListBaseTemplateQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.category_key = value.into();
        self
    }

    pub fn limit(mut self, value: impl Into<Option<i32>>) -> Self {
        self.limit = value.into();
        self
    }

    pub fn offset(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.offset = value.into();
        self
    }

    fn normalized_category_key(&self) -> Option<&str> {
        self.category_key.and_then(|key| {
            let key = key.trim();
            (!key.is_empty()).then_some(key)
        })
    }

    fn normalized_offset(&self) -> Option<&str> {
        self.offset.and_then(|offset| {
            let offset = offset.trim();
            (!offset.is_empty()).then_some(offset)
        })
    }

    fn validate(&self) -> Result<(), LarkError> {
        validate_template_limit(self.limit)
    }
}

/// Query parameters for searching Base templates.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SearchBaseTemplateQuery<'a> {
    pub keyword: &'a str,
    pub limit: Option<i32>,
    pub offset: Option<&'a str>,
}

impl<'a> SearchBaseTemplateQuery<'a> {
    pub fn new(keyword: &'a str) -> Self {
        Self {
            keyword,
            limit: Some(DEFAULT_BASE_TEMPLATE_LIMIT),
            offset: None,
        }
    }

    pub fn limit(mut self, value: impl Into<Option<i32>>) -> Self {
        self.limit = value.into();
        self
    }

    pub fn offset(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.offset = value.into();
        self
    }

    fn keyword(&self) -> Result<&str, LarkError> {
        let keyword = self.keyword.trim();
        if keyword.is_empty() {
            return Err(LarkError::IllegalParam(
                "template keyword must not be blank".to_owned(),
            ));
        }
        Ok(keyword)
    }

    fn normalized_offset(&self) -> Option<&str> {
        self.offset.and_then(|offset| {
            let offset = offset.trim();
            (!offset.is_empty()).then_some(offset)
        })
    }

    fn validate(&self) -> Result<&str, LarkError> {
        validate_template_limit(self.limit)?;
        self.keyword()
    }
}

fn validate_template_limit(limit: Option<i32>) -> Result<(), LarkError> {
    if let Some(limit) = limit
        && !(1..=MAX_BASE_TEMPLATE_LIMIT).contains(&limit)
    {
        return Err(LarkError::IllegalParam(format!(
            "template limit must be between 1 and {MAX_BASE_TEMPLATE_LIMIT}"
        )));
    }
    Ok(())
}

impl_resp!(ListTemplateCategoryResp, ListTemplateCategoryRespData);
impl_resp!(ListBaseTemplateResp, ListBaseTemplateRespData);

/// Query parameters for reading computed data from a BaseApp chart block.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetBaseAppBlockDataQuery<'a> {
    pub app_token: &'a str,
    pub block_id: &'a str,
    pub base_token: &'a str,
}

/// Query parameters for listing dashboards in a Base.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListDashboardsQuery<'a> {
    pub base_token: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListDashboardsQuery<'a> {
    pub fn new(base_token: &'a str) -> Self {
        Self {
            base_token,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Query parameters for listing blocks in a Base dashboard.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListDashboardBlocksQuery<'a> {
    pub base_token: &'a str,
    pub dashboard_id: &'a str,
    pub page: PageQuery<'a>,
}

/// Query parameters for listing forms in a Base table.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ListFormsQuery<'a> {
    pub base_token: &'a str,
    pub table_id: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListFormsQuery<'a> {
    pub fn new(base_token: &'a str, table_id: &'a str) -> Self {
        Self {
            base_token,
            table_id,
            page: PageQuery::new(),
        }
    }
    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Request body for removing questions from a Base v3 form.
///
/// Omitting `keep_field` is intentionally different from setting it to false:
/// the service's default deletes the backing fields and their record data.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct DeleteFormQuestionsReqBody {
    pub question_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_field: Option<bool>,
}

impl DeleteFormQuestionsReqBody {
    pub fn new(question_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            question_ids: question_ids.into_iter().map(Into::into).collect(),
            keep_field: None,
        }
    }
    pub fn keep_field(mut self, value: bool) -> Self {
        self.keep_field = Some(value);
        self
    }
}

impl<'a> ListDashboardBlocksQuery<'a> {
    pub fn new(base_token: &'a str, dashboard_id: &'a str) -> Self {
        Self {
            base_token,
            dashboard_id,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Optional user-ID representation for dashboard block and arrange requests.
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct DashboardUserIdTypeQuery<'a> {
    pub user_id_type: Option<&'a str>,
}

impl<'a> DashboardUserIdTypeQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

impl<'a> GetBaseAppBlockDataQuery<'a> {
    pub fn new(app_token: &'a str, block_id: &'a str, base_token: &'a str) -> Self {
        Self {
            app_token,
            block_id,
            base_token,
        }
    }
}

/// Body parameters for listing workflows in a Base.
///
/// The service uses a POST body for pagination rather than query parameters.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListWorkflowQuery<'a> {
    pub base_token: &'a str,
    pub status: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListWorkflowQuery<'a> {
    pub fn new(base_token: &'a str) -> Self {
        Self {
            base_token,
            status: None,
            page: PageQuery::new(),
        }
    }

    pub fn status(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.status = value.into();
        self
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Parameters for retrieving one Base workflow.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetWorkflowQuery<'a> {
    pub base_token: &'a str,
    pub workflow_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetWorkflowQuery<'a> {
    pub fn new(base_token: &'a str, workflow_id: &'a str) -> Self {
        Self {
            base_token,
            workflow_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Serialize)]
struct ListWorkflowReqBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<&'a str>,
}

pub struct V3<'a> {
    pub record: RecordResource<'a>,
    pub field_extension: FieldExtensionResource<'a>,
    pub template: TemplateResource<'a>,
    pub dashboard: DashboardResource<'a>,
    pub dashboard_block: DashboardBlockResource<'a>,
    pub dashboard_share: DashboardShareResource<'a>,
    pub form_share: FormShareResource<'a>,
    pub form: FormResource<'a>,
    pub form_question: FormQuestionResource<'a>,
    pub workflow: WorkflowResource<'a>,
    pub workspace: WorkspaceResource<'a>,
    pub app: BaseAppResource<'a>,
    pub page: BaseAppPageResource<'a>,
    pub block: BaseAppBlockResource<'a>,
}

impl<'a> V3<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            record: RecordResource { config },
            field_extension: FieldExtensionResource { config },
            template: TemplateResource { config },
            dashboard: DashboardResource { config },
            dashboard_block: DashboardBlockResource { config },
            dashboard_share: DashboardShareResource { config },
            form_share: FormShareResource { config },
            form: FormResource { config },
            form_question: FormQuestionResource { config },
            workflow: WorkflowResource { config },
            workspace: WorkspaceResource { config },
            app: BaseAppResource { config },
            page: BaseAppPageResource { config },
            block: BaseAppBlockResource { config },
        }
    }
}

/// Base v3 automation workflow operations.
///
/// Workflow definitions intentionally accept arbitrary serializable JSON: the
/// server's step schema evolves independently of the stable route contract.
pub struct WorkflowResource<'a> {
    config: &'a Config,
}

impl WorkflowResource<'_> {
    /// Creates a disabled workflow. The body must include a unique
    /// server-recognized `client_token`.
    pub async fn create(
        &self,
        base_token: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/workflows",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Gets one workflow definition, including its untyped step graph.
    pub async fn get(
        &self,
        query: &GetWorkflowQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/workflows/:workflow_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", query.base_token)
        .path_param("workflow_id", query.workflow_id)
        .query("user_id_type", query.user_id_type)
        .send_json()
        .await
    }

    /// Lists workflows using the API's body-paginated list operation.
    pub async fn list(
        &self,
        query: &ListWorkflowQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        let body = ListWorkflowReqBody {
            status: query.status,
            page_size: query.page.page_size,
            page_token: query.page.page_token,
        };
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/workflows/list",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", query.base_token)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Replaces a workflow definition. Preserve fields that should remain set.
    pub async fn update(
        &self,
        base_token: &str,
        workflow_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PUT,
            "/open-apis/base/v3/bases/:base_token/workflows/:workflow_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("workflow_id", workflow_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Enables a workflow.
    pub async fn enable(
        &self,
        base_token: &str,
        workflow_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        self.set_enabled(base_token, workflow_id, true, option)
            .await
    }

    /// Disables a workflow.
    pub async fn disable(
        &self,
        base_token: &str,
        workflow_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        self.set_enabled(base_token, workflow_id, false, option)
            .await
    }

    async fn set_enabled(
        &self,
        base_token: &str,
        workflow_id: &str,
        enabled: bool,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        let action = if enabled { "enable" } else { "disable" };
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            format!("/open-apis/base/v3/bases/:base_token/workflows/:workflow_id/{action}"),
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("workflow_id", workflow_id)
        .json_body(&serde_json::json!({}))?
        .send_json()
        .await
    }
}

/// Template-center discovery operations for Base v3.
pub struct TemplateResource<'a> {
    config: &'a Config,
}

impl TemplateResource<'_> {
    /// Lists the available Base template categories.
    pub async fn list_categories(
        &self,
        option: &RequestOption,
    ) -> Result<ListTemplateCategoryResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/templates/category",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .send_response::<ListTemplateCategoryRespData, ListTemplateCategoryResp>()
        .await
    }

    /// Lists Base templates, optionally filtered by a template category.
    pub async fn list(
        &self,
        query: &ListBaseTemplateQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListBaseTemplateResp, LarkError> {
        query.validate()?;
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/templates",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .query("category_key", query.normalized_category_key())
        .query("limit", query.limit)
        .query("offset", query.normalized_offset())
        .send_response::<ListBaseTemplateRespData, ListBaseTemplateResp>()
        .await
    }

    /// Searches Base templates by keyword.
    pub async fn search(
        &self,
        query: &SearchBaseTemplateQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListBaseTemplateResp, LarkError> {
        let keyword = query.validate()?;
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/templates/search",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .query("keyword", keyword)
        .query("limit", query.limit)
        .query("offset", query.normalized_offset())
        .send_response::<ListBaseTemplateRespData, ListBaseTemplateResp>()
        .await
    }
}

/// Dashboard management operations for Base v3.
pub struct DashboardResource<'a> {
    config: &'a Config,
}

impl DashboardResource<'_> {
    /// Lists dashboards in a Base.
    pub async fn list(
        &self,
        query: &ListDashboardsQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/dashboards",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", query.base_token)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Gets one dashboard.
    pub async fn get(
        &self,
        base_token: &str,
        dashboard_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .send_json()
        .await
    }

    /// Creates a dashboard from the documented JSON request body.
    pub async fn create(
        &self,
        base_token: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/dashboards",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Partially updates a dashboard from the documented JSON request body.
    pub async fn update(
        &self,
        base_token: &str,
        dashboard_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Deletes one dashboard.
    pub async fn delete(
        &self,
        base_token: &str,
        dashboard_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .send_json()
        .await
    }

    /// Asks the service to arrange the blocks in a dashboard.
    pub async fn arrange(
        &self,
        base_token: &str,
        dashboard_id: &str,
        query: &DashboardUserIdTypeQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id/arrange",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .query("user_id_type", query.user_id_type)
        .json_body(&serde_json::json!({}))?
        .send_json()
        .await
    }
}

/// Block operations for Base dashboards.
pub struct DashboardBlockResource<'a> {
    config: &'a Config,
}

impl DashboardBlockResource<'_> {
    /// Lists blocks in a dashboard.
    pub async fn list(
        &self,
        query: &ListDashboardBlocksQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id/blocks",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", query.base_token)
        .path_param("dashboard_id", query.dashboard_id)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Gets one dashboard block.
    pub async fn get(
        &self,
        base_token: &str,
        dashboard_id: &str,
        block_id: &str,
        query: &DashboardUserIdTypeQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id/blocks/:block_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .path_param("block_id", block_id)
        .query("user_id_type", query.user_id_type)
        .send_json()
        .await
    }

    /// Creates a dashboard block from the documented JSON request body.
    pub async fn create(
        &self,
        base_token: &str,
        dashboard_id: &str,
        body: impl Serialize,
        query: &DashboardUserIdTypeQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id/blocks",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .query("user_id_type", query.user_id_type)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Partially updates a dashboard block from the documented JSON request body.
    pub async fn update(
        &self,
        base_token: &str,
        dashboard_id: &str,
        block_id: &str,
        body: impl Serialize,
        query: &DashboardUserIdTypeQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id/blocks/:block_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .path_param("block_id", block_id)
        .query("user_id_type", query.user_id_type)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Deletes one dashboard block.
    pub async fn delete(
        &self,
        base_token: &str,
        dashboard_id: &str,
        block_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id/blocks/:block_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .path_param("block_id", block_id)
        .send_json()
        .await
    }

    /// Reads computed data for a dashboard chart block.
    pub async fn get_data(
        &self,
        base_token: &str,
        block_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/dashboards/blocks/:block_id/data",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("block_id", block_id)
        .send_json()
        .await
    }
}

/// Dashboard sharing operations for Base v3.
pub struct DashboardShareResource<'a> {
    config: &'a Config,
}

impl DashboardShareResource<'_> {
    /// Gets a dashboard's sharing status and settings.
    pub async fn get(
        &self,
        base_token: &str,
        dashboard_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id/share",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .send_json()
        .await
    }

    /// Partially updates a dashboard's sharing settings.
    pub async fn update(
        &self,
        base_token: &str,
        dashboard_id: &str,
        body: &UpdateDashboardShareReqBody,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/bases/:base_token/dashboards/:dashboard_id/share",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("dashboard_id", dashboard_id)
        .json_body(body)?
        .send_json()
        .await
    }
}

/// Form sharing operations for Base v3.
pub struct FormShareResource<'a> {
    config: &'a Config,
}

impl FormShareResource<'_> {
    /// Gets a form's sharing status and settings.
    pub async fn get(
        &self,
        base_token: &str,
        table_id: &str,
        form_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms/:form_id/share",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("form_id", form_id)
        .send_json()
        .await
    }

    /// Partially updates a form's sharing settings.
    pub async fn update(
        &self,
        base_token: &str,
        table_id: &str,
        form_id: &str,
        body: &UpdateFormShareReqBody,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms/:form_id/share",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("form_id", form_id)
        .json_body(body)?
        .send_json()
        .await
    }
}

/// Form lifecycle operations for Base v3.
pub struct FormResource<'a> {
    config: &'a Config,
}

impl FormResource<'_> {
    pub async fn list(
        &self,
        query: &ListFormsQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", query.base_token)
        .path_param("table_id", query.table_id)
        .page_query(query.page)
        .send_json()
        .await
    }
    pub async fn get(
        &self,
        base_token: &str,
        table_id: &str,
        form_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms/:form_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("form_id", form_id)
        .send_json()
        .await
    }
    pub async fn create(
        &self,
        base_token: &str,
        table_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .json_body(&body)?
        .send_json()
        .await
    }
    pub async fn update(
        &self,
        base_token: &str,
        table_id: &str,
        form_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms/:form_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("form_id", form_id)
        .json_body(&body)?
        .send_json()
        .await
    }
    pub async fn delete(
        &self,
        base_token: &str,
        table_id: &str,
        form_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms/:form_id",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("form_id", form_id)
        .send_json()
        .await
    }
}

/// Question operations for Base v3 forms.
pub struct FormQuestionResource<'a> {
    config: &'a Config,
}

impl FormQuestionResource<'_> {
    pub async fn list(
        &self,
        base_token: &str,
        table_id: &str,
        form_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms/:form_id/questions",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("form_id", form_id)
        .send_json()
        .await
    }
    pub async fn create(
        &self,
        base_token: &str,
        table_id: &str,
        form_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms/:form_id/questions",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("form_id", form_id)
        .json_body(&body)?
        .send_json()
        .await
    }
    pub async fn update(
        &self,
        base_token: &str,
        table_id: &str,
        form_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms/:form_id/questions",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("form_id", form_id)
        .json_body(&body)?
        .send_json()
        .await
    }
    pub async fn delete(
        &self,
        base_token: &str,
        table_id: &str,
        form_id: &str,
        body: &DeleteFormQuestionsReqBody,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/forms/:form_id/questions",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("form_id", form_id)
        .json_body(body)?
        .send_json()
        .await
    }
}

pub struct RecordResource<'a> {
    config: &'a Config,
}

impl RecordResource<'_> {
    /// Lists records from a Base v3 table.
    pub async fn list(
        &self,
        query: &ListRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListRecordResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/records",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", query.base_token)
        .path_param("table_id", query.table_id)
        .query_values("field_id", query.field_ids)
        .query("view_id", query.view_id)
        .query("filter", query.filter)
        .query("sort", query.sort)
        .query("offset", query.offset)
        .query("limit", query.limit)
        .send_json()
        .await
    }

    /// Searches records from a Base v3 table with the upstream JSON request body.
    pub async fn search(
        &self,
        base_token: &str,
        table_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<SearchRecordResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/records/search",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Generates share links for up to 100 records. The response may omit
    /// record IDs the service cannot expose to the caller.
    pub async fn create_share_links(
        &self,
        base_token: &str,
        table_id: &str,
        body: &CreateRecordShareLinksReqBody,
        option: &RequestOption,
    ) -> Result<CreateRecordShareLinksResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/records/share_links/batch",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .json_body(body)?
        .send_response::<CreateRecordShareLinksRespData, CreateRecordShareLinksResp>()
        .await
    }
}

/// Field-extension configuration and execution operations for Base v3.
pub struct FieldExtensionResource<'a> {
    config: &'a Config,
}

impl FieldExtensionResource<'_> {
    /// Reads a field's extension configuration.
    pub async fn get(
        &self,
        base_token: &str,
        table_id: &str,
        field_id: &str,
        option: &RequestOption,
    ) -> Result<GetFieldExtensionResp, LarkError> {
        self.request(
            http::Method::GET,
            base_token,
            table_id,
            field_id,
            None::<&UpdateFieldExtensionReqBody>,
            option,
        )
        .await
    }

    /// Installs, updates, or clears a field extension.
    pub async fn update(
        &self,
        base_token: &str,
        table_id: &str,
        field_id: &str,
        body: &UpdateFieldExtensionReqBody,
        option: &RequestOption,
    ) -> Result<UpdateFieldExtensionResp, LarkError> {
        self.request(
            http::Method::PUT,
            base_token,
            table_id,
            field_id,
            Some(body),
            option,
        )
        .await
    }

    /// Starts a field-extension update task for a row set or a column view.
    pub async fn update_cells(
        &self,
        base_token: &str,
        table_id: &str,
        field_id: &str,
        body: &UpdateFieldExtensionCellsReqBody,
        option: &RequestOption,
    ) -> Result<UpdateFieldExtensionCellsResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/fields/:field_id/field_extensions/update_cells",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("field_id", field_id)
        .json_body(body)?
        .send_json()
        .await
    }

    async fn request<T: Serialize>(
        &self,
        method: http::Method,
        base_token: &str,
        table_id: &str,
        field_id: &str,
        body: Option<T>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        let request = RestRequest::new(
            self.config,
            method,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/fields/:field_id/field_extensions",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .path_param("field_id", field_id);
        let request = match body {
            Some(body) => request.json_body(&body)?,
            None => request,
        };
        request.send_json().await
    }
}

/// Base v3 workspace operations proven by the official CLI's Open Platform client.
pub struct WorkspaceResource<'a> {
    config: &'a Config,
}

impl WorkspaceResource<'_> {
    /// Creates a workspace from the documented JSON request body.
    pub async fn create(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/workspaces",
            vec![AccessTokenType::User],
            &option,
        )
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Lists Base and BaseApp entities in a workspace.
    pub async fn list_entities(
        &self,
        query: &ListWorkspaceEntitiesQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/workspaces/:workspace_token/entities",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("workspace_token", query.workspace_token)
        .query("entity_type", query.entity_type)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Moves a Base or BaseApp entity into a workspace.
    pub async fn move_in(
        &self,
        workspace_token: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/workspaces/:workspace_token/move_in",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("workspace_token", workspace_token)
        .json_body(&body)?
        .send_json()
        .await
    }
}

/// BaseApp application-mode operations proven by the official CLI's Open Platform client.
pub struct BaseAppResource<'a> {
    config: &'a Config,
}

impl BaseAppResource<'_> {
    /// Creates a BaseApp from the documented JSON request body.
    pub async fn create(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/base_apps",
            vec![AccessTokenType::User],
            &option,
        )
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Gets a BaseApp and its page summaries.
    pub async fn get(
        &self,
        app_token: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .send_json()
        .await
    }
}

/// Page operations for BaseApps.
pub struct BaseAppPageResource<'a> {
    config: &'a Config,
}

impl BaseAppPageResource<'_> {
    /// Lists pages in a BaseApp.
    pub async fn list(
        &self,
        query: &ListBaseAppPagesQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/pages",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", query.app_token)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Gets one BaseApp page.
    pub async fn get(
        &self,
        app_token: &str,
        page_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .send_json()
        .await
    }

    /// Creates a top-level BaseApp page.
    pub async fn create(
        &self,
        app_token: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/base_apps/:app_token/pages",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Renames a BaseApp page with the documented JSON request body.
    pub async fn rename(
        &self,
        app_token: &str,
        page_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Deletes a BaseApp page.
    pub async fn delete(
        &self,
        app_token: &str,
        page_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .send_json()
        .await
    }
}

/// Block operations for BaseApp pages.
pub struct BaseAppBlockResource<'a> {
    config: &'a Config,
}

impl BaseAppBlockResource<'_> {
    /// Lists blocks on a BaseApp page.
    pub async fn list(
        &self,
        query: &ListBaseAppBlocksQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id/blocks",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", query.app_token)
        .path_param("page_id", query.page_id)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Gets one BaseApp block.
    pub async fn get(
        &self,
        app_token: &str,
        page_id: &str,
        block_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id/blocks/:block_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .path_param("block_id", block_id)
        .send_json()
        .await
    }

    /// Creates a BaseApp block from the documented JSON request body.
    pub async fn create(
        &self,
        app_token: &str,
        page_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id/blocks",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Updates a BaseApp block with the documented JSON request body.
    pub async fn update(
        &self,
        app_token: &str,
        page_id: &str,
        block_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id/blocks/:block_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .path_param("block_id", block_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Reads computed data for a BaseApp chart block.
    ///
    /// The endpoint takes a chart token rather than the page-scoped widget ID,
    /// and does not support text blocks.
    pub async fn get_data(
        &self,
        query: &GetBaseAppBlockDataQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/blocks/:block_id/data",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", query.app_token)
        .path_param("block_id", query.block_id)
        .query("base_token", query.base_token)
        .send_json()
        .await
    }
}

fn with_app_id(config: &Config, option: &RequestOption) -> Result<RequestOption, LarkError> {
    let mut option = option.clone();
    option
        .headers
        .get_or_insert_with(http::HeaderMap::new)
        .insert(
            http::HeaderName::from_static("x-app-id"),
            http::HeaderValue::from_str(config.app_id())
                .map_err(|err| LarkError::IllegalParam(err.to_string()))?,
        );
    Ok(option)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_queries_use_the_cli_default_page_size() {
        let list = ListBaseTemplateQuery::new();
        let search = SearchBaseTemplateQuery::new("templates");

        assert_eq!(list.limit, Some(DEFAULT_BASE_TEMPLATE_LIMIT));
        assert_eq!(search.limit, Some(DEFAULT_BASE_TEMPLATE_LIMIT));
        assert!(list.validate().is_ok());
        assert_eq!(search.validate().unwrap(), "templates");
    }

    #[test]
    fn template_queries_reject_out_of_range_page_sizes() {
        for limit in [0, MAX_BASE_TEMPLATE_LIMIT + 1] {
            let list = ListBaseTemplateQuery::new().limit(limit);
            let search = SearchBaseTemplateQuery::new("templates").limit(limit);

            assert!(matches!(list.validate(), Err(LarkError::IllegalParam(_))));
            assert!(matches!(search.validate(), Err(LarkError::IllegalParam(_))));
        }
    }

    #[test]
    fn template_search_rejects_blank_keywords_and_normalizes_whitespace() {
        assert!(matches!(
            SearchBaseTemplateQuery::new(" \t ").validate(),
            Err(LarkError::IllegalParam(_))
        ));
        assert_eq!(
            SearchBaseTemplateQuery::new(" AI ").validate().unwrap(),
            "AI"
        );
    }
}
