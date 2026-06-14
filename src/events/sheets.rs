//! Sheets v3 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SheetsSpreadsheetCreatedV1 {
    #[serde(default)]
    pub spreadsheet_token: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SheetsValueChangedV3 {
    #[serde(default)]
    pub spreadsheet_token: String,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub sheet_id: String,
    #[serde(default)]
    pub changes: Vec<serde_json::Value>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_sheets_spreadsheet_created_v1 => P2SheetsSpreadsheetCreatedV1
        : "sheets.spreadsheet.created_v1",
    on_p2_sheets_value_changed_v3 => P2SheetsValueChangedV3
        : "drive.file.bitable_record_changed_v1",
}
