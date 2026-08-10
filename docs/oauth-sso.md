# OAuth SSO for Feishu and Lark

The SDK provides the server-side OpenAPI calls needed after a user authorizes
your application. Your application owns the browser redirect, callback route,
CSRF state, PKCE verifier generation, local session, and durable token storage.

## Platform configuration

Feishu is the default client configuration. Use `LARK_BASE_URL` for an
international Lark application.

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, LARK_BASE_URL};

let feishu = LarkClient::builder("APP_ID", "APP_SECRET").build()?;
let lark = LarkClient::builder("APP_ID", "APP_SECRET")
    .base_url(LARK_BASE_URL)
    .build()?;
# Ok::<(), larksuite_oapi_sdk_rs::LarkError>(())
```

| Step | Feishu | Lark |
| --- | --- | --- |
| Browser authorization | `https://accounts.feishu.cn/open-apis/authen/v1/authorize` | `https://accounts.larksuite.com/open-apis/authen/v1/authorize` |
| Code exchange and refresh | `https://open.feishu.cn/open-apis/authen/v2/oauth/token` | `https://open.larksuite.com/open-apis/authen/v2/oauth/token` |
| User information | `https://open.feishu.cn/open-apis/authen/v1/user_info` | `https://open.larksuite.com/open-apis/authen/v1/user_info` |

Create the application in the matching open-platform region. A Feishu
application cannot authorize against Lark, and vice versa.

## Application flow

1. Register the callback URL in the application's security settings.
2. Generate and store a random `state`, plus a PKCE verifier and S256 challenge.
3. Redirect the browser to the platform authorization URL with `client_id`,
   `response_type=code`, `redirect_uri`, `state`, requested `scope`,
   `code_challenge`, and `code_challenge_method=S256`.
4. At the callback, handle `access_denied`, validate `state`, and exchange the
   one-time `code` promptly with the original verifier.
5. Map `open_id` or another suitable identity field to your local user and
   create your own application session.

The SDK deliberately does not create the authorization URL because state,
callback routing, and session policy are application-specific.

## Exchange code and retrieve identity

```rust,no_run
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

# async fn login(
#     client: &LarkClient,
#     code: &str,
#     redirect_uri: &str,
#     code_verifier: &str,
# ) -> Result<(), larksuite_oapi_sdk_rs::LarkError> {
let option = RequestOption::default();
let token = client
    .authen()
    .oauth
    .retrieve_by_authorization_code(
        code,
        Some(redirect_uri),
        Some(code_verifier),
        None,
        &option,
    )
    .await?;
let access_token = token
    .data
    .as_ref()
    .and_then(|data| data.access_token.as_deref())
    .ok_or_else(|| larksuite_oapi_sdk_rs::LarkError::Token("OAuth response omitted access_token".into()))?;

let mut user_option = RequestOption::default();
user_option.user_access_token = Some(access_token.to_owned());
let user = client.authen().user_info.get(&user_option).await?;
# let _ = user;
# Ok(())
# }
```

## Maintainer live Feishu verification

The ignored live test exchanges a freshly authorized code and immediately refreshes the
returned token through this SDK. It is intentionally excluded from CI and prints no
credentials or tokens.

Set `FEISHU_APP_ID`, `FEISHU_APP_SECRET`, `FEISHU_OAUTH_CODE`, and
`FEISHU_CODE_VERIFIER`. `FEISHU_REDIRECT_URI` defaults to
`http://127.0.0.1:8787/oauth/feishu/callback`. Run:

```bash
cargo test --test oauth_live feishu_user_oauth_live_code_exchange_and_refresh -- --ignored --exact
```

Request `offline_access` when constructing the authorization URL so the authorization
response includes a refresh token. Use a new authorization code for every invocation.

Scopes belong on the browser authorization request. The authorization-code
exchange accepts the original `redirect_uri` and PKCE verifier. The app secret
is required for these user OAuth calls; JWT client assertion remains a
tenant-token credential mechanism.

## Refresh-token rotation

Request the `offline_access` scope and enable token refresh in the application
security settings when required. Store tokens encrypted, use `expires_in` and
`refresh_token_expires_in` rather than hard-coded lifetimes, and replace the
stored refresh token after every successful refresh because the old token is
single-use.

```rust,no_run
# async fn refresh(client: &larksuite_oapi_sdk_rs::LarkClient, refresh_token: &str) -> Result<(), larksuite_oapi_sdk_rs::LarkError> {
use larksuite_oapi_sdk_rs::RequestOption;

let token = client
    .authen()
    .oauth
    .refresh(refresh_token, None, &RequestOption::default())
    .await?;
# let _ = token;
# Ok(())
# }
```
