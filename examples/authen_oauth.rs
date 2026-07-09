#![recursion_limit = "256"]

use larksuite_oapi_sdk_rs::{Client, RequestOption};

fn print_token_summary(label: &str, token: Option<&str>, expires_in: Option<i64>) {
    let token_len = token.map(str::len).unwrap_or(0);
    println!("{label}: received={}", token_len > 0);
    if token_len > 0 {
        println!("{label}_length: {token_len}");
    }
    if let Some(expires_in) = expires_in {
        println!("{label}_expires_in: {expires_in}");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let auth_code = std::env::var("AUTH_CODE").expect("AUTH_CODE env var required");
    let redirect_uri = std::env::var("REDIRECT_URI").ok();
    let refresh_token = std::env::var("REFRESH_TOKEN").ok();

    let client = Client::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    let token = client
        .authen()
        .oauth
        .retrieve_by_authorization_code(&auth_code, redirect_uri.as_deref(), None, None, &option)
        .await?;

    let token_data = token.data.as_ref();
    let access_token = token_data
        .and_then(|data| data.access_token.as_deref())
        .unwrap_or("");
    print_token_summary(
        "access_token",
        token_data.and_then(|data| data.access_token.as_deref()),
        token_data.and_then(|data| data.expires_in),
    );
    if let Some(scope) = token_data.and_then(|data| data.scope.as_deref()) {
        println!("scope: {scope}");
    }

    if let Some(refresh_token) = refresh_token {
        let refreshed = client
            .authen()
            .oauth
            .refresh(&refresh_token, None, &option)
            .await?;
        let refreshed_data = refreshed.data.as_ref();
        print_token_summary(
            "refreshed_access_token",
            refreshed_data.and_then(|data| data.access_token.as_deref()),
            refreshed_data.and_then(|data| data.expires_in),
        );
    }

    if !access_token.is_empty() {
        let mut user_option = option.clone();
        user_option.user_access_token = Some(access_token.to_string());
        let user = client.authen().user_info.get(&user_option).await?;
        if user.success() {
            println!(
                "user: {:?}",
                user.data.as_ref().and_then(|data| data.open_id.as_deref())
            );
        } else {
            println!("user_info error: {}", user.code_error);
        }
    }

    Ok(())
}
