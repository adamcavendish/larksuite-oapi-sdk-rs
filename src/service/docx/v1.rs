use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Document {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_setting: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Block {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading1: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading2: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading3: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading4: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading5: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading6: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading7: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading8: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading9: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bullet: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordered: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equation: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub todo: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitable: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callout: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_card: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagram: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divider: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grid: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grid_column: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iframe: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isv: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mindnote: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_cell: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undefined: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_container: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okr: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okr_objective: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okr_key_result: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okr_progress: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_ons: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jira_issue: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wiki_catalog: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub board: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced_block: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iframe_v2: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildingBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_type: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateDocumentReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBlockReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateBlockReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_text_elements: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_text_style: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_table_property: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_table_row: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_table_column: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_table_rows: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_table_columns: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_table_cells: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmerge_table_cells: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_table_column_width: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_image: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_file: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_grid: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_grid_column: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_grid_column: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_grid_column_width_ratio: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_code_block: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_iframe: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_diagram: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_callout: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_todo: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_bitable: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_quote_container: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_synced_block_source: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchUpdateBlockReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBlocksReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<Block>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockListData {
    #[serde(default)]
    pub items: Vec<Block>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBlockData {
    #[serde(default)]
    pub children: Vec<Block>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_revision_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBlockData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<Block>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_revision_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchUpdateBlockData {
    #[serde(default)]
    pub blocks: Vec<Block>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_revision_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuildingBlockData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub building_block: Option<BuildingBlock>,
}

impl_resp!(CreateDocumentResp, DocumentData);
impl_resp!(GetDocumentResp, DocumentData);
impl_resp!(GetDocumentRawContentResp, serde_json::Value);
impl_resp!(CreateBlockResp, CreateBlockData);
impl_resp!(GetBlockResp, BlockData);
impl_resp!(UpdateBlockResp, UpdateBlockData);
impl_resp!(BatchUpdateBlockResp, BatchUpdateBlockData);
impl_resp!(ListBlockResp, BlockListData);
impl_resp!(GetBuildingBlockResp, BuildingBlockData);

impl_resp_v2!(GetChatAnnouncementResp, GetChatAnnouncementRespData);
impl_resp_v2!(
    BatchUpdateChatAnnouncementBlockResp,
    BatchUpdateChatAnnouncementBlockRespData
);
impl_resp_v2!(
    GetChatAnnouncementBlockResp,
    GetChatAnnouncementBlockRespData
);
impl_resp_v2!(
    ListChatAnnouncementBlockResp,
    ListChatAnnouncementBlockRespData
);
impl_resp_v2!(
    BatchDeleteChatAnnouncementBlockChildrenResp,
    BatchDeleteChatAnnouncementBlockChildrenRespData
);
impl_resp_v2!(
    CreateChatAnnouncementBlockChildrenResp,
    CreateChatAnnouncementBlockChildrenRespData
);
impl_resp_v2!(
    GetChatAnnouncementBlockChildrenResp,
    GetChatAnnouncementBlockChildrenRespData
);
impl_resp_v2!(
    CreateDocumentBlockDescendantResp,
    CreateDocumentBlockDescendantRespData
);
impl_resp_v2!(ConvertDocumentResp, ConvertDocumentRespData);

#[derive(Debug, Clone, Copy)]
pub struct CreateDocumentQuery<'a> {
    pub body: &'a CreateDocumentReqBody,
}

impl<'a> CreateDocumentQuery<'a> {
    pub fn new(body: &'a CreateDocumentReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetDocumentQuery<'a> {
    pub document_id: &'a str,
}

impl<'a> GetDocumentQuery<'a> {
    pub fn new(document_id: &'a str) -> Self {
        Self { document_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConvertDocumentQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> ConvertDocumentQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetDocumentRawContentQuery<'a> {
    pub document_id: &'a str,
    pub lang: Option<i32>,
}

impl<'a> GetDocumentRawContentQuery<'a> {
    pub fn new(document_id: &'a str) -> Self {
        Self {
            document_id,
            lang: None,
        }
    }

    pub fn lang(mut self, value: impl Into<Option<i32>>) -> Self {
        self.lang = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateDocumentBlockQuery<'a> {
    pub document_id: &'a str,
    pub block_id: &'a str,
    pub body: &'a CreateBlockReqBody,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateDocumentBlockQuery<'a> {
    pub fn new(document_id: &'a str, block_id: &'a str, body: &'a CreateBlockReqBody) -> Self {
        Self {
            document_id,
            block_id,
            body,
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetDocumentBlockQuery<'a> {
    pub document_id: &'a str,
    pub block_id: &'a str,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetDocumentBlockQuery<'a> {
    pub fn new(document_id: &'a str, block_id: &'a str) -> Self {
        Self {
            document_id,
            block_id,
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateDocumentBlockQuery<'a> {
    pub document_id: &'a str,
    pub block_id: &'a str,
    pub body: &'a UpdateBlockReqBody,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateDocumentBlockQuery<'a> {
    pub fn new(document_id: &'a str, block_id: &'a str, body: &'a UpdateBlockReqBody) -> Self {
        Self {
            document_id,
            block_id,
            body,
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BatchUpdateDocumentBlockQuery<'a> {
    pub document_id: &'a str,
    pub body: &'a BatchUpdateBlockReqBody,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchUpdateDocumentBlockQuery<'a> {
    pub fn new(document_id: &'a str, body: &'a BatchUpdateBlockReqBody) -> Self {
        Self {
            document_id,
            body,
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteDocumentBlockChildrenQuery<'a> {
    pub document_id: &'a str,
    pub block_id: &'a str,
    pub body: &'a DeleteBlocksReqBody,
    pub document_revision_id: Option<i64>,
}

impl<'a> DeleteDocumentBlockChildrenQuery<'a> {
    pub fn new(document_id: &'a str, block_id: &'a str, body: &'a DeleteBlocksReqBody) -> Self {
        Self {
            document_id,
            block_id,
            body,
            document_revision_id: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListDocumentBlockQuery<'a> {
    pub document_id: &'a str,
    pub page: PageQuery<'a>,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListDocumentBlockQuery<'a> {
    pub fn new(document_id: &'a str) -> Self {
        Self {
            document_id,
            page: PageQuery::default(),
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListDocumentBlockChildrenQuery<'a> {
    pub document_id: &'a str,
    pub block_id: &'a str,
    pub page: PageQuery<'a>,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListDocumentBlockChildrenQuery<'a> {
    pub fn new(document_id: &'a str, block_id: &'a str) -> Self {
        Self {
            document_id,
            block_id,
            page: PageQuery::default(),
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetBuildingBlockQuery<'a> {
    pub document_id: &'a str,
    pub block_id: &'a str,
}

impl<'a> GetBuildingBlockQuery<'a> {
    pub fn new(document_id: &'a str, block_id: &'a str) -> Self {
        Self {
            document_id,
            block_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetChatAnnouncementQuery<'a> {
    pub chat_id: &'a str,
}

impl<'a> GetChatAnnouncementQuery<'a> {
    pub fn new(chat_id: &'a str) -> Self {
        Self { chat_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BatchUpdateChatAnnouncementBlockQuery<'a> {
    pub chat_id: &'a str,
    pub body: &'a serde_json::Value,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> BatchUpdateChatAnnouncementBlockQuery<'a> {
    pub fn new(chat_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            chat_id,
            body,
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetChatAnnouncementBlockQuery<'a> {
    pub chat_id: &'a str,
    pub block_id: &'a str,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetChatAnnouncementBlockQuery<'a> {
    pub fn new(chat_id: &'a str, block_id: &'a str) -> Self {
        Self {
            chat_id,
            block_id,
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListChatAnnouncementBlockQuery<'a> {
    pub chat_id: &'a str,
    pub page: PageQuery<'a>,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListChatAnnouncementBlockQuery<'a> {
    pub fn new(chat_id: &'a str) -> Self {
        Self {
            chat_id,
            page: PageQuery::default(),
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BatchDeleteChatAnnouncementBlockChildrenQuery<'a> {
    pub chat_id: &'a str,
    pub block_id: &'a str,
    pub body: &'a serde_json::Value,
    pub document_revision_id: Option<i64>,
}

impl<'a> BatchDeleteChatAnnouncementBlockChildrenQuery<'a> {
    pub fn new(chat_id: &'a str, block_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            chat_id,
            block_id,
            body,
            document_revision_id: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateChatAnnouncementBlockChildrenQuery<'a> {
    pub chat_id: &'a str,
    pub block_id: &'a str,
    pub body: &'a serde_json::Value,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateChatAnnouncementBlockChildrenQuery<'a> {
    pub fn new(chat_id: &'a str, block_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            chat_id,
            block_id,
            body,
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetChatAnnouncementBlockChildrenQuery<'a> {
    pub chat_id: &'a str,
    pub block_id: &'a str,
    pub page: PageQuery<'a>,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetChatAnnouncementBlockChildrenQuery<'a> {
    pub fn new(chat_id: &'a str, block_id: &'a str) -> Self {
        Self {
            chat_id,
            block_id,
            page: PageQuery::default(),
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateDocumentBlockDescendantQuery<'a> {
    pub document_id: &'a str,
    pub block_id: &'a str,
    pub body: &'a serde_json::Value,
    pub document_revision_id: Option<i64>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateDocumentBlockDescendantQuery<'a> {
    pub fn new(document_id: &'a str, block_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            document_id,
            block_id,
            body,
            document_revision_id: None,
            user_id_type: None,
        }
    }

    pub fn document_revision_id(mut self, value: impl Into<Option<i64>>) -> Self {
        self.document_revision_id = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatAnnouncementRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifier_id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub announcement_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time_v2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time_v2: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchUpdateChatAnnouncementBlockRespData {
    #[serde(default)]
    pub blocks: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlockRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListChatAnnouncementBlockRespData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchDeleteChatAnnouncementBlockChildrenRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateChatAnnouncementBlockChildrenRespData {
    #[serde(default)]
    pub children: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlockChildrenRespData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDocumentBlockDescendantRespData {
    #[serde(default)]
    pub children: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_revision_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub block_id_relations: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConvertDocumentRespData {
    #[serde(default)]
    pub first_level_block_ids: Vec<String>,
    #[serde(default)]
    pub blocks: Vec<serde_json::Value>,
    #[serde(default)]
    pub block_id_to_image_urls: Vec<serde_json::Value>,
}
// ── Resources ──

pub struct DocumentResource<'a> {
    config: &'a Config,
}

impl<'a> DocumentResource<'a> {
    pub async fn create(
        &self,
        body: &CreateDocumentReqBody,
        option: &RequestOption,
    ) -> Result<CreateDocumentResp, LarkError> {
        self.create_by_query(&CreateDocumentQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDocumentQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateDocumentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/docx/v1/documents",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_response::<DocumentData, CreateDocumentResp>()
        .await
    }

    pub async fn get(
        &self,
        document_id: &str,
        option: &RequestOption,
    ) -> Result<GetDocumentResp, LarkError> {
        self.get_by_query(&GetDocumentQuery::new(document_id), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetDocumentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDocumentResp, LarkError> {
        let path = format!("/open-apis/docx/v1/documents/{}", query.document_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<DocumentData, GetDocumentResp>()
        .await
    }

    pub async fn convert(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<ConvertDocumentResp, LarkError> {
        self.convert_by_query(&ConvertDocumentQuery::new(body), option)
            .await
    }

    pub async fn convert_by_query(
        &self,
        query: &ConvertDocumentQuery<'_>,
        option: &RequestOption,
    ) -> Result<ConvertDocumentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/docx/v1/documents/blocks/convert",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<ConvertDocumentRespData, ConvertDocumentResp>()
        .await
    }

    pub async fn raw_content(
        &self,
        document_id: &str,
        lang: Option<i32>,
        option: &RequestOption,
    ) -> Result<GetDocumentRawContentResp, LarkError> {
        let query = GetDocumentRawContentQuery::new(document_id).lang(lang);
        self.raw_content_by_query(&query, option).await
    }

    pub async fn raw_content_by_query(
        &self,
        query: &GetDocumentRawContentQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetDocumentRawContentResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/documents/{}/raw_content",
            query.document_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("lang", query.lang)
        .send_response::<serde_json::Value, GetDocumentRawContentResp>()
        .await
    }
}

pub struct DocumentBlockResource<'a> {
    config: &'a Config,
}

impl<'a> DocumentBlockResource<'a> {
    pub async fn create(
        &self,
        document_id: &str,
        block_id: &str,
        body: &CreateBlockReqBody,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateBlockResp, LarkError> {
        let query = CreateDocumentBlockQuery::new(document_id, block_id, body)
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDocumentBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateBlockResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/documents/{}/blocks/{}/children",
            query.document_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<CreateBlockData, CreateBlockResp>()
        .await
    }

    pub async fn get(
        &self,
        document_id: &str,
        block_id: &str,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetBlockResp, LarkError> {
        let query = GetDocumentBlockQuery::new(document_id, block_id)
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetDocumentBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetBlockResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/documents/{}/blocks/{}",
            query.document_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .send_response::<BlockData, GetBlockResp>()
        .await
    }

    pub async fn update(
        &self,
        document_id: &str,
        block_id: &str,
        body: &UpdateBlockReqBody,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateBlockResp, LarkError> {
        let query = UpdateDocumentBlockQuery::new(document_id, block_id, body)
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateDocumentBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateBlockResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/documents/{}/blocks/{}",
            query.document_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<UpdateBlockData, UpdateBlockResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        document_id: &str,
        body: &BatchUpdateBlockReqBody,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchUpdateBlockResp, LarkError> {
        let query = BatchUpdateDocumentBlockQuery::new(document_id, body)
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.batch_update_by_query(&query, option).await
    }

    pub async fn batch_update_by_query(
        &self,
        query: &BatchUpdateDocumentBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchUpdateBlockResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/documents/{}/blocks/batch_update",
            query.document_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_response::<BatchUpdateBlockData, BatchUpdateBlockResp>()
        .await
    }

    pub async fn delete(
        &self,
        document_id: &str,
        block_id: &str,
        body: &DeleteBlocksReqBody,
        document_revision_id: Option<i64>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteDocumentBlockChildrenQuery::new(document_id, block_id, body)
            .document_revision_id(document_revision_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteDocumentBlockChildrenQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/documents/{}/blocks/{}/children/batch_delete",
            query.document_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .json_body(query.body)?
        .send_empty()
        .await
    }

    pub async fn list(
        &self,
        document_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListBlockResp, LarkError> {
        let query = ListDocumentBlockQuery::new(document_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListDocumentBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListBlockResp, LarkError> {
        let path = format!("/open-apis/docx/v1/documents/{}/blocks", query.document_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .send_response::<BlockListData, ListBlockResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list_children(
        &self,
        document_id: &str,
        block_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListBlockResp, LarkError> {
        let query = ListDocumentBlockChildrenQuery::new(document_id, block_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.list_children_by_query(&query, option).await
    }

    pub async fn list_children_by_query(
        &self,
        query: &ListDocumentBlockChildrenQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListBlockResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/documents/{}/blocks/{}/children",
            query.document_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .send_response::<BlockListData, ListBlockResp>()
        .await
    }
}

pub struct BuildingBlockResource<'a> {
    config: &'a Config,
}

impl<'a> BuildingBlockResource<'a> {
    pub async fn get(
        &self,
        document_id: &str,
        block_id: &str,
        option: &RequestOption,
    ) -> Result<GetBuildingBlockResp, LarkError> {
        self.get_by_query(&GetBuildingBlockQuery::new(document_id, block_id), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetBuildingBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetBuildingBlockResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/documents/{}/building_blocks/{}",
            query.document_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<BuildingBlockData, GetBuildingBlockResp>()
        .await
    }
}

pub struct ChatAnnouncementResource<'a> {
    config: &'a Config,
}

impl ChatAnnouncementResource<'_> {
    pub async fn get(
        &self,
        chat_id: &str,
        option: &RequestOption,
    ) -> Result<GetChatAnnouncementResp, LarkError> {
        self.get_by_query(&GetChatAnnouncementQuery::new(chat_id), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetChatAnnouncementQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetChatAnnouncementResp, LarkError> {
        let path = format!("/open-apis/docx/v1/chats/{}/announcement", query.chat_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<GetChatAnnouncementRespData, GetChatAnnouncementResp>()
        .await
    }
}

pub struct ChatAnnouncementBlockResource<'a> {
    config: &'a Config,
}

impl ChatAnnouncementBlockResource<'_> {
    pub async fn batch_update(
        &self,
        chat_id: &str,
        body: &serde_json::Value,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchUpdateChatAnnouncementBlockResp, LarkError> {
        let query = BatchUpdateChatAnnouncementBlockQuery::new(chat_id, body)
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.batch_update_by_query(&query, option).await
    }

    pub async fn batch_update_by_query(
        &self,
        query: &BatchUpdateChatAnnouncementBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchUpdateChatAnnouncementBlockResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/chats/{}/announcement/blocks/batch_update",
            query.chat_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<BatchUpdateChatAnnouncementBlockRespData, BatchUpdateChatAnnouncementBlockResp>()
        .await
    }

    pub async fn get(
        &self,
        chat_id: &str,
        block_id: &str,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetChatAnnouncementBlockResp, LarkError> {
        let query = GetChatAnnouncementBlockQuery::new(chat_id, block_id)
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetChatAnnouncementBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetChatAnnouncementBlockResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/chats/{}/announcement/blocks/{}",
            query.chat_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetChatAnnouncementBlockRespData, GetChatAnnouncementBlockResp>()
        .await
    }

    pub async fn list(
        &self,
        chat_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListChatAnnouncementBlockResp, LarkError> {
        let query = ListChatAnnouncementBlockQuery::new(chat_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListChatAnnouncementBlockQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListChatAnnouncementBlockResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/chats/{}/announcement/blocks",
            query.chat_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ListChatAnnouncementBlockRespData, ListChatAnnouncementBlockResp>()
        .await
    }
}

pub struct ChatAnnouncementBlockChildrenResource<'a> {
    config: &'a Config,
}

impl ChatAnnouncementBlockChildrenResource<'_> {
    pub async fn batch_delete(
        &self,
        chat_id: &str,
        block_id: &str,
        body: &serde_json::Value,
        document_revision_id: Option<i64>,
        option: &RequestOption,
    ) -> Result<BatchDeleteChatAnnouncementBlockChildrenResp, LarkError> {
        let query = BatchDeleteChatAnnouncementBlockChildrenQuery::new(chat_id, block_id, body)
            .document_revision_id(document_revision_id);
        self.batch_delete_by_query(&query, option).await
    }

    pub async fn batch_delete_by_query(
        &self,
        query: &BatchDeleteChatAnnouncementBlockChildrenQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchDeleteChatAnnouncementBlockChildrenResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/chats/{}/announcement/blocks/{}/children/batch_delete",
            query.chat_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .json_body(query.body)?
        .send_v2_response::<BatchDeleteChatAnnouncementBlockChildrenRespData, BatchDeleteChatAnnouncementBlockChildrenResp>()
        .await
    }

    pub async fn create(
        &self,
        chat_id: &str,
        block_id: &str,
        body: &serde_json::Value,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateChatAnnouncementBlockChildrenResp, LarkError> {
        let query = CreateChatAnnouncementBlockChildrenQuery::new(chat_id, block_id, body)
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateChatAnnouncementBlockChildrenQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateChatAnnouncementBlockChildrenResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/chats/{}/announcement/blocks/{}/children",
            query.chat_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<CreateChatAnnouncementBlockChildrenRespData, CreateChatAnnouncementBlockChildrenResp>()
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        chat_id: &str,
        block_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetChatAnnouncementBlockChildrenResp, LarkError> {
        let query = GetChatAnnouncementBlockChildrenQuery::new(chat_id, block_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetChatAnnouncementBlockChildrenQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetChatAnnouncementBlockChildrenResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/chats/{}/announcement/blocks/{}/children",
            query.chat_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<GetChatAnnouncementBlockChildrenRespData, GetChatAnnouncementBlockChildrenResp>()
        .await
    }
}

pub struct DocumentBlockDescendantResource<'a> {
    config: &'a Config,
}

impl DocumentBlockDescendantResource<'_> {
    pub async fn create(
        &self,
        document_id: &str,
        block_id: &str,
        body: &serde_json::Value,
        document_revision_id: Option<i64>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateDocumentBlockDescendantResp, LarkError> {
        let query = CreateDocumentBlockDescendantQuery::new(document_id, block_id, body)
            .document_revision_id(document_revision_id)
            .user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDocumentBlockDescendantQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateDocumentBlockDescendantResp, LarkError> {
        let path = format!(
            "/open-apis/docx/v1/documents/{}/blocks/{}/descendant",
            query.document_id, query.block_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("document_revision_id", query.document_revision_id)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<CreateDocumentBlockDescendantRespData, CreateDocumentBlockDescendantResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub document: DocumentResource<'a>,
    pub block: DocumentBlockResource<'a>,
    pub building_block: BuildingBlockResource<'a>,
    pub chat_announcement: ChatAnnouncementResource<'a>,
    pub chat_announcement_block: ChatAnnouncementBlockResource<'a>,
    pub chat_announcement_block_children: ChatAnnouncementBlockChildrenResource<'a>,
    pub document_block_descendant: DocumentBlockDescendantResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            document: DocumentResource { config },
            block: DocumentBlockResource { config },
            building_block: BuildingBlockResource { config },
            chat_announcement: ChatAnnouncementResource { config },
            chat_announcement_block: ChatAnnouncementBlockResource { config },
            chat_announcement_block_children: ChatAnnouncementBlockChildrenResource { config },
            document_block_descendant: DocumentBlockDescendantResource { config },
        }
    }
}
