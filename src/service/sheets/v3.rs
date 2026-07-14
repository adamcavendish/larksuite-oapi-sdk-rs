use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
pub struct SheetFilterView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_view_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_view_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
pub struct Chart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConditionalFormat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cf_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataValidation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dv_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dv_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<serde_json::Value>>,
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
    pub requests: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchSheetReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SetRangeValueReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_range: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchSetRangeValueReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_ranges: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AppendValueReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_range: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PrependValueReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_range: Option<serde_json::Value>,
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
    pub options: Option<Vec<serde_json::Value>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpreadsheetData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spreadsheet: Option<Spreadsheet>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SheetListData {
    #[serde(default)]
    pub sheets: Vec<Sheet>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OperateSheetsData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterViewData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_view: Option<SheetFilterView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterViewListData {
    #[serde(default)]
    pub items: Vec<SheetFilterView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterViewConditionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<SheetFilterViewCondition>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterViewConditionListData {
    #[serde(default)]
    pub items: Vec<SheetFilterViewCondition>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConditionalFormatListData {
    #[serde(default)]
    pub sheet_conditional_formats: Vec<ConditionalFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataValidationListData {
    #[serde(default)]
    pub items: Vec<DataValidation>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FindReplaceResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matched_cells: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matched_formula_cells: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FindSheetData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub find_result: Option<FindReplaceResult>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
impl_resp!(GetRangeValueResp, serde_json::Value);
impl_resp!(SetRangeValueResp, serde_json::Value);
impl_resp!(BatchGetRangeValueResp, serde_json::Value);
impl_resp!(BatchSetRangeValueResp, serde_json::Value);
impl_resp!(AppendValueResp, serde_json::Value);
impl_resp!(PrependValueResp, serde_json::Value);
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
pub struct SheetFilterData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet_filter_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
pub struct FloatImageData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub float_image: Option<FloatImage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
        body: &serde_json::Value,
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
        body: &serde_json::Value,
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
        body: &serde_json::Value,
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
        .send_response::<serde_json::Value, GetRangeValueResp>()
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
        .send_response::<serde_json::Value, SetRangeValueResp>()
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
        .send_response::<serde_json::Value, BatchGetRangeValueResp>()
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
        .send_response::<serde_json::Value, BatchSetRangeValueResp>()
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
        .send_response::<serde_json::Value, AppendValueResp>()
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
        .send_response::<serde_json::Value, PrependValueResp>()
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
        body: &serde_json::Value,
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
        body: &serde_json::Value,
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
