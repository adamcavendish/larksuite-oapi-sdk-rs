use larksuite_oapi_sdk_rs::{
    AccessTokenType, ApiReq, HttpMethod, LarkClient, ReqBody, RequestOption,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let user_id = std::env::var("USER_ID").expect("USER_ID env var required");
    let user_access_token =
        std::env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN env var required");

    let client = LarkClient::builder(app_id, app_secret)
        .log_req_at_debug()
        .build()?;

    let mut option = RequestOption {
        user_access_token: Some(user_access_token),
        request_id: Some("raw-api-example".to_string()),
        ..Default::default()
    };

    let mut req = ApiReq::new(HttpMethod::GET, "/open-apis/contact/v3/users/:user_id");
    req.path_params.set("user_id", user_id);
    req.query_params.set("user_id_type", "open_id");

    let (api_resp, raw) = client
        .raw_request_typed_with_token::<serde_json::Value>(req, AccessTokenType::User, &option)
        .await?;

    println!("status: {}", api_resp.status_code);
    println!("request_id: {:?}", api_resp.request_id());
    println!(
        "data_keys: {:?}",
        raw.data
            .as_ref()
            .and_then(|data| data.as_object())
            .map(|object| object.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    );

    if let Ok(chat_id) = std::env::var("CHAT_ID") {
        option.user_access_token = None;

        let mut req = ApiReq::new(HttpMethod::POST, "/open-apis/im/v1/messages");
        req.query_params.set("receive_id_type", "chat_id");
        req.body = Some(ReqBody::Json(serde_json::json!({
            "receive_id": chat_id,
            "msg_type": "text",
            "content": "{\"text\":\"Hello from raw API\"}"
        })));

        let api_resp = client
            .raw_request_with_token(req, AccessTokenType::Tenant, &option)
            .await?;
        println!("message status: {}", api_resp.status_code);
        println!("message request_id: {:?}", api_resp.request_id());
    }

    Ok(())
}
