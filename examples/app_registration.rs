#![recursion_limit = "256"]

use larksuite_oapi_sdk_rs::registration::{
    AppAddons, AppAddonsCallbacks, AppAddonsEventItems, AppAddonsEvents, AppAddonsScopes,
    AppPreset, Options, register_app,
};

fn redact_secret(secret: &str) -> String {
    let char_count = secret.chars().count();
    if char_count <= 8 {
        return format!("<redacted, {char_count} chars>");
    }

    let prefix: String = secret.chars().take(4).collect();
    let suffix: String = secret
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    format!("{prefix}...{suffix} ({char_count} chars)")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let avatar = std::env::var("APP_AVATAR_URL").ok().into_iter().collect();
    let create_only = std::env::var("CREATE_ONLY").is_ok_and(|value| {
        value == "1" || value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("yes")
    });

    let result = register_app(Options {
        source: "example".to_string(),
        domain: std::env::var("REGISTRATION_DOMAIN").unwrap_or_default(),
        lark_domain: std::env::var("REGISTRATION_LARK_DOMAIN").unwrap_or_default(),
        app_preset: Some(AppPreset {
            avatar,
            name: "Rust SDK {user}".to_string(),
            desc: "Created from larksuite-oapi-sdk-rs".to_string(),
        }),
        addons: Some(AppAddons {
            scopes: AppAddonsScopes {
                tenant: vec!["im:message:send_as_bot".to_string()],
                user: vec!["calendar:calendar:read".to_string()],
            },
            events: AppAddonsEvents {
                items: AppAddonsEventItems {
                    tenant: vec!["im.message.receive_v1".to_string()],
                    user: Vec::new(),
                },
            },
            callbacks: AppAddonsCallbacks {
                items: vec!["card.action.trigger".to_string()],
            },
        }),
        create_only,
        app_id: std::env::var("EXISTING_APP_ID").unwrap_or_default(),
        on_qr_code: Box::new(|info| {
            println!("open or scan this URL: {}", info.url);
            println!("expires in {} seconds", info.expire_in);
        }),
        on_status_change: Some(Box::new(|info| {
            println!(
                "registration status: {}, next poll after {} seconds",
                info.status, info.interval
            );
        })),
    })
    .await?;

    println!("App ID: {}", result.client_id);
    println!("App Secret: {}", redact_secret(&result.client_secret));
    println!("Store the returned client_secret securely before using the new app.");
    if let Some(user_info) = result.user_info {
        println!("registered by open_id: {}", user_info.open_id);
        println!("tenant brand: {}", user_info.tenant_brand);
    }

    Ok(())
}
