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

#[allow(dead_code)]
pub async fn mock_server_with_gated_body(
    status: u16,
    headers: &str,
    first_chunk: &'static str,
    rest_chunks: Vec<&'static str>,
) -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<()>,
    Arc<Mutex<Vec<String>>>,
    tokio::sync::oneshot::Sender<()>,
) {
    mock_server_with_gated_body_framing(
        status,
        headers,
        first_chunk,
        rest_chunks,
        GatedBodyFraming::ContentLength,
    )
    .await
}

#[allow(dead_code)]
pub async fn mock_server_with_gated_chunked_body(
    status: u16,
    headers: &str,
    first_chunk: &'static str,
    rest_chunks: Vec<&'static str>,
) -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<()>,
    Arc<Mutex<Vec<String>>>,
    tokio::sync::oneshot::Sender<()>,
) {
    mock_server_with_gated_body_framing(
        status,
        headers,
        first_chunk,
        rest_chunks,
        GatedBodyFraming::Chunked,
    )
    .await
}

#[derive(Clone, Copy)]
enum GatedBodyFraming {
    ContentLength,
    Chunked,
}

impl GatedBodyFraming {
    fn response_head(self, status: u16, headers: &str, content_length: usize) -> String {
        match self {
            Self::ContentLength => {
                format!("HTTP/1.1 {status} OK\r\n{headers}Content-Length: {content_length}\r\n\r\n")
            }
            Self::Chunked => {
                format!("HTTP/1.1 {status} OK\r\n{headers}Transfer-Encoding: chunked\r\n\r\n")
            }
        }
    }

    async fn write_chunk(
        self,
        stream: &mut tokio::net::TcpStream,
        chunk: &[u8],
    ) -> std::io::Result<()> {
        match self {
            Self::ContentLength => {
                use tokio::io::AsyncWriteExt;
                stream.write_all(chunk).await
            }
            Self::Chunked => write_chunk(stream, chunk).await,
        }
    }

    async fn finish(self, stream: &mut tokio::net::TcpStream) -> std::io::Result<()> {
        if matches!(self, Self::Chunked) {
            use tokio::io::AsyncWriteExt;
            stream.write_all(b"0\r\n\r\n").await?;
        }
        Ok(())
    }
}

async fn mock_server_with_gated_body_framing(
    status: u16,
    headers: &str,
    first_chunk: &'static str,
    rest_chunks: Vec<&'static str>,
    framing: GatedBodyFraming,
) -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<()>,
    Arc<Mutex<Vec<String>>>,
    tokio::sync::oneshot::Sender<()>,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let first_chunk = first_chunk.as_bytes().to_vec();
    let rest_chunks = rest_chunks
        .into_iter()
        .map(|chunk| chunk.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let content_length = first_chunk.len() + rest_chunks.iter().map(Vec::len).sum::<usize>();
    let headers = headers.to_string();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let (release, wait_for_release) = tokio::sync::oneshot::channel();

    let handle = tokio::spawn({
        let requests = Arc::clone(&requests);
        async move {
            use tokio::io::{AsyncReadExt, AsyncWriteExt};

            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };

            let mut buf = vec![0u8; 8192];
            let Ok(n) = stream.read(&mut buf).await else {
                return;
            };
            let request = String::from_utf8_lossy(&buf[..n]).to_string();
            requests.lock().unwrap().push(request);

            let head = framing.response_head(status, &headers, content_length);
            let _ = stream.write_all(head.as_bytes()).await;
            let _ = framing.write_chunk(&mut stream, &first_chunk).await;
            let _ = stream.flush().await;

            let _ = wait_for_release.await;

            for chunk in rest_chunks.iter() {
                let _ = framing.write_chunk(&mut stream, chunk).await;
            }
            let _ = framing.finish(&mut stream).await;
            let _ = stream.shutdown().await;
        }
    });

    (addr, handle, requests, release)
}

async fn write_chunk(stream: &mut tokio::net::TcpStream, chunk: &[u8]) -> std::io::Result<()> {
    use tokio::io::AsyncWriteExt;

    let size = format!("{:X}\r\n", chunk.len());
    stream.write_all(size.as_bytes()).await?;
    stream.write_all(chunk).await?;
    stream.write_all(b"\r\n").await
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
