use larksuite_oapi_sdk_rs::{JsonValue, LarkError, ReqBody};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
struct DynamicPayload<'a> {
    title: &'a str,
    enabled: bool,
}

struct FailingSerialize;

impl Serialize for FailingSerialize {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(serde::ser::Error::custom(
            "intentional serialization failure",
        ))
    }
}

#[test]
fn json_value_preserves_dynamic_json_round_trip() {
    let value = JsonValue::from_serializable(DynamicPayload {
        title: "example",
        enabled: true,
    })
    .unwrap();

    assert_eq!(value["title"], "example");
    assert_eq!(value["enabled"], true);
    assert_eq!(
        serde_json::to_value(&value).unwrap(),
        serde_json::json!({"title":"example","enabled":true})
    );

    let decoded: JsonValue = serde_json::from_str(r#"{"title":"example","enabled":true}"#).unwrap();
    assert_eq!(decoded, value);
}

#[test]
fn request_body_json_uses_sdk_dynamic_value() {
    let body = ReqBody::json(&DynamicPayload {
        title: "request",
        enabled: false,
    })
    .unwrap();

    let ReqBody::Json(value) = body else {
        panic!("expected JSON request body");
    };
    assert_eq!(value["title"], "request");
    assert_eq!(value["enabled"], false);
}

#[test]
fn json_value_and_request_body_surface_sdk_json_errors() {
    let value_err = JsonValue::from_serializable(FailingSerialize).unwrap_err();
    let body_err = ReqBody::json(&FailingSerialize).unwrap_err();

    assert!(matches!(value_err, LarkError::Json(_)));
    assert!(matches!(body_err, LarkError::Json(_)));
}
