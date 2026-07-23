use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VerificationTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VerificationDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usci: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_person_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_letter: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateVerificationTaskReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VerificationTaskData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<VerificationTask>,
}

impl_resp!(CreateVerificationTaskResp, VerificationTaskData);
impl_resp!(GetVerificationTaskResp, VerificationTaskData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VerificationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<VerificationDetail>,
}

impl_resp!(GetVerificationResp, VerificationData);

#[derive(Debug, Clone, Copy)]
pub struct CreateVerificationTaskQuery<'a> {
    pub user_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub body: &'a CreateVerificationTaskReqBody,
}

impl<'a> CreateVerificationTaskQuery<'a> {
    pub fn new(user_id: &'a str, body: &'a CreateVerificationTaskReqBody) -> Self {
        Self {
            user_id,
            user_id_type: None,
            body,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetVerificationTaskQuery<'a> {
    pub task_id: &'a str,
}

impl<'a> GetVerificationTaskQuery<'a> {
    pub fn new(task_id: &'a str) -> Self {
        Self { task_id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GetVerificationQuery;

impl GetVerificationQuery {
    pub fn new() -> Self {
        Self
    }
}

// ── Resources ──

pub struct VerificationTaskResource<'a> {
    config: &'a Config,
}

impl<'a> VerificationTaskResource<'a> {
    pub async fn create(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        body: &CreateVerificationTaskReqBody,
        option: &RequestOption,
    ) -> Result<CreateVerificationTaskResp, LarkError> {
        self.create_by_query(
            &CreateVerificationTaskQuery::new(user_id, body).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateVerificationTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateVerificationTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/verification/v1/verification_tasks",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id", query.user_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<VerificationTaskData, CreateVerificationTaskResp>()
        .await
    }

    pub async fn get(
        &self,
        task_id: &str,
        option: &RequestOption,
    ) -> Result<GetVerificationTaskResp, LarkError> {
        self.get_by_query(&GetVerificationTaskQuery::new(task_id), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetVerificationTaskQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetVerificationTaskResp, LarkError> {
        let path = format!(
            "/open-apis/verification/v1/verification_tasks/{}",
            query.task_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<VerificationTaskData, GetVerificationTaskResp>()
        .await
    }
}

// ── Version struct ──

pub struct VerificationResource<'a> {
    config: &'a Config,
}

impl<'a> VerificationResource<'a> {
    pub async fn get(&self, option: &RequestOption) -> Result<GetVerificationResp, LarkError> {
        self.get_by_query(&GetVerificationQuery::new(), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        _query: &GetVerificationQuery,
        option: &RequestOption,
    ) -> Result<GetVerificationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/verification/v1/verification",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_response::<VerificationData, GetVerificationResp>()
        .await
    }
}

pub struct V1<'a> {
    pub verification: VerificationResource<'a>,
    pub verification_task: VerificationTaskResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            verification: VerificationResource { config },
            verification_task: VerificationTaskResource { config },
        }
    }
}
