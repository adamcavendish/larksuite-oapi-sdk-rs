//! Contact v3 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactUserCreatedV3 {
    #[serde(default)]
    pub object: serde_json::Value,
    #[serde(default)]
    pub old_object: serde_json::Value,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactUserUpdatedV3 {
    #[serde(default)]
    pub object: serde_json::Value,
    #[serde(default)]
    pub old_object: serde_json::Value,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactUserDeletedV3 {
    #[serde(default)]
    pub object: serde_json::Value,
    #[serde(default)]
    pub old_object: serde_json::Value,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactDepartmentCreatedV3 {
    #[serde(default)]
    pub object: serde_json::Value,
    #[serde(default)]
    pub old_object: serde_json::Value,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactDepartmentUpdatedV3 {
    #[serde(default)]
    pub object: serde_json::Value,
    #[serde(default)]
    pub old_object: serde_json::Value,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactDepartmentDeletedV3 {
    #[serde(default)]
    pub object: serde_json::Value,
    #[serde(default)]
    pub old_object: serde_json::Value,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactScopeUpdatedV3 {
    #[serde(default)]
    pub added_open_ids: Vec<String>,
    #[serde(default)]
    pub added_department_ids: Vec<String>,
    #[serde(default)]
    pub added_open_group_ids: Vec<String>,
}

// ── Handler registration helpers ──

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

impl EventDispatcher {
    pub fn on_p2_contact_user_created_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactUserCreatedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.user.created_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_user_updated_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactUserUpdatedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.user.updated_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_user_deleted_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactUserDeletedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.user.deleted_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_department_created_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactDepartmentCreatedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.department.created_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_department_updated_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactDepartmentUpdatedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.department.updated_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_department_deleted_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactDepartmentDeletedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.department.deleted_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_scope_updated_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactScopeUpdatedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.scope.updated_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_employee_type_enum_created_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "contact.employee_type_enum.created_v3",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_contact_employee_type_enum_updated_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "contact.employee_type_enum.updated_v3",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_contact_employee_type_enum_deleted_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "contact.employee_type_enum.deleted_v3",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_contact_job_family_created_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.job_family.created_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_job_family_updated_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.job_family.updated_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_job_family_deleted_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.job_family.deleted_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_job_level_created_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.job_level.created_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_job_level_updated_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.job_level.updated_v3", wrap_handler(handler))
    }

    pub fn on_p2_contact_job_level_deleted_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(serde_json::Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event("contact.job_level.deleted_v3", wrap_handler(handler))
    }
}
