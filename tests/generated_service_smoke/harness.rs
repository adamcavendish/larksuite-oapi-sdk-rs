use larksuite_oapi_sdk_rs::LarkClient;

pub(crate) use super::common::{
    http_response, http_response_with_headers, mock_server_with_gated_body,
    mock_server_with_gated_chunked_body, mock_server_with_requests,
};

pub(crate) fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}
