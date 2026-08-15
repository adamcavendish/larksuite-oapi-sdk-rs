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
    let content = std::env::var("SLIDES_AI_RENDER_CONTENT")
        .unwrap_or_else(|_| r#"<slide id="preview"></slide>"#.to_string());

    let response = client.slides_ai().image.render(&content, &option).await?;
    let image = response
        .data
        .and_then(|data| data.slide_image)
        .ok_or_else(|| std::io::Error::other("Slides AI returned no image"))?;
    let bytes = image.decode()?;

    println!(
        "Rendered {} bytes for slide {:?} as {:?}",
        bytes.len(),
        image.slide_id,
        image.format,
    );
    Ok(())
}
