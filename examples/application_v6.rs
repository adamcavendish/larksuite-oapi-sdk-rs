use larksuite_oapi_sdk_rs::service::application::v6::{GetApplicationQuery, ListApplicationQuery};
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let application_app_id = std::env::var("APPLICATION_APP_ID").ok();

    let client = LarkClient::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    let list = client
        .application_v6()
        .application
        .list_by_query(
            &ListApplicationQuery::new()
                .page_size(20)
                .user_id_type("open_id")
                .lang("en_us"),
            &option,
        )
        .await?;
    if list.success() {
        let items = list.data.as_ref().map(|data| &data.app_list);
        println!("applications: {}", items.map(Vec::len).unwrap_or(0));
        println!(
            "first_app_id: {:?}",
            items
                .and_then(|items| items.first())
                .and_then(|app| app.app_id.as_deref())
        );
    } else if let Some(err) = list.code_error.as_ref() {
        println!("list applications error: {err}");
    }

    if let Some(application_app_id) = application_app_id {
        let app = client
            .application_v6()
            .application
            .get_by_query(
                &GetApplicationQuery::new(&application_app_id)
                    .lang("en_us")
                    .user_id_type("open_id"),
                &option,
            )
            .await?;
        if app.success() {
            println!(
                "application_app_id: {:?}",
                app.data
                    .as_ref()
                    .and_then(|data| data.app.as_ref())
                    .and_then(|app| app.app_id.as_deref())
            );
        } else if let Some(err) = app.code_error.as_ref() {
            println!("get application error: {err}");
        }
    }

    Ok(())
}
