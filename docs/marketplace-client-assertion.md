# Marketplace ClientAssertion authentication

Marketplace applications that use the platform's `private_key_jwt` flow can
exchange a caller-signed assertion for a tenant access token without an
`AppSecret`. The SDK does not create or store private keys; implement
`ClientAssertionProvider` in the application and return the signed assertion.

## Configure the client

Pass an empty secret to the builder, select marketplace mode, and provide the
assertion provider:

```rust,no_run
use std::sync::Arc;
use larksuite_oapi_sdk_rs::{ClientAssertionProvider, LarkClient};

// The provider owns signing, key storage, rotation, and audience validation.
# #[derive(Debug)]
# struct Provider;
# impl ClientAssertionProvider for Provider {
#     fn retrieve_token(&self, _aud: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<larksuite_oapi_sdk_rs::Token, larksuite_oapi_sdk_rs::LarkError>> + Send + '_>> {
#         unimplemented!()
#     }
# }

let client = LarkClient::builder("APP_ID", "")
    .marketplace()
    .client_assertion_provider(Arc::new(Provider))
    .build()?;
# Ok::<(), larksuite_oapi_sdk_rs::LarkError>(())
```

The provider receives the OAuth host as its audience. Keep the assertion,
tenant token, and any signing material out of logs.

## Call a tenant-scoped API

Supply the target tenant on each request. The SDK exchanges
`client_credentials`, `client_id`, `client_assertion_type`, `client_assertion`,
and `tenant_key` at the OAuth token endpoint, then sends the returned Bearer
token to the resource API.

```rust,no_run
use larksuite_oapi_sdk_rs::{AccessTokenType, ApiReq, LarkClient, RequestOption};
use http::Method;

# async fn run(client: LarkClient) -> Result<(), larksuite_oapi_sdk_rs::LarkError> {
let mut request = ApiReq::new(Method::GET, "/open-apis/example");
request.supported_access_token_types = vec![AccessTokenType::Tenant];
let option = RequestOption {
    tenant_key: Some("TENANT_KEY".into()),
    ..Default::default()
};
client.raw_request(&request, &option).await?;
# Ok(())
# }
```

Tokens are cached by application, tenant, client-credentials mode, and the
effective OAuth endpoint. A fresh response must contain a Bearer token and a
positive `expires_in`; OAuth error fields are retained in the returned error.

With `disable_token_cache()`, automatic marketplace assertion exchange is
disabled for resource requests. Pass an explicit `tenant_access_token` in
`RequestOption` when making those calls. Direct `TokenManager` exchange remains
available for callers that need to manage the resulting token themselves.

App access token APIs remain unavailable in ClientAssertion mode. Legacy
marketplace app-ticket and app-access-token flows continue to use their
existing configuration.
