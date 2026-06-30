use larksuite_oapi_sdk_rs::EventDispatcher;
use larksuite_oapi_sdk_rs::events::auth::P2UserAccessTokenRevokedV4;

#[test]
fn auth_user_access_token_revoked_event_is_typed() {
    let event: P2UserAccessTokenRevokedV4 = serde_json::from_value(serde_json::json!({
        "event": {
            "revoke_token_type": "user_access_token",
            "revoke_reason": "user_logout",
            "open_id": "ou_1",
            "union_id": "on_1",
            "user_id": "user_1"
        }
    }))
    .unwrap();

    let data = event.event.as_ref().unwrap();
    assert_eq!(data.revoke_token_type.as_deref(), Some("user_access_token"));
    assert_eq!(data.revoke_reason.as_deref(), Some("user_logout"));
    assert_eq!(data.open_id.as_deref(), Some("ou_1"));
    assert_eq!(data.union_id.as_deref(), Some("on_1"));
    assert_eq!(data.user_id.as_deref(), Some("user_1"));

    let _dispatcher =
        EventDispatcher::new("", "").on_p2_auth_user_access_token_revoked_v4(|_| async { Ok(()) });
}

#[test]
fn auth_user_access_token_revoked_event_accepts_empty_payload() {
    let event: P2UserAccessTokenRevokedV4 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(event.event.is_none());
}
