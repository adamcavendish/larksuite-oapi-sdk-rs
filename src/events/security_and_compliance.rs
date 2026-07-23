//! Security and Compliance v2 event handlers.

use serde::{Deserialize, Serialize};

pub use crate::events::common::UserId;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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

event_handlers! {
    on_p2_security_and_compliance_device_apply_record_device_apply_event_v2 => P2SecurityAndComplianceDeviceApplyRecordDeviceApplyEventV2
        : "security_and_compliance.device_apply_record.device_apply_event_v2",
    on_p2_security_and_compliance_device_record_device_change_event_v2 => P2SecurityAndComplianceDeviceRecordDeviceChangeEventV2
        : "security_and_compliance.device_record.device_change_event_v2",
}
