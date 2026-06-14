//! Payroll v1 event handlers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PayrollPaymentActivityApprovedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PayrollPaymentActivityStatusChangedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

event_handlers! {
    on_p2_payroll_payment_activity_approved_v1 => P2PayrollPaymentActivityApprovedV1
        : "payroll.payment_activity.approved_v1",
    on_p2_payroll_payment_activity_status_changed_v1 => P2PayrollPaymentActivityStatusChangedV1
        : "payroll.payment_activity.status_changed_v1",
}
