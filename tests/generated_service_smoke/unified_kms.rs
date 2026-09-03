use super::prelude::*;
use larksuite_oapi_sdk_rs::service::unified_kms::v1::{
    CreateAutonomousKeyReqBody, ListAutonomousKeyQuery,
};

#[tokio::test]
async fn unified_kms_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, body); 8]).await;
    let client = client_for(addr);
    let option = RequestOption {
        tenant_access_token: Some("tenant-token".to_owned()),
        ..RequestOption::default()
    };

    client
        .unified_kms()
        .autonomous_key
        .create(
            &CreateAutonomousKeyReqBody::new()
                .encrypted_token("token")
                .public_encrypted_key("cipher")
                .algorithm_type("AES256")
                .feature_code("Feature_IM")
                .key_alias("key-1"),
            &option,
        )
        .await
        .unwrap();
    client
        .unified_kms()
        .autonomous_key
        .get("key id", &option)
        .await
        .unwrap();
    client
        .unified_kms()
        .autonomous_key
        .list(
            &ListAutonomousKeyQuery::new()
                .page(PageQuery::new().page_size(20).page_token("next"))
                .feature_code("Feature_IM")
                .key_alias("rotation")
                .top_class("enterprise")
                .operator_id("ou_operator"),
            &option,
        )
        .await
        .unwrap();
    client
        .unified_kms()
        .autonomous_key
        .delete("key id", Some("Feature_IM"), &option)
        .await
        .unwrap();
    client
        .unified_kms()
        .autonomous_key_deletion_plan
        .create("key id", json_value!({"delete_time":"1700000000"}), &option)
        .await
        .unwrap();
    client
        .unified_kms()
        .autonomous_key_deletion_plan
        .delete("key id", &option)
        .await
        .unwrap();
    client
        .unified_kms()
        .autonomous_key_recover
        .create("key id", json_value!({}), &option)
        .await
        .unwrap();
    client
        .unified_kms()
        .key_import_material
        .get(&option)
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    for needle in [
        "POST /open-apis/unified_kms/v1/autonomous_keys ",
        "GET /open-apis/unified_kms/v1/autonomous_keys/key%20id ",
        "GET /open-apis/unified_kms/v1/autonomous_keys?",
        "DELETE /open-apis/unified_kms/v1/autonomous_keys/key%20id?feature_code=Feature_IM ",
        "POST /open-apis/unified_kms/v1/autonomous_keys/key%20id/deletion_plan ",
        "DELETE /open-apis/unified_kms/v1/autonomous_keys/key%20id/deletion_plan ",
        "POST /open-apis/unified_kms/v1/autonomous_keys/key%20id/recover ",
        "GET /open-apis/unified_kms/v1/key_import_material ",
        "authorization: Bearer tenant-token",
        "key_alias=rotation",
        "top_class=enterprise",
        "operator_id=ou_operator",
        r#""public_encrypted_key":"cipher""#,
    ] {
        assert!(request.contains(needle), "missing {needle}:\n{request}");
    }
}
