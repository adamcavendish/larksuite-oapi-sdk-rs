use larksuite_oapi_sdk_rs::service::application::v7::{
    AppSlashCommandDescription, AppSlashCommandI18n, AppSlashCommandIcon,
    CreateAppSlashCommandReqBody,
};
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID")?;
    let app_secret = std::env::var("APP_SECRET")?;
    let client = LarkClient::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    let commands = client
        .application_v7()
        .app_slash_command
        .list(&option)
        .await?;
    for command in commands
        .data
        .as_ref()
        .map(|data| &data.items)
        .into_iter()
        .flatten()
    {
        println!(
            "{}: {}",
            command.command_id.as_deref().unwrap_or_default(),
            command.command.as_deref().unwrap_or_default()
        );
    }

    let Some(command) = std::env::var("SLASH_COMMAND").ok() else {
        return Ok(());
    };
    let description = std::env::var("SLASH_COMMAND_DESCRIPTION")
        .unwrap_or_else(|_| "Run a slash command".to_string());
    let body = CreateAppSlashCommandReqBody::new(
        command,
        AppSlashCommandDescription::new(description.clone())
            .i18n(AppSlashCommandI18n::new().insert("en_us", description))
            .icon(AppSlashCommandIcon::new("skill_outlined")),
    );
    let created = client
        .application_v7()
        .app_slash_command
        .create(&body, &option)
        .await?;
    println!(
        "created command_id={}",
        created
            .data
            .as_ref()
            .and_then(|data| data.command_id.as_deref())
            .unwrap_or_default()
    );

    Ok(())
}
