use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::parse_v2;
use crate::transport;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PayrollRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<serde_json::Value>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordListData {
    #[serde(default)]
    pub items: Vec<PayrollRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListPayrollRecordResp, RecordListData);

impl_resp_v2!(ListAcctItemResp, serde_json::Value);
impl_resp_v2!(ListCostAllocationDetailResp, serde_json::Value);
impl_resp_v2!(ListCostAllocationPlanResp, serde_json::Value);
impl_resp_v2!(ListCostAllocationReportResp, serde_json::Value);
impl_resp_v2!(ListDatasourceResp, serde_json::Value);
impl_resp_v2!(QueryDatasourceRecordResp, serde_json::Value);
impl_resp_v2!(SaveDatasourceRecordResp, serde_json::Value);
impl_resp_v2!(ListPaygroupResp, serde_json::Value);
impl_resp_v2!(ArchivePaymentActivityResp, serde_json::Value);
impl_resp_v2!(ListPaymentActivityResp, serde_json::Value);
impl_resp_v2!(ListPaymentActivityDetailResp, serde_json::Value);
impl_resp_v2!(QueryPaymentDetailResp, serde_json::Value);

// ── Resources ──

pub struct PayrollRecordResource<'a> {
    config: &'a Config,
}

impl<'a> PayrollRecordResource<'a> {
    pub async fn list(
        &self,
        period: Option<&str>,
        employee_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListPayrollRecordResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/payroll/v1/payroll_records");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = period {
            api_req.query_params.set("period", v);
        }
        if let Some(v) = employee_id {
            api_req.query_params.set("employee_id", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<RecordListData>(self.config, &api_req, option).await?;
        Ok(ListPayrollRecordResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── AcctItem resource ──

pub struct AcctItemResource<'a> {
    config: &'a Config,
}

impl AcctItemResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAcctItemResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/payroll/v1/acct_items");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListAcctItemResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── CostAllocationDetail resource ──

pub struct CostAllocationDetailResource<'a> {
    config: &'a Config,
}

impl CostAllocationDetailResource<'_> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        cost_allocation_plan_id: Option<&str>,
        pay_period: Option<&str>,
        report_type: Option<i32>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCostAllocationDetailResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/payroll/v1/cost_allocation_details",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = cost_allocation_plan_id {
            api_req.query_params.set("cost_allocation_plan_id", v);
        }
        if let Some(v) = pay_period {
            api_req.query_params.set("pay_period", v);
        }
        if let Some(v) = report_type {
            api_req.query_params.set("report_type", v.to_string());
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListCostAllocationDetailResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── CostAllocationPlan resource ──

pub struct CostAllocationPlanResource<'a> {
    config: &'a Config,
}

impl CostAllocationPlanResource<'_> {
    pub async fn list(
        &self,
        pay_period: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCostAllocationPlanResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/payroll/v1/cost_allocation_plans",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = pay_period {
            api_req.query_params.set("pay_period", v);
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListCostAllocationPlanResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── CostAllocationReport resource ──

pub struct CostAllocationReportResource<'a> {
    config: &'a Config,
}

impl CostAllocationReportResource<'_> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        cost_allocation_plan_id: Option<&str>,
        pay_period: Option<&str>,
        report_type: Option<i32>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCostAllocationReportResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/payroll/v1/cost_allocation_reports",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = cost_allocation_plan_id {
            api_req.query_params.set("cost_allocation_plan_id", v);
        }
        if let Some(v) = pay_period {
            api_req.query_params.set("pay_period", v);
        }
        if let Some(v) = report_type {
            api_req.query_params.set("report_type", v.to_string());
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListCostAllocationReportResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Datasource resource ──

pub struct DatasourceResource<'a> {
    config: &'a Config,
}

impl DatasourceResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDatasourceResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/payroll/v1/datasources");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListDatasourceResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── DatasourceRecord resource ──

pub struct DatasourceRecordResource<'a> {
    config: &'a Config,
}

impl DatasourceRecordResource<'_> {
    pub async fn query(
        &self,
        body: &serde_json::Value,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<QueryDatasourceRecordResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/payroll/v1/datasource_records/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryDatasourceRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn save(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<SaveDatasourceRecordResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/payroll/v1/datasource_records/save",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SaveDatasourceRecordResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Paygroup resource ──

pub struct PaygroupResource<'a> {
    config: &'a Config,
}

impl PaygroupResource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListPaygroupResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/payroll/v1/paygroups");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListPaygroupResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── PaymentActivity resource ──

pub struct PaymentActivityResource<'a> {
    config: &'a Config,
}

impl PaymentActivityResource<'_> {
    pub async fn archive(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<ArchivePaymentActivityResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/payroll/v1/payment_activitys/archive",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ArchivePaymentActivityResp {
            api_resp,
            code_error,
            data,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        pay_period_start_date: Option<&str>,
        pay_period_end_date: Option<&str>,
        statuses: Option<&[i32]>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListPaymentActivityResp, LarkError> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/payroll/v1/payment_activitys");
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        if let Some(v) = pay_period_start_date {
            api_req.query_params.set("pay_period_start_date", v);
        }
        if let Some(v) = pay_period_end_date {
            api_req.query_params.set("pay_period_end_date", v);
        }
        if let Some(statuses) = statuses {
            for s in statuses {
                api_req.query_params.add("statuses", s.to_string());
            }
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListPaymentActivityResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── PaymentActivityDetail resource ──

pub struct PaymentActivityDetailResource<'a> {
    config: &'a Config,
}

impl PaymentActivityDetailResource<'_> {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        activity_id: &str,
        page_index: Option<i32>,
        page_size: Option<i32>,
        include_segment_data: Option<bool>,
        acct_item_ids: Option<&[&str]>,
        option: &RequestOption,
    ) -> Result<ListPaymentActivityDetailResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/payroll/v1/payment_activity_details",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.query_params.set("activity_id", activity_id);
        if let Some(v) = page_index {
            api_req.query_params.set("page_index", v.to_string());
        }
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = include_segment_data {
            api_req
                .query_params
                .set("include_segment_data", v.to_string());
        }
        if let Some(ids) = acct_item_ids {
            for id in ids {
                api_req.query_params.add("acct_item_ids", *id);
            }
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListPaymentActivityDetailResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── PaymentDetail resource ──

pub struct PaymentDetailResource<'a> {
    config: &'a Config,
}

impl PaymentDetailResource<'_> {
    pub async fn query(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryPaymentDetailResp, LarkError> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/payroll/v1/payment_detail/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryPaymentDetailResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub payroll_record: PayrollRecordResource<'a>,
    pub acct_item: AcctItemResource<'a>,
    pub cost_allocation_detail: CostAllocationDetailResource<'a>,
    pub cost_allocation_plan: CostAllocationPlanResource<'a>,
    pub cost_allocation_report: CostAllocationReportResource<'a>,
    pub datasource: DatasourceResource<'a>,
    pub datasource_record: DatasourceRecordResource<'a>,
    pub paygroup: PaygroupResource<'a>,
    pub payment_activity: PaymentActivityResource<'a>,
    pub payment_activity_detail: PaymentActivityDetailResource<'a>,
    pub payment_detail: PaymentDetailResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            payroll_record: PayrollRecordResource { config },
            acct_item: AcctItemResource { config },
            cost_allocation_detail: CostAllocationDetailResource { config },
            cost_allocation_plan: CostAllocationPlanResource { config },
            cost_allocation_report: CostAllocationReportResource { config },
            datasource: DatasourceResource { config },
            datasource_record: DatasourceRecordResource { config },
            paygroup: PaygroupResource { config },
            payment_activity: PaymentActivityResource { config },
            payment_activity_detail: PaymentActivityDetailResource { config },
            payment_detail: PaymentDetailResource { config },
        }
    }
}
