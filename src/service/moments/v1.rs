use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Post {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PostListData {
    #[serde(default)]
    pub items: Vec<Post>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp!(ListPostResp, PostListData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PostGetData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post: Option<Post>,
}

impl_resp!(GetPostResp, PostGetData);

#[derive(Debug, Clone, Copy)]
pub struct GetPostQuery<'a> {
    pub post_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetPostQuery<'a> {
    pub fn new(post_id: &'a str) -> Self {
        Self {
            post_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListPostQuery<'a> {
    pub category_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListPostQuery<'a> {
    pub fn new() -> Self {
        Self {
            category_id: None,
            user_id_type: None,
            page: PageQuery::default(),
        }
    }

    pub fn category_id(mut self, category_id: impl Into<Option<&'a str>>) -> Self {
        self.category_id = category_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

impl<'a> Default for ListPostQuery<'a> {
    fn default() -> Self {
        Self::new()
    }
}

// ── Resources ──

pub struct PostResource<'a> {
    config: &'a Config,
}

impl<'a> PostResource<'a> {
    pub async fn get(
        &self,
        post_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetPostResp, LarkError> {
        self.get_by_query(
            &GetPostQuery::new(post_id).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetPostQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetPostResp, LarkError> {
        let path = format!("/open-apis/moments/v1/posts/{}", query.post_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_response::<PostGetData, GetPostResp>()
        .await
    }

    pub async fn list(
        &self,
        category_id: Option<&str>,
        user_id_type: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListPostResp, LarkError> {
        let query = ListPostQuery::new()
            .category_id(category_id)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPostQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPostResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/moments/v1/posts",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("category_id", query.category_id)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_response::<PostListData, ListPostResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub post: PostResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            post: PostResource { config },
        }
    }
}
