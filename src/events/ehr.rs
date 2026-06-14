//! eHR v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EhrEmployeeAddedV1 {
    #[serde(default)]
    pub emp_id: String,
    #[serde(default)]
    pub emp: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2EhrEmployeeUpdatedV1 {
    #[serde(default)]
    pub emp_id: String,
    #[serde(default)]
    pub emp: serde_json::Value,
    #[serde(default)]
    pub changed_fields: Vec<String>,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_ehr_employee_added_v1 => P2EhrEmployeeAddedV1
        : "ehr.employee.added_v1",
    on_p2_ehr_employee_updated_v1 => P2EhrEmployeeUpdatedV1
        : "ehr.employee.updated_v1",
}
