use std::io::Write;
use std::path::{Path, PathBuf};

use larksuite_oapi_sdk_rs::service::im::v1::GetMessageResourceDownloadQuery;
use larksuite_oapi_sdk_rs::{Client, RequestOption};
use sha2::{Digest, Sha256};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let message_id = std::env::var("MESSAGE_ID").expect("MESSAGE_ID env var required");
    let file_key = std::env::var("FILE_KEY").expect("FILE_KEY env var required");
    let resource_type = std::env::var("RESOURCE_TYPE").unwrap_or_else(|_| "file".to_string());
    let max_bytes = parse_optional_u64("MAX_BYTES")?;

    let client = Client::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();
    let query =
        GetMessageResourceDownloadQuery::new(&message_id, &file_key, resource_type.as_str());

    let mut resp = client
        .im()
        .message_resource
        .get_stream_by_query(&query, &option)
        .await?;

    if let (Some(content_length), Some(max_bytes)) = (resp.content_length, max_bytes)
        && content_length > max_bytes
    {
        return Err(
            format!("content length {content_length} exceeds MAX_BYTES limit {max_bytes}").into(),
        );
    }

    let output_path = output_path(resp.file_name.as_deref());
    let mut output = std::fs::File::create(&output_path)?;
    let mut hasher = Sha256::new();
    let mut written = 0u64;

    while let Some(chunk) = resp.body.next_chunk().await? {
        written += chunk.len() as u64;
        if let Some(max_bytes) = max_bytes
            && written > max_bytes
        {
            return Err(format!("download exceeded MAX_BYTES limit {max_bytes}").into());
        }

        hasher.update(&chunk);
        output.write_all(&chunk)?;
    }

    output.flush()?;
    let sha256 = hex::encode(hasher.finalize());

    println!("status: {}", resp.api_resp.status_code);
    println!("file_name: {:?}", resp.file_name);
    println!("content_length: {:?}", resp.content_length);
    println!("bytes_written: {written}");
    println!("sha256: {sha256}");
    println!("output_path: {}", output_path.display());

    Ok(())
}

fn parse_optional_u64(name: &str) -> Result<Option<u64>, Box<dyn std::error::Error>> {
    match std::env::var(name) {
        Ok(value) if !value.is_empty() => Ok(Some(value.parse()?)),
        _ => Ok(None),
    }
}

fn output_path(file_name: Option<&str>) -> PathBuf {
    if let Some(path) = std::env::var_os("OUTPUT_PATH") {
        return PathBuf::from(path);
    }

    let file_name = file_name
        .and_then(|name| Path::new(name).file_name())
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("lark-message-resource.bin");

    PathBuf::from(file_name)
}
