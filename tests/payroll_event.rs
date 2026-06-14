use larksuite_oapi_sdk_rs::events::payroll::{
    P2PayrollPaymentActivityApprovedV1, P2PayrollPaymentActivityStatusChangedV1,
};

#[test]
fn payroll_payment_activity_events_match_go_shape() {
    let approved: P2PayrollPaymentActivityApprovedV1 = serde_json::from_value(serde_json::json!({
        "activity_id": "activity_1"
    }))
    .unwrap();
    assert_eq!(approved.activity_id.as_deref(), Some("activity_1"));

    let status_changed: P2PayrollPaymentActivityStatusChangedV1 =
        serde_json::from_value(serde_json::json!({
            "activity_id": "activity_2",
            "status": 400
        }))
        .unwrap();
    assert_eq!(status_changed.activity_id.as_deref(), Some("activity_2"));
    assert_eq!(status_changed.status, Some(400));
}

#[test]
fn payroll_payment_activity_events_accept_empty_and_null_payloads() {
    let approved: P2PayrollPaymentActivityApprovedV1 =
        serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(approved.activity_id.is_none());

    let status_changed: P2PayrollPaymentActivityStatusChangedV1 =
        serde_json::from_value(serde_json::json!({
            "activity_id": null,
            "status": null
        }))
        .unwrap();
    assert!(status_changed.activity_id.is_none());
    assert!(status_changed.status.is_none());
}
