//! Auth v4 event handlers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2UserAccessTokenRevokedV4Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoke_token_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoke_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct P2UserAccessTokenRevokedV4 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<P2UserAccessTokenRevokedV4Data>,
}

event_handlers! {
    on_p2_auth_user_access_token_revoked_v4 => P2UserAccessTokenRevokedV4
        : "auth.user_access_token.revoked_v4",
}
