# DPoP proofs and token bindings

DPoP (Demonstrating Proof of Possession) is opt-in. The SDK can generate
P-256/ES256 proofs for app-registration polling, client-assertion OAuth token
requests, DPoP-bound user-token refresh, and resource requests.

## Configure a client

Choose `Preferred` when the client-assertion token endpoint may return either a
DPoP or Bearer token, or `Required` when each client-assertion token and bound
resource request must use the configured key:

```rust,no_run
use std::sync::Arc;
use larksuite_oapi_sdk_rs::{DPoPMode, LarkClient, MemoryDPoPKeyStore};

let key_store = Arc::new(MemoryDPoPKeyStore::new());
let client = LarkClient::builder("APP_ID", "APP_SECRET")
    .dpop_mode(DPoPMode::Preferred)
    .dpop_key_store(key_store)
    .dpop_key_id("production")
    .build()?;
# Ok::<(), larksuite_oapi_sdk_rs::LarkError>(())
```

`Preferred` falls back to a normal Bearer token when the token endpoint does
not return `token_type=DPoP`. `Required` rejects a missing or incompatible
binding. These modes apply to the client-assertion tenant-token flow; configure
a `ClientAssertionProvider` when using DPoP-bound OAuth issuance. Legacy app
and tenant token endpoints remain Bearer by design. When the endpoint returns
a DPoP token, the SDK carries its key binding through the token cache and
resource-request retries. A manually supplied token remains Bearer unless the
caller supplies a matching `DPoPBinding` in `RequestOption`.

The default key store is in memory. Applications that need persistence can
implement `DPoPKeyStore` and inject it with `dpop_key_store`. The SDK never
chooses a filesystem path, permissions, backup policy, or secret-management
system; [`dpop_file_key_store.rs`](../examples/dpop_file_key_store.rs) is an
application-owned example only.

## Rotation and clock correction

When a key store is configured, call `client.rotate_dpop_key()` to remove the
selected key. The next DPoP request generates a new key and obtains a new
binding. An explicitly supplied `DPoPKey` cannot be rotated through the client;
replace it in the builder instead.

Choose the maximum allowed offset when creating an `OffsetDPoPClock`.
Call `client.authen().oauth.sync_dpop_clock(&clock)` to calibrate it against
the Accounts heartbeat, or apply a trusted timestamp directly with
`adjust_to_server_time`. The heartbeat sends no credentials and has a
deadline of at most five seconds. Share the same clock with the client
through `dpop_clock` to apply the correction to resource proofs.

Use a separate clock for each trusted Accounts destination. The OAuth base
override and platform URL resolver determine which service supplies the time.
See [DPoP-bound user token refresh](oauth-sso.md#dpop-bound-user-token-refresh)
for the refresh method and its error handling.

## App registration

Pass an optional key to `registration::register_app_with_dpop` to attach DPoP
proofs to the registration polling requests:

```rust,no_run
use larksuite_oapi_sdk_rs::dpop::DPoPKey;
use larksuite_oapi_sdk_rs::registration::{register_app_with_dpop, Options};

# async fn example(options: Options) -> Result<(), larksuite_oapi_sdk_rs::LarkError> {
let result = register_app_with_dpop(options, Some(DPoPKey::generate())).await?;
let _ = result;
# Ok(())
# }
```

`register_app` remains the compatible no-proof path. With a trusted platform
URL resolver, registration resolves each platform request before any polling
proof is generated and rejects redirects so a proof or device-code form is not
replayed to another endpoint.

Registration and resource requests carrying a DPoP proof use the no-redirect
client when a trusted URL resolver is configured. Accounts user refresh and
heartbeat always reject redirects. Keep resolver destinations under
organizational control and treat key material as a credential.
