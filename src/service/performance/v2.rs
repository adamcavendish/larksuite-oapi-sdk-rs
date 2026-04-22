use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::parse_v2;
use crate::transport;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricTagListData {
    #[serde(default)]
    pub items: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl_resp_v2!(QueryActivityV2Resp, serde_json::Value);
impl_resp_v2!(ImportAdditionalInformationV2Resp, serde_json::Value);
impl_resp_v2!(QueryAdditionalInformationV2Resp, serde_json::Value);
impl_resp_v2!(DeleteAdditionalInformationsBatchV2Resp, ());
impl_resp_v2!(QueryIndicatorV2Resp, serde_json::Value);
impl_resp_v2!(ImportMetricDetailV2Resp, serde_json::Value);
impl_resp_v2!(QueryMetricDetailV2Resp, serde_json::Value);
impl_resp_v2!(QueryMetricFieldV2Resp, serde_json::Value);
impl_resp_v2!(QueryMetricLibV2Resp, serde_json::Value);
impl_resp_v2!(ListMetricTagV2Resp, MetricTagListData);
impl_resp_v2!(QueryMetricTemplateV2Resp, serde_json::Value);
impl_resp_v2!(QueryQuestionV2Resp, serde_json::Value);
impl_resp_v2!(QueryReviewDataV2Resp, serde_json::Value);
impl_resp_v2!(QueryReviewTemplateV2Resp, serde_json::Value);
impl_resp_v2!(QueryRevieweeV2Resp, serde_json::Value);
impl_resp_v2!(WriteUserGroupUserRelV2Resp, serde_json::Value);
impl_resp_v2!(QueryUserInfoV2Resp, serde_json::Value);

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
    ($struct_name:ident, $method:ident, $resp:ident, $path:literal) => {
        pub struct $struct_name<'a> {
            config: &'a Config,
        }
        impl $struct_name<'_> {
            pub async fn $method(
                &self,
                body: serde_json::Value,
                option: &RequestOption,
            ) -> Result<$resp> {
                let mut api_req = ApiReq::new(http::Method::POST, $path);
                api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
                api_req.body = Some(ReqBody::json(&body)?);
                let (api_resp, raw) =
                    transport::request_typed::<serde_json::Value>(self.config, &api_req, option)
                        .await?;
                let (api_resp, code_error, data) = parse_v2(api_resp, raw);
                Ok($resp {
                    api_resp,
                    code_error,
                    data,
                })
            }
        }
    };
}

post_query!(
    ActivityV2Resource,
    query,
    QueryActivityV2Resp,
    "/open-apis/performance/v2/activity/query"
);

pub struct AdditionalInformationV2Resource<'a> {
    config: &'a Config,
}

impl AdditionalInformationV2Resource<'_> {
    pub async fn import(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ImportAdditionalInformationV2Resp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/performance/v2/additional_informations/import",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ImportAdditionalInformationV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryAdditionalInformationV2Resp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/performance/v2/additional_informations/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryAdditionalInformationV2Resp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<DeleteAdditionalInformationsBatchV2Resp> {
        let mut api_req = ApiReq::new(
            http::Method::DELETE,
            "/open-apis/performance/v2/additional_informations/batch",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteAdditionalInformationsBatchV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

post_query!(
    IndicatorV2Resource,
    query,
    QueryIndicatorV2Resp,
    "/open-apis/performance/v2/indicators/query"
);

pub struct MetricDetailV2Resource<'a> {
    config: &'a Config,
}

impl MetricDetailV2Resource<'_> {
    pub async fn import(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ImportMetricDetailV2Resp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/performance/v2/metric_details/import",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ImportMetricDetailV2Resp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn query(
        &self,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryMetricDetailV2Resp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/performance/v2/metric_details/query",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(&body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryMetricDetailV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

post_query!(
    MetricFieldV2Resource,
    query,
    QueryMetricFieldV2Resp,
    "/open-apis/performance/v2/metric_fields/query"
);
post_query!(
    MetricLibV2Resource,
    query,
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
    ) -> Result<ListMetricTagV2Resp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/performance/v2/metric_tags");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<MetricTagListData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListMetricTagV2Resp {
            api_resp,
            code_error,
            data,
        })
    }
}

post_query!(
    MetricTemplateV2Resource,
    query,
    QueryMetricTemplateV2Resp,
    "/open-apis/performance/v2/metric_templates/query"
);
post_query!(
    QuestionV2Resource,
    query,
    QueryQuestionV2Resp,
    "/open-apis/performance/v2/questions/query"
);
post_query!(
    ReviewDataV2Resource,
    query,
    QueryReviewDataV2Resp,
    "/open-apis/performance/v2/review_datas/query"
);
post_query!(
    ReviewTemplateV2Resource,
    query,
    QueryReviewTemplateV2Resp,
    "/open-apis/performance/v2/review_templates/query"
);
post_query!(
    RevieweeV2Resource,
    query,
    QueryRevieweeV2Resp,
    "/open-apis/performance/v2/reviewees/query"
);
post_query!(
    UserGroupUserRelV2Resource,
    write,
    WriteUserGroupUserRelV2Resp,
    "/open-apis/performance/v2/user_group_user_rels/write"
);
post_query!(
    UserInfoV2Resource,
    query,
    QueryUserInfoV2Resp,
    "/open-apis/performance/v2/user_info/query"
);
