use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Spreadsheet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spreadsheet_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Sheet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grid_properties: Option<SheetGridProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges: Option<Vec<GridRange>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetGridProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frozen_row_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frozen_column_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GridRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_style: Option<SegmentStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SegmentStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affected_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<SegmentStyleStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SegmentStyleStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strike_through: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fore_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetMentionUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_style: Option<SegmentStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetMentionDocument {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_style: Option<SegmentStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetValueElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetDateTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_style: Option<SegmentStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_styles: Option<Vec<SegmentStyle>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetReminder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_date_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_user_id: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetFormula {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula: Option<String>,
}

/// A rich-text element in a Sheets cell.
///
/// This mirrors the Go SDK's `CellValue` model. A rich-text cell is represented
/// as a list of these elements, while a plain-text cell uses `SheetCellValue`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CellValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<SheetText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_user: Option<SheetMentionUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_document: Option<SheetMentionDocument>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<SheetValueElement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_time: Option<SheetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<SheetFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<SheetImage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<SheetLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminder: Option<SheetReminder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formula: Option<SheetFormula>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SheetCellValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    RichText(Vec<CellValue>),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlainTextValueRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major_dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Vec<Option<SheetCellValue>>>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextValueRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major_dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Vec<Vec<CellValue>>>>,
}

/// A Sheets value range whose cells can contain either scalar or rich-text data.
///
/// The Go SDK exposes separate plain- and rich-text request models. Responses
/// do not carry a discriminator, so this unified representation preserves both
/// wire shapes without ambiguous deserialization.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SheetValueRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major_dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Vec<Option<SheetCellValue>>>>,
}

impl From<PlainTextValueRange> for SheetValueRange {
    fn from(value: PlainTextValueRange) -> Self {
        Self {
            major_dimension: value.major_dimension,
            range: value.range,
            values: value.values,
        }
    }
}

impl From<RichTextValueRange> for SheetValueRange {
    fn from(value: RichTextValueRange) -> Self {
        Self {
            major_dimension: value.major_dimension,
            range: value.range,
            values: value.values.map(|rows| {
                rows.into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|elements| Some(SheetCellValue::RichText(elements)))
                            .collect()
                    })
                    .collect()
            }),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RangeValueData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "spreadsheetToken"
    )]
    pub spreadsheet_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "valueRange")]
    pub value_range: Option<SheetValueRange>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "valueRanges"
    )]
    pub value_ranges: Option<Vec<SheetValueRange>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "updatedRange"
    )]
    pub updated_range: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "updatedRows"
    )]
    pub updated_rows: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "updatedColumns"
    )]
    pub updated_columns: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        alias = "updatedCells"
    )]
    pub updated_cells: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frozen_row_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frozen_column_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SheetDimension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major_dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FindCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_entire_cell: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_by_regex: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_formulas: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FindSheetReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find_condition: Option<FindCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MoveDimensionReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<SheetDimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplaceSheetReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find_condition: Option<FindCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SheetFilterCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetFilterInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filtered_out_rows: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_infos: Option<Vec<SheetFilterColumn>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetFilterColumn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub col: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<SheetFilterCondition>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSheetFilterReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub col: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<SheetFilterCondition>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSheetFilterReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub col: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<SheetFilterCondition>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetFilterView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_view_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetFilterViewCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Chart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConditionalFormat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cf_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Vec<crate::JsonValue>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DataValidationOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_color: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DataValidation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dv_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dv_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<DataValidationOption>>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSpreadsheetReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchSpreadsheetReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct OperateSheetsReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<crate::JsonValue>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchSheetReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SheetProperties>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SetRangeValueReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_range: Option<SheetValueRange>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchSetRangeValueReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_ranges: Option<Vec<SheetValueRange>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AppendValueReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_range: Option<SheetValueRange>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PrependValueReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_range: Option<SheetValueRange>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFilterViewReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchFilterViewReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateFilterViewConditionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateFilterViewConditionReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateConditionalFormatReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_conditional_formats: Option<Vec<ConditionalFormat>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateConditionalFormatReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_conditional_formats: Option<Vec<ConditionalFormat>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteConditionalFormatReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cf_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SetDataValidationDropdownReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dv_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<DataValidationOption>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SpreadsheetData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spreadsheet: Option<Spreadsheet>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetListData {
    #[serde(default)]
    pub sheets: Vec<Sheet>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OperateSheetsData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<crate::JsonValue>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FilterViewData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_view: Option<SheetFilterView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FilterViewListData {
    #[serde(default)]
    pub items: Vec<SheetFilterView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FilterViewConditionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<SheetFilterViewCondition>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FilterViewConditionListData {
    #[serde(default)]
    pub items: Vec<SheetFilterViewCondition>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConditionalFormatListData {
    #[serde(default)]
    pub sheet_conditional_formats: Vec<ConditionalFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DataValidationListData {
    #[serde(default)]
    pub items: Vec<DataValidation>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FindReplaceResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matched_cells: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matched_formula_cells: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FindSheetData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find_result: Option<FindReplaceResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReplaceSheetData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace_result: Option<FindReplaceResult>,
}

impl_resp!(CreateSpreadsheetResp, SpreadsheetData);
impl_resp!(GetSpreadsheetResp, SpreadsheetData);
impl_resp!(PatchSpreadsheetResp, SpreadsheetData);
impl_resp!(GetSheetResp, Sheet);
impl_resp!(QuerySheetResp, SheetListData);
impl_resp!(OperateSheetsResp, OperateSheetsData);
impl_resp!(GetRangeValueResp, RangeValueData);
impl_resp!(SetRangeValueResp, RangeValueData);
impl_resp!(BatchGetRangeValueResp, RangeValueData);
impl_resp!(BatchSetRangeValueResp, RangeValueData);
impl_resp!(AppendValueResp, RangeValueData);
impl_resp!(PrependValueResp, RangeValueData);
impl_resp!(CreateFilterViewResp, FilterViewData);
impl_resp!(GetFilterViewResp, FilterViewData);
impl_resp!(PatchFilterViewResp, FilterViewData);
impl_resp!(QueryFilterViewResp, FilterViewListData);
impl_resp!(CreateFilterViewConditionResp, FilterViewConditionData);
impl_resp!(GetFilterViewConditionResp, FilterViewConditionData);
impl_resp!(UpdateFilterViewConditionResp, FilterViewConditionData);
impl_resp!(QueryFilterViewConditionResp, FilterViewConditionListData);
impl_resp!(QueryConditionalFormatResp, ConditionalFormatListData);
impl_resp!(QueryDataValidationResp, DataValidationListData);
impl_resp!(FindSheetResp, FindSheetData);
impl_resp!(MoveDimensionSheetResp, ());
impl_resp!(ReplaceSheetResp, ReplaceSheetData);
impl_resp!(GetSheetFilterResp, SheetFilterData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SheetFilterData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet_filter_info: Option<SheetFilterInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FloatImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub float_image_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub float_image_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FloatImageData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub float_image: Option<FloatImage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FloatImageQueryData {
    #[serde(default)]
    pub items: Vec<FloatImage>,
}

impl_resp!(CreateFloatImageResp, FloatImageData);
impl_resp!(GetFloatImageResp, FloatImageData);
impl_resp!(PatchFloatImageResp, FloatImageData);
impl_resp!(QueryFloatImageResp, FloatImageQueryData);

// ── Resources ──

pub struct SpreadsheetResource<'a> {
    config: &'a Config,
}

impl<'a> SpreadsheetResource<'a> {
    pub async fn create(
        &self,
        body: &CreateSpreadsheetReqBody,
        option: &RequestOption,
    ) -> Result<CreateSpreadsheetResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/sheets/v3/spreadsheets",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<SpreadsheetData, CreateSpreadsheetResp>()
        .await
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetSpreadsheetResp, LarkError> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", user_id_type)
        .send_response::<SpreadsheetData, GetSpreadsheetResp>()
        .await
    }

    pub async fn patch(
        &self,
        spreadsheet_token: &str,
        body: &PatchSpreadsheetReqBody,
        option: &RequestOption,
    ) -> Result<PatchSpreadsheetResp, LarkError> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<SpreadsheetData, PatchSpreadsheetResp>()
        .await
    }
}

pub struct SheetResource<'a> {
    config: &'a Config,
}

impl<'a> SheetResource<'a> {
    pub async fn get(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        option: &RequestOption,
    ) -> Result<GetSheetResp, LarkError> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<Sheet, GetSheetResp>()
        .await
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        option: &RequestOption,
    ) -> Result<QuerySheetResp, LarkError> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/query");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<SheetListData, QuerySheetResp>()
        .await
    }

    pub async fn patch(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &PatchSheetReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn operate(
        &self,
        spreadsheet_token: &str,
        body: &OperateSheetsReqBody,
        option: &RequestOption,
    ) -> Result<OperateSheetsResp, LarkError> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets_batch_update");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<OperateSheetsData, OperateSheetsResp>()
        .await
    }

    pub async fn find(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &FindSheetReqBody,
        option: &RequestOption,
    ) -> Result<FindSheetResp, LarkError> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/find");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<FindSheetData, FindSheetResp>()
        .await
    }

    pub async fn move_dimension(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &MoveDimensionReqBody,
        option: &RequestOption,
    ) -> Result<MoveDimensionSheetResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/move_dimension"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<(), MoveDimensionSheetResp>()
        .await
    }

    pub async fn replace(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &ReplaceSheetReqBody,
        option: &RequestOption,
    ) -> Result<ReplaceSheetResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/replace"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_response::<ReplaceSheetData, ReplaceSheetResp>()
        .await
    }
}

pub struct RangeResource<'a> {
    config: &'a Config,
}

impl<'a> RangeResource<'a> {
    pub async fn get(
        &self,
        spreadsheet_token: &str,
        range: &str,
        value_render_option: Option<&str>,
        date_time_render_option: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetRangeValueResp, LarkError> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values/{range}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("valueRenderOption", value_render_option)
        .query("dateTimeRenderOption", date_time_render_option)
        .query("user_id_type", user_id_type)
        .send_response::<RangeValueData, GetRangeValueResp>()
        .await
    }

    pub async fn set(
        &self,
        spreadsheet_token: &str,
        range: &str,
        body: &SetRangeValueReqBody,
        option: &RequestOption,
    ) -> Result<SetRangeValueResp, LarkError> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values/{range}");
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<RangeValueData, SetRangeValueResp>()
        .await
    }

    pub async fn batch_get(
        &self,
        spreadsheet_token: &str,
        ranges: &[&str],
        value_render_option: Option<&str>,
        date_time_render_option: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchGetRangeValueResp, LarkError> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_batch_get");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query_values("ranges", Some(ranges.iter().copied()))
        .query("valueRenderOption", value_render_option)
        .query("dateTimeRenderOption", date_time_render_option)
        .query("user_id_type", user_id_type)
        .send_response::<RangeValueData, BatchGetRangeValueResp>()
        .await
    }

    pub async fn batch_set(
        &self,
        spreadsheet_token: &str,
        body: &BatchSetRangeValueReqBody,
        option: &RequestOption,
    ) -> Result<BatchSetRangeValueResp, LarkError> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_batch_update");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<RangeValueData, BatchSetRangeValueResp>()
        .await
    }

    pub async fn append(
        &self,
        spreadsheet_token: &str,
        range: &str,
        body: &AppendValueReqBody,
        insert_data_option: Option<&str>,
        option: &RequestOption,
    ) -> Result<AppendValueResp, LarkError> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_append");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("range", range)
        .query("insertDataOption", insert_data_option)
        .json_body(body)?
        .send_response::<RangeValueData, AppendValueResp>()
        .await
    }

    pub async fn prepend(
        &self,
        spreadsheet_token: &str,
        range: &str,
        body: &PrependValueReqBody,
        option: &RequestOption,
    ) -> Result<PrependValueResp, LarkError> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_prepend");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("range", range)
        .json_body(body)?
        .send_response::<RangeValueData, PrependValueResp>()
        .await
    }

    pub async fn clear(
        &self,
        spreadsheet_token: &str,
        range: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values/{range}");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }
}

pub struct FilterViewResource<'a> {
    config: &'a Config,
}

impl<'a> FilterViewResource<'a> {
    pub async fn create(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &CreateFilterViewReqBody,
        option: &RequestOption,
    ) -> Result<CreateFilterViewResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<FilterViewData, CreateFilterViewResp>()
        .await
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        option: &RequestOption,
    ) -> Result<GetFilterViewResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<FilterViewData, GetFilterViewResp>()
        .await
    }

    pub async fn patch(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        body: &PatchFilterViewReqBody,
        option: &RequestOption,
    ) -> Result<PatchFilterViewResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<FilterViewData, PatchFilterViewResp>()
        .await
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        option: &RequestOption,
    ) -> Result<QueryFilterViewResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/query"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<FilterViewListData, QueryFilterViewResp>()
        .await
    }
}

pub struct FilterViewConditionResource<'a> {
    config: &'a Config,
}

impl<'a> FilterViewConditionResource<'a> {
    pub async fn create(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        body: &CreateFilterViewConditionReqBody,
        option: &RequestOption,
    ) -> Result<CreateFilterViewConditionResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<FilterViewConditionData, CreateFilterViewConditionResp>()
        .await
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        condition_id: &str,
        option: &RequestOption,
    ) -> Result<GetFilterViewConditionResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<FilterViewConditionData, GetFilterViewConditionResp>()
        .await
    }

    pub async fn update(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        condition_id: &str,
        body: &UpdateFilterViewConditionReqBody,
        option: &RequestOption,
    ) -> Result<UpdateFilterViewConditionResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<FilterViewConditionData, UpdateFilterViewConditionResp>()
        .await
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        condition_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        option: &RequestOption,
    ) -> Result<QueryFilterViewConditionResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/query"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<FilterViewConditionListData, QueryFilterViewConditionResp>()
        .await
    }
}

pub struct ConditionalFormatResource<'a> {
    config: &'a Config,
}

impl<'a> ConditionalFormatResource<'a> {
    pub async fn create(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &CreateConditionalFormatReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/conditional_formats"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn update(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &UpdateConditionalFormatReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/conditional_formats/batch_update"
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &DeleteConditionalFormatReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/conditional_formats/batch_delete"
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<QueryConditionalFormatResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/conditional_formats/query"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("page_token", page_token)
        .query("page_size", page_size)
        .send_response::<ConditionalFormatListData, QueryConditionalFormatResp>()
        .await
    }
}

pub struct DataValidationResource<'a> {
    config: &'a Config,
}

impl<'a> DataValidationResource<'a> {
    pub async fn set_dropdown(
        &self,
        spreadsheet_token: &str,
        body: &SetDataValidationDropdownReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/data_validations/dropdown"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        range: &str,
        option: &RequestOption,
    ) -> Result<QueryDataValidationResp, LarkError> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/data_validations");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("range", range)
        .send_response::<DataValidationListData, QueryDataValidationResp>()
        .await
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        dv_id: i32,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/data_validations/{sheet_id}/{dv_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }
}

pub struct SpreadsheetSheetFloatImageResource<'a> {
    config: &'a Config,
}

impl<'a> SpreadsheetSheetFloatImageResource<'a> {
    pub async fn create(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &FloatImage,
        option: &RequestOption,
    ) -> Result<CreateFloatImageResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<FloatImageData, CreateFloatImageResp>()
        .await
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        float_image_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        float_image_id: &str,
        option: &RequestOption,
    ) -> Result<GetFloatImageResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<FloatImageData, GetFloatImageResp>()
        .await
    }

    pub async fn patch(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        float_image_id: &str,
        body: &FloatImage,
        option: &RequestOption,
    ) -> Result<PatchFloatImageResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<FloatImageData, PatchFloatImageResp>()
        .await
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        option: &RequestOption,
    ) -> Result<QueryFloatImageResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/query"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<FloatImageQueryData, QueryFloatImageResp>()
        .await
    }
}

pub struct SheetFilterResource<'a> {
    config: &'a Config,
}

impl<'a> SheetFilterResource<'a> {
    pub async fn create(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &CreateSheetFilterReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter"
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_empty()
        .await
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        option: &RequestOption,
    ) -> Result<GetSheetFilterResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<SheetFilterData, GetSheetFilterResp>()
        .await
    }

    pub async fn update(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &UpdateSheetFilterReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter"
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_empty()
        .await
    }
}

// ── Version struct ──

pub struct V3<'a> {
    pub spreadsheet: SpreadsheetResource<'a>,
    pub sheet: SheetResource<'a>,
    pub range: RangeResource<'a>,
    pub filter_view: FilterViewResource<'a>,
    pub filter_view_condition: FilterViewConditionResource<'a>,
    pub conditional_format: ConditionalFormatResource<'a>,
    pub data_validation: DataValidationResource<'a>,
    pub float_image: SpreadsheetSheetFloatImageResource<'a>,
    pub sheet_filter: SheetFilterResource<'a>,
}

impl<'a> V3<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            spreadsheet: SpreadsheetResource { config },
            sheet: SheetResource { config },
            range: RangeResource { config },
            filter_view: FilterViewResource { config },
            filter_view_condition: FilterViewConditionResource { config },
            conditional_format: ConditionalFormatResource { config },
            data_validation: DataValidationResource { config },
            float_image: SpreadsheetSheetFloatImageResource { config },
            sheet_filter: SheetFilterResource { config },
        }
    }
}
