use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Task {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<TaskMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<TaskMember>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<Vec<TaskMember>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<Reminder>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub richtext_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<Href>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<TaskOrigin>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskDue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Reminder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_fire_minute: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_notify_app_center: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_notify_lark: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Href {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskOrigin {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform_i18n_name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<Href>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Comment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_milli_time: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTaskReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TaskOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<TaskMember>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<Vec<TaskMember>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TaskDue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<Reminder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub richtext_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Href>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchTaskReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCommentReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_milli_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateCommentReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFollowerReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCollaboratorReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateReminderReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_fire_minute: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_notify_app_center: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_notify_lark: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateReminderReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_fire_minute: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_notify_app_center: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_notify_lark: Option<bool>,
}

// ── Response wrappers ──

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct EmptyResp {
    pub api_resp: ApiResp,
    pub code_error: CodeError,
}

impl EmptyResp {
    pub fn success(&self) -> bool {
        self.code_error.success()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskListData {
    #[serde(default)]
    pub items: Vec<Task>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentListData {
    #[serde(default)]
    pub items: Vec<Comment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FollowerData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower: Option<TaskMember>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemberListData {
    #[serde(default)]
    pub items: Vec<TaskMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaboratorData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaborator: Option<TaskMember>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReminderData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminder: Option<Reminder>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReminderListData {
    #[serde(default)]
    pub items: Vec<Reminder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(CreateTaskResp, TaskData);
impl_resp!(GetTaskResp, TaskData);
impl_resp!(PatchTaskResp, TaskData);
impl_resp!(ListTaskResp, TaskListData);
impl_resp!(CreateCommentResp, CommentData);
impl_resp!(GetCommentResp, CommentData);
impl_resp!(UpdateCommentResp, CommentData);
impl_resp!(ListCommentResp, CommentListData);
impl_resp!(CreateFollowerResp, FollowerData);
impl_resp!(ListFollowerResp, MemberListData);
impl_resp!(CreateCollaboratorResp, CollaboratorData);
impl_resp!(ListCollaboratorResp, MemberListData);
impl_resp!(CreateReminderResp, ReminderData);
impl_resp!(GetReminderResp, ReminderData);
impl_resp!(UpdateReminderResp, ReminderData);
impl_resp!(ListReminderResp, ReminderListData);

// ── Resources ──

pub struct TaskResource<'a> {
    config: &'a Config,
}

impl<'a> TaskResource<'a> {
    pub async fn create(
        &self,
        body: &CreateTaskReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTaskResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/task/v1/tasks");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskData>(self.config, &api_req, option).await?;
        Ok(CreateTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        task_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetTaskResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TaskData>(self.config, &api_req, option).await?;
        Ok(GetTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        task_id: &str,
        body: &PatchTaskReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchTaskResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaskData>(self.config, &api_req, option).await?;
        Ok(PatchTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(&self, task_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn complete(&self, task_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/complete");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn uncomplete(&self, task_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/uncomplete");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        start_create_time: Option<&str>,
        end_create_time: Option<&str>,
        task_completed: Option<bool>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTaskResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/task/v1/tasks");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = start_create_time {
            api_req.query_params.set("start_create_time", v);
        }
        if let Some(v) = end_create_time {
            api_req.query_params.set("end_create_time", v);
        }
        if let Some(v) = task_completed {
            api_req.query_params.set("task_completed", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TaskListData>(self.config, &api_req, option).await?;
        Ok(ListTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct TaskCommentResource<'a> {
    config: &'a Config,
}

impl<'a> TaskCommentResource<'a> {
    pub async fn create(
        &self,
        task_id: &str,
        body: &CreateCommentReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateCommentResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/comments");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CommentData>(self.config, &api_req, option).await?;
        Ok(CreateCommentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        task_id: &str,
        comment_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetCommentResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/comments/{comment_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<CommentData>(self.config, &api_req, option).await?;
        Ok(GetCommentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        task_id: &str,
        comment_id: &str,
        body: &UpdateCommentReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateCommentResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/comments/{comment_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CommentData>(self.config, &api_req, option).await?;
        Ok(UpdateCommentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        task_id: &str,
        comment_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/comments/{comment_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        task_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCommentResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/comments");
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
            transport::request_typed::<CommentListData>(self.config, &api_req, option).await?;
        Ok(ListCommentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct TaskFollowerResource<'a> {
    config: &'a Config,
}

impl<'a> TaskFollowerResource<'a> {
    pub async fn create(
        &self,
        task_id: &str,
        body: &CreateFollowerReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateFollowerResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/followers");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FollowerData>(self.config, &api_req, option).await?;
        Ok(CreateFollowerResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        task_id: &str,
        follower_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/followers/{follower_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
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

    pub async fn list(
        &self,
        task_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListFollowerResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/followers");
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
            transport::request_typed::<MemberListData>(self.config, &api_req, option).await?;
        Ok(ListFollowerResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct TaskCollaboratorResource<'a> {
    config: &'a Config,
}

impl<'a> TaskCollaboratorResource<'a> {
    pub async fn create(
        &self,
        task_id: &str,
        body: &CreateCollaboratorReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateCollaboratorResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/collaborators");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<CollaboratorData>(self.config, &api_req, option).await?;
        Ok(CreateCollaboratorResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        task_id: &str,
        collaborator_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/collaborators/{collaborator_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
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

    pub async fn list(
        &self,
        task_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCollaboratorResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/collaborators");
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
            transport::request_typed::<MemberListData>(self.config, &api_req, option).await?;
        Ok(ListCollaboratorResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct TaskReminderResource<'a> {
    config: &'a Config,
}

impl<'a> TaskReminderResource<'a> {
    pub async fn create(
        &self,
        task_id: &str,
        body: &CreateReminderReqBody,
        option: &RequestOption,
    ) -> Result<CreateReminderResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/reminders");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ReminderData>(self.config, &api_req, option).await?;
        Ok(CreateReminderResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        task_id: &str,
        reminder_id: &str,
        option: &RequestOption,
    ) -> Result<GetReminderResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/reminders/{reminder_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<ReminderData>(self.config, &api_req, option).await?;
        Ok(GetReminderResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        task_id: &str,
        reminder_id: &str,
        body: &UpdateReminderReqBody,
        option: &RequestOption,
    ) -> Result<UpdateReminderResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/reminders/{reminder_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ReminderData>(self.config, &api_req, option).await?;
        Ok(UpdateReminderResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        task_id: &str,
        reminder_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/reminders/{reminder_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        task_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListReminderResp> {
        let path = format!("/open-apis/task/v1/tasks/{task_id}/reminders");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<ReminderListData>(self.config, &api_req, option).await?;
        Ok(ListReminderResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub task: TaskResource<'a>,
    pub comment: TaskCommentResource<'a>,
    pub follower: TaskFollowerResource<'a>,
    pub collaborator: TaskCollaboratorResource<'a>,
    pub reminder: TaskReminderResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            task: TaskResource { config },
            comment: TaskCommentResource { config },
            follower: TaskFollowerResource { config },
            collaborator: TaskCollaboratorResource { config },
            reminder: TaskReminderResource { config },
        }
    }
}
