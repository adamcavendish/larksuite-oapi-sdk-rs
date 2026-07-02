use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

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
impl_resp!(BatchDeleteCollaboratorResp, serde_json::Value);
impl_resp!(BatchDeleteFollowerResp, serde_json::Value);

// ── Resources ──

pub struct TaskResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ListTaskQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub start_create_time: Option<&'a str>,
    pub end_create_time: Option<&'a str>,
    pub task_completed: Option<bool>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListTaskQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub(crate) fn page_query(&self) -> PageQuery<'a> {
        PageQuery::from_parts(self.page_size, self.page_token)
    }

    pub fn start_create_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.start_create_time = value.into();
        self
    }

    pub fn end_create_time(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.end_create_time = value.into();
        self
    }

    pub fn task_completed(mut self, value: impl Into<Option<bool>>) -> Self {
        self.task_completed = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateTaskQuery<'a> {
    pub body: &'a CreateTaskReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateTaskQuery<'a> {
    pub fn new(body: &'a CreateTaskReqBody) -> Self {
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
pub struct GetTaskQuery<'a> {
    pub task_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetTaskQuery<'a> {
    pub fn new(task_id: &'a str) -> Self {
        Self {
            task_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PatchTaskQuery<'a> {
    pub task_id: &'a str,
    pub body: &'a PatchTaskReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchTaskQuery<'a> {
    pub fn new(task_id: &'a str, body: &'a PatchTaskReqBody) -> Self {
        Self {
            task_id,
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
pub struct DeleteTaskQuery<'a> {
    pub task_id: &'a str,
}

impl<'a> DeleteTaskQuery<'a> {
    pub fn new(task_id: &'a str) -> Self {
        Self { task_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CompleteTaskQuery<'a> {
    pub task_id: &'a str,
}

impl<'a> CompleteTaskQuery<'a> {
    pub fn new(task_id: &'a str) -> Self {
        Self { task_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UncompleteTaskQuery<'a> {
    pub task_id: &'a str,
}

impl<'a> UncompleteTaskQuery<'a> {
    pub fn new(task_id: &'a str) -> Self {
        Self { task_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BatchDeleteTaskMemberQuery<'a> {
    pub task_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchDeleteTaskMemberQuery<'a> {
    pub fn new(task_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            task_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

pub type BatchDeleteCollaboratorQuery<'a> = BatchDeleteTaskMemberQuery<'a>;
pub type BatchDeleteFollowerQuery<'a> = BatchDeleteTaskMemberQuery<'a>;

#[derive(Debug, Clone, Copy)]
pub struct CreateCommentQuery<'a> {
    pub task_id: &'a str,
    pub body: &'a CreateCommentReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateCommentQuery<'a> {
    pub fn new(task_id: &'a str, body: &'a CreateCommentReqBody) -> Self {
        Self {
            task_id,
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
pub struct GetCommentQuery<'a> {
    pub task_id: &'a str,
    pub comment_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetCommentQuery<'a> {
    pub fn new(task_id: &'a str, comment_id: &'a str) -> Self {
        Self {
            task_id,
            comment_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateCommentQuery<'a> {
    pub task_id: &'a str,
    pub comment_id: &'a str,
    pub body: &'a UpdateCommentReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateCommentQuery<'a> {
    pub fn new(task_id: &'a str, comment_id: &'a str, body: &'a UpdateCommentReqBody) -> Self {
        Self {
            task_id,
            comment_id,
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
pub struct DeleteCommentQuery<'a> {
    pub task_id: &'a str,
    pub comment_id: &'a str,
}

impl<'a> DeleteCommentQuery<'a> {
    pub fn new(task_id: &'a str, comment_id: &'a str) -> Self {
        Self {
            task_id,
            comment_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListCommentQuery<'a> {
    pub task_id: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListCommentQuery<'a> {
    pub fn new(task_id: &'a str) -> Self {
        Self {
            task_id,
            page: PageQuery::default(),
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateFollowerQuery<'a> {
    pub task_id: &'a str,
    pub body: &'a CreateFollowerReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateFollowerQuery<'a> {
    pub fn new(task_id: &'a str, body: &'a CreateFollowerReqBody) -> Self {
        Self {
            task_id,
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
pub struct DeleteFollowerQuery<'a> {
    pub task_id: &'a str,
    pub follower_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> DeleteFollowerQuery<'a> {
    pub fn new(task_id: &'a str, follower_id: &'a str) -> Self {
        Self {
            task_id,
            follower_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListFollowerQuery<'a> {
    pub task_id: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListFollowerQuery<'a> {
    pub fn new(task_id: &'a str) -> Self {
        Self {
            task_id,
            page: PageQuery::default(),
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateCollaboratorQuery<'a> {
    pub task_id: &'a str,
    pub body: &'a CreateCollaboratorReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateCollaboratorQuery<'a> {
    pub fn new(task_id: &'a str, body: &'a CreateCollaboratorReqBody) -> Self {
        Self {
            task_id,
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
pub struct DeleteCollaboratorQuery<'a> {
    pub task_id: &'a str,
    pub collaborator_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> DeleteCollaboratorQuery<'a> {
    pub fn new(task_id: &'a str, collaborator_id: &'a str) -> Self {
        Self {
            task_id,
            collaborator_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListCollaboratorQuery<'a> {
    pub task_id: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListCollaboratorQuery<'a> {
    pub fn new(task_id: &'a str) -> Self {
        Self {
            task_id,
            page: PageQuery::default(),
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateReminderQuery<'a> {
    pub task_id: &'a str,
    pub body: &'a CreateReminderReqBody,
}

impl<'a> CreateReminderQuery<'a> {
    pub fn new(task_id: &'a str, body: &'a CreateReminderReqBody) -> Self {
        Self { task_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetReminderQuery<'a> {
    pub task_id: &'a str,
    pub reminder_id: &'a str,
}

impl<'a> GetReminderQuery<'a> {
    pub fn new(task_id: &'a str, reminder_id: &'a str) -> Self {
        Self {
            task_id,
            reminder_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateReminderQuery<'a> {
    pub task_id: &'a str,
    pub reminder_id: &'a str,
    pub body: &'a UpdateReminderReqBody,
}

impl<'a> UpdateReminderQuery<'a> {
    pub fn new(task_id: &'a str, reminder_id: &'a str, body: &'a UpdateReminderReqBody) -> Self {
        Self {
            task_id,
            reminder_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteReminderQuery<'a> {
    pub task_id: &'a str,
    pub reminder_id: &'a str,
}

impl<'a> DeleteReminderQuery<'a> {
    pub fn new(task_id: &'a str, reminder_id: &'a str) -> Self {
        Self {
            task_id,
            reminder_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListReminderQuery<'a> {
    pub task_id: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListReminderQuery<'a> {
    pub fn new(task_id: &'a str) -> Self {
        Self {
            task_id,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

impl<'a> TaskResource<'a> {
    pub async fn create(
        &self,
        body: &CreateTaskReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTaskResp, LarkError> {
        let query = CreateTaskQuery::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateTaskResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/task/v1/tasks",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<TaskData>()
        .await?;
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
    ) -> Result<GetTaskResp, LarkError> {
        let query = GetTaskQuery::new(task_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetTaskResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send::<TaskData>()
        .await?;
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
    ) -> Result<PatchTaskResp, LarkError> {
        let query = PatchTaskQuery::new(task_id, body).user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchTaskResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<TaskData>()
        .await?;
        Ok(PatchTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteTaskQuery::new(task_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn complete(
        &self,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = CompleteTaskQuery::new(task_id);
        self.complete_by_query(&query, option).await
    }

    pub async fn complete_by_query(
        &self,
        query: &CompleteTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/complete", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn uncomplete(
        &self,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = UncompleteTaskQuery::new(task_id);
        self.uncomplete_by_query(&query, option).await
    }

    pub async fn uncomplete_by_query(
        &self,
        query: &UncompleteTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/uncomplete", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;
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
    ) -> Result<ListTaskResp, LarkError> {
        let query = ListTaskQuery::new()
            .page_size(page_size)
            .page_token(page_token)
            .start_create_time(start_create_time)
            .end_create_time(end_create_time)
            .task_completed(task_completed)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListTaskResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v1/tasks",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page_query())
        .query("start_create_time", query.start_create_time)
        .query("end_create_time", query.end_create_time)
        .query("task_completed", query.task_completed)
        .query("user_id_type", query.user_id_type)
        .send::<TaskListData>()
        .await?;
        Ok(ListTaskResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_delete_collaborator(
        &self,
        task_id: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchDeleteCollaboratorResp, LarkError> {
        let query = BatchDeleteCollaboratorQuery::new(task_id, body).user_id_type(user_id_type);
        self.batch_delete_collaborator_by_query(&query, option)
            .await
    }

    pub async fn batch_delete_collaborator_by_query(
        &self,
        query: &BatchDeleteCollaboratorQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchDeleteCollaboratorResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/batch_delete_collaborator",
            query.task_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<serde_json::Value>()
        .await?;
        Ok(BatchDeleteCollaboratorResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_delete_follower(
        &self,
        task_id: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchDeleteFollowerResp, LarkError> {
        let query = BatchDeleteFollowerQuery::new(task_id, body).user_id_type(user_id_type);
        self.batch_delete_follower_by_query(&query, option).await
    }

    pub async fn batch_delete_follower_by_query(
        &self,
        query: &BatchDeleteFollowerQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchDeleteFollowerResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/batch_delete_follower",
            query.task_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<serde_json::Value>()
        .await?;
        Ok(BatchDeleteFollowerResp {
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
    ) -> Result<CreateCommentResp, LarkError> {
        let query = CreateCommentQuery::new(task_id, body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateCommentQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateCommentResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/comments", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<CommentData>()
        .await?;
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
    ) -> Result<GetCommentResp, LarkError> {
        let query = GetCommentQuery::new(task_id, comment_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetCommentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetCommentResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/comments/{}",
            query.task_id, query.comment_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send::<CommentData>()
        .await?;
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
    ) -> Result<UpdateCommentResp, LarkError> {
        let query = UpdateCommentQuery::new(task_id, comment_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateCommentQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateCommentResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/comments/{}",
            query.task_id, query.comment_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<CommentData>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteCommentQuery::new(task_id, comment_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteCommentQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/comments/{}",
            query.task_id, query.comment_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;
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
    ) -> Result<ListCommentResp, LarkError> {
        let query = ListCommentQuery::new(task_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCommentQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCommentResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/comments", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send::<CommentListData>()
        .await?;
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
    ) -> Result<CreateFollowerResp, LarkError> {
        let query = CreateFollowerQuery::new(task_id, body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateFollowerQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateFollowerResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/followers", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<FollowerData>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteFollowerQuery::new(task_id, follower_id).user_id_type(user_id_type);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteFollowerQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/followers/{}",
            query.task_id, query.follower_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send::<serde_json::Value>()
        .await?;
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
    ) -> Result<ListFollowerResp, LarkError> {
        let query = ListFollowerQuery::new(task_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListFollowerQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListFollowerResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/followers", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send::<MemberListData>()
        .await?;
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
    ) -> Result<CreateCollaboratorResp, LarkError> {
        let query = CreateCollaboratorQuery::new(task_id, body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateCollaboratorQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateCollaboratorResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/collaborators", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<CollaboratorData>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query =
            DeleteCollaboratorQuery::new(task_id, collaborator_id).user_id_type(user_id_type);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteCollaboratorQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/collaborators/{}",
            query.task_id, query.collaborator_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send::<serde_json::Value>()
        .await?;
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
    ) -> Result<ListCollaboratorResp, LarkError> {
        let query = ListCollaboratorQuery::new(task_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCollaboratorQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCollaboratorResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/collaborators", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send::<MemberListData>()
        .await?;
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
    ) -> Result<CreateReminderResp, LarkError> {
        let query = CreateReminderQuery::new(task_id, body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateReminderQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateReminderResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/reminders", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send::<ReminderData>()
        .await?;
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
    ) -> Result<GetReminderResp, LarkError> {
        let query = GetReminderQuery::new(task_id, reminder_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetReminderQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetReminderResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/reminders/{}",
            query.task_id, query.reminder_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send::<ReminderData>()
        .await?;
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
    ) -> Result<UpdateReminderResp, LarkError> {
        let query = UpdateReminderQuery::new(task_id, reminder_id, body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateReminderQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateReminderResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/reminders/{}",
            query.task_id, query.reminder_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send::<ReminderData>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteReminderQuery::new(task_id, reminder_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteReminderQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/task/v1/tasks/{}/reminders/{}",
            query.task_id, query.reminder_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send::<serde_json::Value>()
        .await?;
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
    ) -> Result<ListReminderResp, LarkError> {
        let query =
            ListReminderQuery::new(task_id).page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListReminderQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListReminderResp, LarkError> {
        let path = format!("/open-apis/task/v1/tasks/{}/reminders", query.task_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .send::<ReminderListData>()
        .await?;
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
