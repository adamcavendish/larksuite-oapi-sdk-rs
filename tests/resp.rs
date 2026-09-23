use http::HeaderMap;
use larksuite_oapi_sdk_rs::resp::{ApiResp, CodeError, CodeErrorInfo, RawResponse};

fn make_api_resp(headers: Vec<(&str, &str)>, body: &[u8]) -> ApiResp {
    let mut header = HeaderMap::new();
    for (k, v) in headers {
        header.insert(
            http::header::HeaderName::from_bytes(k.as_bytes()).unwrap(),
            http::header::HeaderValue::from_str(v).unwrap(),
        );
    }
    ApiResp {
        status_code: 200,
        header,
        raw_body: body.to_vec(),
    }
}

#[test]
fn request_id_from_x_request_id() {
    let resp = make_api_resp(vec![("X-Request-Id", "req-123")], b"");
    assert_eq!(resp.request_id(), Some("req-123"));
}

#[test]
fn request_id_from_request_id_fallback() {
    let resp = make_api_resp(vec![("Request-Id", "req-456")], b"");
    assert_eq!(resp.request_id(), Some("req-456"));
}

#[test]
fn request_id_missing() {
    let resp = make_api_resp(vec![], b"");
    assert_eq!(resp.request_id(), None);
}

#[test]
fn log_id_present() {
    let resp = make_api_resp(vec![("X-Tt-Logid", "log-789")], b"");
    assert_eq!(resp.log_id(), Some("log-789"));
}

#[test]
fn log_id_missing() {
    let resp = make_api_resp(vec![], b"");
    assert_eq!(resp.log_id(), None);
}

#[test]
fn content_length_from_header() {
    let resp = make_api_resp(vec![("Content-Length", "42")], b"");
    assert_eq!(resp.content_length(), Some(42));
}

#[test]
fn content_length_missing_or_invalid() {
    let missing = make_api_resp(vec![], b"");
    assert_eq!(missing.content_length(), None);

    let invalid = make_api_resp(vec![("Content-Length", "not-a-number")], b"");
    assert_eq!(invalid.content_length(), None);
}

#[test]
fn file_name_from_content_disposition() {
    let resp = make_api_resp(
        vec![("Content-Disposition", "attachment; filename=\"report.pdf\"")],
        b"",
    );
    assert_eq!(resp.file_name_by_header(), Some("report.pdf".to_string()));
}

#[test]
fn file_name_without_quotes() {
    let resp = make_api_resp(
        vec![("Content-Disposition", "attachment; filename=data.csv")],
        b"",
    );
    assert_eq!(resp.file_name_by_header(), Some("data.csv".to_string()));
}

#[test]
fn file_name_missing_disposition() {
    let resp = make_api_resp(vec![], b"");
    assert_eq!(resp.file_name_by_header(), None);
}

#[test]
fn file_name_disposition_without_filename() {
    let resp = make_api_resp(vec![("Content-Disposition", "inline")], b"");
    assert_eq!(resp.file_name_by_header(), None);
}

#[test]
fn code_error_success() {
    let err = CodeError {
        code: 0,
        msg: "ok".to_string(),
        error: None,
    };
    assert!(err.success());
}

#[test]
fn code_error_not_success() {
    let err = CodeError {
        code: 99991671,
        msg: "invalid token".to_string(),
        error: None,
    };
    assert!(!err.success());
}

#[test]
fn code_error_display_without_log_id() {
    let err = CodeError {
        code: 100,
        msg: "bad request".to_string(),
        error: None,
    };
    assert_eq!(format!("{err}"), "code: 100, msg: bad request");
}

#[test]
fn code_error_display_with_log_id() {
    let err = CodeError {
        code: 200,
        msg: "something".to_string(),
        error: Some(CodeErrorInfo {
            log_id: Some("log-abc".to_string()),
            ..Default::default()
        }),
    };
    assert_eq!(
        format!("{err}"),
        "code: 200, msg: something, log_id: log-abc"
    );
}

#[test]
fn code_error_display_with_error_but_no_log_id() {
    let err = CodeError {
        code: 300,
        msg: "err".to_string(),
        error: Some(CodeErrorInfo {
            log_id: None,
            ..Default::default()
        }),
    };
    assert_eq!(format!("{err}"), "code: 300, msg: err");
}

#[test]
fn code_error_is_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(CodeError {
        code: 1,
        msg: "test".to_string(),
        error: None,
    });
    assert!(err.to_string().contains("code: 1"));
}

#[test]
fn code_error_serde_roundtrip() {
    let err = CodeError {
        code: 42,
        msg: "test error".to_string(),
        error: Some(CodeErrorInfo {
            log_id: Some("lid".to_string()),
            troubleshooter: Some("http://help".to_string()),
            details: None,
            permission_violations: None,
            field_violations: None,
            helps: None,
            message: None,
        }),
    };
    let json = serde_json::to_string(&err).unwrap();
    let deserialized: CodeError = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.code, 42);
    assert_eq!(deserialized.msg, "test error");
    assert_eq!(deserialized.error.unwrap().log_id.as_deref(), Some("lid"));
}

#[test]
fn code_error_deserialize_minimal() {
    let json = r#"{"code": 0, "msg": ""}"#;
    let err: CodeError = serde_json::from_str(json).unwrap();
    assert!(err.success());
    assert!(err.error.is_none());
}

#[test]
fn code_error_rendered_message_preserves_wire_fields() {
    let wire = serde_json::json!({
        "code": 3350001,
        "msg": "invalid param",
        "error": {
            "message": "Invalid request parameter: request. Invalid reason: insertion is required.",
            "log_id": "log-rendered",
            "troubleshooter": "https://example.com/help",
            "details": [{"key": "request", "value": "insert"}],
            "permission_violations": [{"type": "scope", "subject": "example:read", "description": "missing"}],
            "field_violations": [{"field": "request", "value": "", "description": "insertion is required"}],
            "helps": [{"url": "https://example.com/help", "description": "help"}]
        }
    });
    let err: CodeError = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(err.code, 3350001);
    assert_eq!(err.msg, "invalid param");
    assert!(!err.success());
    let rendered = wire["error"]["message"].as_str().unwrap();
    assert_eq!(err.effective_message(), rendered);
    assert_eq!(
        err.to_string(),
        format!("code: 3350001, msg: {rendered}, log_id: log-rendered")
    );
    assert_eq!(serde_json::to_value(&err).unwrap(), wire);
}

#[test]
fn code_error_rendered_message_fallbacks() {
    for error in [
        serde_json::Value::Null,
        serde_json::json!({}),
        serde_json::json!({"message": null}),
        serde_json::json!({"message": ""}),
        serde_json::json!({"message": 42}),
        serde_json::json!({"message": true}),
        serde_json::json!({"message": ["diagnostic"]}),
        serde_json::json!({"message": {"text": "diagnostic"}}),
    ] {
        let err: CodeError = serde_json::from_value(serde_json::json!({
            "code": 3350001, "msg": "invalid param", "error": error
        }))
        .unwrap();
        assert_eq!(err.effective_message(), "invalid param", "{error}");
        assert_eq!(err.to_string(), "code: 3350001, msg: invalid param");
        if let Some(info) = &err.error {
            assert_eq!(info.message.as_deref(), error["message"].as_str());
            let serialized = serde_json::to_value(info).unwrap();
            if error["message"] == "" {
                assert_eq!(serialized["message"], "");
            } else {
                assert!(serialized.get("message").is_none());
            }
        }
    }
    let err: CodeError = serde_json::from_str(r#"{"code":1,"msg":""}"#).unwrap();
    assert_eq!(err.effective_message(), "");
    assert_eq!(err.to_string(), "code: 1, msg: ");
}

#[test]
fn code_error_rendered_message_does_not_change_success_or_trim() {
    let err: CodeError =
        serde_json::from_str(r#"{"code":0,"msg":"ok","error":{"message":"  "}}"#).unwrap();
    assert!(err.success());
    assert_eq!(err.msg, "ok");
    assert_eq!(err.effective_message(), "  ");
    assert_eq!(err.to_string(), "code: 0, msg:   ");
}

#[test]
fn raw_response_deserialize_with_data() {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct TestData {
        name: String,
    }

    let json = r#"{"code": 0, "msg": "ok", "data": {"name": "hello"}}"#;
    let resp: RawResponse<TestData> = serde_json::from_str(json).unwrap();
    assert!(resp.code_error.success());
    assert_eq!(resp.data.as_ref().unwrap().name, "hello");
}

#[test]
fn raw_response_deserialize_without_data() {
    let json = r#"{"code": 10001, "msg": "not found"}"#;
    let resp: RawResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
    assert!(!resp.code_error.success());
    assert!(resp.data.is_none());
}
