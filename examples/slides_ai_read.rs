use larksuite_oapi_sdk_rs::service::slides_ai::v1::GetXmlPresentationQuery;
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
    let presentation_id = std::env::var("SLIDES_AI_PRESENTATION_ID")?;

    let response = client
        .slides_ai()
        .presentation
        .get(&GetXmlPresentationQuery::new(&presentation_id), &option)
        .await?;

    println!("Slides AI presentation: {:#?}", response.data);
    Ok(())
}
