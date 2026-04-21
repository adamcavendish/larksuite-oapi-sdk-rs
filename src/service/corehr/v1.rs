use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Shared sub-types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct I18nText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnumValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectFieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Employee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub race_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_type: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_contract_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_expected_end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_status: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_rate: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason: Option<EnumValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Department {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_manager: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staffing_model: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<I18nText>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Company {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nature: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<Vec<EnumValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_representative: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_payer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<Vec<EnumValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_company: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_manager: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registered_office_address: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_address: Option<Vec<I18nText>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiberarchy_common: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_usage: Option<Vec<EnumValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_language_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Currency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_alpha_3_code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkingHoursType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_region_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attendance_type: Option<EnumValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_for_new_job: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
}

// ── Response wrappers ──

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment: Option<Employee>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeListData {
    #[serde(default)]
    pub items: Vec<Employee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<Department>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentListData {
    #[serde(default)]
    pub items: Vec<Department>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevelData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level: Option<JobLevel>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobLevelListData {
    #[serde(default)]
    pub items: Vec<JobLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompanyData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompanyListData {
    #[serde(default)]
    pub items: Vec<Company>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationListData {
    #[serde(default)]
    pub items: Vec<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CurrencyListData {
    #[serde(default)]
    pub items: Vec<Currency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkingHoursTypeListData {
    #[serde(default)]
    pub items: Vec<WorkingHoursType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(GetEmployeeResp, EmployeeData);
impl_resp!(ListEmployeeResp, EmployeeListData);
impl_resp!(GetDepartmentResp, DepartmentData);
impl_resp!(ListDepartmentResp, DepartmentListData);
impl_resp!(GetJobLevelResp, JobLevelData);
impl_resp!(ListJobLevelResp, JobLevelListData);
impl_resp!(GetCompanyResp, CompanyData);
impl_resp!(ListCompanyResp, CompanyListData);
impl_resp!(GetLocationResp, LocationData);
impl_resp!(ListLocationResp, LocationListData);
impl_resp!(ListCurrencyResp, CurrencyListData);
impl_resp!(ListWorkingHoursTypeResp, WorkingHoursTypeListData);

// ── Resources ──

pub struct EmployeeResource<'a> {
    config: &'a Config,
}

impl<'a> EmployeeResource<'a> {
    pub async fn get(
        &self,
        employment_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetEmployeeResp> {
        let path = format!("/open-apis/corehr/v1/employments/{employment_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<EmployeeData>(self.config, &api_req, option).await?;
        Ok(GetEmployeeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEmployeeResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/employments");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<EmployeeListData>(self.config, &api_req, option).await?;
        Ok(ListEmployeeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct DepartmentResource<'a> {
    config: &'a Config,
}

impl<'a> DepartmentResource<'a> {
    pub async fn get(
        &self,
        department_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetDepartmentResp> {
        let path = format!("/open-apis/corehr/v1/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DepartmentData>(self.config, &api_req, option).await?;
        Ok(GetDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDepartmentResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/departments");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        if let Some(v) = user_id_type {
            api_req.query_params.set("user_id_type", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<DepartmentListData>(self.config, &api_req, option).await?;
        Ok(ListDepartmentResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct JobLevelResource<'a> {
    config: &'a Config,
}

impl<'a> JobLevelResource<'a> {
    pub async fn get(&self, job_level_id: &str, option: &RequestOption) -> Result<GetJobLevelResp> {
        let path = format!("/open-apis/corehr/v1/job_levels/{job_level_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<JobLevelData>(self.config, &api_req, option).await?;
        Ok(GetJobLevelResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobLevelResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/job_levels");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<JobLevelListData>(self.config, &api_req, option).await?;
        Ok(ListJobLevelResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct CompanyResource<'a> {
    config: &'a Config,
}

impl<'a> CompanyResource<'a> {
    pub async fn get(&self, company_id: &str, option: &RequestOption) -> Result<GetCompanyResp> {
        let path = format!("/open-apis/corehr/v1/companies/{company_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<CompanyData>(self.config, &api_req, option).await?;
        Ok(GetCompanyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCompanyResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/companies");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<CompanyListData>(self.config, &api_req, option).await?;
        Ok(ListCompanyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateCompanyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/companies");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateCompanyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        company_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCompanyResp> {
        let path = format!("/open-apis/corehr/v1/companies/{company_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteCompanyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        company_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchCompanyResp> {
        let path = format!("/open-apis/corehr/v1/companies/{company_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchCompanyResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct LocationResource<'a> {
    config: &'a Config,
}

impl<'a> LocationResource<'a> {
    pub async fn get(&self, location_id: &str, option: &RequestOption) -> Result<GetLocationResp> {
        let path = format!("/open-apis/corehr/v1/locations/{location_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<LocationData>(self.config, &api_req, option).await?;
        Ok(GetLocationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListLocationResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/locations");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<LocationListData>(self.config, &api_req, option).await?;
        Ok(ListLocationResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct CurrencyResource<'a> {
    config: &'a Config,
}

impl<'a> CurrencyResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCurrencyResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/currencies");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<CurrencyListData>(self.config, &api_req, option).await?;
        Ok(ListCurrencyResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

pub struct WorkingHoursTypeResource<'a> {
    config: &'a Config,
}

impl<'a> WorkingHoursTypeResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWorkingHoursTypeResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/working_hours_types",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<WorkingHoursTypeListData>(self.config, &api_req, option)
                .await?;
        Ok(ListWorkingHoursTypeResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Helpers for newer resources (use Option<CodeError> pattern) ──

fn parse_v2<T: for<'de> serde::Deserialize<'de>>(
    api_resp: ApiResp,
    raw: crate::resp::RawResponse<T>,
) -> impl FnOnce() -> (ApiResp, Option<CodeError>, Option<T>) {
    move || {
        let code_error = if raw.code_error.code != 0 {
            Some(raw.code_error)
        } else {
            None
        };
        (api_resp, code_error, raw.data)
    }
}

macro_rules! impl_resp_v2 {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: Option<CodeError>,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.as_ref().is_none_or(|e| e.code == 0)
            }
        }
    };
}

// ── Response types for new resources ──

impl_resp_v2!(SearchAssignedUserResp, serde_json::Value);
impl_resp_v2!(AddRoleAssignAuthorizationResp, serde_json::Value);
impl_resp_v2!(GetByParamAuthorizationResp, serde_json::Value);
impl_resp_v2!(QueryAuthorizationResp, serde_json::Value);
impl_resp_v2!(RemoveRoleAssignAuthorizationResp, serde_json::Value);
impl_resp_v2!(UpdateRoleAssignAuthorizationResp, serde_json::Value);
impl_resp_v2!(ConvertCommonDataIdResp, serde_json::Value);
impl_resp_v2!(AddEnumOptionCommonDataMetaDataResp, serde_json::Value);
impl_resp_v2!(EditEnumOptionCommonDataMetaDataResp, serde_json::Value);
impl_resp_v2!(MatchCompensationStandardResp, serde_json::Value);
impl_resp_v2!(CreateContractResp, serde_json::Value);
impl_resp_v2!(DeleteContractResp, ());
impl_resp_v2!(GetContractResp, serde_json::Value);
impl_resp_v2!(ListContractResp, serde_json::Value);
impl_resp_v2!(PatchContractResp, serde_json::Value);
impl_resp_v2!(GetCountryRegionResp, serde_json::Value);
impl_resp_v2!(ListCountryRegionResp, serde_json::Value);
impl_resp_v2!(GetCurrencyResp, serde_json::Value);
impl_resp_v2!(GetByParamCustomFieldResp, serde_json::Value);
impl_resp_v2!(ListObjectApiNameCustomFieldResp, serde_json::Value);
impl_resp_v2!(QueryCustomFieldResp, serde_json::Value);
impl_resp_v2!(CreateDepartmentResp, serde_json::Value);
impl_resp_v2!(DeleteDepartmentResp, ());
impl_resp_v2!(PatchDepartmentResp, serde_json::Value);
impl_resp_v2!(CreateEmployeeTypeResp, serde_json::Value);
impl_resp_v2!(DeleteEmployeeTypeResp, ());
impl_resp_v2!(GetEmployeeTypeResp, serde_json::Value);
impl_resp_v2!(ListEmployeeTypeResp, serde_json::Value);
impl_resp_v2!(PatchEmployeeTypeResp, serde_json::Value);
impl_resp_v2!(CreateEmploymentResp, serde_json::Value);
impl_resp_v2!(DeleteEmploymentResp, ());
impl_resp_v2!(PatchEmploymentResp, serde_json::Value);
impl_resp_v2!(GetFileResp, serde_json::Value);
impl_resp_v2!(CreateJobResp, serde_json::Value);
impl_resp_v2!(DeleteJobResp, ());
impl_resp_v2!(GetJobResp, serde_json::Value);
impl_resp_v2!(ListJobResp, serde_json::Value);
impl_resp_v2!(PatchJobResp, serde_json::Value);
impl_resp_v2!(CreateJobChangeResp, serde_json::Value);
impl_resp_v2!(CreateJobDataResp, serde_json::Value);
impl_resp_v2!(DeleteJobDataResp, ());
impl_resp_v2!(GetJobDataResp, serde_json::Value);
impl_resp_v2!(ListJobDataResp, serde_json::Value);
impl_resp_v2!(PatchJobDataResp, serde_json::Value);
impl_resp_v2!(CreateJobFamilyResp, serde_json::Value);
impl_resp_v2!(DeleteJobFamilyResp, ());
impl_resp_v2!(GetJobFamilyResp, serde_json::Value);
impl_resp_v2!(ListJobFamilyResp, serde_json::Value);
impl_resp_v2!(PatchJobFamilyResp, serde_json::Value);
impl_resp_v2!(CreateJobLevelResp, serde_json::Value);
impl_resp_v2!(DeleteJobLevelResp, ());
impl_resp_v2!(PatchJobLevelResp, serde_json::Value);
impl_resp_v2!(CalendarByScopeLeaveResp, serde_json::Value);
impl_resp_v2!(LeaveBalancesLeaveResp, serde_json::Value);
impl_resp_v2!(LeaveRequestHistoryLeaveResp, serde_json::Value);
impl_resp_v2!(LeaveTypesLeaveResp, serde_json::Value);
impl_resp_v2!(WorkCalendarLeaveResp, serde_json::Value);
impl_resp_v2!(WorkCalendarDateLeaveResp, serde_json::Value);
impl_resp_v2!(CreateLeaveGrantingRecordResp, serde_json::Value);
impl_resp_v2!(DeleteLeaveGrantingRecordResp, ());
impl_resp_v2!(CreateLocationResp, serde_json::Value);
impl_resp_v2!(DeleteLocationResp, ());
impl_resp_v2!(CreateNationalIdTypeResp, serde_json::Value);
impl_resp_v2!(DeleteNationalIdTypeResp, ());
impl_resp_v2!(GetNationalIdTypeResp, serde_json::Value);
impl_resp_v2!(ListNationalIdTypeResp, serde_json::Value);
impl_resp_v2!(PatchNationalIdTypeResp, serde_json::Value);
impl_resp_v2!(QueryOffboardingResp, serde_json::Value);
impl_resp_v2!(SearchOffboardingResp, serde_json::Value);
impl_resp_v2!(SubmitOffboardingResp, serde_json::Value);
impl_resp_v2!(CreatePersonResp, serde_json::Value);
impl_resp_v2!(DeletePersonResp, ());
impl_resp_v2!(GetPersonResp, serde_json::Value);
impl_resp_v2!(PatchPersonResp, serde_json::Value);
impl_resp_v2!(UploadPersonResp, serde_json::Value);
impl_resp_v2!(DeletePreHireResp, ());
impl_resp_v2!(GetPreHireResp, serde_json::Value);
impl_resp_v2!(ListPreHireResp, serde_json::Value);
impl_resp_v2!(PatchPreHireResp, serde_json::Value);
impl_resp_v2!(GetProcessFormVariableDataResp, serde_json::Value);
impl_resp_v2!(ListSecurityGroupResp, serde_json::Value);
impl_resp_v2!(QuerySecurityGroupResp, serde_json::Value);
impl_resp_v2!(GetSubdivisionResp, serde_json::Value);
impl_resp_v2!(ListSubdivisionResp, serde_json::Value);
impl_resp_v2!(GetSubregionResp, serde_json::Value);
impl_resp_v2!(ListSubregionResp, serde_json::Value);
impl_resp_v2!(QueryTransferReasonResp, serde_json::Value);
impl_resp_v2!(QueryTransferTypeResp, serde_json::Value);
impl_resp_v2!(CreateWorkingHoursTypeResp, serde_json::Value);
impl_resp_v2!(DeleteWorkingHoursTypeResp, ());
impl_resp_v2!(GetWorkingHoursTypeResp, serde_json::Value);
impl_resp_v2!(PatchWorkingHoursTypeResp, serde_json::Value);
impl_resp_v2!(CreateCompanyResp, serde_json::Value);
impl_resp_v2!(DeleteCompanyResp, ());
impl_resp_v2!(PatchCompanyResp, serde_json::Value);

// ── New Resource Structs ──

pub struct AssignedUserResource<'a> {
    config: &'a Config,
}

impl<'a> AssignedUserResource<'a> {
    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchAssignedUserResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/assigned_users/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(SearchAssignedUserResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct AuthorizationResource<'a> {
    config: &'a Config,
}

impl<'a> AuthorizationResource<'a> {
    pub async fn add_role_assign(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddRoleAssignAuthorizationResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/authorizations/add_role_assign",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(AddRoleAssignAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get_by_param(
        &self,
        option: &RequestOption,
    ) -> Result<GetByParamAuthorizationResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/authorizations/get_by_param",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetByParamAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(&self, option: &RequestOption) -> Result<QueryAuthorizationResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/authorizations/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(QueryAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn remove_role_assign(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<RemoveRoleAssignAuthorizationResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/authorizations/remove_role_assign",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(RemoveRoleAssignAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update_role_assign(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UpdateRoleAssignAuthorizationResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/authorizations/update_role_assign",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(UpdateRoleAssignAuthorizationResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CommonDataIdResource<'a> {
    config: &'a Config,
}

impl<'a> CommonDataIdResource<'a> {
    pub async fn convert(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ConvertCommonDataIdResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/common_data/id/convert",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ConvertCommonDataIdResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CommonDataMetaDataResource<'a> {
    config: &'a Config,
}

impl<'a> CommonDataMetaDataResource<'a> {
    pub async fn add_enum_option(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddEnumOptionCommonDataMetaDataResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/common_data/meta_data/add_enum_option",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(AddEnumOptionCommonDataMetaDataResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn edit_enum_option(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<EditEnumOptionCommonDataMetaDataResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/common_data/meta_data/edit_enum_option",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(EditEnumOptionCommonDataMetaDataResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CompensationStandardResource<'a> {
    config: &'a Config,
}

impl<'a> CompensationStandardResource<'a> {
    pub async fn match_standard(
        &self,
        option: &RequestOption,
    ) -> Result<MatchCompensationStandardResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/compensation_standards/match",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(MatchCompensationStandardResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ContractResource<'a> {
    config: &'a Config,
}

impl<'a> ContractResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateContractResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/contracts");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateContractResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        contract_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteContractResp> {
        let path = format!("/open-apis/corehr/v1/contracts/{contract_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteContractResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(&self, contract_id: &str, option: &RequestOption) -> Result<GetContractResp> {
        let path = format!("/open-apis/corehr/v1/contracts/{contract_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetContractResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListContractResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/contracts");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListContractResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        contract_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchContractResp> {
        let path = format!("/open-apis/corehr/v1/contracts/{contract_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchContractResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CountryRegionResource<'a> {
    config: &'a Config,
}

impl<'a> CountryRegionResource<'a> {
    pub async fn get(
        &self,
        country_region_id: &str,
        option: &RequestOption,
    ) -> Result<GetCountryRegionResp> {
        let path = format!("/open-apis/corehr/v1/country_regions/{country_region_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetCountryRegionResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCountryRegionResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/country_regions");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListCountryRegionResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CustomFieldResource<'a> {
    config: &'a Config,
}

impl<'a> CustomFieldResource<'a> {
    pub async fn get_by_param(&self, option: &RequestOption) -> Result<GetByParamCustomFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/custom_fields/get_by_param",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetByParamCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list_object_api_name(
        &self,
        option: &RequestOption,
    ) -> Result<ListObjectApiNameCustomFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/custom_fields/list_object_api_name",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListObjectApiNameCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(&self, option: &RequestOption) -> Result<QueryCustomFieldResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/custom_fields/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(QueryCustomFieldResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct EmployeeTypeResource<'a> {
    config: &'a Config,
}

impl<'a> EmployeeTypeResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateEmployeeTypeResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/employee_types");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateEmployeeTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        employee_type_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteEmployeeTypeResp> {
        let path = format!("/open-apis/corehr/v1/employee_types/{employee_type_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteEmployeeTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        employee_type_id: &str,
        option: &RequestOption,
    ) -> Result<GetEmployeeTypeResp> {
        let path = format!("/open-apis/corehr/v1/employee_types/{employee_type_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetEmployeeTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListEmployeeTypeResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/employee_types");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListEmployeeTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        employee_type_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmployeeTypeResp> {
        let path = format!("/open-apis/corehr/v1/employee_types/{employee_type_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchEmployeeTypeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct EmploymentResource<'a> {
    config: &'a Config,
}

impl<'a> EmploymentResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateEmploymentResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/employments");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateEmploymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        employment_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteEmploymentResp> {
        let path = format!("/open-apis/corehr/v1/employments/{employment_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteEmploymentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        employment_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchEmploymentResp> {
        let path = format!("/open-apis/corehr/v1/employments/{employment_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchEmploymentResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct FileResource<'a> {
    config: &'a Config,
}

impl<'a> FileResource<'a> {
    pub async fn get(&self, id: &str, option: &RequestOption) -> Result<GetFileResp> {
        let path = format!("/open-apis/corehr/v1/files/{id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetFileResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct JobResource<'a> {
    config: &'a Config,
}

impl<'a> JobResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/jobs");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(&self, job_id: &str, option: &RequestOption) -> Result<DeleteJobResp> {
        let path = format!("/open-apis/corehr/v1/jobs/{job_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(&self, job_id: &str, option: &RequestOption) -> Result<GetJobResp> {
        let path = format!("/open-apis/corehr/v1/jobs/{job_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/jobs");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListJobResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        job_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobResp> {
        let path = format!("/open-apis/corehr/v1/jobs/{job_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchJobResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct JobChangeResource<'a> {
    config: &'a Config,
}

impl<'a> JobChangeResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobChangeResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/job_changes");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateJobChangeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct JobDataResource<'a> {
    config: &'a Config,
}

impl<'a> JobDataResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobDataResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/job_datas");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateJobDataResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        job_data_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobDataResp> {
        let path = format!("/open-apis/corehr/v1/job_datas/{job_data_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteJobDataResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(&self, job_data_id: &str, option: &RequestOption) -> Result<GetJobDataResp> {
        let path = format!("/open-apis/corehr/v1/job_datas/{job_data_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetJobDataResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobDataResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/job_datas");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListJobDataResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        job_data_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobDataResp> {
        let path = format!("/open-apis/corehr/v1/job_datas/{job_data_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchJobDataResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct JobFamilyResource<'a> {
    config: &'a Config,
}

impl<'a> JobFamilyResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobFamilyResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/job_families");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateJobFamilyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        job_family_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobFamilyResp> {
        let path = format!("/open-apis/corehr/v1/job_families/{job_family_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteJobFamilyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        job_family_id: &str,
        option: &RequestOption,
    ) -> Result<GetJobFamilyResp> {
        let path = format!("/open-apis/corehr/v1/job_families/{job_family_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetJobFamilyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListJobFamilyResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/job_families");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListJobFamilyResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        job_family_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobFamilyResp> {
        let path = format!("/open-apis/corehr/v1/job_families/{job_family_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchJobFamilyResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct LeaveResource<'a> {
    config: &'a Config,
}

impl<'a> LeaveResource<'a> {
    pub async fn calendar_by_scope(
        &self,
        option: &RequestOption,
    ) -> Result<CalendarByScopeLeaveResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/calendar_by_scope",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CalendarByScopeLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn leave_balances(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<LeaveBalancesLeaveResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/leave_balances",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(LeaveBalancesLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn leave_request_history(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<LeaveRequestHistoryLeaveResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/leaves/leave_request_history",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(LeaveRequestHistoryLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn leave_types(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<LeaveTypesLeaveResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/leaves/leave_types");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(LeaveTypesLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn work_calendar(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<WorkCalendarLeaveResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/leaves/work_calendar",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(WorkCalendarLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn work_calendar_date(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<WorkCalendarDateLeaveResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/leaves/work_calendar_date",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(WorkCalendarDateLeaveResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct LeaveGrantingRecordResource<'a> {
    config: &'a Config,
}

impl<'a> LeaveGrantingRecordResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateLeaveGrantingRecordResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/leave_granting_records",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateLeaveGrantingRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        leave_granting_record_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteLeaveGrantingRecordResp> {
        let path =
            format!("/open-apis/corehr/v1/leave_granting_records/{leave_granting_record_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteLeaveGrantingRecordResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct NationalIdTypeResource<'a> {
    config: &'a Config,
}

impl<'a> NationalIdTypeResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateNationalIdTypeResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/national_id_types");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateNationalIdTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        national_id_type_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteNationalIdTypeResp> {
        let path = format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteNationalIdTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        national_id_type_id: &str,
        option: &RequestOption,
    ) -> Result<GetNationalIdTypeResp> {
        let path = format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetNationalIdTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListNationalIdTypeResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/national_id_types");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListNationalIdTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        national_id_type_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchNationalIdTypeResp> {
        let path = format!("/open-apis/corehr/v1/national_id_types/{national_id_type_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchNationalIdTypeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct OffboardingResource<'a> {
    config: &'a Config,
}

impl<'a> OffboardingResource<'a> {
    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryOffboardingResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/offboardings/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(QueryOffboardingResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn search(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchOffboardingResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/offboardings/search",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(SearchOffboardingResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn submit(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<SubmitOffboardingResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/offboardings/submit",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(SubmitOffboardingResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct PersonResource<'a> {
    config: &'a Config,
}

impl<'a> PersonResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreatePersonResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/persons");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreatePersonResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        person_id: &str,
        option: &RequestOption,
    ) -> Result<DeletePersonResp> {
        let path = format!("/open-apis/corehr/v1/persons/{person_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeletePersonResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(&self, person_id: &str, option: &RequestOption) -> Result<GetPersonResp> {
        let path = format!("/open-apis/corehr/v1/persons/{person_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetPersonResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        person_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPersonResp> {
        let path = format!("/open-apis/corehr/v1/persons/{person_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchPersonResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn upload(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadPersonResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/persons/upload");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(UploadPersonResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct PreHireResource<'a> {
    config: &'a Config,
}

impl<'a> PreHireResource<'a> {
    pub async fn delete(
        &self,
        pre_hire_id: &str,
        option: &RequestOption,
    ) -> Result<DeletePreHireResp> {
        let path = format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeletePreHireResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(&self, pre_hire_id: &str, option: &RequestOption) -> Result<GetPreHireResp> {
        let path = format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetPreHireResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListPreHireResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/pre_hires");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListPreHireResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        pre_hire_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPreHireResp> {
        let path = format!("/open-apis/corehr/v1/pre_hires/{pre_hire_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchPreHireResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ProcessFormVariableDataResource<'a> {
    config: &'a Config,
}

impl<'a> ProcessFormVariableDataResource<'a> {
    pub async fn get(
        &self,
        process_id: &str,
        option: &RequestOption,
    ) -> Result<GetProcessFormVariableDataResp> {
        let path = format!("/open-apis/corehr/v1/processes/{process_id}/form_variable_data");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetProcessFormVariableDataResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct SecurityGroupResource<'a> {
    config: &'a Config,
}

impl<'a> SecurityGroupResource<'a> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSecurityGroupResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/security_groups");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListSecurityGroupResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QuerySecurityGroupResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/security_groups/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(QuerySecurityGroupResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct SubdivisionResource<'a> {
    config: &'a Config,
}

impl<'a> SubdivisionResource<'a> {
    pub async fn get(
        &self,
        subdivision_id: &str,
        option: &RequestOption,
    ) -> Result<GetSubdivisionResp> {
        let path = format!("/open-apis/corehr/v1/subdivisions/{subdivision_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetSubdivisionResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSubdivisionResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/subdivisions");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListSubdivisionResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct SubregionResource<'a> {
    config: &'a Config,
}

impl<'a> SubregionResource<'a> {
    pub async fn get(
        &self,
        subregion_id: &str,
        option: &RequestOption,
    ) -> Result<GetSubregionResp> {
        let path = format!("/open-apis/corehr/v1/subregions/{subregion_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetSubregionResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSubregionResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/corehr/v1/subregions");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(ListSubregionResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct TransferReasonResource<'a> {
    config: &'a Config,
}

impl<'a> TransferReasonResource<'a> {
    pub async fn query(&self, option: &RequestOption) -> Result<QueryTransferReasonResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/transfer_reasons/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(QueryTransferReasonResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct TransferTypeResource<'a> {
    config: &'a Config,
}

impl<'a> TransferTypeResource<'a> {
    pub async fn query(&self, option: &RequestOption) -> Result<QueryTransferTypeResp> {
        let mut api_req = ApiReq::new(
            http::Method::GET,
            "/open-apis/corehr/v1/transfer_types/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(QueryTransferTypeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Methods added to existing resources (JobLevel, Currency, Location, WorkingHoursType) ──
// These are added via new impl blocks on the existing structs.

impl<'a> JobLevelResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateJobLevelResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/job_levels");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateJobLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        job_level_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteJobLevelResp> {
        let path = format!("/open-apis/corehr/v1/job_levels/{job_level_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteJobLevelResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        job_level_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchJobLevelResp> {
        let path = format!("/open-apis/corehr/v1/job_levels/{job_level_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchJobLevelResp {
            api_resp,
            code_error,
            data,
        })
    }
}

impl<'a> CurrencyResource<'a> {
    pub async fn get(&self, currency_id: &str, option: &RequestOption) -> Result<GetCurrencyResp> {
        let path = format!("/open-apis/corehr/v1/currencies/{currency_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetCurrencyResp {
            api_resp,
            code_error,
            data,
        })
    }
}

impl<'a> LocationResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateLocationResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/locations");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateLocationResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        location_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteLocationResp> {
        let path = format!("/open-apis/corehr/v1/locations/{location_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteLocationResp {
            api_resp,
            code_error,
            data,
        })
    }
}

impl<'a> DepartmentResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateDepartmentResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/corehr/v1/departments");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        department_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteDepartmentResp> {
        let path = format!("/open-apis/corehr/v1/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        department_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchDepartmentResp> {
        let path = format!("/open-apis/corehr/v1/departments/{department_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchDepartmentResp {
            api_resp,
            code_error,
            data,
        })
    }
}

impl<'a> WorkingHoursTypeResource<'a> {
    pub async fn create(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateWorkingHoursTypeResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/corehr/v1/working_hours_types",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(CreateWorkingHoursTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        working_hours_type_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteWorkingHoursTypeResp> {
        let path = format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(DeleteWorkingHoursTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn get(
        &self,
        working_hours_type_id: &str,
        option: &RequestOption,
    ) -> Result<GetWorkingHoursTypeResp> {
        let path = format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(GetWorkingHoursTypeResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        working_hours_type_id: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchWorkingHoursTypeResp> {
        let path = format!("/open-apis/corehr/v1/working_hours_types/{working_hours_type_id}");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw)();
        Ok(PatchWorkingHoursTypeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub employee: EmployeeResource<'a>,
    pub department: DepartmentResource<'a>,
    pub job_level: JobLevelResource<'a>,
    pub company: CompanyResource<'a>,
    pub location: LocationResource<'a>,
    pub currency: CurrencyResource<'a>,
    pub working_hours_type: WorkingHoursTypeResource<'a>,
    pub assigned_user: AssignedUserResource<'a>,
    pub authorization: AuthorizationResource<'a>,
    pub common_data_id: CommonDataIdResource<'a>,
    pub common_data_meta_data: CommonDataMetaDataResource<'a>,
    pub compensation_standard: CompensationStandardResource<'a>,
    pub contract: ContractResource<'a>,
    pub country_region: CountryRegionResource<'a>,
    pub custom_field: CustomFieldResource<'a>,
    pub employee_type: EmployeeTypeResource<'a>,
    pub employment: EmploymentResource<'a>,
    pub file: FileResource<'a>,
    pub job: JobResource<'a>,
    pub job_change: JobChangeResource<'a>,
    pub job_data: JobDataResource<'a>,
    pub job_family: JobFamilyResource<'a>,
    pub leave: LeaveResource<'a>,
    pub leave_granting_record: LeaveGrantingRecordResource<'a>,
    pub national_id_type: NationalIdTypeResource<'a>,
    pub offboarding: OffboardingResource<'a>,
    pub person: PersonResource<'a>,
    pub pre_hire: PreHireResource<'a>,
    pub process_form_variable_data: ProcessFormVariableDataResource<'a>,
    pub security_group: SecurityGroupResource<'a>,
    pub subdivision: SubdivisionResource<'a>,
    pub subregion: SubregionResource<'a>,
    pub transfer_reason: TransferReasonResource<'a>,
    pub transfer_type: TransferTypeResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            employee: EmployeeResource { config },
            department: DepartmentResource { config },
            job_level: JobLevelResource { config },
            company: CompanyResource { config },
            location: LocationResource { config },
            currency: CurrencyResource { config },
            working_hours_type: WorkingHoursTypeResource { config },
            assigned_user: AssignedUserResource { config },
            authorization: AuthorizationResource { config },
            common_data_id: CommonDataIdResource { config },
            common_data_meta_data: CommonDataMetaDataResource { config },
            compensation_standard: CompensationStandardResource { config },
            contract: ContractResource { config },
            country_region: CountryRegionResource { config },
            custom_field: CustomFieldResource { config },
            employee_type: EmployeeTypeResource { config },
            employment: EmploymentResource { config },
            file: FileResource { config },
            job: JobResource { config },
            job_change: JobChangeResource { config },
            job_data: JobDataResource { config },
            job_family: JobFamilyResource { config },
            leave: LeaveResource { config },
            leave_granting_record: LeaveGrantingRecordResource { config },
            national_id_type: NationalIdTypeResource { config },
            offboarding: OffboardingResource { config },
            person: PersonResource { config },
            pre_hire: PreHireResource { config },
            process_form_variable_data: ProcessFormVariableDataResource { config },
            security_group: SecurityGroupResource { config },
            subdivision: SubdivisionResource { config },
            subregion: SubregionResource { config },
            transfer_reason: TransferReasonResource { config },
            transfer_type: TransferTypeResource { config },
        }
    }
}
