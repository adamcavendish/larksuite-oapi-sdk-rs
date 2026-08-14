use larksuite_oapi_sdk_rs::service::base::v3::{
    ListBaseAppBlocksQuery, ListBaseAppPagesQuery, ListWorkspaceEntitiesQuery,
};
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::{LarkClient, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder(std::env::var("APP_ID")?, std::env::var("APP_SECRET")?)
        .disable_token_cache()
        .build()?;
    let option = RequestOption {
        user_access_token: Some(std::env::var("USER_ACCESS_TOKEN")?),
        ..RequestOption::default()
    };
    let workspace_token = std::env::var("BASE_V3_WORKSPACE_TOKEN")?;
    let app_token = std::env::var("BASE_V3_APP_TOKEN")?;

    let entities = client
        .base_v3()
        .workspace
        .list_entities(
            &ListWorkspaceEntitiesQuery::new(&workspace_token)
                .entity_type("baseapp")
                .page(PageQuery::new().page_size(20)),
            &option,
        )
        .await?;
    println!("Workspace BaseApps: {:#?}", entities.data);

    let app = client.base_v3().app.get(&app_token, &option).await?;
    println!("BaseApp: {:#?}", app.data);

    let pages = client
        .base_v3()
        .page
        .list(
            &ListBaseAppPagesQuery::new(&app_token).page(PageQuery::new().page_size(20)),
            &option,
        )
        .await?;
    println!("Pages: {:#?}", pages.data);

    if let Some(page_id) = std::env::var("BASE_V3_PAGE_ID")
        .ok()
        .filter(|id| !id.is_empty())
    {
        let blocks = client
            .base_v3()
            .block
            .list(
                &ListBaseAppBlocksQuery::new(&app_token, &page_id)
                    .page(PageQuery::new().page_size(20)),
                &option,
            )
            .await?;
        println!("Blocks: {:#?}", blocks.data);
    }

    Ok(())
}
