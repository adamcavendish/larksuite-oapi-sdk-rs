use larksuite_oapi_sdk_rs::{Client, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let image_path = std::env::var("IMAGE_PATH").ok();
    let file_path = std::env::var("FILE_PATH").ok();
    let image_key = std::env::var("IMAGE_KEY").ok();
    let file_key = std::env::var("FILE_KEY").ok();

    let client = Client::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    if let Some(image_path) = image_path {
        let data = std::fs::read(&image_path)?;
        let resp = client.im().image.create("message", data, &option).await?;
        if resp.success() {
            println!(
                "uploaded image_key: {:?}",
                resp.data
                    .as_ref()
                    .and_then(|data| data.image_key.as_deref())
            );
        } else {
            println!("image upload error: {}", resp.code_error);
        }
    }

    if let Some(file_path) = file_path {
        let data = std::fs::read(&file_path)?;
        let file_name = std::path::Path::new(&file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("upload.bin");
        let resp = client
            .im()
            .file
            .create("stream", file_name, None, data, &option)
            .await?;
        if resp.success() {
            println!(
                "uploaded file_key: {:?}",
                resp.data.as_ref().and_then(|data| data.file_key.as_deref())
            );
        } else {
            println!("file upload error: {}", resp.code_error);
        }
    }

    if let Some(image_key) = image_key {
        let resp = client.im().image.get(&image_key, &option).await?;
        println!(
            "downloaded image: file_name={:?}, bytes={}",
            resp.file_name,
            resp.data.len()
        );
    }

    if let Some(file_key) = file_key {
        let resp = client.im().file.get(&file_key, &option).await?;
        println!(
            "downloaded file: file_name={:?}, bytes={}",
            resp.file_name,
            resp.data.len()
        );
    }

    Ok(())
}
