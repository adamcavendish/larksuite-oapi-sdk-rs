use larksuite_oapi_sdk_rs::events::security_and_compliance::{
    P2SecurityAndComplianceDeviceApplyRecordDeviceApplyEventV2,
    P2SecurityAndComplianceDeviceRecordDeviceChangeEventV2,
};

#[test]
fn security_device_apply_event_is_typed() {
    let event: P2SecurityAndComplianceDeviceApplyRecordDeviceApplyEventV2 =
        serde_json::from_value(serde_json::json!({
            "device_apply_record_id": "apply_1",
            "device_record": {
                "device_record_id": "device_1",
                "version": "v1",
                "current_user_id": { "open_id": "ou_user" },
                "device_name": "MacBook Pro",
                "model": "MacBookPro18,3",
                "device_system": 1,
                "serial_number": "serial_1",
                "disk_serial_number": "disk_1",
                "uuid": "uuid_1",
                "mac_address": "00:11:22:33:44:55",
                "android_id": "android_1",
                "idfv": "idfv_1",
                "aaid": "aaid_1",
                "device_ownership": 2,
                "device_status": 3,
                "certification_level": 4,
                "device_terminal_type": 5,
                "is_managed": true,
                "mdm_device_id": "mdm_1",
                "mdm_provider_name": "provider",
                "device_env_detect_status": 6,
                "is_public": false,
                "source": 7
            },
            "apply_time": "1710000000",
            "apply_status": 1,
            "operator": { "union_id": "on_operator" },
            "apply_device_ownership": 2,
            "apply_reason": "replacement"
        }))
        .unwrap();

    let record = event.device_record.as_ref().unwrap();
    assert_eq!(event.device_apply_record_id.as_deref(), Some("apply_1"));
    assert_eq!(record.device_record_id.as_deref(), Some("device_1"));
    assert_eq!(
        record.current_user_id.as_ref().unwrap().open_id(),
        Some("ou_user")
    );
    assert_eq!(record.device_name.as_deref(), Some("MacBook Pro"));
    assert_eq!(record.is_managed, Some(true));
    assert_eq!(record.source, Some(7));
    assert_eq!(event.apply_time.as_deref(), Some("1710000000"));
    assert_eq!(event.apply_status, Some(1));
    assert_eq!(
        event.operator.as_ref().unwrap().union_id(),
        Some("on_operator")
    );
    assert_eq!(event.apply_device_ownership, Some(2));
    assert_eq!(event.apply_reason.as_deref(), Some("replacement"));
}

#[test]
fn security_device_change_event_is_typed() {
    let event: P2SecurityAndComplianceDeviceRecordDeviceChangeEventV2 =
        serde_json::from_value(serde_json::json!({
            "device_record_id": "device_1",
            "version": "v2",
            "change_type": 2,
            "before": {
                "device_record_id": "device_1",
                "current_user_id": { "user_id": "user_1" },
                "device_status": 1,
                "is_public": false
            },
            "after": {
                "device_record_id": "device_1",
                "current_user_id": { "union_id": "on_user" },
                "device_status": 2,
                "is_public": true
            }
        }))
        .unwrap();

    let before = event.before.as_ref().unwrap();
    let after = event.after.as_ref().unwrap();
    assert_eq!(event.device_record_id.as_deref(), Some("device_1"));
    assert_eq!(event.version.as_deref(), Some("v2"));
    assert_eq!(event.change_type, Some(2));
    assert_eq!(
        before.current_user_id.as_ref().unwrap().user_id(),
        Some("user_1")
    );
    assert_eq!(before.device_status, Some(1));
    assert_eq!(
        after.current_user_id.as_ref().unwrap().union_id(),
        Some("on_user")
    );
    assert_eq!(after.device_status, Some(2));
    assert_eq!(after.is_public, Some(true));
}

#[test]
fn security_events_accept_empty_and_null_payloads() {
    let apply: P2SecurityAndComplianceDeviceApplyRecordDeviceApplyEventV2 =
        serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(apply.device_apply_record_id.is_none());
    assert!(apply.device_record.is_none());

    let change: P2SecurityAndComplianceDeviceRecordDeviceChangeEventV2 =
        serde_json::from_value(serde_json::json!({
            "before": null,
            "after": null
        }))
        .unwrap();
    assert!(change.before.is_none());
    assert!(change.after.is_none());
}
