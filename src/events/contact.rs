//! Contact v3 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;
use crate::service::contact::v3::{
    AvatarInfo, Department, DepartmentLeader, DepartmentStatus, EmployeeTypeEnum, User,
    UserCustomAttr, UserOrder, UserPosition, UserStatus,
};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
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
pub struct UserEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<AvatarInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_station: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_tenant_manager: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<UserPosition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<UserOrder>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attrs: Option<Vec<UserCustomAttr>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dotted_line_leader_user_ids: Option<Vec<String>>,
}

impl UserEvent {
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
pub struct OldUserObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DepartmentStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaders: Option<Vec<DepartmentLeader>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_hrbps: Option<Vec<UserId>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OldDepartmentObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DepartmentStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Scope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departments: Option<Vec<Department>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<UserGroup>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomAttrEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_field_key: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_open_query: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactUserCreatedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<UserEvent>,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactUserUpdatedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<UserEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_object: Option<UserEvent>,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactUserDeletedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<UserEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_object: Option<OldUserObject>,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactDepartmentCreatedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<DepartmentEvent>,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactDepartmentUpdatedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<DepartmentEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_object: Option<DepartmentEvent>,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactDepartmentDeletedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<DepartmentEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_object: Option<OldDepartmentObject>,
    #[serde(default)]
    pub subscription_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactScopeUpdatedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added: Option<Scope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed: Option<Scope>,
    #[serde(default)]
    pub added_open_ids: Vec<String>,
    #[serde(default)]
    pub added_department_ids: Vec<String>,
    #[serde(default)]
    pub added_open_group_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactCustomAttrEventUpdatedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<CustomAttrEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_object: Option<CustomAttrEvent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactEmployeeTypeEnumCreatedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_enum: Option<EmployeeTypeEnum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactEmployeeTypeEnumUpdatedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_enum: Option<EmployeeTypeEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_enum: Option<EmployeeTypeEnum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactEmployeeTypeEnumDeletedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_enum: Option<EmployeeTypeEnum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactEmployeeTypeEnumActivedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_enum: Option<EmployeeTypeEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_enum: Option<EmployeeTypeEnum>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2ContactEmployeeTypeEnumDeactivatedV3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_enum: Option<EmployeeTypeEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_enum: Option<EmployeeTypeEnum>,
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
        F: Fn(P2ContactEmployeeTypeEnumCreatedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "contact.employee_type_enum.created_v3",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_contact_employee_type_enum_updated_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactEmployeeTypeEnumUpdatedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "contact.employee_type_enum.updated_v3",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_contact_employee_type_enum_deleted_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactEmployeeTypeEnumDeletedV3) -> Fut + Send + Sync + 'static,
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

    pub fn on_p2_contact_custom_attr_event_updated_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactCustomAttrEventUpdatedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "contact.custom_attr_event.updated_v3",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_contact_employee_type_enum_actived_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactEmployeeTypeEnumActivedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "contact.employee_type_enum.actived_v3",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_contact_employee_type_enum_deactivated_v3<F, Fut>(self, handler: F) -> Self
    where
        F: Fn(P2ContactEmployeeTypeEnumDeactivatedV3) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "contact.employee_type_enum.deactivated_v3",
            wrap_handler(handler),
        )
    }
}
