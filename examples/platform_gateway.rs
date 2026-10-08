//! Route SDK-owned API/OAuth requests through a trusted enterprise gateway.
use std::sync::Arc;

use larksuite_oapi_sdk_rs::{LarkClient, LarkError};

fn main() -> Result<(), LarkError> {
    let _client = LarkClient::builder("APP_ID", "APP_SECRET")
        .platform_url_resolver(Arc::new(|logical: &str| {
            let mut url = url::Url::parse(logical)
                .map_err(|_| LarkError::IllegalParam("invalid logical platform URL".into()))?;
            // Preserve escaped path/query. The gateway receives platform
            // credentials, so use a destination controlled by your organization.
            url.set_host(Some("lark-gateway.example.com"))
                .map_err(|_| LarkError::IllegalParam("invalid gateway host".into()))?;
            Ok(url.to_string())
        }))
        .build()?;
    Ok(())
}
