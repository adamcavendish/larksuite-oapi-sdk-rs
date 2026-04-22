use larksuite_oapi_sdk_rs::constants::{AccessTokenType, AppType};

#[test]
fn access_token_type_display() {
    assert_eq!(format!("{}", AccessTokenType::None), "none_access_token");
    assert_eq!(format!("{}", AccessTokenType::App), "app_access_token");
    assert_eq!(
        format!("{}", AccessTokenType::Tenant),
        "tenant_access_token"
    );
    assert_eq!(format!("{}", AccessTokenType::User), "user_access_token");
}

#[test]
fn app_type_default_is_self_built() {
    assert_eq!(AppType::default(), AppType::SelfBuilt);
}

#[test]
fn app_type_serde_roundtrip() {
    let val = AppType::Marketplace;
    let json = serde_json::to_string(&val).unwrap();
    assert_eq!(json, "\"Marketplace\"");
    let deserialized: AppType = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, AppType::Marketplace);
}

#[test]
fn access_token_type_serde_roundtrip() {
    for variant in [
        AccessTokenType::None,
        AccessTokenType::App,
        AccessTokenType::Tenant,
        AccessTokenType::User,
    ] {
        let json = serde_json::to_string(&variant).unwrap();
        let deserialized: AccessTokenType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, variant);
    }
}

#[test]
fn access_token_type_debug() {
    let debug = format!("{:?}", AccessTokenType::Tenant);
    assert_eq!(debug, "Tenant");
}

#[test]
fn app_type_debug() {
    let debug = format!("{:?}", AppType::Marketplace);
    assert_eq!(debug, "Marketplace");
}

#[test]
fn app_type_display() {
    assert_eq!(format!("{}", AppType::SelfBuilt), "SelfBuilt");
    assert_eq!(format!("{}", AppType::Marketplace), "Marketplace");
}
