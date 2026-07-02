use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyRespV2 as EmptyResp, PageQuery, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_rule_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_month: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_month: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_list: Option<Vec<OkrObjective>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrObjective {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kr_list: Option<Vec<OkrKeyResult>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aligned_objective_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aligning_objective_list: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrKeyResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kr_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<serde_json::Value>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PeriodListData {
    #[serde(default)]
    pub items: Vec<OkrPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OkrListData {
    #[serde(default)]
    pub okr_list: Vec<OkrItem>,
}

impl_resp!(ListPeriodResp, PeriodListData);
impl_resp!(GetUserOkrResp, OkrListData);

impl_resp_v2!(UploadImageResp, serde_json::Value);
impl_resp_v2!(BatchGetOkrResp, serde_json::Value);
impl_resp_v2!(CreatePeriodResp, serde_json::Value);
impl_resp_v2!(PatchPeriodResp, serde_json::Value);
impl_resp_v2!(ListPeriodRuleResp, serde_json::Value);
impl_resp_v2!(CreateProgressRecordResp, serde_json::Value);
impl_resp_v2!(GetProgressRecordResp, serde_json::Value);
impl_resp_v2!(UpdateProgressRecordResp, serde_json::Value);
impl_resp_v2!(QueryReviewResp, serde_json::Value);

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListPeriodQuery<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListPeriodQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }

    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page.page_size = value.into();
        self
    }

    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page.page_token = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreatePeriodQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> CreatePeriodQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchPeriodQuery<'a> {
    pub period_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> PatchPeriodQuery<'a> {
    pub fn new(period_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { period_id, body }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct ListPeriodRuleQuery;

impl ListPeriodRuleQuery {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UploadImageQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> UploadImageQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BatchGetOkrQuery<'a> {
    pub okr_ids: &'a [&'a str],
    pub user_id_type: Option<&'a str>,
    pub lang: Option<&'a str>,
}

impl<'a> BatchGetOkrQuery<'a> {
    pub fn new(okr_ids: &'a [&'a str]) -> Self {
        Self {
            okr_ids,
            user_id_type: None,
            lang: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateProgressRecordQuery<'a> {
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateProgressRecordQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteProgressRecordQuery<'a> {
    pub progress_id: &'a str,
}

impl<'a> DeleteProgressRecordQuery<'a> {
    pub fn new(progress_id: &'a str) -> Self {
        Self { progress_id }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetProgressRecordQuery<'a> {
    pub progress_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetProgressRecordQuery<'a> {
    pub fn new(progress_id: &'a str) -> Self {
        Self {
            progress_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateProgressRecordQuery<'a> {
    pub progress_id: &'a str,
    pub body: &'a serde_json::Value,
    pub user_id_type: Option<&'a str>,
}

impl<'a> UpdateProgressRecordQuery<'a> {
    pub fn new(progress_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            progress_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct QueryReviewQuery<'a> {
    pub user_ids: &'a [&'a str],
    pub period_ids: Option<&'a [&'a str]>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> QueryReviewQuery<'a> {
    pub fn new(user_ids: &'a [&'a str]) -> Self {
        Self {
            user_ids,
            period_ids: None,
            user_id_type: None,
        }
    }

    pub fn period_ids(mut self, value: impl Into<Option<&'a [&'a str]>>) -> Self {
        self.period_ids = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GetUserOkrQuery<'a> {
    pub user_id: &'a str,
    pub user_id_type: Option<&'a str>,
    pub period_ids: Option<Vec<&'a str>>,
    pub offset: Option<&'a str>,
    pub limit: Option<&'a str>,
    pub lang: Option<&'a str>,
}

impl<'a> GetUserOkrQuery<'a> {
    pub fn new(user_id: &'a str) -> Self {
        Self {
            user_id,
            user_id_type: None,
            period_ids: None,
            offset: None,
            limit: None,
            lang: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn period_ids(mut self, value: impl Into<Option<Vec<&'a str>>>) -> Self {
        self.period_ids = value.into();
        self
    }

    pub fn offset(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.offset = value.into();
        self
    }

    pub fn limit(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.limit = value.into();
        self
    }

    pub fn lang(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.lang = value.into();
        self
    }
}

// ── Resources ──

pub struct PeriodResource<'a> {
    config: &'a Config,
}

impl<'a> PeriodResource<'a> {
    pub async fn list(
        &self,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: &RequestOption,
    ) -> Result<ListPeriodResp, LarkError> {
        let query = ListPeriodQuery::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListPeriodQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListPeriodResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/okr/v1/periods",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send::<PeriodListData>()
        .await?;
        Ok(ListPeriodResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn create(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreatePeriodResp, LarkError> {
        let query = CreatePeriodQuery::new(body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreatePeriodQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreatePeriodResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/okr/v1/periods",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreatePeriodResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        period_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchPeriodResp, LarkError> {
        let query = PatchPeriodQuery::new(period_id, body);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchPeriodQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchPeriodResp, LarkError> {
        let path = format!("/open-apis/okr/v1/periods/{}", query.period_id);
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(PatchPeriodResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── PeriodRule resource ──

pub struct PeriodRuleResource<'a> {
    config: &'a Config,
}

impl PeriodRuleResource<'_> {
    pub async fn list(&self, option: &RequestOption) -> Result<ListPeriodRuleResp, LarkError> {
        let query = ListPeriodRuleQuery::new();
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        _query: &ListPeriodRuleQuery,
        option: &RequestOption,
    ) -> Result<ListPeriodRuleResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/okr/v1/period_rules",
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ListPeriodRuleResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Image resource ──

pub struct ImageResource<'a> {
    config: &'a Config,
}

impl ImageResource<'_> {
    pub async fn upload(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<UploadImageResp, LarkError> {
        let query = UploadImageQuery::new(body);
        self.upload_by_query(&query, option).await
    }

    pub async fn upload_by_query(
        &self,
        query: &UploadImageQuery<'_>,
        option: &RequestOption,
    ) -> Result<UploadImageResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/okr/v1/images/upload",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UploadImageResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Okr resource ──

pub struct OkrResource<'a> {
    config: &'a Config,
}

impl OkrResource<'_> {
    pub async fn batch_get(
        &self,
        okr_ids: &[&str],
        user_id_type: Option<&str>,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchGetOkrResp, LarkError> {
        let query = BatchGetOkrQuery::new(okr_ids)
            .user_id_type(user_id_type)
            .lang(lang);
        self.batch_get_by_query(&query, option).await
    }

    pub async fn batch_get_by_query(
        &self,
        query: &BatchGetOkrQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchGetOkrResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/okr/v1/okrs/batch_get",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query_values("okr_ids", Some(query.okr_ids.iter().copied()))
        .query("user_id_type", query.user_id_type)
        .query("lang", query.lang)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchGetOkrResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── ProgressRecord resource ──

pub struct ProgressRecordResource<'a> {
    config: &'a Config,
}

impl ProgressRecordResource<'_> {
    pub async fn create(
        &self,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateProgressRecordResp, LarkError> {
        let query = CreateProgressRecordQuery::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateProgressRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateProgressRecordResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/okr/v1/progress_records",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateProgressRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn delete(
        &self,
        progress_id: &str,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let query = DeleteProgressRecordQuery::new(progress_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteProgressRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/okr/v1/progress_records/{}", query.progress_id);
        let (api_resp, code_error, _) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(EmptyResp {
            api_resp,
            code_error,
        })
    }

    pub async fn get(
        &self,
        progress_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetProgressRecordResp, LarkError> {
        let query = GetProgressRecordQuery::new(progress_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetProgressRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetProgressRecordResp, LarkError> {
        let path = format!("/open-apis/okr/v1/progress_records/{}", query.progress_id);
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(GetProgressRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        progress_id: &str,
        body: &serde_json::Value,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<UpdateProgressRecordResp, LarkError> {
        let query = UpdateProgressRecordQuery::new(progress_id, body).user_id_type(user_id_type);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateProgressRecordQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateProgressRecordResp, LarkError> {
        let path = format!("/open-apis/okr/v1/progress_records/{}", query.progress_id);
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateProgressRecordResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Review resource ──

pub struct ReviewResource<'a> {
    config: &'a Config,
}

impl ReviewResource<'_> {
    pub async fn query(
        &self,
        user_ids: &[&str],
        period_ids: Option<&[&str]>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<QueryReviewResp, LarkError> {
        let query = QueryReviewQuery::new(user_ids)
            .period_ids(period_ids)
            .user_id_type(user_id_type);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryReviewQuery<'_>,
        option: &RequestOption,
    ) -> Result<QueryReviewResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/okr/v1/reviews/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query_values("user_ids", Some(query.user_ids.iter().copied()))
        .query_values(
            "period_ids",
            query.period_ids.map(|ids| ids.iter().copied()),
        )
        .query("user_id_type", query.user_id_type)
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(QueryReviewResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct UserOkrResource<'a> {
    config: &'a Config,
}

impl<'a> UserOkrResource<'a> {
    #[allow(clippy::too_many_arguments)]
    pub async fn get(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        period_ids: Option<Vec<&str>>,
        offset: Option<&str>,
        limit: Option<&str>,
        lang: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetUserOkrResp, LarkError> {
        let query = GetUserOkrQuery::new(user_id)
            .user_id_type(user_id_type)
            .period_ids(period_ids)
            .offset(offset)
            .limit(limit)
            .lang(lang);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetUserOkrQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetUserOkrResp, LarkError> {
        let path = format!("/open-apis/okr/v1/users/{}/okrs", query.user_id);
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .query_values(
            "period_ids",
            query.period_ids.as_ref().map(|ids| ids.iter().copied()),
        )
        .query("offset", query.offset)
        .query("limit", query.limit)
        .query("lang", query.lang)
        .send::<OkrListData>()
        .await?;
        Ok(GetUserOkrResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub period: PeriodResource<'a>,
    pub period_rule: PeriodRuleResource<'a>,
    pub image: ImageResource<'a>,
    pub okr: OkrResource<'a>,
    pub progress_record: ProgressRecordResource<'a>,
    pub review: ReviewResource<'a>,
    pub user_okr: UserOkrResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            period: PeriodResource { config },
            period_rule: PeriodRuleResource { config },
            image: ImageResource { config },
            okr: OkrResource { config },
            progress_record: ProgressRecordResource { config },
            review: ReviewResource { config },
            user_okr: UserOkrResource { config },
        }
    }
}
