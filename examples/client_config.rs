use std::time::Duration;

use http::{HeaderMap, HeaderValue};
use larksuite_oapi_sdk_rs::{Client, LARK_BASE_URL};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt().try_init();

    let app_id = std::env::var("APP_ID").unwrap_or_else(|_| "cli_xxx".to_string());
    let app_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| "app_secret".to_string());

    let default_client = Client::builder(&app_id, &app_secret).build()?;
    println!("default base URL: {}", default_client.config().base_url());

    let mut headers = HeaderMap::new();
    headers.insert(
        "x-example-source",
        HeaderValue::from_static("larksuite-oapi-sdk-rs"),
    );

    let tuned_client = Client::builder(app_id, app_secret)
        .base_url(LARK_BASE_URL)
        .oauth_base_url("https://accounts.larksuite.com")
        .timeout(Duration::from_secs(3))
        .disable_token_cache()
        .helpdesk_credential("helpdesk_id", "helpdesk_token")
        .default_headers(headers)
        .max_retries(3)
        .log_level(tracing::Level::DEBUG)
        .log_req_at_debug()
        .build()?;

    let config = tuned_client.config();
    println!("configured base URL: {}", config.base_url());
    println!("configured timeout: {:?}", config.req_timeout());
    println!("configured max retries: {}", config.max_retries());
    println!("token cache enabled: {}", config.enable_token_cache());
    println!("default header count: {}", config.default_headers().len());

    Ok(())
}
