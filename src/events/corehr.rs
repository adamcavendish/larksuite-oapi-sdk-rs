use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Shared sub-types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserId {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub union_id: String,
    #[serde(default)]
    pub open_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManagementScope {
    #[serde(default)]
    pub management_dimension: String,
    #[serde(default)]
    pub obj_id: String,
}

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CommonDataIdUserMappingChangedV1 {
    #[serde(default)]
    pub change_type: String,
    #[serde(default)]
    pub id_transform_type: i32,
    #[serde(default)]
    pub corehr_id: String,
    #[serde(default)]
    pub people_admin_id: String,
    #[serde(default)]
    pub feishu_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CommonDataMetaDataUpdatedV1 {
    #[serde(default)]
    pub api_name: String,
    #[serde(default)]
    pub field_changes: Vec<String>,
    #[serde(default)]
    pub metadata_type: String,
    #[serde(default)]
    pub enum_value_changes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContractV1 {
    #[serde(default)]
    pub contract_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DepartmentCreatedV1 {
    #[serde(default)]
    pub department_id: String,
    #[serde(default)]
    pub object_type: i32,
    #[serde(default)]
    pub operator: serde_json::Value,
    #[serde(default)]
    pub opt_type: String,
    #[serde(default)]
    pub department: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DepartmentUpdatedV1 {
    #[serde(default)]
    pub department_id: String,
    #[serde(default)]
    pub object_type: i32,
    #[serde(default)]
    pub operator: serde_json::Value,
    #[serde(default)]
    pub opt_type: String,
    #[serde(default)]
    pub updated_fields: Vec<String>,
    #[serde(default)]
    pub department: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DepartmentDeletedV1 {
    #[serde(default)]
    pub department_id: String,
    #[serde(default)]
    pub object_type: i32,
    #[serde(default)]
    pub operator: serde_json::Value,
    #[serde(default)]
    pub opt_type: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentCreatedV1 {
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub object_type: i32,
    #[serde(default)]
    pub operator: serde_json::Value,
    #[serde(default)]
    pub opt_type: String,
    #[serde(default)]
    pub employment: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentUpdatedV1 {
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub object_type: i32,
    #[serde(default)]
    pub operator: serde_json::Value,
    #[serde(default)]
    pub opt_type: String,
    #[serde(default)]
    pub updated_fields: Vec<String>,
    #[serde(default)]
    pub employment: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentConvertedV1 {
    #[serde(default)]
    pub employment_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentDeletedV1 {
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub target_user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentResignedV1 {
    #[serde(default)]
    pub employment_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobV1 {
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobChangeUpdatedV1 {
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub job_change: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub effective_date: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobDataChangedV1 {
    #[serde(default)]
    pub job_data_id: String,
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub target_user_id: Option<UserId>,
    #[serde(default)]
    pub job_change_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobDataIdV1 {
    #[serde(default)]
    pub job_data_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobDataEmployedV1 {
    #[serde(default)]
    pub job_data_id: String,
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub target_user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2OffboardingUpdatedV1 {
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub offboarding_reason: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2OrgRoleAuthorizationUpdatedV1 {
    #[serde(default)]
    pub role_id: String,
    #[serde(default)]
    pub management_scope_list: Vec<ManagementScope>,
    #[serde(default)]
    pub employment_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PersonCreatedV1 {
    #[serde(default)]
    pub person_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PersonDeletedV1 {
    #[serde(default)]
    pub person_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PersonUpdatedV1 {
    #[serde(default)]
    pub person_id: String,
    #[serde(default)]
    pub field_changes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PreHireUpdatedV1 {
    #[serde(default)]
    pub pre_hire_id: String,
    #[serde(default)]
    pub field_changes: Vec<String>,
}

// ── Handler wrapper ──

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync + 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => {
                Box::pin(handler(typed)) as Pin<Box<dyn Future<Output = Result<()>> + Send>>
            }
            Err(e) => Box::pin(async move {
                Err(Error::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher extension methods (all 28 corehr/v1 handlers) ──

macro_rules! corehr_v1_handler {
    ($method:ident, $event_key:literal, $payload_type:ty) => {
        pub fn $method<F, Fut>(self, handler: F) -> Self
        where
            F: Fn($payload_type) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<()>> + Send + 'static,
        {
            self.on_event($event_key, wrap_handler(handler))
        }
    };
}

impl EventDispatcher {
    corehr_v1_handler!(
        on_p2_corehr_common_data_id_user_mapping_changed_v1,
        "corehr.common_data.id.user_mapping_changed_v1",
        P2CommonDataIdUserMappingChangedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_common_data_meta_data_updated_v1,
        "corehr.common_data.meta_data.updated_v1",
        P2CommonDataMetaDataUpdatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_contract_created_v1,
        "corehr.contract.created_v1",
        P2ContractV1
    );
    corehr_v1_handler!(
        on_p2_corehr_contract_deleted_v1,
        "corehr.contract.deleted_v1",
        P2ContractV1
    );
    corehr_v1_handler!(
        on_p2_corehr_contract_updated_v1,
        "corehr.contract.updated_v1",
        P2ContractV1
    );
    corehr_v1_handler!(
        on_p2_corehr_department_created_v1,
        "corehr.department.created_v1",
        P2DepartmentCreatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_department_deleted_v1,
        "corehr.department.deleted_v1",
        P2DepartmentDeletedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_department_updated_v1,
        "corehr.department.updated_v1",
        P2DepartmentUpdatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_employment_converted_v1,
        "corehr.employment.converted_v1",
        P2EmploymentConvertedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_employment_created_v1,
        "corehr.employment.created_v1",
        P2EmploymentCreatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_employment_deleted_v1,
        "corehr.employment.deleted_v1",
        P2EmploymentDeletedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_employment_resigned_v1,
        "corehr.employment.resigned_v1",
        P2EmploymentResignedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_employment_updated_v1,
        "corehr.employment.updated_v1",
        P2EmploymentUpdatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_job_created_v1,
        "corehr.job.created_v1",
        P2JobV1
    );
    corehr_v1_handler!(
        on_p2_corehr_job_deleted_v1,
        "corehr.job.deleted_v1",
        P2JobV1
    );
    corehr_v1_handler!(
        on_p2_corehr_job_updated_v1,
        "corehr.job.updated_v1",
        P2JobV1
    );
    corehr_v1_handler!(
        on_p2_corehr_job_change_updated_v1,
        "corehr.job_change.updated_v1",
        P2JobChangeUpdatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_job_data_changed_v1,
        "corehr.job_data.changed_v1",
        P2JobDataChangedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_job_data_created_v1,
        "corehr.job_data.created_v1",
        P2JobDataIdV1
    );
    corehr_v1_handler!(
        on_p2_corehr_job_data_deleted_v1,
        "corehr.job_data.deleted_v1",
        P2JobDataIdV1
    );
    corehr_v1_handler!(
        on_p2_corehr_job_data_employed_v1,
        "corehr.job_data.employed_v1",
        P2JobDataEmployedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_job_data_updated_v1,
        "corehr.job_data.updated_v1",
        P2JobDataIdV1
    );
    corehr_v1_handler!(
        on_p2_corehr_offboarding_updated_v1,
        "corehr.offboarding.updated_v1",
        P2OffboardingUpdatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_org_role_authorization_updated_v1,
        "corehr.org_role_authorization.updated_v1",
        P2OrgRoleAuthorizationUpdatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_person_created_v1,
        "corehr.person.created_v1",
        P2PersonCreatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_person_deleted_v1,
        "corehr.person.deleted_v1",
        P2PersonDeletedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_person_updated_v1,
        "corehr.person.updated_v1",
        P2PersonUpdatedV1
    );
    corehr_v1_handler!(
        on_p2_corehr_pre_hire_updated_v1,
        "corehr.pre_hire.updated_v1",
        P2PreHireUpdatedV1
    );

    // ── Deprecated aliases for old event key names ──

    #[deprecated(note = "use on_p2_corehr_employment_created_v1 instead")]
    pub fn on_p2_corehr_employee_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2EmploymentCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.employment.created_v1", wrap_handler(handler))
    }

    #[deprecated(note = "use on_p2_corehr_employment_updated_v1 instead")]
    pub fn on_p2_corehr_employee_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2EmploymentUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.employment.updated_v1", wrap_handler(handler))
    }

    #[deprecated(note = "use on_p2_corehr_offboarding_updated_v1 instead")]
    pub fn on_p2_corehr_employee_offboarding_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2OffboardingUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.offboarding.updated_v1", wrap_handler(handler))
    }

    #[deprecated(note = "use on_p2_corehr_job_change_updated_v1 instead")]
    pub fn on_p2_corehr_job_changed_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2JobChangeUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_change.updated_v1", wrap_handler(handler))
    }

    pub fn on_p2_corehr_probation_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "corehr.probation_management.updated_v1",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_corehr_org_role_authorization_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event(
            "corehr.org_role_authorization.updated_v2",
            wrap_handler(handler),
        )
    }
}
