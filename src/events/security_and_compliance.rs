//! Security and Compliance v2 event handlers.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::error::LarkError;
use crate::event::EventDispatcher;

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
pub struct DeviceRecordEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_system: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aaid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_ownership: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certification_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_terminal_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdm_device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdm_provider_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_env_detect_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceChangeEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_system: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aaid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_ownership: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certification_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_terminal_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdm_device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdm_provider_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_env_detect_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SecurityAndComplianceDeviceApplyRecordDeviceApplyEventV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_apply_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_record: Option<DeviceRecordEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_device_ownership: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2SecurityAndComplianceDeviceRecordDeviceChangeEventV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<DeviceChangeEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<DeviceChangeEvent>,
}

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

impl EventDispatcher {
    pub fn on_p2_security_and_compliance_device_apply_record_device_apply_event_v2<F, Fut>(
        self,
        handler: F,
    ) -> Self
    where
        F: Fn(P2SecurityAndComplianceDeviceApplyRecordDeviceApplyEventV2) -> Fut
            + Send
            + Sync
            + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "security_and_compliance.device_apply_record.device_apply_event_v2",
            wrap_handler(handler),
        )
    }

    pub fn on_p2_security_and_compliance_device_record_device_change_event_v2<F, Fut>(
        self,
        handler: F,
    ) -> Self
    where
        F: Fn(P2SecurityAndComplianceDeviceRecordDeviceChangeEventV2) -> Fut
            + Send
            + Sync
            + 'static,
        Fut: Future<Output = Result<(), LarkError>> + Send + 'static,
    {
        self.on_event(
            "security_and_compliance.device_record.device_change_event_v2",
            wrap_handler(handler),
        )
    }
}
