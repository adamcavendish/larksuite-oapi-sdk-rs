use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::events::common::UserId;
use crate::resp::CodeError;
use crate::{Client, LarkError, RequestOption};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BotIdentityCacheConfig {
    pub ttl: Duration,
    pub min_refresh_interval: Duration,
}

impl Default for BotIdentityCacheConfig {
    fn default() -> Self {
        Self {
            ttl: Duration::from_secs(30 * 60),
            min_refresh_interval: Duration::from_secs(60),
        }
    }
}

impl BotIdentityCacheConfig {
    pub(super) fn normalized(mut self) -> Self {
        let defaults = Self::default();
        if self.ttl.is_zero() {
            self.ttl = defaults.ttl;
        }
        if self.min_refresh_interval.is_zero() {
            self.min_refresh_interval = defaults.min_refresh_interval;
        }
        self.min_refresh_interval = self.min_refresh_interval.max(Duration::from_secs(30));
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotIdentity {
    pub app_id: Option<String>,
    pub open_id: Option<String>,
    pub user_id: Option<String>,
    pub union_id: Option<String>,
    pub name: Option<String>,
    pub activate_status: Option<i64>,
}

#[derive(Debug)]
pub(super) struct BotIdentityCache {
    pub(super) identity: Option<BotIdentity>,
    fetched_at: Option<Instant>,
    pub(super) last_failure_at: Option<Instant>,
    config: BotIdentityCacheConfig,
}

impl BotIdentityCache {
    pub(super) fn new(config: BotIdentityCacheConfig) -> Self {
        Self {
            identity: None,
            fetched_at: None,
            last_failure_at: None,
            config: config.normalized(),
        }
    }

    pub(super) fn remember(&mut self, identity: BotIdentity) {
        self.identity = Some(identity);
        self.fetched_at = Some(Instant::now());
        self.last_failure_at = None;
    }

    pub(super) fn fresh(&self) -> Option<BotIdentity> {
        let identity = self.identity.as_ref()?;
        let fetched_at = self.fetched_at?;
        (fetched_at.elapsed() < self.config.ttl).then(|| identity.clone())
    }

    pub(super) fn throttled_stale(&self) -> Option<BotIdentity> {
        let identity = self.identity.as_ref()?;
        let last_failure_at = self.last_failure_at?;
        (last_failure_at.elapsed() < self.config.min_refresh_interval).then(|| identity.clone())
    }

    pub(super) fn matches_user(&self, user: Option<&UserId>) -> bool {
        let Some(user) = user else {
            return false;
        };
        self.identity.as_ref().is_some_and(|identity| {
            identity
                .open_id
                .as_deref()
                .is_some_and(|id| user.open_id() == Some(id))
                || identity
                    .user_id
                    .as_deref()
                    .is_some_and(|id| user.user_id() == Some(id))
                || identity
                    .union_id
                    .as_deref()
                    .is_some_and(|id| user.union_id() == Some(id))
        })
    }
}

#[derive(Debug, Deserialize)]
struct BotInfoResp {
    #[serde(default)]
    code: i64,
    #[serde(default)]
    msg: String,
    #[serde(default)]
    bot: Option<BotInfo>,
}

#[derive(Debug, Deserialize)]
struct BotInfo {
    #[serde(default)]
    app_id: Option<String>,
    #[serde(default)]
    open_id: Option<String>,
    #[serde(default)]
    user_id: Option<String>,
    #[serde(default)]
    union_id: Option<String>,
    #[serde(default)]
    app_name: Option<String>,
    #[serde(default)]
    activate_status: Option<i64>,
}

pub(super) async fn fetch_bot_identity(
    client: &Client,
    option: &RequestOption,
) -> Result<BotIdentity, LarkError> {
    let resp = client.get("/open-apis/bot/v3/info", option).await?;
    if resp.status_code != 200 {
        return Err(LarkError::Api(Box::new(CodeError {
            code: resp.status_code as i64,
            msg: "bot/v3/info returned non-200 status".into(),
            ..Default::default()
        })));
    }
    let parsed: BotInfoResp = serde_json::from_slice(&resp.raw_body)?;
    if parsed.code != 0 {
        return Err(LarkError::Api(Box::new(CodeError {
            code: parsed.code,
            msg: parsed.msg,
            ..Default::default()
        })));
    }
    let bot = parsed.bot.unwrap_or(BotInfo {
        app_id: None,
        open_id: None,
        user_id: None,
        union_id: None,
        app_name: None,
        activate_status: None,
    });
    Ok(BotIdentity {
        app_id: bot.app_id,
        open_id: bot.open_id,
        user_id: bot.user_id,
        union_id: bot.union_id,
        name: bot.app_name,
        activate_status: bot.activate_status,
    })
}
