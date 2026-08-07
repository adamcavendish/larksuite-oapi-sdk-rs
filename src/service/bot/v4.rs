use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{
    PageIteratorPage, PageIteratorState, RestRequest, impl_page_iterator_controls,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BotSearchFilter {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chat_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_chatter: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BotSearchMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_join_group: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_agent: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BotSearchItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta_data: Option<BotSearchMeta>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchBotReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<BotSearchFilter>,
}

impl SearchBotReqBody {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: Some(query.into()),
            ..Default::default()
        }
    }

    pub fn filter(mut self, value: BotSearchFilter) -> Self {
        self.filter = Some(value);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SearchBotRespData {
    #[serde(default)]
    pub items: Vec<BotSearchItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp_v2!(SearchBotResp, SearchBotRespData);

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SearchBotQuery<'a> {
    pub body: &'a SearchBotReqBody,
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> SearchBotQuery<'a> {
    pub fn new(body: &'a SearchBotReqBody) -> Self {
        Self {
            body,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

pub struct BotResource<'a> {
    config: &'a Config,
}

impl BotResource<'_> {
    pub async fn search(
        &self,
        body: &SearchBotReqBody,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<SearchBotResp, LarkError> {
        let query = SearchBotQuery::new(body)
            .page_size(page_size)
            .page_token(page_token)
            .user_id_type(user_id_type);
        self.search_by_query(&query, option).await
    }

    pub async fn search_by_query(
        &self,
        query: &SearchBotQuery<'_>,
        option: &RequestOption,
    ) -> Result<SearchBotResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/bot/v4/bot/search",
            vec![AccessTokenType::User],
            option,
        )
        .query("page_size", query.page_size)
        .query("page_token", query.page_token)
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<SearchBotRespData, SearchBotResp>()
        .await
    }

    pub fn search_by_iterator(
        &self,
        body: &SearchBotReqBody,
        page_size: Option<i32>,
        user_id_type: Option<&str>,
    ) -> SearchBotIterator<'_> {
        SearchBotIterator {
            config: self.config,
            state: PageIteratorState::default(),
            body: body.clone(),
            page_size,
            user_id_type: user_id_type.map(ToOwned::to_owned),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchBotIterator<'a> {
    config: &'a Config,
    state: PageIteratorState<BotSearchItem>,
    body: SearchBotReqBody,
    page_size: Option<i32>,
    user_id_type: Option<String>,
}

impl_page_iterator_controls!(SearchBotIterator);

impl SearchBotIterator<'_> {
    pub async fn next(
        &mut self,
        option: &RequestOption,
    ) -> Result<Option<BotSearchItem>, LarkError> {
        let config = self.config;
        let body = &self.body;
        let page_size = self.page_size;
        let user_id_type = self.user_id_type.as_deref();
        self.state
            .next_page(|page_token| async move {
                let resource = BotResource { config };
                let response = resource
                    .search(body, page_size, page_token.as_deref(), user_id_type, option)
                    .await?;
                let data = response.data.unwrap_or_default();
                Ok(PageIteratorPage::new(
                    Some(data.items),
                    data.page_token,
                    data.has_more,
                ))
            })
            .await
    }
}

pub struct V4<'a> {
    pub bot: BotResource<'a>,
}

impl<'a> V4<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            bot: BotResource { config },
        }
    }
}
