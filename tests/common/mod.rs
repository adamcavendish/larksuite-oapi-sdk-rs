use std::sync::Arc;
use std::sync::Mutex;

/// Spawn a TCP listener that returns canned HTTP responses.
/// Each accepted connection reads one request then writes the response from
/// `responses` in order. If responses run out, the last one is reused.
#[allow(dead_code)]
pub async fn mock_server(
    responses: Vec<String>,
) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let responses = Arc::new(responses);
    let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    let handle = tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let responses = Arc::clone(&responses);
            let counter = Arc::clone(&counter);
            tokio::spawn(async move {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};

                let mut buf = vec![0u8; 8192];
                let _ = stream.read(&mut buf).await;

                let idx = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let resp_idx = idx.min(responses.len() - 1);
                let _ = stream.write_all(responses[resp_idx].as_bytes()).await;
                let _ = stream.shutdown().await;
            });
        }
    });

    (addr, handle)
}

#[allow(dead_code)]
pub async fn mock_server_with_requests(
    responses: Vec<String>,
) -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<()>,
    Arc<Mutex<Vec<String>>>,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let responses = Arc::new(responses);
    let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let requests = Arc::new(Mutex::new(Vec::new()));

    let handle = tokio::spawn({
        let requests = Arc::clone(&requests);
        async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let responses = Arc::clone(&responses);
                let counter = Arc::clone(&counter);
                let requests = Arc::clone(&requests);
                tokio::spawn(async move {
                    use tokio::io::{AsyncReadExt, AsyncWriteExt};

                    let mut buf = vec![0u8; 8192];
                    let Ok(n) = stream.read(&mut buf).await else {
                        return;
                    };
                    let request = String::from_utf8_lossy(&buf[..n]).to_string();
                    requests.lock().unwrap().push(request);

                    let idx = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let resp_idx = idx.min(responses.len() - 1);
                    let _ = stream.write_all(responses[resp_idx].as_bytes()).await;
                    let _ = stream.shutdown().await;
                });
            }
        }
    });

    (addr, handle, requests)
}

pub fn http_response(status: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    )
}

#[allow(dead_code)]
pub fn http_response_with_headers(status: u16, headers: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {status} OK\r\n{headers}Content-Length: {}\r\n\r\n{body}",
        body.len()
    )
}
