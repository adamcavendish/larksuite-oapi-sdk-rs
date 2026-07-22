use larksuite_oapi_sdk_rs::LarkError;
use larksuite_oapi_sdk_rs::service::hire::v1::ReplayableRequestBody;
use serde::{Serialize, Serializer};

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
fn replayable_request_body_surfaces_sdk_json_errors() {
    let err = ReplayableRequestBody::from_serializable(FailingSerialize).unwrap_err();

    assert!(matches!(err, LarkError::Json(_)));
}
