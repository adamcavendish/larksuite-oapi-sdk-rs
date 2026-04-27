use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::ApiResp;
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Board {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whiteboard_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBoardReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whiteboard: Option<Board>,
}

// ── Request body types (whiteboard theme) ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateThemeWhiteboardReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<serde_json::Value>,
}

// ── Request body types (whiteboard node) ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePlantumlWhiteboardNodeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<serde_json::Value>,
}

impl_resp!(CreateBoardResp, BoardData);
impl_resp!(GetBoardResp, BoardData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<serde_json::Value>,
}

impl_resp!(ThemeWhiteboardResp, ThemeData);
impl_resp!(UpdateThemeWhiteboardResp, serde_json::Value);

#[derive(Debug, Clone)]
pub struct DownloadAsImageWhiteboardResp {
    pub api_resp: ApiResp,
    pub file_name: Option<String>,
    pub data: Vec<u8>,
}

impl DownloadAsImageWhiteboardResp {
    pub fn success(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WhiteboardNodeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<serde_json::Value>,
}

impl_resp!(CreateWhiteboardNodeResp, WhiteboardNodeData);
impl_resp!(CreatePlantumlWhiteboardNodeResp, WhiteboardNodeData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WhiteboardNodeListData {
    #[serde(default)]
    pub nodes: Vec<serde_json::Value>,
}

impl_resp!(ListWhiteboardNodeResp, WhiteboardNodeListData);

// ── Resources ──

pub struct WhiteboardResource<'a> {
    config: &'a Config,
}

impl<'a> WhiteboardResource<'a> {
    pub async fn create(
        &self,
        body: &CreateBoardReqBody,
        option: &RequestOption,
    ) -> Result<CreateBoardResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/board/v1/whiteboards");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BoardData>(self.config, &api_req, option).await?;
        Ok(CreateBoardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        whiteboard_id: &str,
        option: &RequestOption,
    ) -> Result<GetBoardResp, LarkError> {
        let path = format!("/open-apis/board/v1/whiteboards/{whiteboard_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<BoardData>(self.config, &api_req, option).await?;
        Ok(GetBoardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn download_as_image(
        &self,
        whiteboard_id: &str,
        option: &RequestOption,
    ) -> Result<DownloadAsImageWhiteboardResp, LarkError> {
        let path = format!("/open-apis/board/v1/whiteboards/{whiteboard_id}/download_as_image");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let mut opt = option.clone();
        opt.file_download = true;
        let api_resp = transport::request(self.config, &api_req, &opt).await?;
        let file_name = api_resp.file_name_by_header();
        let data = api_resp.raw_body.clone();
        Ok(DownloadAsImageWhiteboardResp {
            api_resp,
            file_name,
            data,
        })
    }

    pub async fn theme(
        &self,
        whiteboard_id: &str,
        option: &RequestOption,
    ) -> Result<ThemeWhiteboardResp, LarkError> {
        let path = format!("/open-apis/board/v1/whiteboards/{whiteboard_id}/theme");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<ThemeData>(self.config, &api_req, option).await?;
        Ok(ThemeWhiteboardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update_theme(
        &self,
        whiteboard_id: &str,
        body: &UpdateThemeWhiteboardReqBody,
        option: &RequestOption,
    ) -> Result<UpdateThemeWhiteboardResp, LarkError> {
        let path = format!("/open-apis/board/v1/whiteboards/{whiteboard_id}/update_theme");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(UpdateThemeWhiteboardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct WhiteboardNodeResource<'a> {
    config: &'a Config,
}

impl<'a> WhiteboardNodeResource<'a> {
    pub async fn create(
        &self,
        whiteboard_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateWhiteboardNodeResp, LarkError> {
        let path = format!("/open-apis/board/v1/whiteboards/{whiteboard_id}/nodes");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<WhiteboardNodeData>(self.config, &api_req, option).await?;
        Ok(CreateWhiteboardNodeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn create_plantuml(
        &self,
        whiteboard_id: &str,
        body: &CreatePlantumlWhiteboardNodeReqBody,
        option: &RequestOption,
    ) -> Result<CreatePlantumlWhiteboardNodeResp, LarkError> {
        let path = format!("/open-apis/board/v1/whiteboards/{whiteboard_id}/nodes/plantuml");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<WhiteboardNodeData>(self.config, &api_req, option).await?;
        Ok(CreatePlantumlWhiteboardNodeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        whiteboard_id: &str,
        option: &RequestOption,
    ) -> Result<ListWhiteboardNodeResp, LarkError> {
        let path = format!("/open-apis/board/v1/whiteboards/{whiteboard_id}/nodes");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<WhiteboardNodeListData>(self.config, &api_req, option)
                .await?;
        Ok(ListWhiteboardNodeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub whiteboard: WhiteboardResource<'a>,
    pub whiteboard_node: WhiteboardNodeResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            whiteboard: WhiteboardResource { config },
            whiteboard_node: WhiteboardNodeResource { config },
        }
    }
}
