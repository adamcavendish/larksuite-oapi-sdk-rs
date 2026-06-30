use crate::config::Config;
use crate::service::go_v397::GoV397;

pub struct V1<'a> {
    config: &'a Config,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn go_v397(&self) -> GoV397<'a> {
        GoV397::new(self.config)
    }
}
