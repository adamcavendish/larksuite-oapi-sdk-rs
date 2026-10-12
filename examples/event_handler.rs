use std::sync::{Arc, RwLock};

use larksuite_oapi_sdk_rs::event::{
    EventDispatcher, EventReq, WebhookCredentials, WebhookCredentialsError,
};
use larksuite_oapi_sdk_rs::events::im::P2MessageReceiveV1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A configuration watcher can replace this complete pair atomically.
    let current = Arc::new(RwLock::new(WebhookCredentials {
        verification_token: "verification_token".into(),
        encrypt_key: String::new(),
    }));
    let read = current.clone();
    let dispatcher = EventDispatcher::new("", "")
        .with_webhook_credentials_provider(Arc::new(move || {
            read.read()
                .map(|snapshot| snapshot.clone())
                .map_err(|_| WebhookCredentialsError)
        }))
        .on_p2_im_message_receive_v1(|event: P2MessageReceiveV1| async move {
            println!(
                "received message: id={} content={}",
                event.message.message_id, event.message.content
            );
            Ok(())
        });

    // Simulate an incoming webhook payload
    let payload = serde_json::json!({
        "schema": "2.0",
        "header": {
            "token": "verification_token",
            "event_id": "test-event-id",
            "event_type": "im.message.receive_v1",
            "app_id": "cli_test",
            "tenant_key": "tenant_test",
            "create_time": "1234567890"
        },
        "event": {
            "sender": { "sender_type": "user", "tenant_key": "tenant_test" },
            "message": {
                "message_id": "om_abc123",
                "message_type": "text",
                "content": "{\"text\":\"hello\"}",
                "chat_id": "oc_test",
                "chat_type": "p2p",
                "create_time": "1234567890",
                "update_time": "1234567890",
                "root_id": "",
                "parent_id": ""
            }
        }
    });

    let req = EventReq {
        headers: std::collections::HashMap::new(),
        body: serde_json::to_vec(&payload)?,
        request_uri: "/webhook/event".to_string(),
    };

    let resp = dispatcher.handle(req).await;
    println!("response status: {}", resp.status_code);

    // Rotate both credentials together; in-flight requests retain their snapshot.
    *current.write().map_err(|_| WebhookCredentialsError)? = WebhookCredentials {
        verification_token: "next_verification_token".into(),
        encrypt_key: "next_encrypt_key".into(),
    };
    Ok(())
}
