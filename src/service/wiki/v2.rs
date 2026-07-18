use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Space {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Node {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_space_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_node_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_child: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_edit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpaceMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSpaceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_sharing: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateNodeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_node_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MoveNodeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parent_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_space_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateNodeTitleReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CopyNodeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parent_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_space_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AddMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<SpaceMember>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_notification: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteMemberReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<SpaceMember>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MoveDocsToWikiReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_wiki_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply: Option<bool>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpaceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space: Option<Space>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpaceListData {
    #[serde(default)]
    pub items: Vec<Space>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<Node>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NodeListData {
    #[serde(default)]
    pub items: Vec<Node>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemberListData {
    #[serde(default)]
    pub members: Vec<SpaceMember>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(CreateSpaceResp, SpaceData);
impl_resp!(GetSpaceResp, SpaceData);
impl_resp!(ListSpaceResp, SpaceListData);
impl_resp!(CreateNodeResp, NodeData);
impl_resp!(GetNodeResp, NodeData);
impl_resp!(MoveNodeResp, NodeData);
impl_resp!(CopyNodeResp, NodeData);
impl_resp!(ListNodeResp, NodeListData);
impl_resp!(ListMemberResp, MemberListData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MoveDocsToWikiData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wiki_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied: Option<bool>,
}

impl_resp!(MoveDocsToWikiResp, MoveDocsToWikiData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<crate::JsonValue>,
}

impl_resp!(GetTaskResp, TaskData);

// ── Resources ──

pub struct SpaceResource<'a> {
    config: &'a Config,
}

impl<'a> SpaceResource<'a> {
    pub async fn create(
        &self,
        body: &CreateSpaceReqBody,
        option: &RequestOption,
    ) -> Result<CreateSpaceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/wiki/v2/spaces",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<SpaceData, CreateSpaceResp>()
        .await
    }

    pub async fn get(
        &self,
        space_id: &str,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetSpaceResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("lang", lang)
        .send_response::<SpaceData, GetSpaceResp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSpaceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/wiki/v2/spaces",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_response::<SpaceListData, ListSpaceResp>()
        .await
    }

    pub async fn get_node(
        &self,
        token: &str,
        obj_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetNodeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/wiki/v2/spaces/get_node",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("token", token)
        .query("obj_type", obj_type)
        .send_response::<NodeData, GetNodeResp>()
        .await
    }
}

pub struct NodeResource<'a> {
    config: &'a Config,
}

impl<'a> NodeResource<'a> {
    pub async fn create(
        &self,
        space_id: &str,
        body: &CreateNodeReqBody,
        option: &RequestOption,
    ) -> Result<CreateNodeResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<NodeData, CreateNodeResp>()
        .await
    }

    pub async fn get(
        &self,
        space_id: &str,
        node_token: &str,
        obj_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetNodeResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("obj_type", obj_type)
        .send_response::<NodeData, GetNodeResp>()
        .await
    }

    pub async fn delete(
        &self,
        space_id: &str,
        node_token: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn move_node(
        &self,
        space_id: &str,
        node_token: &str,
        body: &MoveNodeReqBody,
        option: &RequestOption,
    ) -> Result<MoveNodeResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/move");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<NodeData, MoveNodeResp>()
        .await
    }

    pub async fn update_title(
        &self,
        space_id: &str,
        node_token: &str,
        body: &UpdateNodeTitleReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/update_title");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn copy(
        &self,
        space_id: &str,
        node_token: &str,
        body: &CopyNodeReqBody,
        option: &RequestOption,
    ) -> Result<CopyNodeResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/copy");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<NodeData, CopyNodeResp>()
        .await
    }

    pub async fn list(
        &self,
        space_id: &str,
        parent_node_token: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListNodeResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("parent_node_token", parent_node_token)
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_response::<NodeListData, ListNodeResp>()
        .await
    }

    pub async fn move_docs_to_wiki(
        &self,
        space_id: &str,
        body: &MoveDocsToWikiReqBody,
        option: &RequestOption,
    ) -> Result<MoveDocsToWikiResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/move_docs_to_wiki");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<MoveDocsToWikiData, MoveDocsToWikiResp>()
        .await
    }
}

pub struct SpaceMemberResource<'a> {
    config: &'a Config,
}

impl<'a> SpaceMemberResource<'a> {
    pub async fn add(
        &self,
        space_id: &str,
        body: &AddMemberReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/members");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn delete(
        &self,
        space_id: &str,
        member_id: &str,
        member_type: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/members/{member_id}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("member_type", member_type)
        .send_empty()
        .await
    }

    pub async fn list(
        &self,
        space_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMemberResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/members");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .query("user_id_type", user_id_type)
        .send_response::<MemberListData, ListMemberResp>()
        .await
    }
}

pub struct SpaceSettingResource<'a> {
    config: &'a Config,
}

impl<'a> SpaceSettingResource<'a> {
    pub async fn update(
        &self,
        space_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/setting");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }
}

pub struct TaskResource<'a> {
    config: &'a Config,
}

impl<'a> TaskResource<'a> {
    pub async fn get(
        &self,
        task_id: &str,
        task_type: &str,
        option: &RequestOption,
    ) -> Result<GetTaskResp, LarkError> {
        let path = format!("/open-apis/wiki/v2/tasks/{task_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("task_type", task_type)
        .send_response::<TaskData, GetTaskResp>()
        .await
    }
}

// ── Version struct ──

pub struct V2<'a> {
    pub space: SpaceResource<'a>,
    pub node: NodeResource<'a>,
    pub member: SpaceMemberResource<'a>,
    pub space_setting: SpaceSettingResource<'a>,
    pub task: TaskResource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            space: SpaceResource { config },
            node: NodeResource { config },
            member: SpaceMemberResource { config },
            space_setting: SpaceSettingResource { config },
            task: TaskResource { config },
        }
    }
}
