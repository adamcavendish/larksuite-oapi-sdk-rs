use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_status_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_title: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_setting: Option<serde_json::Value>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSystemStatusReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_status: Option<SystemStatus>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchOpenSystemStatusReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<serde_json::Value>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_status: Option<SystemStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusListData {
    #[serde(default)]
    pub items: Vec<SystemStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchOpenData {
    #[serde(default)]
    pub result_list: Vec<serde_json::Value>,
}

impl_resp!(CreateSystemStatusResp, SystemStatusData);
impl_resp!(ListSystemStatusResp, SystemStatusListData);
impl_resp!(BatchOpenSystemStatusResp, BatchOpenData);
impl_resp!(PatchSystemStatusResp, SystemStatusData);

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateSystemStatusQuery<'a> {
    pub body: &'a CreateSystemStatusReqBody,
}

impl<'a> CreateSystemStatusQuery<'a> {
    pub fn new(body: &'a CreateSystemStatusReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteSystemStatusQuery<'a> {
    pub system_status_id: &'a str,
}

impl<'a> DeleteSystemStatusQuery<'a> {
    pub fn new(system_status_id: &'a str) -> Self {
        Self { system_status_id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListSystemStatusQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListSystemStatusQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BatchOpenSystemStatusQuery<'a> {
    pub system_status_id: &'a str,
    pub body: &'a BatchOpenSystemStatusReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchOpenSystemStatusQuery<'a> {
    pub fn new(system_status_id: &'a str, body: &'a BatchOpenSystemStatusReqBody) -> Self {
        Self {
            system_status_id,
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
#[non_exhaustive]
pub struct BatchCloseSystemStatusQuery<'a> {
    pub system_status_id: &'a str,
    pub body: &'a BatchOpenSystemStatusReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchCloseSystemStatusQuery<'a> {
    pub fn new(system_status_id: &'a str, body: &'a BatchOpenSystemStatusReqBody) -> Self {
        Self {
            system_status_id,
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
#[non_exhaustive]
pub struct PatchSystemStatusQuery<'a> {
    pub system_status_id: &'a str,
    pub body: &'a CreateSystemStatusReqBody,
}

impl<'a> PatchSystemStatusQuery<'a> {
    pub fn new(system_status_id: &'a str, body: &'a CreateSystemStatusReqBody) -> Self {
        Self {
            system_status_id,
            body,
        }
    }
}

// ── Resources ──

pub struct SystemStatusResource<'a> {
    config: &'a Config,
}

impl<'a> SystemStatusResource<'a> {
    pub async fn create(
        &self,
        body: &CreateSystemStatusReqBody,
        option: &RequestOption,
    ) -> Result<CreateSystemStatusResp, LarkError> {
        let query = CreateSystemStatusQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateSystemStatusQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateSystemStatusResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/personal_settings/v1/system_statuses",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send::<SystemStatusData>()
        .await?;
        Ok(CreateSystemStatusResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        system_status_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteSystemStatusQuery::new(system_status_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteSystemStatusQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}",
            query.system_status_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
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
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSystemStatusResp, LarkError> {
        let query = ListSystemStatusQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListSystemStatusQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListSystemStatusResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/personal_settings/v1/system_statuses",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send::<SystemStatusListData>()
        .await?;
        Ok(ListSystemStatusResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_open(
        &self,
        system_status_id: &str,
        body: &BatchOpenSystemStatusReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchOpenSystemStatusResp, LarkError> {
        let query =
            BatchOpenSystemStatusQuery::new(system_status_id, body).user_id_type(user_id_type);
        self.batch_open_by_query(&query, option).await
    }

    pub async fn batch_open_by_query(
        &self,
        query: &BatchOpenSystemStatusQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchOpenSystemStatusResp, LarkError> {
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}/batch_open",
            query.system_status_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<BatchOpenData>()
        .await?;
        Ok(BatchOpenSystemStatusResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_close(
        &self,
        system_status_id: &str,
        body: &BatchOpenSystemStatusReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchOpenSystemStatusResp, LarkError> {
        let query =
            BatchCloseSystemStatusQuery::new(system_status_id, body).user_id_type(user_id_type);
        self.batch_close_by_query(&query, option).await
    }

    pub async fn batch_close_by_query(
        &self,
        query: &BatchCloseSystemStatusQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchOpenSystemStatusResp, LarkError> {
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}/batch_close",
            query.system_status_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<BatchOpenData>()
        .await?;
        Ok(BatchOpenSystemStatusResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        system_status_id: &str,
        body: &CreateSystemStatusReqBody,
        option: &RequestOption,
    ) -> Result<PatchSystemStatusResp, LarkError> {
        let query = PatchSystemStatusQuery::new(system_status_id, body);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchSystemStatusQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchSystemStatusResp, LarkError> {
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}",
            query.system_status_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send::<SystemStatusData>()
        .await?;
        Ok(PatchSystemStatusResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub system_status: SystemStatusResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            system_status: SystemStatusResource { config },
        }
    }
}
