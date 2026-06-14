//! Passport v1 event handlers.

use serde::{Deserialize, Serialize};

// ── Event payload types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PassportSessionCreatedV1 {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub login_time: i64,
    #[serde(default)]
    pub terminal_type: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct P2PassportSessionDeletedV1 {
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub logout_time: i64,
    #[serde(default)]
    pub terminal_type: i32,
}

// ── EventDispatcher extension methods ──

event_handlers! {
    on_p2_passport_session_created_v1 => P2PassportSessionCreatedV1
        : "passport.session.created_v1",
    on_p2_passport_session_deleted_v1 => P2PassportSessionDeletedV1
        : "passport.session.deleted_v1",
}
