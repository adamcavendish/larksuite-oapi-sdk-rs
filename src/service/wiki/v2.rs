use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

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

// ── Resources ──

pub struct SpaceResource<'a> {
    config: &'a Config,
}

impl<'a> SpaceResource<'a> {
    pub async fn create(
        &self,
        body: &CreateSpaceReqBody,
        option: &RequestOption,
    ) -> Result<CreateSpaceResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/wiki/v2/spaces");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SpaceData>(self.config, &api_req, option).await?;
        Ok(CreateSpaceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        space_id: &str,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetSpaceResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = lang {
            api_req.query_params.set("lang", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<SpaceData>(self.config, &api_req, option).await?;
        Ok(GetSpaceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSpaceResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/wiki/v2/spaces");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<SpaceListData>(self.config, &api_req, option).await?;
        Ok(ListSpaceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<CreateNodeResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<NodeData>(self.config, &api_req, option).await?;
        Ok(CreateNodeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        space_id: &str,
        node_token: &str,
        obj_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetNodeResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = obj_type {
            api_req.query_params.set("obj_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<NodeData>(self.config, &api_req, option).await?;
        Ok(GetNodeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        space_id: &str,
        node_token: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn move_node(
        &self,
        space_id: &str,
        node_token: &str,
        body: &MoveNodeReqBody,
        option: &RequestOption,
    ) -> Result<MoveNodeResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/move");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<NodeData>(self.config, &api_req, option).await?;
        Ok(MoveNodeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update_title(
        &self,
        space_id: &str,
        node_token: &str,
        body: &UpdateNodeTitleReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/update_title");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn copy(
        &self,
        space_id: &str,
        node_token: &str,
        body: &CopyNodeReqBody,
        option: &RequestOption,
    ) -> Result<CopyNodeResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/copy");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<NodeData>(self.config, &api_req, option).await?;
        Ok(CopyNodeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        space_id: &str,
        parent_node_token: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListNodeResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/nodes");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = parent_node_token {
            api_req.query_params.set("parent_node_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<NodeListData>(self.config, &api_req, option).await?;
        Ok(ListNodeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/members");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete(
        &self,
        space_id: &str,
        body: &DeleteMemberReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/members/batch_delete");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        space_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMemberResp> {
        let path = format!("/open-apis/wiki/v2/spaces/{space_id}/members");
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
        Ok(ListMemberResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V2<'a> {
    pub space: SpaceResource<'a>,
    pub node: NodeResource<'a>,
    pub member: SpaceMemberResource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            space: SpaceResource { config },
            node: NodeResource { config },
            member: SpaceMemberResource { config },
        }
    }
}
