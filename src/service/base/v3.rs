use serde::Serialize;

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{JsonResp, PageQuery, RestRequest};

pub type ListRecordResp = JsonResp;
pub type SearchRecordResp = JsonResp;

/// Query parameters for reading Base v3 records.
///
/// `filter` and `sort` are JSON-encoded strings, as required by the Base v3
/// list endpoint. Search-specific filters belong in the JSON body passed to
/// [`RecordResource::search`].
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListRecordQuery<'a> {
    pub base_token: &'a str,
    pub table_id: &'a str,
    pub field_ids: Option<&'a [&'a str]>,
    pub view_id: Option<&'a str>,
    pub filter: Option<&'a str>,
    pub sort: Option<&'a str>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

impl<'a> ListRecordQuery<'a> {
    pub fn new(base_token: &'a str, table_id: &'a str) -> Self {
        Self {
            base_token,
            table_id,
            ..Self::default()
        }
    }

    pub fn field_ids(mut self, value: &'a [&'a str]) -> Self {
        self.field_ids = Some(value);
        self
    }

    pub fn view_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.view_id = value.into();
        self
    }

    pub fn filter(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.filter = value.into();
        self
    }

    pub fn sort(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.sort = value.into();
        self
    }

    pub fn offset(mut self, value: impl Into<Option<i32>>) -> Self {
        self.offset = value.into();
        self
    }

    pub fn limit(mut self, value: impl Into<Option<i32>>) -> Self {
        self.limit = value.into();
        self
    }
}

/// Query parameters for listing entities in a Base workspace.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListWorkspaceEntitiesQuery<'a> {
    pub workspace_token: &'a str,
    pub entity_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListWorkspaceEntitiesQuery<'a> {
    pub fn new(workspace_token: &'a str) -> Self {
        Self {
            workspace_token,
            entity_type: None,
            page: PageQuery::default(),
        }
    }

    pub fn entity_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.entity_type = value.into();
        self
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Query parameters for listing pages in a BaseApp.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListBaseAppPagesQuery<'a> {
    pub app_token: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListBaseAppPagesQuery<'a> {
    pub fn new(app_token: &'a str) -> Self {
        Self {
            app_token,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Query parameters for listing blocks on a BaseApp page.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListBaseAppBlocksQuery<'a> {
    pub app_token: &'a str,
    pub page_id: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListBaseAppBlocksQuery<'a> {
    pub fn new(app_token: &'a str, page_id: &'a str) -> Self {
        Self {
            app_token,
            page_id,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Query parameters for reading computed data from a BaseApp chart block.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetBaseAppBlockDataQuery<'a> {
    pub app_token: &'a str,
    pub block_id: &'a str,
    pub base_token: &'a str,
}

impl<'a> GetBaseAppBlockDataQuery<'a> {
    pub fn new(app_token: &'a str, block_id: &'a str, base_token: &'a str) -> Self {
        Self {
            app_token,
            block_id,
            base_token,
        }
    }
}

pub struct V3<'a> {
    pub record: RecordResource<'a>,
    pub workspace: WorkspaceResource<'a>,
    pub app: BaseAppResource<'a>,
    pub page: BaseAppPageResource<'a>,
    pub block: BaseAppBlockResource<'a>,
}

impl<'a> V3<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            record: RecordResource { config },
            workspace: WorkspaceResource { config },
            app: BaseAppResource { config },
            page: BaseAppPageResource { config },
            block: BaseAppBlockResource { config },
        }
    }
}

pub struct RecordResource<'a> {
    config: &'a Config,
}

impl RecordResource<'_> {
    /// Lists records from a Base v3 table.
    pub async fn list(
        &self,
        query: &ListRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListRecordResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/records",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", query.base_token)
        .path_param("table_id", query.table_id)
        .query_values("field_id", query.field_ids)
        .query("view_id", query.view_id)
        .query("filter", query.filter)
        .query("sort", query.sort)
        .query("offset", query.offset)
        .query("limit", query.limit)
        .send_json()
        .await
    }

    /// Searches records from a Base v3 table with the upstream JSON request body.
    pub async fn search(
        &self,
        base_token: &str,
        table_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<SearchRecordResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/bases/:base_token/tables/:table_id/records/search",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            &option,
        )
        .path_param("base_token", base_token)
        .path_param("table_id", table_id)
        .json_body(&body)?
        .send_json()
        .await
    }
}

/// Base v3 workspace operations proven by the official CLI's Open Platform client.
pub struct WorkspaceResource<'a> {
    config: &'a Config,
}

impl WorkspaceResource<'_> {
    /// Creates a workspace from the documented JSON request body.
    pub async fn create(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/workspaces",
            vec![AccessTokenType::User],
            &option,
        )
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Lists Base and BaseApp entities in a workspace.
    pub async fn list_entities(
        &self,
        query: &ListWorkspaceEntitiesQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/workspaces/:workspace_token/entities",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("workspace_token", query.workspace_token)
        .query("entity_type", query.entity_type)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Moves a Base or BaseApp entity into a workspace.
    pub async fn move_in(
        &self,
        workspace_token: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/workspaces/:workspace_token/move_in",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("workspace_token", workspace_token)
        .json_body(&body)?
        .send_json()
        .await
    }
}

/// BaseApp application-mode operations proven by the official CLI's Open Platform client.
pub struct BaseAppResource<'a> {
    config: &'a Config,
}

impl BaseAppResource<'_> {
    /// Creates a BaseApp from the documented JSON request body.
    pub async fn create(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/base_apps",
            vec![AccessTokenType::User],
            &option,
        )
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Gets a BaseApp and its page summaries.
    pub async fn get(
        &self,
        app_token: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .send_json()
        .await
    }
}

/// Page operations for BaseApps.
pub struct BaseAppPageResource<'a> {
    config: &'a Config,
}

impl BaseAppPageResource<'_> {
    /// Lists pages in a BaseApp.
    pub async fn list(
        &self,
        query: &ListBaseAppPagesQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/pages",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", query.app_token)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Gets one BaseApp page.
    pub async fn get(
        &self,
        app_token: &str,
        page_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .send_json()
        .await
    }

    /// Creates a top-level BaseApp page.
    pub async fn create(
        &self,
        app_token: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/base_apps/:app_token/pages",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Renames a BaseApp page with the documented JSON request body.
    pub async fn rename(
        &self,
        app_token: &str,
        page_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Deletes a BaseApp page.
    pub async fn delete(
        &self,
        app_token: &str,
        page_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .send_json()
        .await
    }
}

/// Block operations for BaseApp pages.
pub struct BaseAppBlockResource<'a> {
    config: &'a Config,
}

impl BaseAppBlockResource<'_> {
    /// Lists blocks on a BaseApp page.
    pub async fn list(
        &self,
        query: &ListBaseAppBlocksQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id/blocks",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", query.app_token)
        .path_param("page_id", query.page_id)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Gets one BaseApp block.
    pub async fn get(
        &self,
        app_token: &str,
        page_id: &str,
        block_id: &str,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id/blocks/:block_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .path_param("block_id", block_id)
        .send_json()
        .await
    }

    /// Creates a BaseApp block from the documented JSON request body.
    pub async fn create(
        &self,
        app_token: &str,
        page_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id/blocks",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Updates a BaseApp block with the documented JSON request body.
    pub async fn update(
        &self,
        app_token: &str,
        page_id: &str,
        block_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            "/open-apis/base/v3/base_apps/:app_token/pages/:page_id/blocks/:block_id",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", app_token)
        .path_param("page_id", page_id)
        .path_param("block_id", block_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Reads computed data for a BaseApp chart block.
    ///
    /// The endpoint takes a chart token rather than the page-scoped widget ID,
    /// and does not support text blocks.
    pub async fn get_data(
        &self,
        query: &GetBaseAppBlockDataQuery<'_>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError> {
        let option = with_app_id(self.config, option)?;
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/base/v3/base_apps/:app_token/blocks/:block_id/data",
            vec![AccessTokenType::User],
            &option,
        )
        .path_param("app_token", query.app_token)
        .path_param("block_id", query.block_id)
        .query("base_token", query.base_token)
        .send_json()
        .await
    }
}

fn with_app_id(config: &Config, option: &RequestOption) -> Result<RequestOption, LarkError> {
    let mut option = option.clone();
    option
        .headers
        .get_or_insert_with(http::HeaderMap::new)
        .insert(
            http::HeaderName::from_static("x-app-id"),
            http::HeaderValue::from_str(config.app_id())
                .map_err(|err| LarkError::IllegalParam(err.to_string()))?,
        );
    Ok(option)
}
