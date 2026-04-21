#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("illegal parameter: {0}")]
    IllegalParam(String),

    #[error("client timeout: {0}")]
    ClientTimeout(String),

    #[error("server timeout: {0}")]
    ServerTimeout(String),

    #[error("dial failed: {0}")]
    DialFailed(String),

    #[error("api error: {0}")]
    Api(Box<crate::resp::CodeError>),

    #[error("http error: {0}")]
    Http(#[from] aioduct::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("token error: {0}")]
    Token(String),

    #[error("max retries exceeded")]
    MaxRetries,

    #[error("event error: {0}")]
    Event(String),

    #[error("crypto error: {0}")]
    Crypto(String),
}

impl From<crate::resp::CodeError> for Error {
    fn from(e: crate::resp::CodeError) -> Self {
        Self::Api(Box::new(e))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
