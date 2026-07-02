use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{EmptyResp, PageQuery, RestRequest};
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

#[derive(Debug, Clone, Copy)]
pub struct CreateDataSourceQuery<'a> {
    pub body: &'a CreateDataSourceReqBody,
}

impl<'a> CreateDataSourceQuery<'a> {
    pub fn new(body: &'a CreateDataSourceReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetDataSourceQuery<'a> {
    pub data_source_id: &'a str,
}

impl<'a> GetDataSourceQuery<'a> {
    pub fn new(data_source_id: &'a str) -> Self {
        Self { data_source_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PatchDataSourceQuery<'a> {
    pub data_source_id: &'a str,
    pub body: &'a PatchDataSourceReqBody,
}

impl<'a> PatchDataSourceQuery<'a> {
    pub fn new(data_source_id: &'a str, body: &'a PatchDataSourceReqBody) -> Self {
        Self {
            data_source_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteDataSourceQuery<'a> {
    pub data_source_id: &'a str,
}

impl<'a> DeleteDataSourceQuery<'a> {
    pub fn new(data_source_id: &'a str) -> Self {
        Self { data_source_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListDataSourceQuery<'a> {
    pub view: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListDataSourceQuery<'a> {
    pub fn new() -> Self {
        Self {
            view: None,
            page: PageQuery::default(),
        }
    }

    pub fn view(mut self, view: impl Into<Option<&'a str>>) -> Self {
        self.view = view.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

impl<'a> Default for ListDataSourceQuery<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateDataRecordQuery<'a> {
    pub data_source_id: &'a str,
    pub body: &'a CreateDataRecordReqBody,
}

impl<'a> CreateDataRecordQuery<'a> {
    pub fn new(data_source_id: &'a str, body: &'a CreateDataRecordReqBody) -> Self {
        Self {
            data_source_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetDataRecordQuery<'a> {
    pub data_source_id: &'a str,
    pub item_id: &'a str,
}

impl<'a> GetDataRecordQuery<'a> {
    pub fn new(data_source_id: &'a str, item_id: &'a str) -> Self {
        Self {
            data_source_id,
            item_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteDataRecordQuery<'a> {
    pub data_source_id: &'a str,
    pub item_id: &'a str,
}

impl<'a> DeleteDataRecordQuery<'a> {
    pub fn new(data_source_id: &'a str, item_id: &'a str) -> Self {
        Self {
            data_source_id,
            item_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateMessageSearchQuery<'a> {
    pub body: &'a SearchMessageReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateMessageSearchQuery<'a> {
    pub fn new(body: &'a SearchMessageReqBody) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateAppSearchQuery<'a> {
    pub body: &'a SearchAppReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateAppSearchQuery<'a> {
    pub fn new(body: &'a SearchAppReqBody) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SearchDocWikiQuery<'a> {
    pub body: &'a SearchDocWikiReqBody,
}

impl<'a> SearchDocWikiQuery<'a> {
    pub fn new(body: &'a SearchDocWikiReqBody) -> Self {
        Self { body }
    }
}

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
    ) -> Result<CreateDataSourceResp, LarkError> {
        self.create_by_query(&CreateDataSourceQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDataSourceQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateDataSourceResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/search/v2/data_sources",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send::<DataSourceData>()
        .await?;
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
    ) -> Result<GetDataSourceResp, LarkError> {
        self.get_by_query(&GetDataSourceQuery::new(data_source_id), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetDataSourceQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDataSourceResp, LarkError> {
        let path = format!("/open-apis/search/v2/data_sources/{}", query.data_source_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send::<DataSourceData>()
        .await?;
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
    ) -> Result<PatchDataSourceResp, LarkError> {
        self.patch_by_query(&PatchDataSourceQuery::new(data_source_id, body), option)
            .await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchDataSourceQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchDataSourceResp, LarkError> {
        let path = format!("/open-apis/search/v2/data_sources/{}", query.data_source_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send::<DataSourceData>()
        .await?;
        Ok(PatchDataSourceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        data_source_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        self.delete_by_query(&DeleteDataSourceQuery::new(data_source_id), option)
            .await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteDataSourceQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/search/v2/data_sources/{}", query.data_source_id);
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
        view: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDataSourceResp, LarkError> {
        let query = ListDataSourceQuery::new()
            .view(view)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListDataSourceQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListDataSourceResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/search/v2/data_sources",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("view", query.view)
        .page_query(query.page)
        .send::<DataSourceListData>()
        .await?;
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
    ) -> Result<CreateDataRecordResp, LarkError> {
        self.create_by_query(&CreateDataRecordQuery::new(data_source_id, body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDataRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateDataRecordResp, LarkError> {
        let path = format!(
            "/open-apis/search/v2/data_sources/{}/items",
            query.data_source_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send::<DataRecordData>()
        .await?;
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
    ) -> Result<GetDataRecordResp, LarkError> {
        self.get_by_query(&GetDataRecordQuery::new(data_source_id, item_id), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetDataRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDataRecordResp, LarkError> {
        let path = format!(
            "/open-apis/search/v2/data_sources/{}/items/{}",
            query.data_source_id, query.item_id
        );
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send::<DataRecordData>()
        .await?;
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
    ) -> Result<EmptyResp, LarkError> {
        self.delete_by_query(&DeleteDataRecordQuery::new(data_source_id, item_id), option)
            .await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteDataRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/search/v2/data_sources/{}/items/{}",
            query.data_source_id, query.item_id
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
    ) -> Result<SearchMessageResp, LarkError> {
        self.create_by_query(
            &CreateMessageSearchQuery::new(body).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateMessageSearchQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchMessageResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/search/v2/message",
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<SearchMessageData>()
        .await?;
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
    ) -> Result<SearchAppResp, LarkError> {
        self.create_by_query(
            &CreateAppSearchQuery::new(body).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateAppSearchQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchAppResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/search/v2/app",
            vec![AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send::<SearchAppData>()
        .await?;
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
    ) -> Result<CreateSchemaResp, LarkError> {
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
    ) -> Result<GetSchemaResp, LarkError> {
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
    ) -> Result<PatchSchemaResp, LarkError> {
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
    ) -> Result<EmptyResp, LarkError> {
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
    ) -> Result<CreateSchemaResp, LarkError> {
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

    pub async fn delete(
        &self,
        schema_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
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

    pub async fn get(
        &self,
        schema_id: &str,
        option: &RequestOption,
    ) -> Result<GetSchemaResp, LarkError> {
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
    ) -> Result<PatchSchemaResp, LarkError> {
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
    ) -> Result<SearchDocWikiResp, LarkError> {
        self.search_by_query(&SearchDocWikiQuery::new(body), option)
            .await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchDocWikiQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchDocWikiResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/search/v2/doc_wiki/search",
            vec![AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send::<SearchDocWikiData>()
        .await?;
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
