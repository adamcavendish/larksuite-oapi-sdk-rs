use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{PageQuery, RestRequest, parse_v2};
use crate::transport;

// ── Generic response data types ───────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2ListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentV2Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentV2ListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_field: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SectionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SectionListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasklistData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasklist: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasklistListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivitySubscriptionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_subscription: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivitySubscriptionListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// ── Params types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default)]
pub struct TaskListParams<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub completed: Option<bool>,
    pub created_from: Option<&'a str>,
    pub created_to: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> TaskListParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn completed(mut self, value: impl Into<Option<bool>>) -> Self {
        self.completed = value.into();
        self
    }

    pub fn created_from(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.created_from = value.into();
        self
    }

    pub fn created_to(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.created_to = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListTaskV2Query<'a> {
    pub page: PageQuery<'a>,
    pub completed: Option<bool>,
    pub task_type: Option<&'a str>,
    pub agent_task_status: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListTaskV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn completed(mut self, value: impl Into<Option<bool>>) -> Self {
        self.completed = value.into();
        self
    }

    pub fn task_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.task_type = value.into();
        self
    }

    pub fn agent_task_status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.agent_task_status = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetTaskV2Query<'a> {
    pub task_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetTaskV2Query<'a> {
    pub fn new(task_guid: &'a str) -> Self {
        Self {
            task_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetAttachmentV2Query<'a> {
    pub attachment_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetAttachmentV2Query<'a> {
    pub fn new(attachment_guid: &'a str) -> Self {
        Self {
            attachment_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListAttachmentV2Query<'a> {
    pub resource_type: Option<&'a str>,
    pub resource_id: Option<&'a str>,
    pub updated_mesc: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListAttachmentV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_type = value.into();
        self
    }

    pub fn resource_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_id = value.into();
        self
    }

    pub fn updated_mesc(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.updated_mesc = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetCommentV2Query<'a> {
    pub comment_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetCommentV2Query<'a> {
    pub fn new(comment_id: &'a str) -> Self {
        Self {
            comment_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListCommentV2Query<'a> {
    pub resource_type: Option<&'a str>,
    pub resource_id: Option<&'a str>,
    pub direction: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListCommentV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_type = value.into();
        self
    }

    pub fn resource_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_id = value.into();
        self
    }

    pub fn direction(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.direction = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetCustomFieldV2Query<'a> {
    pub custom_field_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetCustomFieldV2Query<'a> {
    pub fn new(custom_field_guid: &'a str) -> Self {
        Self {
            custom_field_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListCustomFieldV2Query<'a> {
    pub resource_type: Option<&'a str>,
    pub resource_id: Option<&'a str>,
    pub update_msec: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListCustomFieldV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_type = value.into();
        self
    }

    pub fn resource_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_id = value.into();
        self
    }

    pub fn update_msec(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_msec = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

// ── Response types ─────────────────────────────────────────────────────────────

impl_resp_v2!(CreateTaskV2Resp, TaskV2Data);
impl_resp_v2!(GetTaskV2Resp, TaskV2Data);
impl_resp_v2!(PatchTaskV2Resp, TaskV2Data);
impl_resp_v2!(ListTaskV2Resp, TaskV2ListData);
impl_resp_v2!(AddMembersTaskV2Resp, TaskV2Data);
impl_resp_v2!(RemoveMembersTaskV2Resp, TaskV2Data);
impl_resp_v2!(AddRemindersTaskV2Resp, TaskV2Data);
impl_resp_v2!(RemoveRemindersTaskV2Resp, TaskV2Data);
impl_resp_v2!(AddDependenciesTaskV2Resp, TaskV2Data);
impl_resp_v2!(RemoveDependenciesTaskV2Resp, TaskV2Data);
impl_resp_v2!(AddTasklistTaskV2Resp, TaskV2Data);
impl_resp_v2!(RemoveTasklistTaskV2Resp, TaskV2Data);
impl_resp_v2!(TasklistsTaskV2Resp, TasklistListData);
impl_resp_v2!(CreateTaskSubtaskV2Resp, TaskV2Data);
impl_resp_v2!(ListTaskSubtaskV2Resp, TaskV2ListData);
impl_resp_v2!(DeleteTaskV2Resp, ());
impl_resp_v2!(GetAttachmentV2Resp, AttachmentData);
impl_resp_v2!(ListAttachmentV2Resp, AttachmentListData);
impl_resp_v2!(DeleteAttachmentV2Resp, ());
impl_resp_v2!(UploadAttachmentV2Resp, AttachmentData);
impl_resp_v2!(CreateCommentV2Resp, CommentV2Data);
impl_resp_v2!(GetCommentV2Resp, CommentV2Data);
impl_resp_v2!(PatchCommentV2Resp, CommentV2Data);
impl_resp_v2!(ListCommentV2Resp, CommentV2ListData);
impl_resp_v2!(DeleteCommentV2Resp, ());
impl_resp_v2!(CreateCustomFieldV2Resp, CustomFieldData);
impl_resp_v2!(GetCustomFieldV2Resp, CustomFieldData);
impl_resp_v2!(PatchCustomFieldV2Resp, CustomFieldData);
impl_resp_v2!(ListCustomFieldV2Resp, CustomFieldListData);
impl_resp_v2!(AddCustomFieldV2Resp, CustomFieldData);
impl_resp_v2!(RemoveCustomFieldV2Resp, ());
impl_resp_v2!(CreateCustomFieldOptionV2Resp, serde_json::Value);
impl_resp_v2!(PatchCustomFieldOptionV2Resp, serde_json::Value);
impl_resp_v2!(CreateSectionV2Resp, SectionData);
impl_resp_v2!(GetSectionV2Resp, SectionData);
impl_resp_v2!(PatchSectionV2Resp, SectionData);
impl_resp_v2!(ListSectionV2Resp, SectionListData);
impl_resp_v2!(DeleteSectionV2Resp, ());
impl_resp_v2!(TasksSectionV2Resp, TaskV2ListData);
impl_resp_v2!(CreateTasklistV2Resp, TasklistData);
impl_resp_v2!(GetTasklistV2Resp, TasklistData);
impl_resp_v2!(PatchTasklistV2Resp, TasklistData);
impl_resp_v2!(ListTasklistV2Resp, TasklistListData);
impl_resp_v2!(DeleteTasklistV2Resp, ());
impl_resp_v2!(AddMembersTasklistV2Resp, TasklistData);
impl_resp_v2!(RemoveMembersTasklistV2Resp, TasklistData);
impl_resp_v2!(TasksTasklistV2Resp, TaskV2ListData);
impl_resp_v2!(CreateActivitySubscriptionV2Resp, ActivitySubscriptionData);
impl_resp_v2!(GetActivitySubscriptionV2Resp, ActivitySubscriptionData);
impl_resp_v2!(PatchActivitySubscriptionV2Resp, ActivitySubscriptionData);
impl_resp_v2!(ListActivitySubscriptionV2Resp, ActivitySubscriptionListData);
impl_resp_v2!(DeleteActivitySubscriptionV2Resp, ());

// ── V2 service entry ──────────────────────────────────────────────────────────

pub struct V2<'a> {
    pub task: TaskV2Resource<'a>,
    pub attachment: AttachmentV2Resource<'a>,
    pub comment: CommentV2Resource<'a>,
    pub custom_field: CustomFieldV2Resource<'a>,
    pub section: SectionV2Resource<'a>,
    pub tasklist: TasklistV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            task: TaskV2Resource { config },
            attachment: AttachmentV2Resource { config },
            comment: CommentV2Resource { config },
            custom_field: CustomFieldV2Resource { config },
            section: SectionV2Resource { config },
            tasklist: TasklistV2Resource { config },
        }
    }
}

// ── Task resource ─────────────────────────────────────────────────────────────

pub struct TaskV2Resource<'a> {
    config: &'a Config,
}

impl<'a> TaskV2Resource<'a> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTaskV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/task/v2/tasks");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        task_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetTaskV2Resp, LarkError> {
        let query = GetTaskV2Query::new(task_guid).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}", query.task_guid);
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2::<TaskV2Data>()
        .await?;
        Ok(GetTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        task_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTaskV2Resp, LarkError> {
        let query = ListTaskV2Query::new()
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListTaskV2Resp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/tasks",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("completed", query.completed)
        .query("type", query.task_type)
        .query("agent_task_status", query.agent_task_status)
        .query("user_id_type", query.user_id_type)
        .send_v2::<TaskV2ListData>()
        .await?;
        Ok(ListTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn add_members(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddMembersTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/add_members");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(AddMembersTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn remove_members(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveMembersTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/remove_members");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RemoveMembersTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn add_reminders(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddRemindersTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/add_reminders");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(AddRemindersTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn remove_reminders(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveRemindersTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/remove_reminders");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RemoveRemindersTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn add_dependencies(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddDependenciesTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/add_dependencies");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(AddDependenciesTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn remove_dependencies(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveDependenciesTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/remove_dependencies");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RemoveDependenciesTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn add_tasklist(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddTasklistTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/add_tasklist");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(AddTasklistTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn remove_tasklist(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveTasklistTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/remove_tasklist");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RemoveTasklistTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn tasklists(
        &self,
        task_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<TasklistsTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/tasklists");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TasklistListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(TasklistsTaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn create_subtask(
        &self,
        task_guid: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTaskSubtaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/subtasks");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateTaskSubtaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list_subtasks(
        &self,
        task_guid: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTaskSubtaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{task_guid}/subtasks");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
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
            transport::request_typed::<TaskV2ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListTaskSubtaskV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Attachment resource ───────────────────────────────────────────────────────

pub struct AttachmentV2Resource<'a> {
    config: &'a Config,
}

impl<'a> AttachmentV2Resource<'a> {
    pub async fn get(
        &self,
        attachment_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetAttachmentV2Resp, LarkError> {
        let query = GetAttachmentV2Query::new(attachment_guid).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetAttachmentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetAttachmentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/attachments/{}", query.attachment_guid);
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2::<AttachmentData>()
        .await?;
        Ok(GetAttachmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        attachment_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteAttachmentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/attachments/{attachment_guid}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteAttachmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAttachmentV2Resp, LarkError> {
        let query = ListAttachmentV2Query::new()
            .resource_type(resource_type)
            .resource_id(resource_id)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAttachmentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListAttachmentV2Resp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/attachments",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("resource_type", query.resource_type)
        .query("resource_id", query.resource_id)
        .query("updated_mesc", query.updated_mesc)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2::<AttachmentListData>()
        .await?;
        Ok(ListAttachmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn upload(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadAttachmentV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/task/v2/attachments/upload");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<AttachmentData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(UploadAttachmentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Comment resource ──────────────────────────────────────────────────────────

pub struct CommentV2Resource<'a> {
    config: &'a Config,
}

impl<'a> CommentV2Resource<'a> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateCommentV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/task/v2/comments");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CommentV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateCommentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        comment_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetCommentV2Resp, LarkError> {
        let query = GetCommentV2Query::new(comment_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetCommentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetCommentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/comments/{}", query.comment_id);
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2::<CommentV2Data>()
        .await?;
        Ok(GetCommentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        comment_id: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchCommentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/comments/{comment_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CommentV2Data>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchCommentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        comment_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCommentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/comments/{comment_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteCommentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCommentV2Resp, LarkError> {
        let query = ListCommentV2Query::new()
            .resource_type(resource_type)
            .resource_id(resource_id)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCommentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListCommentV2Resp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/comments",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("resource_type", query.resource_type)
        .query("resource_id", query.resource_id)
        .query("direction", query.direction)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2::<CommentV2ListData>()
        .await?;
        Ok(ListCommentV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── CustomField resource ──────────────────────────────────────────────────────

pub struct CustomFieldV2Resource<'a> {
    config: &'a Config,
}

impl<'a> CustomFieldV2Resource<'a> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateCustomFieldV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/task/v2/custom_fields");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CustomFieldData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateCustomFieldV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        custom_field_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetCustomFieldV2Resp, LarkError> {
        let query = GetCustomFieldV2Query::new(custom_field_guid).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetCustomFieldV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetCustomFieldV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/custom_fields/{}",
            query.custom_field_guid
        );
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2::<CustomFieldData>()
        .await?;
        Ok(GetCustomFieldV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        custom_field_guid: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchCustomFieldV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/custom_fields/{custom_field_guid}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CustomFieldData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchCustomFieldV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCustomFieldV2Resp, LarkError> {
        let query = ListCustomFieldV2Query::new()
            .resource_type(resource_type)
            .resource_id(resource_id)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCustomFieldV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListCustomFieldV2Resp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/custom_fields",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("resource_type", query.resource_type)
        .query("resource_id", query.resource_id)
        .query("update_msec", query.update_msec)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2::<CustomFieldListData>()
        .await?;
        Ok(ListCustomFieldV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn add(
        &self,
        custom_field_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddCustomFieldV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/custom_fields/{custom_field_guid}/add");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CustomFieldData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(AddCustomFieldV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn remove(
        &self,
        custom_field_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveCustomFieldV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/custom_fields/{custom_field_guid}/remove");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RemoveCustomFieldV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn create_option(
        &self,
        custom_field_guid: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateCustomFieldOptionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/custom_fields/{custom_field_guid}/options");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateCustomFieldOptionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch_option(
        &self,
        custom_field_guid: &str,
        option_guid: &str,
        body: &serde_json::Value,
        req_option: &RequestOption,
    ) -> Result<PatchCustomFieldOptionV2Resp, LarkError> {
        let path =
            format!("/open-apis/task/v2/custom_fields/{custom_field_guid}/options/{option_guid}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, req_option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchCustomFieldOptionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Section resource ──────────────────────────────────────────────────────────

pub struct SectionV2Resource<'a> {
    config: &'a Config,
}

impl<'a> SectionV2Resource<'a> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateSectionV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/task/v2/sections");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SectionData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateSectionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        section_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetSectionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/sections/{section_guid}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<SectionData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetSectionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        section_guid: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchSectionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/sections/{section_guid}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SectionData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchSectionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        section_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteSectionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/sections/{section_guid}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteSectionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSectionV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/task/v2/sections");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = resource_type {
            api_req.query_params.set("resource_type", v);
        }
        if let Some(v) = resource_id {
            api_req.query_params.set("resource_id", v);
        }
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
            transport::request_typed::<SectionListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListSectionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn tasks(
        &self,
        section_guid: &str,
        params: TaskListParams<'_>,
        option: &RequestOption,
    ) -> Result<TasksSectionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/sections/{section_guid}/tasks");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = params.page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = params.page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = params.completed {
            api_req
                .query_params
                .set("completed", if v { "true" } else { "false" });
        }
        if let Some(v) = params.created_from {
            api_req.query_params.set("created_from", v);
        }
        if let Some(v) = params.created_to {
            api_req.query_params.set("created_to", v);
        }
        if let Some(v) = params.user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TaskV2ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(TasksSectionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Tasklist resource ─────────────────────────────────────────────────────────

pub struct TasklistV2Resource<'a> {
    config: &'a Config,
}

impl<'a> TasklistV2Resource<'a> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTasklistV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/task/v2/tasklists");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TasklistData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateTasklistV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        tasklist_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{tasklist_guid}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TasklistData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetTasklistV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        tasklist_guid: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{tasklist_guid}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TasklistData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchTasklistV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        tasklist_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{tasklist_guid}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteTasklistV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTasklistV2Resp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/task/v2/tasklists");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
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
            transport::request_typed::<TasklistListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListTasklistV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn add_members(
        &self,
        tasklist_guid: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<AddMembersTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{tasklist_guid}/add_members");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TasklistData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(AddMembersTasklistV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn remove_members(
        &self,
        tasklist_guid: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<RemoveMembersTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{tasklist_guid}/remove_members");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TasklistData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RemoveMembersTasklistV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn tasks(
        &self,
        tasklist_guid: &str,
        params: TaskListParams<'_>,
        option: &RequestOption,
    ) -> Result<TasksTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{tasklist_guid}/tasks");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = params.page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = params.page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = params.completed {
            api_req
                .query_params
                .set("completed", if v { "true" } else { "false" });
        }
        if let Some(v) = params.created_from {
            api_req.query_params.set("created_from", v);
        }
        if let Some(v) = params.created_to {
            api_req.query_params.set("created_to", v);
        }
        if let Some(v) = params.user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TaskV2ListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(TasksTasklistV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn create_activity_subscription(
        &self,
        tasklist_guid: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateActivitySubscriptionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ActivitySubscriptionData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateActivitySubscriptionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_activity_subscription(
        &self,
        tasklist_guid: &str,
        activity_subscription_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetActivitySubscriptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{activity_subscription_guid}"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ActivitySubscriptionData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetActivitySubscriptionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch_activity_subscription(
        &self,
        tasklist_guid: &str,
        activity_subscription_guid: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchActivitySubscriptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{activity_subscription_guid}"
        );
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ActivitySubscriptionData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchActivitySubscriptionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list_activity_subscriptions(
        &self,
        tasklist_guid: &str,
        limit: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListActivitySubscriptionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = limit {
            api_req.query_params.set("limit", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ActivitySubscriptionListData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListActivitySubscriptionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete_activity_subscription(
        &self,
        tasklist_guid: &str,
        activity_subscription_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteActivitySubscriptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{activity_subscription_guid}"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteActivitySubscriptionV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}
