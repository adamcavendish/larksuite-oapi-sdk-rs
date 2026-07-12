mod builder;
mod raw;
mod services;
mod tokens;

pub use builder::LarkClientBuilder;

use crate::config::Config;

/// Lark/Feishu API client. All service accessors borrow `&self` and are zero-cost wrappers.
#[derive(Debug, Clone)]
pub struct LarkClient {
    config: Config,
}

impl LarkClient {
    pub fn builder(app_id: impl Into<String>, app_secret: impl Into<String>) -> LarkClientBuilder {
        LarkClientBuilder::new(app_id, app_secret)
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
