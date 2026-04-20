use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whiteboard: Option<Board>,
}

impl_resp!(CreateBoardResp, BoardData);
impl_resp!(GetBoardResp, BoardData);

// ── Resources ──

pub struct WhiteboardResource<'a> {
    config: &'a Config,
}

impl<'a> WhiteboardResource<'a> {
    pub async fn create(
        &self,
        body: &CreateBoardReqBody,
        option: &RequestOption,
    ) -> Result<CreateBoardResp> {
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

    pub async fn get(&self, whiteboard_id: &str, option: &RequestOption) -> Result<GetBoardResp> {
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
}

// ── Version struct ──

pub struct V1<'a> {
    pub whiteboard: WhiteboardResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            whiteboard: WhiteboardResource { config },
        }
    }
}
