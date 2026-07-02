use larksuite_oapi_sdk_rs::{Client, RequestOption};

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

    let access_token = token
        .data
        .as_ref()
        .and_then(|data| data.access_token.as_deref())
        .unwrap_or("");
    println!("access_token: {access_token}");

    if let Some(refresh_token) = refresh_token {
        let refreshed = client
            .authen()
            .oauth
            .refresh(&refresh_token, None, &option)
            .await?;
        let refreshed_token = refreshed
            .data
            .as_ref()
            .and_then(|data| data.access_token.as_deref())
            .unwrap_or("");
        println!("refreshed_access_token: {refreshed_token}");
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
