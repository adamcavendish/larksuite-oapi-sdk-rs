use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CorehrEmployeeCreatedV1 {
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
pub struct P2CorehrEmployeeUpdatedV1 {
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
pub struct P2CorehrEmployeeOffboardingV1 {
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub offboarding_reason: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2CorehrDepartmentCreatedV1 {
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
pub struct P2CorehrDepartmentUpdatedV1 {
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
pub struct P2CorehrDepartmentDeletedV1 {
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
pub struct P2CorehrJobChangedV1 {
    #[serde(default)]
    pub employment_id: String,
    #[serde(default)]
    pub job_change: serde_json::Value,
    #[serde(default)]
    pub operator_id: serde_json::Value,
    #[serde(default)]
    pub effective_date: String,
}

// ── Handler registration helpers ──

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

// ── EventDispatcher extension methods ──

impl EventDispatcher {
    pub fn on_p2_corehr_employee_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CorehrEmployeeCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.employee.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_corehr_employee_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CorehrEmployeeUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.employee.updated_v1", wrap_handler(handler))
    }

    pub fn on_p2_corehr_employee_offboarding_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CorehrEmployeeOffboardingV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.employment.offboarding_v1", wrap_handler(handler))
    }

    pub fn on_p2_corehr_department_created_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CorehrDepartmentCreatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.department.created_v1", wrap_handler(handler))
    }

    pub fn on_p2_corehr_department_updated_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CorehrDepartmentUpdatedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.department.updated_v1", wrap_handler(handler))
    }

    pub fn on_p2_corehr_department_deleted_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CorehrDepartmentDeletedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.department.deleted_v1", wrap_handler(handler))
    }

    pub fn on_p2_corehr_job_changed_v1<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2CorehrJobChangedV1) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        self.on_event("corehr.job_change.updated_v1", wrap_handler(handler))
    }
}
