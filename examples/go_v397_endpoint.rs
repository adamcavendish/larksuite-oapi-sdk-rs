use larksuite_oapi_sdk_rs::service::go_v397::GoV397Endpoint;
use larksuite_oapi_sdk_rs::{Client, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let note_id = std::env::var("NOTE_ID").expect("NOTE_ID env var required");

    let client = Client::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    let resp = Box::pin(client.go_v397().request_json(
        GoV397Endpoint::GetVcV1NotesByNoteId,
        [("note_id", note_id.as_str())],
        [("user_id_type", "open_id")],
        None,
        &option,
    ))
    .await?;

    if resp.success() {
        println!("note_id: {note_id}");
        println!(
            "response_keys: {:?}",
            resp.data
                .as_ref()
                .and_then(|data| data.as_object())
                .map(|object| object.keys().cloned().collect::<Vec<_>>())
                .unwrap_or_default()
        );
    } else if let Some(err) = resp.code_error.as_ref() {
        println!("error: {err}");
    }

    Ok(())
}
