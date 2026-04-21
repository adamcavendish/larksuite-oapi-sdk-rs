use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::EmptyResp;
use crate::transport;

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
impl_resp!(FindSheetResp, serde_json::Value);
impl_resp!(MoveDimensionSheetResp, serde_json::Value);
impl_resp!(ReplaceSheetResp, serde_json::Value);
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
    ) -> Result<CreateSpreadsheetResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/sheets/v3/spreadsheets");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SpreadsheetData>(self.config, &api_req, option).await?;
        Ok(CreateSpreadsheetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetSpreadsheetResp> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<SpreadsheetData>(self.config, &api_req, option).await?;
        Ok(GetSpreadsheetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        spreadsheet_token: &str,
        body: &PatchSpreadsheetReqBody,
        option: &RequestOption,
    ) -> Result<PatchSpreadsheetResp> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<SpreadsheetData>(self.config, &api_req, option).await?;
        Ok(PatchSpreadsheetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<GetSheetResp> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<Sheet>(self.config, &api_req, option).await?;
        Ok(GetSheetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        option: &RequestOption,
    ) -> Result<QuerySheetResp> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/query");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<SheetListData>(self.config, &api_req, option).await?;
        Ok(QuerySheetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &PatchSheetReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}");
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

    pub async fn operate(
        &self,
        spreadsheet_token: &str,
        body: &OperateSheetsReqBody,
        option: &RequestOption,
    ) -> Result<OperateSheetsResp> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets_batch_update");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<OperateSheetsData>(self.config, &api_req, option).await?;
        Ok(OperateSheetsResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn find(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<FindSheetResp> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/find");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(FindSheetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn move_dimension(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<MoveDimensionSheetResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/move_dimension"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(MoveDimensionSheetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn replace(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<ReplaceSheetResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/replace"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(ReplaceSheetResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<GetRangeValueResp> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values/{range}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = value_render_option {
            api_req.query_params.set("valueRenderOption", v);
        }
        if let Some(v) = date_time_render_option {
            api_req.query_params.set("dateTimeRenderOption", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(GetRangeValueResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn set(
        &self,
        spreadsheet_token: &str,
        range: &str,
        body: &SetRangeValueReqBody,
        option: &RequestOption,
    ) -> Result<SetRangeValueResp> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values/{range}");
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(SetRangeValueResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_get(
        &self,
        spreadsheet_token: &str,
        ranges: &[&str],
        value_render_option: Option<&str>,
        date_time_render_option: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchGetRangeValueResp> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_batch_get");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        for r in ranges {
            api_req.query_params.add("ranges", *r);
        }
        if let Some(v) = value_render_option {
            api_req.query_params.set("valueRenderOption", v);
        }
        if let Some(v) = date_time_render_option {
            api_req.query_params.set("dateTimeRenderOption", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(BatchGetRangeValueResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn batch_set(
        &self,
        spreadsheet_token: &str,
        body: &BatchSetRangeValueReqBody,
        option: &RequestOption,
    ) -> Result<BatchSetRangeValueResp> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_batch_update");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(BatchSetRangeValueResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn append(
        &self,
        spreadsheet_token: &str,
        range: &str,
        body: &AppendValueReqBody,
        insert_data_option: Option<&str>,
        option: &RequestOption,
    ) -> Result<AppendValueResp> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_append");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("range", range);
        if let Some(v) = insert_data_option {
            api_req.query_params.set("insertDataOption", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(AppendValueResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn prepend(
        &self,
        spreadsheet_token: &str,
        range: &str,
        body: &PrependValueReqBody,
        option: &RequestOption,
    ) -> Result<PrependValueResp> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_prepend");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("range", range);
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(PrependValueResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn clear(
        &self,
        spreadsheet_token: &str,
        range: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values/{range}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
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
    ) -> Result<CreateFilterViewResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FilterViewData>(self.config, &api_req, option).await?;
        Ok(CreateFilterViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        option: &RequestOption,
    ) -> Result<GetFilterViewResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<FilterViewData>(self.config, &api_req, option).await?;
        Ok(GetFilterViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        body: &PatchFilterViewReqBody,
        option: &RequestOption,
    ) -> Result<PatchFilterViewResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
        );
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FilterViewData>(self.config, &api_req, option).await?;
        Ok(PatchFilterViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        option: &RequestOption,
    ) -> Result<QueryFilterViewResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/query"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<FilterViewListData>(self.config, &api_req, option).await?;
        Ok(QueryFilterViewResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<CreateFilterViewConditionResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FilterViewConditionData>(self.config, &api_req, option)
                .await?;
        Ok(CreateFilterViewConditionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        condition_id: &str,
        option: &RequestOption,
    ) -> Result<GetFilterViewConditionResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<FilterViewConditionData>(self.config, &api_req, option)
                .await?;
        Ok(GetFilterViewConditionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        condition_id: &str,
        body: &UpdateFilterViewConditionReqBody,
        option: &RequestOption,
    ) -> Result<UpdateFilterViewConditionResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
        );
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FilterViewConditionData>(self.config, &api_req, option)
                .await?;
        Ok(UpdateFilterViewConditionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        condition_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        filter_view_id: &str,
        option: &RequestOption,
    ) -> Result<QueryFilterViewConditionResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/query"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<FilterViewConditionListData>(self.config, &api_req, option)
                .await?;
        Ok(QueryFilterViewConditionResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/conditional_formats"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn update(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &UpdateConditionalFormatReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/conditional_formats/batch_update"
        );
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &DeleteConditionalFormatReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/conditional_formats/batch_delete"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<QueryConditionalFormatResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/conditional_formats/query"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        let (api_resp, raw) =
            transport::request_typed::<ConditionalFormatListData>(self.config, &api_req, option)
                .await?;
        Ok(QueryConditionalFormatResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/data_validations/dropdown"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        range: &str,
        option: &RequestOption,
    ) -> Result<QueryDataValidationResp> {
        let path =
            format!("/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/data_validations");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.query_params.set("range", range);
        let (api_resp, raw) =
            transport::request_typed::<DataValidationListData>(self.config, &api_req, option)
                .await?;
        Ok(QueryDataValidationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        dv_id: i32,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/data_validations/{sheet_id}/{dv_id}"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
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
    ) -> Result<CreateFloatImageResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FloatImageData>(self.config, &api_req, option).await?;
        Ok(CreateFloatImageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        float_image_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        float_image_id: &str,
        option: &RequestOption,
    ) -> Result<GetFloatImageResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<FloatImageData>(self.config, &api_req, option).await?;
        Ok(GetFloatImageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn patch(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        float_image_id: &str,
        body: &FloatImage,
        option: &RequestOption,
    ) -> Result<PatchFloatImageResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}"
        );
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FloatImageData>(self.config, &api_req, option).await?;
        Ok(PatchFloatImageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn query(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        option: &RequestOption,
    ) -> Result<QueryFloatImageResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/query"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<FloatImageQueryData>(self.config, &api_req, option).await?;
        Ok(QueryFloatImageResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
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
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn delete(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
    }

    pub async fn get(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        option: &RequestOption,
    ) -> Result<GetSheetFilterResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<SheetFilterData>(self.config, &api_req, option).await?;
        Ok(GetSheetFilterResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn update(
        &self,
        spreadsheet_token: &str,
        sheet_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<EmptyResp> {
        let path = format!(
            "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter"
        );
        let mut api_req = ApiReq::new(http::Method::PUT, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        Ok(EmptyResp {
            api_resp,
            code_error: raw.code_error,
        })
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
