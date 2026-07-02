use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

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

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListPayrollRecordQuery<'a> {
    pub period: Option<&'a str>,
    pub employee_id: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListPayrollRecordQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn period(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.period = value.into();
        self
    }

    pub fn employee_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.employee_id = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListAcctItemQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListAcctItemQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListCostAllocationDetailQuery<'a> {
    pub cost_allocation_plan_id: Option<&'a str>,
    pub pay_period: Option<&'a str>,
    pub report_type: Option<i32>,
    pub page: PageQuery<'a>,
}

impl<'a> ListCostAllocationDetailQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cost_allocation_plan_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.cost_allocation_plan_id = value.into();
        self
    }

    pub fn pay_period(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.pay_period = value.into();
        self
    }

    pub fn report_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.report_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListCostAllocationPlanQuery<'a> {
    pub pay_period: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListCostAllocationPlanQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pay_period(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.pay_period = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListCostAllocationReportQuery<'a> {
    pub cost_allocation_plan_id: Option<&'a str>,
    pub pay_period: Option<&'a str>,
    pub report_type: Option<i32>,
    pub page: PageQuery<'a>,
}

impl<'a> ListCostAllocationReportQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cost_allocation_plan_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.cost_allocation_plan_id = value.into();
        self
    }

    pub fn pay_period(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.pay_period = value.into();
        self
    }

    pub fn report_type(mut self, value: impl Into<Option<i32>>) -> Self {
        self.report_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListDatasourceQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListDatasourceQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct QueryDatasourceRecordQuery<'a> {
    pub body: &'a serde_json::Value,
    pub page: PageQuery<'a>,
}

impl<'a> QueryDatasourceRecordQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            page: PageQuery::new(),
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SaveDatasourceRecordQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> SaveDatasourceRecordQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListPaygroupQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListPaygroupQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListPaymentActivityQuery<'a> {
    pub pay_period_start_date: Option<&'a str>,
    pub pay_period_end_date: Option<&'a str>,
    pub statuses: Option<&'a [i32]>,
    pub page: PageQuery<'a>,
}

impl<'a> ListPaymentActivityQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pay_period_start_date(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.pay_period_start_date = value.into();
        self
    }

    pub fn pay_period_end_date(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.pay_period_end_date = value.into();
        self
    }

    pub fn statuses(mut self, value: impl Into<Option<&'a [i32]>>) -> Self {
        self.statuses = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ArchivePaymentActivityQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> ArchivePaymentActivityQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListPaymentActivityDetailQuery<'a> {
    pub activity_id: &'a str,
    pub page_index: Option<i32>,
    pub page_size: Option<i32>,
    pub include_segment_data: Option<bool>,
    pub acct_item_ids: Option<&'a [&'a str]>,
}

impl<'a> ListPaymentActivityDetailQuery<'a> {
    pub fn new(activity_id: &'a str) -> Self {
        Self {
            activity_id,
            page_index: None,
            page_size: None,
            include_segment_data: None,
            acct_item_ids: None,
        }
    }

    pub fn page_index(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_index = value.into();
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn include_segment_data(mut self, value: impl Into<Option<bool>>) -> Self {
        self.include_segment_data = value.into();
        self
    }

    pub fn acct_item_ids(mut self, value: impl Into<Option<&'a [&'a str]>>) -> Self {
        self.acct_item_ids = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct QueryPaymentDetailQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> QueryPaymentDetailQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

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
        let query = ListPayrollRecordQuery::new()
            .period(period)
            .employee_id(employee_id)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPayrollRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPayrollRecordResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/payroll/v1/payroll_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("period", query.period)
        .query("employee_id", query.employee_id)
        .page_query(query.page)
        .send_response::<RecordListData, ListPayrollRecordResp>()
        .await
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
        let query = ListAcctItemQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAcctItemQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListAcctItemResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/payroll/v1/acct_items",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = ListCostAllocationDetailQuery::new()
            .cost_allocation_plan_id(cost_allocation_plan_id)
            .pay_period(pay_period)
            .report_type(report_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCostAllocationDetailQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCostAllocationDetailResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/payroll/v1/cost_allocation_details",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("cost_allocation_plan_id", query.cost_allocation_plan_id)
        .query("pay_period", query.pay_period)
        .query("report_type", query.report_type)
        .page_query(query.page)
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = ListCostAllocationPlanQuery::new()
            .pay_period(pay_period)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCostAllocationPlanQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCostAllocationPlanResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/payroll/v1/cost_allocation_plans",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("pay_period", query.pay_period)
        .page_query(query.page)
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = ListCostAllocationReportQuery::new()
            .cost_allocation_plan_id(cost_allocation_plan_id)
            .pay_period(pay_period)
            .report_type(report_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCostAllocationReportQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListCostAllocationReportResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/payroll/v1/cost_allocation_reports",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("cost_allocation_plan_id", query.cost_allocation_plan_id)
        .query("pay_period", query.pay_period)
        .query("report_type", query.report_type)
        .page_query(query.page)
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = ListDatasourceQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListDatasourceQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListDatasourceResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/payroll/v1/datasources",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = QueryDatasourceRecordQuery::new(body)
            .page(PageQuery::from_parts(page_size, page_token));
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryDatasourceRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryDatasourceRecordResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/payroll/v1/datasource_records/query",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = SaveDatasourceRecordQuery::new(body);
        self.save_by_query(&query, option).await
    }

    pub async fn save_by_query(
        &self,
        query: &SaveDatasourceRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<SaveDatasourceRecordResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/payroll/v1/datasource_records/save",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = ListPaygroupQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPaygroupQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPaygroupResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/payroll/v1/paygroups",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = ArchivePaymentActivityQuery::new(body);
        self.archive_by_query(&query, option).await
    }

    pub async fn archive_by_query(
        &self,
        query: &ArchivePaymentActivityQuery<'_>,
        option: &RequestOption,
    ) -> Result<ArchivePaymentActivityResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/payroll/v1/payment_activitys/archive",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = ListPaymentActivityQuery::new()
            .pay_period_start_date(pay_period_start_date)
            .pay_period_end_date(pay_period_end_date)
            .statuses(statuses)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPaymentActivityQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPaymentActivityResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/payroll/v1/payment_activitys",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("pay_period_start_date", query.pay_period_start_date)
        .query("pay_period_end_date", query.pay_period_end_date)
        .query_values("statuses", query.statuses)
        .page_query(query.page)
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = ListPaymentActivityDetailQuery::new(activity_id)
            .page_index(page_index)
            .page_size(page_size)
            .include_segment_data(include_segment_data)
            .acct_item_ids(acct_item_ids);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPaymentActivityDetailQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPaymentActivityDetailResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/payroll/v1/payment_activity_details",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("activity_id", query.activity_id)
        .query("page_index", query.page_index)
        .query("page_size", query.page_size)
        .query("include_segment_data", query.include_segment_data)
        .query_values("acct_item_ids", query.acct_item_ids)
        .send_v2::<serde_json::Value>()
        .await?;
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
        let query = QueryPaymentDetailQuery::new(body);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryPaymentDetailQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryPaymentDetailResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/payroll/v1/payment_detail/query",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
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
