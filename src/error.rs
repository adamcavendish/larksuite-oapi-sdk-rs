/// Errors returned by SDK operations.
#[derive(Debug, thiserror::Error)]
pub enum LarkError {
    /// A required parameter is missing or invalid (caught before sending the request).
    #[error("illegal parameter: {0}")]
    IllegalParam(String),

    /// The HTTP client's configured timeout was exceeded.
    #[error("client timeout: {0}")]
    ClientTimeout(String),

    /// The server returned HTTP 504 Gateway Timeout.
    #[error("server timeout: {0}")]
    ServerTimeout(String),

    /// The server returned HTTP 429 Too Many Requests.
    #[error("rate limited: {0}")]
    RateLimited(String),

    /// TCP connection to the server failed.
    #[error("dial failed: {0}")]
    DialFailed(String),

    /// The Lark API returned a non-zero business error code.
    #[error("api error: {0}")]
    Api(Box<crate::resp::CodeError>),

    /// An HTTP-level error from the underlying HTTP client.
    #[error("http error: {0}")]
    Http(#[from] aioduct::Error),

    /// JSON serialization or deserialization failed.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Failed to obtain an access token (e.g. missing app ticket for marketplace apps).
    #[error("token error: {0}")]
    Token(String),

    /// The request was retried the maximum number of times without success.
    #[error("max retries exceeded")]
    MaxRetries,

    /// An error in the event dispatch pipeline.
    #[error("event error: {0}")]
    Event(String),

    /// AES decryption or signature verification failed.
    #[error("crypto error: {0}")]
    Crypto(String),

    /// App registration flow error.
    #[error("registration error: {0}")]
    Registration(String),
}

impl From<crate::resp::CodeError> for LarkError {
    fn from(e: crate::resp::CodeError) -> Self {
        Self::Api(Box::new(e))
    }
}
