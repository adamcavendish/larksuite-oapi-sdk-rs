//! Compensation v1 event handlers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2CompensationArchiveChangedV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operate_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_tid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_tid: Option<String>,
}

event_handlers! {
    on_p2_compensation_archive_changed_v1 => P2CompensationArchiveChangedV1
        : "compensation.archive.changed_v1",
}
