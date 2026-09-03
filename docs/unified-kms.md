# Unified KMS

## Unified KMS

`client.unified_kms()` exposes autonomous-key administration. Every operation
requires a tenant access token; user access tokens are not accepted by the
Open Platform contract.

The resource groups are `autonomous_key`, `autonomous_key_deletion_plan`,
`autonomous_key_recover`, and `key_import_material`. Obtain import material
before importing a key, and keep encrypted tokens and key ciphertext out of
logs.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};
use larksuite_oapi_sdk_rs::service::unified_kms::v1::CreateAutonomousKeyReqBody;

# async fn example(client: LarkClient) -> Result<(), Box<dyn std::error::Error>> {
let option = RequestOption::default();
let body = CreateAutonomousKeyReqBody::new()
    .encrypted_token("encrypted-token")
    .public_encrypted_key("encrypted-key-material")
    .algorithm_type("AES256")
    .feature_code("Feature_IM")
    .key_alias("rotation-2026-09");
client.unified_kms().autonomous_key.create(&body, &option).await?;
# Ok(())
# }
```

Deletion-plan and recovery bodies are `Serialize`-generic so callers can use
new policy fields without waiting for a crate release.
