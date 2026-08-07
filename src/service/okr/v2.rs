use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{
    PageIteratorPage, PageIteratorState, RestRequest, impl_page_iterator_controls,
};

fn request<'a>(
    config: &'a Config,
    method: http::Method,
    path: impl Into<String>,
    option: &'a RequestOption,
) -> RestRequest<'a> {
    RestRequest::new(
        config,
        method,
        path,
        vec![AccessTokenType::User, AccessTokenType::Tenant],
        option,
    )
}

// ── Domain models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Owner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Category {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Cycle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cycle_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndicatorUnit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Indicator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<IndicatorUnit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Objective {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KeyResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_result_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Alignment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_entity_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Progress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Owner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectiveWeight {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KeyResultWeight {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_result_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

// ── Request bodies ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct ObjectivesPositionReqBody {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objective_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ObjectivesWeightReqBody {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objective_weights: Vec<ObjectiveWeight>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateObjectiveReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<crate::JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchIndicatorReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value_calculate_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_calculate_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<IndicatorUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicator_status: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchKeyResultReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KeyResultsPositionReqBody {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub key_result_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct KeyResultsWeightReqBody {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub key_result_weights: Vec<KeyResultWeight>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PatchObjectiveReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<crate::JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateObjectiveAlignmentReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_entity_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_entity_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateObjectiveKeyResultReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<crate::JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

// ── Query and response models ──

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct OkrPageQuery<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub user_id: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> OkrPageQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn page_size(mut self, value: impl Into<Option<i32>>) -> Self {
        self.page_size = value.into();
        self
    }
    pub fn page_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.page_token = value.into();
        self
    }
    pub fn user_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id = value.into();
        self
    }
    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

fn page_query<'a>(request: RestRequest<'a>, query: &OkrPageQuery<'_>) -> RestRequest<'a> {
    request
        .query("page_size", query.page_size)
        .query("page_token", query.page_token)
        .query("user_id", query.user_id)
        .query("user_id_type", query.user_id_type)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OkrPage<T> {
    #[serde(default)]
    pub items: Vec<T>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IdData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_result_id: Option<String>,
}

macro_rules! wrapped_data {
    ($name:ident, $field:ident, $type:ty) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        #[non_exhaustive]
        pub struct $name {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub $field: Option<$type>,
        }
    };
}
wrapped_data!(AlignmentData, alignment, Alignment);
wrapped_data!(ObjectiveData, objective, Objective);
wrapped_data!(KeyResultData, key_result, KeyResult);
wrapped_data!(IndicatorData, indicator, Indicator);
wrapped_data!(ObjectivesData, items, Vec<Objective>);
wrapped_data!(KeyResultsData, items, Vec<KeyResult>);

impl_resp_v2!(DeleteOkrAlignmentResp, IdData);
impl_resp_v2!(GetOkrAlignmentResp, AlignmentData);
impl_resp_v2!(ListOkrCategoryResp, OkrPage<Category>);
impl_resp_v2!(ListOkrCycleResp, OkrPage<Cycle>);
impl_resp_v2!(ObjectivesPositionOkrCycleResp, ObjectivesData);
impl_resp_v2!(ObjectivesWeightOkrCycleResp, ObjectivesData);
impl_resp_v2!(CreateOkrCycleObjectiveResp, IdData);
impl_resp_v2!(ListOkrCycleObjectiveResp, OkrPage<Objective>);
impl_resp_v2!(PatchOkrIndicatorResp, IndicatorData);
impl_resp_v2!(DeleteOkrKeyResultResp, IdData);
impl_resp_v2!(GetOkrKeyResultResp, KeyResultData);
impl_resp_v2!(PatchOkrKeyResultResp, KeyResultData);
impl_resp_v2!(ListOkrKeyResultIndicatorResp, IndicatorData);
impl_resp_v2!(ListOkrKeyResultProgressResp, OkrPage<Progress>);
impl_resp_v2!(DeleteOkrObjectiveResp, IdData);
impl_resp_v2!(GetOkrObjectiveResp, ObjectiveData);
impl_resp_v2!(KeyResultsPositionOkrObjectiveResp, KeyResultsData);
impl_resp_v2!(KeyResultsWeightOkrObjectiveResp, KeyResultsData);
impl_resp_v2!(PatchOkrObjectiveResp, ObjectiveData);
impl_resp_v2!(CreateOkrObjectiveAlignmentResp, IdData);
impl_resp_v2!(ListOkrObjectiveAlignmentResp, OkrPage<Alignment>);
impl_resp_v2!(ListOkrObjectiveIndicatorResp, IndicatorData);
impl_resp_v2!(CreateOkrObjectiveKeyResultResp, IdData);
impl_resp_v2!(ListOkrObjectiveKeyResultResp, OkrPage<KeyResult>);
impl_resp_v2!(ListOkrObjectiveProgressResp, OkrPage<Progress>);

// ── Resources ──

pub struct OkrAlignmentResource<'a> {
    config: &'a Config,
}
impl OkrAlignmentResource<'_> {
    pub async fn delete(
        &self,
        alignment_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteOkrAlignmentResp, LarkError> {
        request(
            self.config,
            http::Method::DELETE,
            format!("/open-apis/okr/v2/alignments/{alignment_id}"),
            option,
        )
        .send_v2_response::<IdData, DeleteOkrAlignmentResp>()
        .await
    }
    pub async fn get(
        &self,
        alignment_id: &str,
        option: &RequestOption,
    ) -> Result<GetOkrAlignmentResp, LarkError> {
        request(
            self.config,
            http::Method::GET,
            format!("/open-apis/okr/v2/alignments/{alignment_id}"),
            option,
        )
        .send_v2_response::<AlignmentData, GetOkrAlignmentResp>()
        .await
    }
}

pub struct OkrCategoryResource<'a> {
    config: &'a Config,
}
impl OkrCategoryResource<'_> {
    pub async fn list(
        &self,
        query: &OkrPageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOkrCategoryResp, LarkError> {
        page_query(
            request(
                self.config,
                http::Method::GET,
                "/open-apis/okr/v2/categories",
                option,
            ),
            query,
        )
        .send_v2_response::<OkrPage<Category>, ListOkrCategoryResp>()
        .await
    }
    pub fn list_by_iterator(&self, query: &OkrPageQuery<'_>) -> ListOkrCategoryIterator<'_> {
        ListOkrCategoryIterator::new(self.config, query)
    }
}

pub struct OkrCycleResource<'a> {
    config: &'a Config,
}
impl OkrCycleResource<'_> {
    pub async fn list(
        &self,
        query: &OkrPageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOkrCycleResp, LarkError> {
        page_query(
            request(
                self.config,
                http::Method::GET,
                "/open-apis/okr/v2/cycles",
                option,
            ),
            query,
        )
        .send_v2_response::<OkrPage<Cycle>, ListOkrCycleResp>()
        .await
    }
    pub fn list_by_iterator(&self, query: &OkrPageQuery<'_>) -> ListOkrCycleIterator<'_> {
        ListOkrCycleIterator::new(self.config, query)
    }
    pub async fn objectives_position(
        &self,
        cycle_id: &str,
        body: &ObjectivesPositionReqBody,
        option: &RequestOption,
    ) -> Result<ObjectivesPositionOkrCycleResp, LarkError> {
        request(
            self.config,
            http::Method::PUT,
            format!("/open-apis/okr/v2/cycles/{cycle_id}/objectives_position"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<ObjectivesData, ObjectivesPositionOkrCycleResp>()
        .await
    }
    pub async fn objectives_weight(
        &self,
        cycle_id: &str,
        body: &ObjectivesWeightReqBody,
        option: &RequestOption,
    ) -> Result<ObjectivesWeightOkrCycleResp, LarkError> {
        request(
            self.config,
            http::Method::PUT,
            format!("/open-apis/okr/v2/cycles/{cycle_id}/objectives_weight"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<ObjectivesData, ObjectivesWeightOkrCycleResp>()
        .await
    }
}

pub struct OkrCycleObjectiveResource<'a> {
    config: &'a Config,
}
impl OkrCycleObjectiveResource<'_> {
    pub async fn create(
        &self,
        cycle_id: &str,
        body: &CreateObjectiveReqBody,
        option: &RequestOption,
    ) -> Result<CreateOkrCycleObjectiveResp, LarkError> {
        request(
            self.config,
            http::Method::POST,
            format!("/open-apis/okr/v2/cycles/{cycle_id}/objectives"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<IdData, CreateOkrCycleObjectiveResp>()
        .await
    }
    pub async fn list(
        &self,
        cycle_id: &str,
        query: &OkrPageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOkrCycleObjectiveResp, LarkError> {
        page_query(
            request(
                self.config,
                http::Method::GET,
                format!("/open-apis/okr/v2/cycles/{cycle_id}/objectives"),
                option,
            ),
            query,
        )
        .send_v2_response::<OkrPage<Objective>, ListOkrCycleObjectiveResp>()
        .await
    }
    pub fn list_by_iterator(
        &self,
        cycle_id: &str,
        query: &OkrPageQuery<'_>,
    ) -> ListOkrCycleObjectiveIterator<'_> {
        ListOkrCycleObjectiveIterator::new(self.config, cycle_id, query)
    }
}

pub struct OkrIndicatorResource<'a> {
    config: &'a Config,
}
impl OkrIndicatorResource<'_> {
    pub async fn patch(
        &self,
        indicator_id: &str,
        body: &PatchIndicatorReqBody,
        option: &RequestOption,
    ) -> Result<PatchOkrIndicatorResp, LarkError> {
        request(
            self.config,
            http::Method::PATCH,
            format!("/open-apis/okr/v2/indicators/{indicator_id}"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<IndicatorData, PatchOkrIndicatorResp>()
        .await
    }
}

pub struct OkrKeyResultResource<'a> {
    config: &'a Config,
}
impl OkrKeyResultResource<'_> {
    pub async fn delete(
        &self,
        key_result_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteOkrKeyResultResp, LarkError> {
        request(
            self.config,
            http::Method::DELETE,
            format!("/open-apis/okr/v2/key_results/{key_result_id}"),
            option,
        )
        .send_v2_response::<IdData, DeleteOkrKeyResultResp>()
        .await
    }
    pub async fn get(
        &self,
        key_result_id: &str,
        option: &RequestOption,
    ) -> Result<GetOkrKeyResultResp, LarkError> {
        request(
            self.config,
            http::Method::GET,
            format!("/open-apis/okr/v2/key_results/{key_result_id}"),
            option,
        )
        .send_v2_response::<KeyResultData, GetOkrKeyResultResp>()
        .await
    }
    pub async fn patch(
        &self,
        key_result_id: &str,
        body: &PatchKeyResultReqBody,
        option: &RequestOption,
    ) -> Result<PatchOkrKeyResultResp, LarkError> {
        request(
            self.config,
            http::Method::PATCH,
            format!("/open-apis/okr/v2/key_results/{key_result_id}"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<KeyResultData, PatchOkrKeyResultResp>()
        .await
    }
}

pub struct OkrKeyResultIndicatorResource<'a> {
    config: &'a Config,
}
impl OkrKeyResultIndicatorResource<'_> {
    pub async fn list(
        &self,
        key_result_id: &str,
        option: &RequestOption,
    ) -> Result<ListOkrKeyResultIndicatorResp, LarkError> {
        request(
            self.config,
            http::Method::GET,
            format!("/open-apis/okr/v2/key_results/{key_result_id}/indicators"),
            option,
        )
        .send_v2_response::<IndicatorData, ListOkrKeyResultIndicatorResp>()
        .await
    }
}

pub struct OkrKeyResultProgressResource<'a> {
    config: &'a Config,
}
impl OkrKeyResultProgressResource<'_> {
    pub async fn list(
        &self,
        key_result_id: &str,
        query: &OkrPageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOkrKeyResultProgressResp, LarkError> {
        page_query(
            request(
                self.config,
                http::Method::GET,
                format!("/open-apis/okr/v2/key_results/{key_result_id}/progresses"),
                option,
            ),
            query,
        )
        .send_v2_response::<OkrPage<Progress>, ListOkrKeyResultProgressResp>()
        .await
    }
    pub fn list_by_iterator(
        &self,
        key_result_id: &str,
        query: &OkrPageQuery<'_>,
    ) -> ListOkrKeyResultProgressIterator<'_> {
        ListOkrKeyResultProgressIterator::new(self.config, key_result_id, query)
    }
}

pub struct OkrObjectiveResource<'a> {
    config: &'a Config,
}
impl OkrObjectiveResource<'_> {
    pub async fn delete(
        &self,
        objective_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteOkrObjectiveResp, LarkError> {
        request(
            self.config,
            http::Method::DELETE,
            format!("/open-apis/okr/v2/objectives/{objective_id}"),
            option,
        )
        .send_v2_response::<IdData, DeleteOkrObjectiveResp>()
        .await
    }
    pub async fn get(
        &self,
        objective_id: &str,
        option: &RequestOption,
    ) -> Result<GetOkrObjectiveResp, LarkError> {
        request(
            self.config,
            http::Method::GET,
            format!("/open-apis/okr/v2/objectives/{objective_id}"),
            option,
        )
        .send_v2_response::<ObjectiveData, GetOkrObjectiveResp>()
        .await
    }
    pub async fn key_results_position(
        &self,
        objective_id: &str,
        body: &KeyResultsPositionReqBody,
        option: &RequestOption,
    ) -> Result<KeyResultsPositionOkrObjectiveResp, LarkError> {
        request(
            self.config,
            http::Method::PUT,
            format!("/open-apis/okr/v2/objectives/{objective_id}/key_results_position"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<KeyResultsData, KeyResultsPositionOkrObjectiveResp>()
        .await
    }
    pub async fn key_results_weight(
        &self,
        objective_id: &str,
        body: &KeyResultsWeightReqBody,
        option: &RequestOption,
    ) -> Result<KeyResultsWeightOkrObjectiveResp, LarkError> {
        request(
            self.config,
            http::Method::PUT,
            format!("/open-apis/okr/v2/objectives/{objective_id}/key_results_weight"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<KeyResultsData, KeyResultsWeightOkrObjectiveResp>()
        .await
    }
    pub async fn patch(
        &self,
        objective_id: &str,
        body: &PatchObjectiveReqBody,
        option: &RequestOption,
    ) -> Result<PatchOkrObjectiveResp, LarkError> {
        request(
            self.config,
            http::Method::PATCH,
            format!("/open-apis/okr/v2/objectives/{objective_id}"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<ObjectiveData, PatchOkrObjectiveResp>()
        .await
    }
}

pub struct OkrObjectiveAlignmentResource<'a> {
    config: &'a Config,
}
impl OkrObjectiveAlignmentResource<'_> {
    pub async fn create(
        &self,
        objective_id: &str,
        body: &CreateObjectiveAlignmentReqBody,
        option: &RequestOption,
    ) -> Result<CreateOkrObjectiveAlignmentResp, LarkError> {
        request(
            self.config,
            http::Method::POST,
            format!("/open-apis/okr/v2/objectives/{objective_id}/alignments"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<IdData, CreateOkrObjectiveAlignmentResp>()
        .await
    }
    pub async fn list(
        &self,
        objective_id: &str,
        query: &OkrPageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOkrObjectiveAlignmentResp, LarkError> {
        page_query(
            request(
                self.config,
                http::Method::GET,
                format!("/open-apis/okr/v2/objectives/{objective_id}/alignments"),
                option,
            ),
            query,
        )
        .send_v2_response::<OkrPage<Alignment>, ListOkrObjectiveAlignmentResp>()
        .await
    }
    pub fn list_by_iterator(
        &self,
        objective_id: &str,
        query: &OkrPageQuery<'_>,
    ) -> ListOkrObjectiveAlignmentIterator<'_> {
        ListOkrObjectiveAlignmentIterator::new(self.config, objective_id, query)
    }
}

pub struct OkrObjectiveIndicatorResource<'a> {
    config: &'a Config,
}
impl OkrObjectiveIndicatorResource<'_> {
    pub async fn list(
        &self,
        objective_id: &str,
        option: &RequestOption,
    ) -> Result<ListOkrObjectiveIndicatorResp, LarkError> {
        request(
            self.config,
            http::Method::GET,
            format!("/open-apis/okr/v2/objectives/{objective_id}/indicators"),
            option,
        )
        .send_v2_response::<IndicatorData, ListOkrObjectiveIndicatorResp>()
        .await
    }
}

pub struct OkrObjectiveKeyResultResource<'a> {
    config: &'a Config,
}
impl OkrObjectiveKeyResultResource<'_> {
    pub async fn create(
        &self,
        objective_id: &str,
        body: &CreateObjectiveKeyResultReqBody,
        option: &RequestOption,
    ) -> Result<CreateOkrObjectiveKeyResultResp, LarkError> {
        request(
            self.config,
            http::Method::POST,
            format!("/open-apis/okr/v2/objectives/{objective_id}/key_results"),
            option,
        )
        .json_body(body)?
        .send_v2_response::<IdData, CreateOkrObjectiveKeyResultResp>()
        .await
    }
    pub async fn list(
        &self,
        objective_id: &str,
        query: &OkrPageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOkrObjectiveKeyResultResp, LarkError> {
        page_query(
            request(
                self.config,
                http::Method::GET,
                format!("/open-apis/okr/v2/objectives/{objective_id}/key_results"),
                option,
            ),
            query,
        )
        .send_v2_response::<OkrPage<KeyResult>, ListOkrObjectiveKeyResultResp>()
        .await
    }
    pub fn list_by_iterator(
        &self,
        objective_id: &str,
        query: &OkrPageQuery<'_>,
    ) -> ListOkrObjectiveKeyResultIterator<'_> {
        ListOkrObjectiveKeyResultIterator::new(self.config, objective_id, query)
    }
}

pub struct OkrObjectiveProgressResource<'a> {
    config: &'a Config,
}
impl OkrObjectiveProgressResource<'_> {
    pub async fn list(
        &self,
        objective_id: &str,
        query: &OkrPageQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListOkrObjectiveProgressResp, LarkError> {
        page_query(
            request(
                self.config,
                http::Method::GET,
                format!("/open-apis/okr/v2/objectives/{objective_id}/progresses"),
                option,
            ),
            query,
        )
        .send_v2_response::<OkrPage<Progress>, ListOkrObjectiveProgressResp>()
        .await
    }
    pub fn list_by_iterator(
        &self,
        objective_id: &str,
        query: &OkrPageQuery<'_>,
    ) -> ListOkrObjectiveProgressIterator<'_> {
        ListOkrObjectiveProgressIterator::new(self.config, objective_id, query)
    }
}

#[derive(Debug, Clone)]
struct PageQueryOwned {
    page_size: Option<i32>,
    user_id: Option<String>,
    user_id_type: Option<String>,
}
impl PageQueryOwned {
    fn from_query(query: &OkrPageQuery<'_>) -> Self {
        Self {
            page_size: query.page_size,
            user_id: query.user_id.map(ToOwned::to_owned),
            user_id_type: query.user_id_type.map(ToOwned::to_owned),
        }
    }
    fn query<'a>(&'a self, page_token: Option<&'a str>) -> OkrPageQuery<'a> {
        OkrPageQuery::new()
            .page_size(self.page_size)
            .page_token(page_token)
            .user_id(self.user_id.as_deref())
            .user_id_type(self.user_id_type.as_deref())
    }
}

macro_rules! page_iterator_without_id {
    ($name:ident, $item:ty, $resource:ident, $method:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name<'a> {
            config: &'a Config,
            state: PageIteratorState<$item>,
            query: PageQueryOwned,
        }
        impl<'a> $name<'a> {
            fn new(config: &'a Config, query: &OkrPageQuery<'_>) -> Self {
                Self {
                    config,
                    state: PageIteratorState::default()
                        .with_page_token(query.page_token.map(ToOwned::to_owned)),
                    query: PageQueryOwned::from_query(query),
                }
            }
        }
        impl_page_iterator_controls!($name);
        impl $name<'_> {
            pub async fn next(
                &mut self,
                option: &RequestOption,
            ) -> Result<Option<$item>, LarkError> {
                let config = self.config;
                let query = &self.query;
                self.state
                    .next_page(|token| async move {
                        let response = $resource { config }
                            .$method(&query.query(token.as_deref()), option)
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
    };
}
macro_rules! page_iterator_with_id {
    ($name:ident, $item:ty, $resource:ident, $method:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name<'a> {
            config: &'a Config,
            state: PageIteratorState<$item>,
            id: String,
            query: PageQueryOwned,
        }
        impl<'a> $name<'a> {
            fn new(config: &'a Config, id: &str, query: &OkrPageQuery<'_>) -> Self {
                Self {
                    config,
                    state: PageIteratorState::default()
                        .with_page_token(query.page_token.map(ToOwned::to_owned)),
                    id: id.to_string(),
                    query: PageQueryOwned::from_query(query),
                }
            }
        }
        impl_page_iterator_controls!($name);
        impl $name<'_> {
            pub async fn next(
                &mut self,
                option: &RequestOption,
            ) -> Result<Option<$item>, LarkError> {
                let config = self.config;
                let id = &self.id;
                let query = &self.query;
                self.state
                    .next_page(|token| async move {
                        let response = $resource { config }
                            .$method(id, &query.query(token.as_deref()), option)
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
    };
}
page_iterator_without_id!(ListOkrCategoryIterator, Category, OkrCategoryResource, list);
page_iterator_without_id!(ListOkrCycleIterator, Cycle, OkrCycleResource, list);
page_iterator_with_id!(
    ListOkrCycleObjectiveIterator,
    Objective,
    OkrCycleObjectiveResource,
    list
);
page_iterator_with_id!(
    ListOkrKeyResultProgressIterator,
    Progress,
    OkrKeyResultProgressResource,
    list
);
page_iterator_with_id!(
    ListOkrObjectiveAlignmentIterator,
    Alignment,
    OkrObjectiveAlignmentResource,
    list
);
page_iterator_with_id!(
    ListOkrObjectiveKeyResultIterator,
    KeyResult,
    OkrObjectiveKeyResultResource,
    list
);
page_iterator_with_id!(
    ListOkrObjectiveProgressIterator,
    Progress,
    OkrObjectiveProgressResource,
    list
);

pub struct V2<'a> {
    pub okr_alignment: OkrAlignmentResource<'a>,
    pub okr_category: OkrCategoryResource<'a>,
    pub okr_cycle: OkrCycleResource<'a>,
    pub okr_cycle_objective: OkrCycleObjectiveResource<'a>,
    pub okr_indicator: OkrIndicatorResource<'a>,
    pub okr_key_result: OkrKeyResultResource<'a>,
    pub okr_key_result_indicator: OkrKeyResultIndicatorResource<'a>,
    pub okr_key_result_progress: OkrKeyResultProgressResource<'a>,
    pub okr_objective: OkrObjectiveResource<'a>,
    pub okr_objective_alignment: OkrObjectiveAlignmentResource<'a>,
    pub okr_objective_indicator: OkrObjectiveIndicatorResource<'a>,
    pub okr_objective_key_result: OkrObjectiveKeyResultResource<'a>,
    pub okr_objective_progress: OkrObjectiveProgressResource<'a>,
}
impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            okr_alignment: OkrAlignmentResource { config },
            okr_category: OkrCategoryResource { config },
            okr_cycle: OkrCycleResource { config },
            okr_cycle_objective: OkrCycleObjectiveResource { config },
            okr_indicator: OkrIndicatorResource { config },
            okr_key_result: OkrKeyResultResource { config },
            okr_key_result_indicator: OkrKeyResultIndicatorResource { config },
            okr_key_result_progress: OkrKeyResultProgressResource { config },
            okr_objective: OkrObjectiveResource { config },
            okr_objective_alignment: OkrObjectiveAlignmentResource { config },
            okr_objective_indicator: OkrObjectiveIndicatorResource { config },
            okr_objective_key_result: OkrObjectiveKeyResultResource { config },
            okr_objective_progress: OkrObjectiveProgressResource { config },
        }
    }
}
