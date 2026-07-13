use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};
use crate::service::go_v397::GoV397;

impl_resp_v2!(FavouriteApplicationResp, FavouriteApplicationRespData);

impl_resp_v2!(RecommendApplicationResp, RecommendApplicationRespData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FavouriteApplicationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub app_list: Vec<Application>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendApplicationRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default)]
    pub app_list: Vec<Application>,
}
// ── Generated nested response models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Application {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub open_methods: Vec<OpenMethod>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenMethod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applink: Option<String>,
}
pub struct ApplicationResource<'a> {
    config: &'a Config,
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct FavouriteApplicationQuery<'a> {
    pub page: PageQuery<'a>,
    pub language: Option<&'a str>,
}

impl<'a> FavouriteApplicationQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn language(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.language = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct RecommendApplicationQuery<'a> {
    pub page: PageQuery<'a>,
    pub language: Option<&'a str>,
    pub recommend_type: Option<&'a str>,
}

impl<'a> RecommendApplicationQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }

    pub fn language(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.language = value.into();
        self
    }

    pub fn recommend_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.recommend_type = value.into();
        self
    }
}

impl<'a> ApplicationResource<'a> {
    /// Favourite applications — GET /open-apis/application/v5/applications/favourite
    pub async fn favourite(
        &self,
        language: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<FavouriteApplicationResp, LarkError> {
        let query = FavouriteApplicationQuery {
            page: PageQuery::from_parts(page_size, page_token),
            language,
        };
        self.favourite_by_query(&query, option).await
    }

    pub async fn favourite_by_query(
        &self,
        query: &FavouriteApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<FavouriteApplicationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v5/applications/favourite",
            vec![AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("language", query.language)
        .send_v2_response::<FavouriteApplicationRespData, FavouriteApplicationResp>()
        .await
    }

    /// Recommended applications — GET /open-apis/application/v5/applications/recommend
    pub async fn recommend(
        &self,
        language: Option<&str>,
        recommend_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<RecommendApplicationResp, LarkError> {
        let query = RecommendApplicationQuery {
            page: PageQuery::from_parts(page_size, page_token),
            language,
            recommend_type,
        };
        self.recommend_by_query(&query, option).await
    }

    pub async fn recommend_by_query(
        &self,
        query: &RecommendApplicationQuery<'_>,
        option: &RequestOption,
    ) -> Result<RecommendApplicationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/application/v5/applications/recommend",
            vec![AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("language", query.language)
        .query("recommend_type", query.recommend_type)
        .send_v2_response::<RecommendApplicationRespData, RecommendApplicationResp>()
        .await
    }
}

pub struct V5<'a> {
    pub application: ApplicationResource<'a>,
    config: &'a Config,
}

impl<'a> V5<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            application: ApplicationResource { config },
            config,
        }
    }

    pub fn go_v397(&self) -> GoV397<'a> {
        GoV397::new(self.config)
    }
}
