use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct App {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_public_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<AppTableFieldDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableFieldDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_sync: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppTableRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dashboard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateAppReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_advanced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTableReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<AppTable>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateTableReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<AppTable>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchTableReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateViewReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchViewReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFieldReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AppTableFieldDescription>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateFieldReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<AppTableFieldDescription>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchCreateRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<AppTableRecord>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchUpdateRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<AppTableRecord>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchDeleteRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<String>>,
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
pub struct AppData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateTableData {
    #[serde(default)]
    pub table_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableListData {
    #[serde(default)]
    pub items: Vec<AppTable>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<AppTableView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewListData {
    #[serde(default)]
    pub items: Vec<AppTableView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<AppTableField>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldListData {
    #[serde(default)]
    pub items: Vec<AppTableField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<AppTableRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordListData {
    #[serde(default)]
    pub items: Vec<AppTableRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchCreateRecordData {
    #[serde(default)]
    pub records: Vec<AppTableRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchUpdateRecordData {
    #[serde(default)]
    pub records: Vec<AppTableRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchDeleteRecordData {
    #[serde(default)]
    pub records: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardListData {
    #[serde(default)]
    pub dashboards: Vec<Dashboard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(GetAppResp, AppData);
impl_resp!(UpdateAppResp, AppData);
impl_resp!(CreateTableResp, TableData);
impl_resp!(BatchCreateTableResp, BatchCreateTableData);
impl_resp!(ListTableResp, TableListData);
impl_resp!(CreateViewResp, ViewData);
impl_resp!(GetViewResp, ViewData);
impl_resp!(PatchViewResp, ViewData);
impl_resp!(ListViewResp, ViewListData);
impl_resp!(CreateFieldResp, FieldData);
impl_resp!(UpdateFieldResp, FieldData);
impl_resp!(ListFieldResp, FieldListData);
impl_resp!(CreateRecordResp, RecordData);
impl_resp!(UpdateRecordResp, RecordData);
impl_resp!(GetRecordResp, RecordData);
impl_resp!(ListRecordResp, RecordListData);
impl_resp!(BatchCreateRecordResp, BatchCreateRecordData);
impl_resp!(BatchUpdateRecordResp, BatchUpdateRecordData);
impl_resp!(BatchDeleteRecordResp, BatchDeleteRecordData);
impl_resp!(ListDashboardResp, DashboardListData);

// ── Resources ──

pub struct AppResource<'a> {
    config: &'a Config,
}

impl<'a> AppResource<'a> {
    pub async fn get(&self, app_token: &str, option: &RequestOption) -> Result<GetAppResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<AppData>(self.config, &api_req, option).await?;
        Ok(GetAppResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        app_token: &str,
        body: &UpdateAppReqBody,
        option: &RequestOption,
    ) -> Result<UpdateAppResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<AppData>(self.config, &api_req, option).await?;
        Ok(UpdateAppResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppTableResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableResource<'a> {
    pub async fn create(
        &self,
        app_token: &str,
        body: &CreateTableReqBody,
        option: &RequestOption,
    ) -> Result<CreateTableResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TableData>(self.config, &api_req, option).await?;
        Ok(CreateTableResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_create(
        &self,
        app_token: &str,
        body: &BatchCreateTableReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchCreateTableResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/batch_create");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchCreateTableData>(self.config, &api_req, option).await?;
        Ok(BatchCreateTableResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        app_token: &str,
        table_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn batch_delete(
        &self,
        app_token: &str,
        table_ids: &[&str],
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/batch_delete");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let body = serde_json::json!({ "table_ids": table_ids });
        api_req.body = Some(ReqBody::Json(body));
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn patch(
        &self,
        app_token: &str,
        table_id: &str,
        body: &PatchTableReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
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
        app_token: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTableResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<TableListData>(self.config, &api_req, option).await?;
        Ok(ListTableResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppTableViewResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableViewResource<'a> {
    pub async fn create(
        &self,
        app_token: &str,
        table_id: &str,
        body: &CreateViewReqBody,
        option: &RequestOption,
    ) -> Result<CreateViewResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ViewData>(self.config, &api_req, option).await?;
        Ok(CreateViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        app_token: &str,
        table_id: &str,
        view_id: &str,
        option: &RequestOption,
    ) -> Result<GetViewResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<ViewData>(self.config, &api_req, option).await?;
        Ok(GetViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        app_token: &str,
        table_id: &str,
        view_id: &str,
        body: &PatchViewReqBody,
        option: &RequestOption,
    ) -> Result<PatchViewResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ViewData>(self.config, &api_req, option).await?;
        Ok(PatchViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        app_token: &str,
        table_id: &str,
        view_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}");
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
        app_token: &str,
        table_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListViewResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views");
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
            transport::request_typed::<ViewListData>(self.config, &api_req, option).await?;
        Ok(ListViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppTableFieldResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableFieldResource<'a> {
    pub async fn create(
        &self,
        app_token: &str,
        table_id: &str,
        body: &CreateFieldReqBody,
        option: &RequestOption,
    ) -> Result<CreateFieldResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FieldData>(self.config, &api_req, option).await?;
        Ok(CreateFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        app_token: &str,
        table_id: &str,
        field_id: &str,
        body: &UpdateFieldReqBody,
        option: &RequestOption,
    ) -> Result<UpdateFieldResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FieldData>(self.config, &api_req, option).await?;
        Ok(UpdateFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        app_token: &str,
        table_id: &str,
        field_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
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
        app_token: &str,
        table_id: &str,
        view_id: Option<&str>,
        text_field_as_array: Option<bool>,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListFieldResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = view_id {
            api_req.query_params.set("view_id", v);
        }
        if let Some(v) = text_field_as_array {
            api_req
                .query_params
                .set("text_field_as_array", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<FieldListData>(self.config, &api_req, option).await?;
        Ok(ListFieldResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppTableRecordResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableRecordResource<'a> {
    pub async fn create(
        &self,
        app_token: &str,
        table_id: &str,
        body: &CreateRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateRecordResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<RecordData>(self.config, &api_req, option).await?;
        Ok(CreateRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        app_token: &str,
        table_id: &str,
        record_id: &str,
        body: &UpdateRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateRecordResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<RecordData>(self.config, &api_req, option).await?;
        Ok(UpdateRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        app_token: &str,
        table_id: &str,
        record_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetRecordResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<RecordData>(self.config, &api_req, option).await?;
        Ok(GetRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        app_token: &str,
        table_id: &str,
        record_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
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
        app_token: &str,
        table_id: &str,
        view_id: Option<&str>,
        filter: Option<&str>,
        sort: Option<&str>,
        field_names: Option<&str>,
        text_field_as_array: Option<bool>,
        user_id_type: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListRecordResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = view_id {
            api_req.query_params.set("view_id", v);
        }
        if let Some(v) = filter {
            api_req.query_params.set("filter", v);
        }
        if let Some(v) = sort {
            api_req.query_params.set("sort", v);
        }
        if let Some(v) = field_names {
            api_req.query_params.set("field_names", v);
        }
        if let Some(v) = text_field_as_array {
            api_req
                .query_params
                .set("text_field_as_array", v.to_string());
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<RecordListData>(self.config, &api_req, option).await?;
        Ok(ListRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_create(
        &self,
        app_token: &str,
        table_id: &str,
        body: &BatchCreateRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchCreateRecordResp> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchCreateRecordData>(self.config, &api_req, option)
                .await?;
        Ok(BatchCreateRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_update(
        &self,
        app_token: &str,
        table_id: &str,
        body: &BatchUpdateRecordReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchUpdateRecordResp> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_update"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchUpdateRecordData>(self.config, &api_req, option)
                .await?;
        Ok(BatchUpdateRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_delete(
        &self,
        app_token: &str,
        table_id: &str,
        body: &BatchDeleteRecordReqBody,
        option: &RequestOption,
    ) -> Result<BatchDeleteRecordResp> {
        let path = format!(
            "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_delete"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BatchDeleteRecordData>(self.config, &api_req, option)
                .await?;
        Ok(BatchDeleteRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppDashboardResource<'a> {
    config: &'a Config,
}

impl<'a> AppDashboardResource<'a> {
    pub async fn list(
        &self,
        app_token: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDashboardResp> {
        let path = format!("/open-apis/bitable/v1/apps/{app_token}/dashboards");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DashboardListData>(self.config, &api_req, option).await?;
        Ok(ListDashboardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub app: AppResource<'a>,
    pub table: AppTableResource<'a>,
    pub view: AppTableViewResource<'a>,
    pub field: AppTableFieldResource<'a>,
    pub record: AppTableRecordResource<'a>,
    pub dashboard: AppDashboardResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app: AppResource { config },
            table: AppTableResource { config },
            view: AppTableViewResource { config },
            field: AppTableFieldResource { config },
            record: AppTableRecordResource { config },
            dashboard: AppDashboardResource { config },
        }
    }
}
