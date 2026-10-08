//! Trusted routing of SDK-owned platform requests.

use std::sync::Arc;

use crate::LarkError;

/// Maps a complete logical platform URL to its network destination.
///
/// Resolvers are trusted client configuration: credentials are sent to the
/// returned destination. Keep paths and queries unless the gateway requires
/// a different mapping. External downloads, pre-signed URLs, and response
/// links are not resolved. OAuth assertion audiences remain logical.
///
/// Implementations must be fast, local, and must not include credentials or
/// full query strings in returned error messages.
pub trait PlatformUrlResolver: Send + Sync {
    fn resolve(&self, logical_url: &str) -> Result<String, LarkError>;
}

impl<F> PlatformUrlResolver for F
where
    F: Fn(&str) -> Result<String, LarkError> + Send + Sync,
{
    fn resolve(&self, logical_url: &str) -> Result<String, LarkError> {
        self(logical_url)
    }
}

pub(crate) fn resolve(
    resolver: Option<&Arc<dyn PlatformUrlResolver>>,
    logical_url: &str,
) -> Result<String, LarkError> {
    let Some(resolver) = resolver else {
        return Ok(logical_url.to_owned());
    };
    let destination = resolver.resolve(logical_url)?;
    let parsed = url::Url::parse(&destination)
        .map_err(|_| LarkError::IllegalParam("invalid platform resolver destination".into()))?;
    if !matches!(parsed.scheme(), "http" | "https")
        || parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.fragment().is_some()
    {
        return Err(LarkError::IllegalParam(
            "platform resolver destination must be an HTTP(S) URL without userinfo or fragment"
                .into(),
        ));
    }
    Ok(parsed.to_string())
}

pub(crate) fn diagnostic_url(raw: &str) -> String {
    let Ok(mut url) = url::Url::parse(raw) else {
        return "<invalid destination>".into();
    };
    let _ = url.set_username("");
    let _ = url.set_password(None);
    url.set_query(None);
    url.set_fragment(None);
    url.to_string()
}

// aioduct strips Authorization on cross-origin redirects. Reject downgrades
// and cross-origin body replay as well. DPoP requests use a separate client
// that does not follow redirects: their proofs bind the original URL/method.
pub(crate) fn redirect_policy() -> aioduct::RedirectPolicy {
    aioduct::RedirectPolicy::custom(|from, to, _, method| redirect_action(from, to, method))
}

fn redirect_action(
    from: &http::Uri,
    to: &http::Uri,
    method: &http::Method,
) -> aioduct::RedirectAction {
    let same_origin = from.scheme() == to.scheme()
        && from.host() == to.host()
        && from
            .port_u16()
            .unwrap_or(if from.scheme_str() == Some("https") {
                443
            } else {
                80
            })
            == to
                .port_u16()
                .unwrap_or(if to.scheme_str() == Some("https") {
                    443
                } else {
                    80
                });
    if (from.scheme_str() == Some("https") && to.scheme_str() != Some("https"))
        || (!same_origin && method != http::Method::GET && method != http::Method::HEAD)
    {
        aioduct::RedirectAction::Stop
    } else {
        aioduct::RedirectAction::Follow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redirect_decisions_use_effective_origins_and_reject_downgrades() {
        let from = "https://gateway.example/a".parse().unwrap();
        let same = "https://gateway.example:443/b".parse().unwrap();
        let cross = "https://other.example/b".parse().unwrap();
        let downgrade = "http://gateway.example/b".parse().unwrap();
        assert_eq!(
            redirect_action(&from, &same, &http::Method::POST),
            aioduct::RedirectAction::Follow
        );
        assert_eq!(
            redirect_action(&from, &cross, &http::Method::POST),
            aioduct::RedirectAction::Stop
        );
        assert_eq!(
            redirect_action(&from, &cross, &http::Method::GET),
            aioduct::RedirectAction::Follow
        );
        assert_eq!(
            redirect_action(&from, &downgrade, &http::Method::GET),
            aioduct::RedirectAction::Stop
        );
    }

    #[test]
    fn diagnostics_remove_credentials_query_and_fragment() {
        assert_eq!(
            diagnostic_url("https://user:password@example.com/path?secret=value#fragment"),
            "https://example.com/path"
        );
    }

    #[test]
    fn absent_resolver_preserves_exact_url() {
        let url = "https://example.com/a%2Fb?signature=exact%2fvalue";
        assert_eq!(resolve(None, url).unwrap(), url);
    }
}
