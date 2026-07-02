use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{FormDataField, ReqBody, RequestOption};
use crate::service::common::{DownloadResp, JsonResp, PageQuery, RestRequest};
use crate::service::go_v397::{GoV397, GoV397Endpoint};

pub type CreateAppResp = JsonResp;
pub type GetAppVisibilityAppResp = JsonResp;
pub type IconAppResp = JsonResp;
pub type ListAppResp = JsonResp;
pub type PatchAppResp = JsonResp;
pub type SqlCommandsAppResp = JsonResp;
pub type UpdateAppVisibilityAppResp = JsonResp;
pub type UploadHtmlCodeAndReleaseAppResp = JsonResp;
pub type GetEnumDetailAppEnumResp = JsonResp;
pub type GetEnumListAppEnumResp = JsonResp;
pub type UploadAppStorageResp = JsonResp;
pub type UploadCompleteAppStorageResp = JsonResp;
pub type UploadInitializeAppStorageResp = JsonResp;
pub type UploadPartAppStorageResp = JsonResp;
pub type BatchUpdateTableRecordsAppTableResp = JsonResp;
pub type DeleteTableRecordsAppTableResp = JsonResp;
pub type GetTableDetailAppTableResp = JsonResp;
pub type GetTableListAppTableResp = JsonResp;
pub type GetTableRecordListAppTableResp = JsonResp;
pub type PatchTableRecordsAppTableResp = JsonResp;
pub type PostTableRecordsAppTableResp = JsonResp;
pub type GetViewRecordListAppViewResp = JsonResp;
pub type IdConvertDirectoryUserResp = JsonResp;

const EMPTY_PARAMS: [(&str, &str); 0] = [];

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct SparkPageQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> SparkPageQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::new();
        if let Some(page_size) = self.page.page_size {
            pairs.push(("page_size", page_size.to_string()));
        }
        if let Some(page_token) = self.page.page_token {
            pairs.push(("page_token", page_token.to_string()));
        }
        pairs
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DownloadAppStorageQuery<'a> {
    pub app_id: &'a str,
    pub file_key: Option<&'a str>,
    pub file_url: Option<&'a str>,
}

impl<'a> DownloadAppStorageQuery<'a> {
    pub fn new(app_id: &'a str) -> Self {
        Self {
            app_id,
            file_key: None,
            file_url: None,
        }
    }

    pub fn file_key(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.file_key = value.into();
        self
    }

    pub fn file_url(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.file_url = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SparkRecordQuery<'a> {
    pub app_id: &'a str,
    pub table_name: &'a str,
    pub page: PageQuery<'a>,
    pub select: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub order: Option<&'a str>,
    pub env: Option<&'a str>,
    pub user_identifier_type: Option<&'a str>,
}

impl<'a> SparkRecordQuery<'a> {
    pub fn new(app_id: &'a str, table_name: &'a str) -> Self {
        Self {
            app_id,
            table_name,
            page: PageQuery::new(),
            select: None,
            filter: None,
            order: None,
            env: None,
            user_identifier_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn select(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.select = value.into();
        self
    }

    pub fn filter(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.filter = value.into();
        self
    }

    pub fn order(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.order = value.into();
        self
    }

    pub fn env(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.env = value.into();
        self
    }

    pub fn user_identifier_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_identifier_type = value.into();
        self
    }

    fn path_pairs(&self) -> [(&'static str, &'a str); 2] {
        [("app_id", self.app_id), ("table_name", self.table_name)]
    }

    fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::new();
        if let Some(page_size) = self.page.page_size {
            pairs.push(("page_size", page_size.to_string()));
        }
        if let Some(page_token) = self.page.page_token {
            pairs.push(("page_token", page_token.to_string()));
        }
        if let Some(select) = self.select {
            pairs.push(("select", select.to_string()));
        }
        if let Some(filter) = self.filter {
            pairs.push(("filter", filter.to_string()));
        }
        if let Some(order) = self.order {
            pairs.push(("order", order.to_string()));
        }
        if let Some(env) = self.env {
            pairs.push(("env", env.to_string()));
        }
        if let Some(user_identifier_type) = self.user_identifier_type {
            pairs.push(("user_identifier_type", user_identifier_type.to_string()));
        }
        pairs
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SparkTableMutationQuery<'a> {
    pub app_id: &'a str,
    pub table_name: &'a str,
    pub body: &'a serde_json::Value,
    pub filter: Option<&'a str>,
    pub env: Option<&'a str>,
    pub columns: Option<&'a str>,
    pub on_conflict: Option<&'a str>,
    pub user_identifier_type: Option<&'a str>,
}

impl<'a> SparkTableMutationQuery<'a> {
    pub fn new(app_id: &'a str, table_name: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            app_id,
            table_name,
            body,
            filter: None,
            env: None,
            columns: None,
            on_conflict: None,
            user_identifier_type: None,
        }
    }

    pub fn filter(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.filter = value.into();
        self
    }

    pub fn env(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.env = value.into();
        self
    }

    pub fn columns(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.columns = value.into();
        self
    }

    pub fn on_conflict(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.on_conflict = value.into();
        self
    }

    pub fn user_identifier_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_identifier_type = value.into();
        self
    }

    fn path_pairs(&self) -> [(&'static str, &'a str); 2] {
        [("app_id", self.app_id), ("table_name", self.table_name)]
    }

    fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::new();
        if let Some(filter) = self.filter {
            pairs.push(("filter", filter.to_string()));
        }
        if let Some(env) = self.env {
            pairs.push(("env", env.to_string()));
        }
        if let Some(columns) = self.columns {
            pairs.push(("columns", columns.to_string()));
        }
        if let Some(on_conflict) = self.on_conflict {
            pairs.push(("on_conflict", on_conflict.to_string()));
        }
        if let Some(user_identifier_type) = self.user_identifier_type {
            pairs.push(("user_identifier_type", user_identifier_type.to_string()));
        }
        pairs
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SparkViewRecordQuery<'a> {
    pub app_id: &'a str,
    pub view_name: &'a str,
    pub page: PageQuery<'a>,
    pub select: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub order: Option<&'a str>,
    pub env: Option<&'a str>,
    pub user_identifier_type: Option<&'a str>,
}

impl<'a> SparkViewRecordQuery<'a> {
    pub fn new(app_id: &'a str, view_name: &'a str) -> Self {
        Self {
            app_id,
            view_name,
            page: PageQuery::new(),
            select: None,
            filter: None,
            order: None,
            env: None,
            user_identifier_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn select(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.select = value.into();
        self
    }

    pub fn filter(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.filter = value.into();
        self
    }

    pub fn order(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.order = value.into();
        self
    }

    pub fn env(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.env = value.into();
        self
    }

    pub fn user_identifier_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_identifier_type = value.into();
        self
    }

    fn path_pairs(&self) -> [(&'static str, &'a str); 2] {
        [("app_id", self.app_id), ("view_name", self.view_name)]
    }

    fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::new();
        if let Some(page_size) = self.page.page_size {
            pairs.push(("page_size", page_size.to_string()));
        }
        if let Some(page_token) = self.page.page_token {
            pairs.push(("page_token", page_token.to_string()));
        }
        if let Some(select) = self.select {
            pairs.push(("select", select.to_string()));
        }
        if let Some(filter) = self.filter {
            pairs.push(("filter", filter.to_string()));
        }
        if let Some(order) = self.order {
            pairs.push(("order", order.to_string()));
        }
        if let Some(env) = self.env {
            pairs.push(("env", env.to_string()));
        }
        if let Some(user_identifier_type) = self.user_identifier_type {
            pairs.push(("user_identifier_type", user_identifier_type.to_string()));
        }
        pairs
    }
}

pub struct AppResource<'a> {
    config: &'a Config,
}

impl<'a> AppResource<'a> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateAppResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PostSparkV1Apps,
                EMPTY_PARAMS,
                EMPTY_PARAMS,
                Some(body),
                option,
            )
            .await
    }

    pub async fn get_app_visibility(
        &self,
        app_id: &str,
        option: &RequestOption,
    ) -> Result<GetAppVisibilityAppResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::GetSparkV1AppsByAppIdAccessScope,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }

    pub async fn icon(
        &self,
        body: Vec<FormDataField>,
        option: &RequestOption,
    ) -> Result<IconAppResp, LarkError> {
        GoV397::new(self.config)
            .request(
                GoV397Endpoint::PostSparkV1Icon,
                EMPTY_PARAMS,
                EMPTY_PARAMS,
                Some(ReqBody::FormData(body)),
                option,
            )
            .await
    }

    pub async fn list(
        &self,
        query: &SparkPageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAppResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::GetSparkV1Apps,
                EMPTY_PARAMS,
                query.as_pairs(),
                None,
                option,
            )
            .await
    }

    pub async fn patch(
        &self,
        app_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchAppResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PatchSparkV1AppsByAppId,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(body),
                option,
            )
            .await
    }

    pub async fn sql_commands(
        &self,
        app_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<SqlCommandsAppResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PostSparkV1AppsByAppIdSqlCommands,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(body),
                option,
            )
            .await
    }

    pub async fn update_app_visibility(
        &self,
        app_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateAppVisibilityAppResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PutSparkV1AppsByAppIdAccessScope,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(body),
                option,
            )
            .await
    }

    pub async fn upload_html_code_and_release(
        &self,
        app_id: &str,
        body: Vec<FormDataField>,
        option: &RequestOption,
    ) -> Result<UploadHtmlCodeAndReleaseAppResp, LarkError> {
        GoV397::new(self.config)
            .request(
                GoV397Endpoint::PostSparkV1AppsByAppIdUploadAndReleaseHtmlCode,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(ReqBody::FormData(body)),
                option,
            )
            .await
    }
}

pub struct AppEnumResource<'a> {
    config: &'a Config,
}

impl<'a> AppEnumResource<'a> {
    pub async fn get_enum_detail(
        &self,
        app_id: &str,
        enum_name: &str,
        option: &RequestOption,
    ) -> Result<GetEnumDetailAppEnumResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::GetSparkV1AppsByAppIdEnumsByEnumName,
                [("app_id", app_id), ("enum_name", enum_name)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }

    pub async fn get_enum_list(
        &self,
        app_id: &str,
        option: &RequestOption,
    ) -> Result<GetEnumListAppEnumResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::GetSparkV1AppsByAppIdEnums,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }
}

pub struct AppStorageResource<'a> {
    config: &'a Config,
}

impl<'a> AppStorageResource<'a> {
    pub async fn download_by_query(
        &self,
        query: &DownloadAppStorageQuery<'_>,
        option: &RequestOption,
    ) -> Result<DownloadResp, LarkError> {
        let path = format!("/open-apis/spark/v1/apps/{}/storage", query.app_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("file_key", query.file_key)
        .query("file_url", query.file_url)
        .download()
        .await
    }

    pub async fn upload(
        &self,
        app_id: &str,
        body: Vec<FormDataField>,
        option: &RequestOption,
    ) -> Result<UploadAppStorageResp, LarkError> {
        GoV397::new(self.config)
            .request(
                GoV397Endpoint::PostSparkV1AppsByAppIdStorageUpload,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(ReqBody::FormData(body)),
                option,
            )
            .await
    }

    pub async fn upload_complete(
        &self,
        app_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadCompleteAppStorageResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PostSparkV1AppsByAppIdStorageUploadComplete,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(body),
                option,
            )
            .await
    }

    pub async fn upload_initialize(
        &self,
        app_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadInitializeAppStorageResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PostSparkV1AppsByAppIdStorageUploadInitialize,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(body),
                option,
            )
            .await
    }

    pub async fn upload_part(
        &self,
        app_id: &str,
        body: Vec<FormDataField>,
        option: &RequestOption,
    ) -> Result<UploadPartAppStorageResp, LarkError> {
        GoV397::new(self.config)
            .request(
                GoV397Endpoint::PostSparkV1AppsByAppIdStorageUploadPart,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                Some(ReqBody::FormData(body)),
                option,
            )
            .await
    }
}

pub struct AppTableResource<'a> {
    config: &'a Config,
}

impl<'a> AppTableResource<'a> {
    pub async fn batch_update_table_records(
        &self,
        query: &SparkTableMutationQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchUpdateTableRecordsAppTableResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PatchSparkV1AppsByAppIdTablesByTableNameRecordsBatchUpdate,
                query.path_pairs(),
                query.query_pairs(),
                Some(query.body),
                option,
            )
            .await
    }

    pub async fn delete_table_records(
        &self,
        query: &SparkTableMutationQuery<'_>,
        option: &RequestOption,
    ) -> Result<DeleteTableRecordsAppTableResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::DeleteSparkV1AppsByAppIdTablesByTableNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                Some(query.body),
                option,
            )
            .await
    }

    pub async fn get_table_detail(
        &self,
        app_id: &str,
        table_name: &str,
        option: &RequestOption,
    ) -> Result<GetTableDetailAppTableResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::GetSparkV1AppsByAppIdTablesByTableName,
                [("app_id", app_id), ("table_name", table_name)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }

    pub async fn get_table_list(
        &self,
        app_id: &str,
        option: &RequestOption,
    ) -> Result<GetTableListAppTableResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::GetSparkV1AppsByAppIdTables,
                [("app_id", app_id)],
                EMPTY_PARAMS,
                None,
                option,
            )
            .await
    }

    pub async fn get_table_record_list(
        &self,
        query: &SparkRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetTableRecordListAppTableResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::GetSparkV1AppsByAppIdTablesByTableNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                None,
                option,
            )
            .await
    }

    pub async fn patch_table_records(
        &self,
        query: &SparkTableMutationQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchTableRecordsAppTableResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PatchSparkV1AppsByAppIdTablesByTableNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                Some(query.body),
                option,
            )
            .await
    }

    pub async fn post_table_records(
        &self,
        query: &SparkTableMutationQuery<'_>,
        option: &RequestOption,
    ) -> Result<PostTableRecordsAppTableResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PostSparkV1AppsByAppIdTablesByTableNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                Some(query.body),
                option,
            )
            .await
    }
}

pub struct AppViewResource<'a> {
    config: &'a Config,
}

impl<'a> AppViewResource<'a> {
    pub async fn get_view_record_list(
        &self,
        app_id: &str,
        view_name: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetViewRecordListAppViewResp, LarkError> {
        let query = SparkViewRecordQuery::new(app_id, view_name)
            .page_size(page_size)
            .page_token(page_token);
        self.get_view_record_list_by_query(&query, option).await
    }

    pub async fn get_view_record_list_by_query(
        &self,
        query: &SparkViewRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetViewRecordListAppViewResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::GetSparkV1AppsByAppIdViewsByViewNameRecords,
                query.path_pairs(),
                query.query_pairs(),
                None,
                option,
            )
            .await
    }
}

pub struct DirectoryUserResource<'a> {
    config: &'a Config,
}

impl<'a> DirectoryUserResource<'a> {
    pub async fn id_convert(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<IdConvertDirectoryUserResp, LarkError> {
        GoV397::new(self.config)
            .request_json(
                GoV397Endpoint::PostSparkV1DirectoryUserIdConvert,
                EMPTY_PARAMS,
                EMPTY_PARAMS,
                Some(body),
                option,
            )
            .await
    }
}

pub struct V1<'a> {
    pub app: AppResource<'a>,
    pub app_enum: AppEnumResource<'a>,
    pub app_storage: AppStorageResource<'a>,
    pub app_table: AppTableResource<'a>,
    pub app_view: AppViewResource<'a>,
    pub directory_user: DirectoryUserResource<'a>,
    config: &'a Config,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app: AppResource { config },
            app_enum: AppEnumResource { config },
            app_storage: AppStorageResource { config },
            app_table: AppTableResource { config },
            app_view: AppViewResource { config },
            directory_user: DirectoryUserResource { config },
            config,
        }
    }

    pub fn go_v397(&self) -> GoV397<'a> {
        GoV397::new(self.config)
    }
}
