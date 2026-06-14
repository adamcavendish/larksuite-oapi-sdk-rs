//! CoreHR v1 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

// ── Shared sub-types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
}

impl UserId {
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    pub fn open_id(&self) -> Option<&str> {
        self.open_id.as_deref()
    }

    pub fn union_id(&self) -> Option<&str> {
        self.union_id.as_deref()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManagementScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management_dimension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obj_id: Option<String>,
}

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CommonDataIdUserMappingChangedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_transform_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corehr_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub people_admin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feishu_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CommonDataMetaDataUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[serde(default)]
    pub field_changes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_type: Option<String>,
    #[serde(default)]
    pub enum_value_changes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContractV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DepartmentCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DepartmentUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default)]
    pub field_changes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2DepartmentDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
    #[serde(default)]
    pub field_changes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentConvertedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EmploymentResignedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobChangeUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_mode: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_type_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_reason_unique_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobDataChangedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobDataIdV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2JobDataEmployedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2OffboardingUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2OrgRoleAuthorizationUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(default)]
    pub management_scope_list: Vec<ManagementScope>,
    #[serde(default)]
    pub employment_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PersonCreatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PersonDeletedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PersonUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    #[serde(default)]
    pub field_changes: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PreHireUpdatedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_hire_id: Option<String>,
    #[serde(default)]
    pub field_changes: Vec<String>,
}

// ── Handler wrapper ──

fn wrap_handler<T, F, Fut>(
    handler: F,
) -> impl Fn(serde_json::Value) -> Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>
+ Send
+ Sync
+ 'static
where
    T: for<'de> serde::Deserialize<'de> + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
{
    move |val: serde_json::Value| {
        let result: std::result::Result<T, _> = serde_json::from_value(val);
        match result {
            Ok(typed) => Box::pin(handler(typed))
                as Pin<Box<dyn Future<Output = Result<(), LarkError>> + Send>>,
            Err(e) => Box::pin(async move {
                Err(LarkError::Event(format!(
                    "failed to deserialize event payload: {e}"
                )))
            }),
        }
    }
}

// ── EventDispatcher extension methods ──

macro_rules! corehr_v1_handler {
    ($method:ident, $event_key:literal, $payload_type:ty) => {
        pub fn $method<F, Fut>(self, handler: F) -> Self
        where
            F: Fn($payload_type) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
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
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("corehr.employment.created_v1", wrap_handler(handler))
    }

    #[deprecated(note = "use on_p2_corehr_employment_updated_v1 instead")]
    pub fn on_p2_corehr_employee_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2EmploymentUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("corehr.employment.updated_v1", wrap_handler(handler))
    }

    #[deprecated(note = "use on_p2_corehr_offboarding_updated_v1 instead")]
    pub fn on_p2_corehr_employee_offboarding_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2OffboardingUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("corehr.offboarding.updated_v1", wrap_handler(handler))
    }

    #[deprecated(note = "use on_p2_corehr_job_change_updated_v1 instead")]
    pub fn on_p2_corehr_job_changed_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2JobChangeUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("corehr.job_change.updated_v1", wrap_handler(handler))
    }

    pub fn on_p2_corehr_probation_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "corehr.probation_management.updated_v1",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_corehr_org_role_authorization_updated_v2<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "corehr.org_role_authorization.updated_v2",
            wrap_handler(handler),
        )
    }
}
