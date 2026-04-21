use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_exceed_quota: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable_fields: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DataRecordMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structured_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<DataRecordContent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataRecordMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataRecordContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_data: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Schema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SchemaProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<SchemaDisplay>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_searchable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_sortable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_returnable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary_key: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_filterable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer_option: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields_mapping: Option<Vec<serde_json::Value>>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDataSourceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_fields: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchDataSourceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_description: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDataRecordReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DataRecordMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<DataRecordContent>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchMessageReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_chatter_filter_by_member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchAppReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSchemaReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SchemaProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<SchemaDisplay>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchSchemaReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<SchemaDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SchemaProperty>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchDocWikiReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_filter: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_filter: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
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
pub struct DataSourceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataSourceListData {
    #[serde(default)]
    pub items: Vec<DataSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataRecordData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<DataRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessageData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchAppData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchemaData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
}

impl_resp!(CreateDataSourceResp, DataSourceData);
impl_resp!(GetDataSourceResp, DataSourceData);
impl_resp!(PatchDataSourceResp, DataSourceData);
impl_resp!(ListDataSourceResp, DataSourceListData);
impl_resp!(CreateDataRecordResp, DataRecordData);
impl_resp!(GetDataRecordResp, DataRecordData);
impl_resp!(SearchMessageResp, SearchMessageData);
impl_resp!(SearchAppResp, SearchAppData);
impl_resp!(CreateSchemaResp, SchemaData);
impl_resp!(GetSchemaResp, SchemaData);
impl_resp!(PatchSchemaResp, SchemaData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchDocWikiData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub res_units: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl_resp!(SearchDocWikiResp, SearchDocWikiData);

// ── Resources ──

pub struct DataSourceResource<'a> {
    config: &'a Config,
}

impl<'a> DataSourceResource<'a> {
    pub async fn create(
        &self,
        body: &CreateDataSourceReqBody,
        option: &RequestOption,
    ) -> Result<CreateDataSourceResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/search/v2/data_sources");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<DataSourceData>(self.config, &api_req, option).await?;
        Ok(CreateDataSourceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        data_source_id: &str,
        option: &RequestOption,
    ) -> Result<GetDataSourceResp> {
        let path = format!("/open-apis/search/v2/data_sources/{data_source_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<DataSourceData>(self.config, &api_req, option).await?;
        Ok(GetDataSourceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        data_source_id: &str,
        body: &PatchDataSourceReqBody,
        option: &RequestOption,
    ) -> Result<PatchDataSourceResp> {
        let path = format!("/open-apis/search/v2/data_sources/{data_source_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<DataSourceData>(self.config, &api_req, option).await?;
        Ok(PatchDataSourceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(&self, data_source_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/search/v2/data_sources/{data_source_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn list(
        &self,
        view: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDataSourceResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/search/v2/data_sources");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = view {
            api_req.query_params.set("view", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DataSourceListData>(self.config, &api_req, option).await?;
        Ok(ListDataSourceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct DataRecordResource<'a> {
    config: &'a Config,
}

impl<'a> DataRecordResource<'a> {
    pub async fn create(
        &self,
        data_source_id: &str,
        body: &CreateDataRecordReqBody,
        option: &RequestOption,
    ) -> Result<CreateDataRecordResp> {
        let path = format!("/open-apis/search/v2/data_sources/{data_source_id}/items");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<DataRecordData>(self.config, &api_req, option).await?;
        Ok(CreateDataRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        data_source_id: &str,
        item_id: &str,
        option: &RequestOption,
    ) -> Result<GetDataRecordResp> {
        let path = format!("/open-apis/search/v2/data_sources/{data_source_id}/items/{item_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<DataRecordData>(self.config, &api_req, option).await?;
        Ok(GetDataRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        data_source_id: &str,
        item_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/search/v2/data_sources/{data_source_id}/items/{item_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct MessageSearchResource<'a> {
    config: &'a Config,
}

impl<'a> MessageSearchResource<'a> {
    pub async fn create(
        &self,
        body: &SearchMessageReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchMessageResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/search/v2/message");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SearchMessageData>(self.config, &api_req, option).await?;
        Ok(SearchMessageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct AppSearchResource<'a> {
    config: &'a Config,
}

impl<'a> AppSearchResource<'a> {
    pub async fn create(
        &self,
        body: &SearchAppReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchAppResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/search/v2/app");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SearchAppData>(self.config, &api_req, option).await?;
        Ok(SearchAppResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct DataSourceSchemaResource<'a> {
    config: &'a Config,
}

impl<'a> DataSourceSchemaResource<'a> {
    pub async fn create(
        &self,
        data_source_id: &str,
        body: &CreateSchemaReqBody,
        option: &RequestOption,
    ) -> Result<CreateSchemaResp> {
        let path = format!("/open-apis/search/v2/data_sources/{data_source_id}/schemas");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SchemaData>(self.config, &api_req, option).await?;
        Ok(CreateSchemaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        data_source_id: &str,
        schema_id: &str,
        option: &RequestOption,
    ) -> Result<GetSchemaResp> {
        let path =
            format!("/open-apis/search/v2/data_sources/{data_source_id}/schemas/{schema_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<SchemaData>(self.config, &api_req, option).await?;
        Ok(GetSchemaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        data_source_id: &str,
        schema_id: &str,
        body: &PatchSchemaReqBody,
        option: &RequestOption,
    ) -> Result<PatchSchemaResp> {
        let path =
            format!("/open-apis/search/v2/data_sources/{data_source_id}/schemas/{schema_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SchemaData>(self.config, &api_req, option).await?;
        Ok(PatchSchemaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        data_source_id: &str,
        schema_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/search/v2/data_sources/{data_source_id}/schemas/{schema_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }
}

pub struct SchemaResource<'a> {
    config: &'a Config,
}

impl<'a> SchemaResource<'a> {
    pub async fn create(
        &self,
        body: &CreateSchemaReqBody,
        option: &RequestOption,
    ) -> Result<CreateSchemaResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/search/v2/schemas");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SchemaData>(self.config, &api_req, option).await?;
        Ok(CreateSchemaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(&self, schema_id: &str, option: &RequestOption) -> Result<EmptyResp> {
        let path = format!("/open-apis/search/v2/schemas/{schema_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(&self, schema_id: &str, option: &RequestOption) -> Result<GetSchemaResp> {
        let path = format!("/open-apis/search/v2/schemas/{schema_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<SchemaData>(self.config, &api_req, option).await?;
        Ok(GetSchemaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        schema_id: &str,
        body: &PatchSchemaReqBody,
        option: &RequestOption,
    ) -> Result<PatchSchemaResp> {
        let path = format!("/open-apis/search/v2/schemas/{schema_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SchemaData>(self.config, &api_req, option).await?;
        Ok(PatchSchemaResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct DocWikiResource<'a> {
    config: &'a Config,
}

impl<'a> DocWikiResource<'a> {
    pub async fn search(
        &self,
        body: &SearchDocWikiReqBody,
        option: &RequestOption,
    ) -> Result<SearchDocWikiResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/search/v2/doc_wiki/search");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SearchDocWikiData>(self.config, &api_req, option).await?;
        Ok(SearchDocWikiResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct V2<'a> {
    pub data_source: DataSourceResource<'a>,
    pub data_record: DataRecordResource<'a>,
    pub message: MessageSearchResource<'a>,
    pub app: AppSearchResource<'a>,
    pub schema: SchemaResource<'a>,
    pub data_source_schema: DataSourceSchemaResource<'a>,
    pub doc_wiki: DocWikiResource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            data_source: DataSourceResource { config },
            data_record: DataRecordResource { config },
            message: MessageSearchResource { config },
            app: AppSearchResource { config },
            schema: SchemaResource { config },
            data_source_schema: DataSourceSchemaResource { config },
            doc_wiki: DocWikiResource { config },
        }
    }
}
