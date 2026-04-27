use larksuite_oapi_sdk_rs::error::LarkError;
use larksuite_oapi_sdk_rs::resp::CodeError;

#[test]
fn error_display_illegal_param() {
    let e = LarkError::IllegalParam("missing field".to_string());
    assert_eq!(format!("{e}"), "illegal parameter: missing field");
}

#[test]
fn error_display_client_timeout() {
    let e = LarkError::ClientTimeout("timed out".to_string());
    assert_eq!(format!("{e}"), "client timeout: timed out");
}

#[test]
fn error_display_server_timeout() {
    let e = LarkError::ServerTimeout("504".to_string());
    assert_eq!(format!("{e}"), "server timeout: 504");
}

#[test]
fn error_display_rate_limited() {
    let e = LarkError::RateLimited("429".to_string());
    assert_eq!(format!("{e}"), "rate limited: 429");
}

#[test]
fn error_display_dial_failed() {
    let e = LarkError::DialFailed("connection refused".to_string());
    assert_eq!(format!("{e}"), "dial failed: connection refused");
}

#[test]
fn error_display_token() {
    let e = LarkError::Token("ticket required".to_string());
    assert_eq!(format!("{e}"), "token error: ticket required");
}

#[test]
fn error_display_max_retries() {
    let e = LarkError::MaxRetries;
    assert_eq!(format!("{e}"), "max retries exceeded");
}

#[test]
fn error_display_event() {
    let e = LarkError::Event("bad signature".to_string());
    assert_eq!(format!("{e}"), "event error: bad signature");
}

#[test]
fn error_display_crypto() {
    let e = LarkError::Crypto("decryption failed".to_string());
    assert_eq!(format!("{e}"), "crypto error: decryption failed");
}

#[test]
fn error_display_api() {
    let code_err = CodeError {
        code: 42,
        msg: "test".to_string(),
        error: None,
    };
    let e = LarkError::Api(Box::new(code_err));
    assert_eq!(format!("{e}"), "api error: code: 42, msg: test");
}

#[test]
fn error_from_code_error() {
    let code_err = CodeError {
        code: 99,
        msg: "converted".to_string(),
        error: None,
    };
    let e: LarkError = code_err.into();
    assert!(matches!(e, LarkError::Api(_)));
}

#[test]
fn error_from_serde_json() {
    let e: Result<serde_json::Value, _> = serde_json::from_str("not json");
    let err: LarkError = e.unwrap_err().into();
    assert!(matches!(err, LarkError::Json(_)));
}

#[test]
fn error_debug_format() {
    let e = LarkError::MaxRetries;
    let debug = format!("{e:?}");
    assert!(debug.contains("MaxRetries"));
}
