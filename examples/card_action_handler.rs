#![recursion_limit = "256"]

use std::collections::HashMap;

use larksuite_oapi_sdk_rs::{CardAction, CardActionHandler, EventReq};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler =
        CardActionHandler::new("verification_token", "", |action: CardAction| async move {
            println!("card clicked by open_id={}", action.open_id);
            println!("action value: {:?}", action.action.value);

            Ok(serde_json::json!({
                "toast": {
                    "type": "info",
                    "content": "Handled by Rust"
                }
            }))
        })
        .skip_sign_verify();

    let payload = serde_json::json!({
        "open_id": "ou_xxx",
        "open_message_id": "om_xxx",
        "open_chat_id": "oc_xxx",
        "tenant_key": "tenant_xxx",
        "token": "verification_token",
        "action": {
            "tag": "button",
            "value": {
                "action": "acknowledge",
                "ticket_id": "INC-1001"
            }
        }
    });

    let req = EventReq {
        headers: HashMap::new(),
        body: serde_json::to_vec(&payload)?,
        request_uri: "/webhook/card".to_string(),
    };

    let resp = handler.handle(req).await;
    println!("response status: {}", resp.status_code);
    println!("response body: {}", String::from_utf8_lossy(&resp.body));
    Ok(())
}
