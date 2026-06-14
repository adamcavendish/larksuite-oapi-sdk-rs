//! Document (Docx) v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DocxDocumentChangedV1 {
    #[serde(default)]
    pub document_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub update_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DocxDocumentCreatedV1 {
    #[serde(default)]
    pub document_id: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub create_time: String,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_docx_document_changed_v1 => P2DocxDocumentChangedV1
        : "docx.document.v1.changed",
    on_p2_docx_document_created_v1 => P2DocxDocumentCreatedV1
        : "docx.document.v1.created",
}
