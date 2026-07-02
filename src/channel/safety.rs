use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use http::{Method, StatusCode};
use url::Url;

use crate::LarkError;

const SOURCE_URL_TIMEOUT: Duration = Duration::from_secs(15);
const MAX_SOURCE_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UrlSafety {
    PublicOnly,
    #[cfg(test)]
    AllowPrivateForTests,
}

pub(super) async fn fetch_source_url(raw_url: &str) -> Result<Vec<u8>, LarkError> {
    fetch_source_url_with_safety(raw_url, UrlSafety::PublicOnly).await
}

pub(super) async fn fetch_source_url_with_safety(
    raw_url: &str,
    safety: UrlSafety,
) -> Result<Vec<u8>, LarkError> {
    let url = parse_source_url(raw_url)?;
    let client = aioduct::TokioClient::builder()
        .max_redirects(0)
        .timeout(SOURCE_URL_TIMEOUT)
        .build()
        .map_err(|e| LarkError::IllegalParam(format!("create source url client failed: {e}")))?;
    let force_addr = resolve_source_addr(&client, &url, safety).await?;
    let mut request = client
        .request(Method::GET, url.as_str())
        .map_err(|e| LarkError::IllegalParam(format!("invalid source_url: {e}")))?;
    if let Some(addr) = force_addr {
        request = request.force_addr(addr);
    }
    let resp = request
        .send()
        .await
        .map_err(|e| LarkError::Http(e.into_error()))?;

    if resp.status() != StatusCode::OK {
        return Err(LarkError::IllegalParam(format!(
            "fetch source_url failed: status code {}",
            resp.status().as_u16()
        )));
    }

    if let Some(len) = resp.content_length()
        && len > MAX_SOURCE_BYTES
    {
        return Err(LarkError::IllegalParam(format!(
            "source_url body exceeds {} bytes",
            MAX_SOURCE_BYTES
        )));
    }

    read_source_bytes_limited(resp).await
}

#[cfg(test)]
async fn validate_source_url(raw_url: &str, safety: UrlSafety) -> Result<Url, LarkError> {
    let url = parse_source_url(raw_url)?;
    let client = aioduct::TokioClient::builder()
        .max_redirects(0)
        .timeout(SOURCE_URL_TIMEOUT)
        .build()
        .map_err(|e| LarkError::IllegalParam(format!("create source url client failed: {e}")))?;
    resolve_source_addr(&client, &url, safety).await?;
    Ok(url)
}

fn parse_source_url(raw_url: &str) -> Result<Url, LarkError> {
    let url = Url::parse(raw_url)
        .map_err(|e| LarkError::IllegalParam(format!("invalid source_url: {e}")))?;
    match url.scheme() {
        "http" | "https" => {}
        scheme => {
            return Err(LarkError::IllegalParam(format!(
                "source_url scheme {scheme} is not supported"
            )));
        }
    }

    Ok(url)
}

async fn resolve_source_addr(
    client: &aioduct::TokioClient,
    url: &Url,
    safety: UrlSafety,
) -> Result<Option<SocketAddr>, LarkError> {
    match safety {
        UrlSafety::PublicOnly => {}
        #[cfg(test)]
        UrlSafety::AllowPrivateForTests => return Ok(None),
    }

    let host = url
        .host_str()
        .ok_or_else(|| LarkError::IllegalParam("source_url host is required".into()))?;
    let port = url
        .port_or_known_default()
        .ok_or_else(|| LarkError::IllegalParam("source_url port could not be determined".into()))?;
    let addrs = client
        .resolve_all(host, port)
        .await
        .map_err(|e| LarkError::IllegalParam(format!("source_url dns lookup failed: {e}")))?;
    let mut chosen = None;
    for addr in addrs {
        reject_blocked_ip(addr.ip())?;
        chosen.get_or_insert(addr);
    }
    chosen.map(Some).ok_or_else(|| {
        LarkError::IllegalParam("source_url dns lookup returned no addresses".into())
    })
}

async fn read_source_bytes_limited(resp: aioduct::Response) -> Result<Vec<u8>, LarkError> {
    let mut stream = resp.into_bytes_stream();
    let mut data = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(LarkError::Http)?;
        let next_len = data.len() as u64 + chunk.len() as u64;
        if next_len > MAX_SOURCE_BYTES {
            return Err(LarkError::IllegalParam(format!(
                "source_url body exceeds {} bytes",
                MAX_SOURCE_BYTES
            )));
        }
        data.extend_from_slice(&chunk);
    }
    Ok(data)
}

fn reject_blocked_ip(ip: IpAddr) -> Result<(), LarkError> {
    if is_blocked_ip(ip) {
        return Err(LarkError::IllegalParam(format!(
            "source_url resolves to blocked address {ip}"
        )));
    }
    Ok(())
}

fn is_blocked_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            ip.is_unspecified()
                || ip.is_loopback()
                || ip.is_private()
                || ip.is_link_local()
                || ip.is_broadcast()
                || ip.is_documentation()
                || ip.is_multicast()
                || ip.octets()[0] == 100 && (ip.octets()[1] & 0b1100_0000) == 0b0100_0000
                || ip.octets()[0] == 192 && ip.octets()[1] == 0 && ip.octets()[2] == 0
                || ip.octets()[0] == 198 && matches!(ip.octets()[1], 18 | 19)
                || ip.octets()[0] >= 240
        }
        IpAddr::V6(ip) => {
            if let Some(mapped) = ip.to_ipv4_mapped() {
                return is_blocked_ip(IpAddr::V4(mapped));
            }
            ip.is_unspecified()
                || ip.is_loopback()
                || ip.is_unique_local()
                || ip.is_unicast_link_local()
                || ip.is_multicast()
        }
    }
}

pub(super) fn file_name_from_url(raw_url: &str) -> Option<String> {
    let url = Url::parse(raw_url).ok()?;
    let segment = url.path_segments()?.next_back()?;
    (!segment.is_empty()).then(|| segment.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rejects_loopback_source_url() {
        let err = validate_source_url("http://127.0.0.1/file.bin", UrlSafety::PublicOnly)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("blocked address"));
    }

    #[tokio::test]
    async fn rejects_private_dns_source_url() {
        let err = validate_source_url("http://localhost/file.bin", UrlSafety::PublicOnly)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("blocked address"));
    }

    #[tokio::test]
    async fn rejects_ipv4_mapped_loopback_source_url() {
        let err = validate_source_url("http://[::ffff:127.0.0.1]/file.bin", UrlSafety::PublicOnly)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("blocked address"));
    }

    #[test]
    fn extracts_file_name_from_url_path() {
        assert_eq!(
            file_name_from_url("https://example.com/path/video.mp4?x=1").as_deref(),
            Some("video.mp4")
        );
        assert_eq!(file_name_from_url("https://example.com/"), None);
    }

    #[tokio::test]
    async fn fetches_source_url_when_private_allowed_for_tests() {
        let (addr, _handle) =
            source_server("HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\nhello")
                .await;

        let bytes = fetch_source_url_with_safety(
            &format!("http://{addr}/voice.opus"),
            UrlSafety::AllowPrivateForTests,
        )
        .await
        .unwrap();

        assert_eq!(bytes, b"hello");
    }

    #[tokio::test]
    async fn rejects_oversized_source_url_by_content_length() {
        let (addr, _handle) = source_server(
            "HTTP/1.1 200 OK\r\nContent-Length: 67108865\r\nConnection: close\r\n\r\n",
        )
        .await;

        let err = fetch_source_url_with_safety(
            &format!("http://{addr}/huge.bin"),
            UrlSafety::AllowPrivateForTests,
        )
        .await
        .unwrap_err();

        assert!(err.to_string().contains("exceeds"));
    }

    #[tokio::test]
    async fn rejects_non_ok_source_url_status() {
        let (addr, _handle) =
            source_server("HTTP/1.1 204 No Content\r\nConnection: close\r\n\r\n").await;

        let err = fetch_source_url_with_safety(
            &format!("http://{addr}/empty.bin"),
            UrlSafety::AllowPrivateForTests,
        )
        .await
        .unwrap_err();

        assert!(err.to_string().contains("status code 204"));
    }

    async fn source_server(
        response: &'static str,
    ) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };
            use tokio::io::{AsyncReadExt, AsyncWriteExt};
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf).await;
            let _ = stream.write_all(response.as_bytes()).await;
            let _ = stream.shutdown().await;
        });
        (addr, handle)
    }
}
