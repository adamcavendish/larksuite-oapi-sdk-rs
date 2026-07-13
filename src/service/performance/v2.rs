use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricTagListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp_v2!(QueryActivityV2Resp, QueryActivityV2RespData);
impl_resp_v2!(
    ImportAdditionalInformationV2Resp,
    ImportAdditionalInformationV2RespData
);
impl_resp_v2!(
    QueryAdditionalInformationV2Resp,
    QueryAdditionalInformationV2RespData
);
impl_resp_v2!(DeleteAdditionalInformationsBatchV2Resp, ());
impl_resp_v2!(QueryIndicatorV2Resp, QueryIndicatorV2RespData);
impl_resp_v2!(ImportMetricDetailV2Resp, ImportMetricDetailV2RespData);
impl_resp_v2!(QueryMetricDetailV2Resp, QueryMetricDetailV2RespData);
impl_resp_v2!(QueryMetricFieldV2Resp, QueryMetricFieldV2RespData);
impl_resp_v2!(QueryMetricLibV2Resp, QueryMetricLibV2RespData);
impl_resp_v2!(ListMetricTagV2Resp, MetricTagListData);
impl_resp_v2!(QueryMetricTemplateV2Resp, QueryMetricTemplateV2RespData);
impl_resp_v2!(QueryQuestionV2Resp, QueryQuestionV2RespData);
impl_resp_v2!(QueryReviewDataV2Resp, QueryReviewDataV2RespData);
impl_resp_v2!(QueryReviewTemplateV2Resp, QueryReviewTemplateV2RespData);
impl_resp_v2!(QueryRevieweeV2Resp, QueryRevieweeV2RespData);
impl_resp_v2!(WriteUserGroupUserRelV2Resp, WriteUserGroupUserRelV2RespData);
impl_resp_v2!(QueryUserInfoV2Resp, QueryUserInfoV2RespData);

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryActivityV2RespData {
    #[serde(default)]
    pub activities: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportAdditionalInformationV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_record_id: Option<String>,
    #[serde(default)]
    pub additional_informations: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryAdditionalInformationV2RespData {
    #[serde(default)]
    pub additional_informations: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryIndicatorV2RespData {
    #[serde(default)]
    pub indicators: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportMetricDetailV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMetricDetailV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default)]
    pub reviewee_metrics: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMetricFieldV2RespData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMetricLibV2RespData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMetricTemplateV2RespData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryQuestionV2RespData {
    #[serde(default)]
    pub tag_based_questions: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryReviewDataV2RespData {
    #[serde(default)]
    pub datas: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryReviewTemplateV2RespData {
    #[serde(default)]
    pub review_templates: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryRevieweeV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default)]
    pub reviewees: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WriteUserGroupUserRelV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserInfoV2RespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    #[serde(default)]
    pub user_infos: Vec<serde_json::Value>,
}
// -- Query parameter types --

#[derive(Debug, Clone, Copy, Default)]
pub struct ListMetricTagV2Query<'a> {
    pub tag_ids: Option<&'a [&'a str]>,
    pub page: PageQuery<'a>,
}

impl<'a> ListMetricTagV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tag_ids(mut self, value: impl Into<Option<&'a [&'a str]>>) -> Self {
        self.tag_ids = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

pub struct V2<'a> {
    pub activity: ActivityV2Resource<'a>,
    pub additional_information: AdditionalInformationV2Resource<'a>,
    pub additional_informations_batch: AdditionalInformationsBatchV2Resource<'a>,
    pub indicator: IndicatorV2Resource<'a>,
    pub metric_detail: MetricDetailV2Resource<'a>,
    pub metric_field: MetricFieldV2Resource<'a>,
    pub metric_lib: MetricLibV2Resource<'a>,
    pub metric_tag: MetricTagV2Resource<'a>,
    pub metric_template: MetricTemplateV2Resource<'a>,
    pub question: QuestionV2Resource<'a>,
    pub review_data: ReviewDataV2Resource<'a>,
    pub review_template: ReviewTemplateV2Resource<'a>,
    pub reviewee: RevieweeV2Resource<'a>,
    pub user_group_user_rel: UserGroupUserRelV2Resource<'a>,
    pub user_info: UserInfoV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            activity: ActivityV2Resource { config },
            additional_information: AdditionalInformationV2Resource { config },
            additional_informations_batch: AdditionalInformationsBatchV2Resource { config },
            indicator: IndicatorV2Resource { config },
            metric_detail: MetricDetailV2Resource { config },
            metric_field: MetricFieldV2Resource { config },
            metric_lib: MetricLibV2Resource { config },
            metric_tag: MetricTagV2Resource { config },
            metric_template: MetricTemplateV2Resource { config },
            question: QuestionV2Resource { config },
            review_data: ReviewDataV2Resource { config },
            review_template: ReviewTemplateV2Resource { config },
            reviewee: RevieweeV2Resource { config },
            user_group_user_rel: UserGroupUserRelV2Resource { config },
            user_info: UserInfoV2Resource { config },
        }
    }
}

macro_rules! post_query {
    ($struct_name:ident, $query_name:ident, $method:ident, $by_query_method:ident, $data:ty, $resp:ident, $path:literal) => {
        #[derive(Debug, Clone, Copy)]
        #[non_exhaustive]
        pub struct $query_name<'a> {
            pub body: &'a serde_json::Value,
        }

        impl<'a> $query_name<'a> {
            pub fn new(body: &'a serde_json::Value) -> Self {
                Self { body }
            }
        }

        pub struct $struct_name<'a> {
            config: &'a Config,
        }

        impl $struct_name<'_> {
            pub async fn $method(
                &self,
                body: serde_json::Value,
                option: &RequestOption,
            ) -> Result<$resp, LarkError> {
                let query = $query_name::new(&body);
                self.$by_query_method(&query, option).await
            }

            pub async fn $by_query_method(
                &self,
                query: &$query_name<'_>,
                option: &RequestOption,
            ) -> Result<$resp, LarkError> {
                RestRequest::new(
                    self.config,
                    http::Method::POST,
                    $path,
                    vec![AccessTokenType::Tenant],
                    option,
                )
                .json_body(query.body)?
                .send_v2_response::<$data, $resp>()
                .await
            }
        }
    };
}

post_query!(
    ActivityV2Resource,
    QueryActivityV2Query,
    query,
    query_by_query,
    QueryActivityV2RespData,
    QueryActivityV2Resp,
    "/open-apis/performance/v2/activity/query"
);

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ImportAdditionalInformationV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> ImportAdditionalInformationV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct QueryAdditionalInformationV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> QueryAdditionalInformationV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

pub struct AdditionalInformationV2Resource<'a> {
    config: &'a Config,
}

impl AdditionalInformationV2Resource<'_> {
    pub async fn import(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ImportAdditionalInformationV2Resp, LarkError> {
        let query = ImportAdditionalInformationV2Query::new(&body);
        self.import_by_query(&query, option).await
    }

    pub async fn import_by_query(
        &self,
        query: &ImportAdditionalInformationV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ImportAdditionalInformationV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/performance/v2/additional_informations/import",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<ImportAdditionalInformationV2RespData, ImportAdditionalInformationV2Resp>()
        .await
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryAdditionalInformationV2Resp, LarkError> {
        let query = QueryAdditionalInformationV2Query::new(&body);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryAdditionalInformationV2Query<'_>,
        option: &RequestOption,
    ) -> Result<QueryAdditionalInformationV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/performance/v2/additional_informations/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<QueryAdditionalInformationV2RespData, QueryAdditionalInformationV2Resp>(
        )
        .await
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteAdditionalInformationsBatchV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> DeleteAdditionalInformationsBatchV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

pub struct AdditionalInformationsBatchV2Resource<'a> {
    config: &'a Config,
}

impl AdditionalInformationsBatchV2Resource<'_> {
    pub async fn delete(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<DeleteAdditionalInformationsBatchV2Resp, LarkError> {
        let query = DeleteAdditionalInformationsBatchV2Query::new(&body);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteAdditionalInformationsBatchV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteAdditionalInformationsBatchV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/performance/v2/additional_informations/batch",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), DeleteAdditionalInformationsBatchV2Resp>()
        .await
    }
}

post_query!(
    IndicatorV2Resource,
    QueryIndicatorV2Query,
    query,
    query_by_query,
    QueryIndicatorV2RespData,
    QueryIndicatorV2Resp,
    "/open-apis/performance/v2/indicators/query"
);

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ImportMetricDetailV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> ImportMetricDetailV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct QueryMetricDetailV2Query<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> QueryMetricDetailV2Query<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

pub struct MetricDetailV2Resource<'a> {
    config: &'a Config,
}

impl MetricDetailV2Resource<'_> {
    pub async fn import(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ImportMetricDetailV2Resp, LarkError> {
        let query = ImportMetricDetailV2Query::new(&body);
        self.import_by_query(&query, option).await
    }

    pub async fn import_by_query(
        &self,
        query: &ImportMetricDetailV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ImportMetricDetailV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/performance/v2/metric_details/import",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<ImportMetricDetailV2RespData, ImportMetricDetailV2Resp>()
        .await
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryMetricDetailV2Resp, LarkError> {
        let query = QueryMetricDetailV2Query::new(&body);
        self.query_by_query(&query, option).await
    }

    pub async fn query_by_query(
        &self,
        query: &QueryMetricDetailV2Query<'_>,
        option: &RequestOption,
    ) -> Result<QueryMetricDetailV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/performance/v2/metric_details/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<QueryMetricDetailV2RespData, QueryMetricDetailV2Resp>()
        .await
    }
}

post_query!(
    MetricFieldV2Resource,
    QueryMetricFieldV2Query,
    query,
    query_by_query,
    QueryMetricFieldV2RespData,
    QueryMetricFieldV2Resp,
    "/open-apis/performance/v2/metric_fields/query"
);
post_query!(
    MetricLibV2Resource,
    QueryMetricLibV2Query,
    query,
    query_by_query,
    QueryMetricLibV2RespData,
    QueryMetricLibV2Resp,
    "/open-apis/performance/v2/metric_libs/query"
);

pub struct MetricTagV2Resource<'a> {
    config: &'a Config,
}

impl MetricTagV2Resource<'_> {
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListMetricTagV2Resp, LarkError> {
        let query = ListMetricTagV2Query::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListMetricTagV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListMetricTagV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/performance/v2/metric_tags",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query_values("tag_ids", query.tag_ids)
        .send_v2_response::<MetricTagListData, ListMetricTagV2Resp>()
        .await
    }
}

post_query!(
    MetricTemplateV2Resource,
    QueryMetricTemplateV2Query,
    query,
    query_by_query,
    QueryMetricTemplateV2RespData,
    QueryMetricTemplateV2Resp,
    "/open-apis/performance/v2/metric_templates/query"
);
post_query!(
    QuestionV2Resource,
    QueryQuestionV2Query,
    query,
    query_by_query,
    QueryQuestionV2RespData,
    QueryQuestionV2Resp,
    "/open-apis/performance/v2/questions/query"
);
post_query!(
    ReviewDataV2Resource,
    QueryReviewDataV2Query,
    query,
    query_by_query,
    QueryReviewDataV2RespData,
    QueryReviewDataV2Resp,
    "/open-apis/performance/v2/review_datas/query"
);
post_query!(
    ReviewTemplateV2Resource,
    QueryReviewTemplateV2Query,
    query,
    query_by_query,
    QueryReviewTemplateV2RespData,
    QueryReviewTemplateV2Resp,
    "/open-apis/performance/v2/review_templates/query"
);
post_query!(
    RevieweeV2Resource,
    QueryRevieweeV2Query,
    query,
    query_by_query,
    QueryRevieweeV2RespData,
    QueryRevieweeV2Resp,
    "/open-apis/performance/v2/reviewees/query"
);
post_query!(
    UserGroupUserRelV2Resource,
    WriteUserGroupUserRelV2Query,
    write,
    write_by_query,
    WriteUserGroupUserRelV2RespData,
    WriteUserGroupUserRelV2Resp,
    "/open-apis/performance/v2/user_group_user_rels/write"
);
post_query!(
    UserInfoV2Resource,
    QueryUserInfoV2Query,
    query,
    query_by_query,
    QueryUserInfoV2RespData,
    QueryUserInfoV2Resp,
    "/open-apis/performance/v2/user_info/query"
);
