use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{EmptyResp, RestRequest};

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<crate::JsonValue>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCardInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<crate::JsonValue>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateCardInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<crate::JsonValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCardReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCardReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<CardContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchUpdateCardReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdConvertCardReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettingsCardReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCardElementReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_element_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateCardElementReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchCardElementReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_element: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentCardElementReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardInstanceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl_resp!(CreateCardInstanceResp, CardInstanceData);

impl_resp_v2!(CreateCardResp, CreateCardRespData);
impl_resp_v2!(UpdateCardResp, ());
impl_resp_v2!(BatchUpdateCardResp, ());
impl_resp_v2!(IdConvertCardResp, IdConvertCardRespData);
impl_resp_v2!(SettingsCardResp, ());
impl_resp_v2!(CreateCardElementResp, ());
impl_resp_v2!(UpdateCardElementResp, ());
impl_resp_v2!(DeleteCardElementResp, ());
impl_resp_v2!(PatchCardElementResp, ());
impl_resp_v2!(ContentCardElementResp, ());

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CreateCardInstanceQuery<'a> {
    pub body: &'a CreateCardInstanceReqBody,
}

impl<'a> CreateCardInstanceQuery<'a> {
    pub fn new(body: &'a CreateCardInstanceReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UpdateCardInstanceQuery<'a> {
    pub instance_id: &'a str,
    pub body: &'a UpdateCardInstanceReqBody,
}

impl<'a> UpdateCardInstanceQuery<'a> {
    pub fn new(instance_id: &'a str, body: &'a UpdateCardInstanceReqBody) -> Self {
        Self { instance_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateCardQuery<'a> {
    pub body: &'a CreateCardReqBody,
}

impl<'a> CreateCardQuery<'a> {
    pub fn new(body: &'a CreateCardReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateCardQuery<'a> {
    pub card_id: &'a str,
    pub body: &'a UpdateCardReqBody,
}

impl<'a> UpdateCardQuery<'a> {
    pub fn new(card_id: &'a str, body: &'a UpdateCardReqBody) -> Self {
        Self { card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BatchUpdateCardQuery<'a> {
    pub card_id: &'a str,
    pub body: &'a BatchUpdateCardReqBody,
}

impl<'a> BatchUpdateCardQuery<'a> {
    pub fn new(card_id: &'a str, body: &'a BatchUpdateCardReqBody) -> Self {
        Self { card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct IdConvertCardQuery<'a> {
    pub body: &'a IdConvertCardReqBody,
}

impl<'a> IdConvertCardQuery<'a> {
    pub fn new(body: &'a IdConvertCardReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SettingsCardQuery<'a> {
    pub card_id: &'a str,
    pub body: &'a SettingsCardReqBody,
}

impl<'a> SettingsCardQuery<'a> {
    pub fn new(card_id: &'a str, body: &'a SettingsCardReqBody) -> Self {
        Self { card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateCardElementQuery<'a> {
    pub card_id: &'a str,
    pub body: &'a CreateCardElementReqBody,
}

impl<'a> CreateCardElementQuery<'a> {
    pub fn new(card_id: &'a str, body: &'a CreateCardElementReqBody) -> Self {
        Self { card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateCardElementQuery<'a> {
    pub card_id: &'a str,
    pub element_id: &'a str,
    pub body: &'a UpdateCardElementReqBody,
}

impl<'a> UpdateCardElementQuery<'a> {
    pub fn new(card_id: &'a str, element_id: &'a str, body: &'a UpdateCardElementReqBody) -> Self {
        Self {
            card_id,
            element_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteCardElementQuery<'a> {
    pub card_id: &'a str,
    pub element_id: &'a str,
}

impl<'a> DeleteCardElementQuery<'a> {
    pub fn new(card_id: &'a str, element_id: &'a str) -> Self {
        Self {
            card_id,
            element_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PatchCardElementQuery<'a> {
    pub card_id: &'a str,
    pub element_id: &'a str,
    pub body: &'a PatchCardElementReqBody,
}

impl<'a> PatchCardElementQuery<'a> {
    pub fn new(card_id: &'a str, element_id: &'a str, body: &'a PatchCardElementReqBody) -> Self {
        Self {
            card_id,
            element_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ContentCardElementQuery<'a> {
    pub card_id: &'a str,
    pub element_id: &'a str,
    pub body: &'a ContentCardElementReqBody,
}

impl<'a> ContentCardElementQuery<'a> {
    pub fn new(card_id: &'a str, element_id: &'a str, body: &'a ContentCardElementReqBody) -> Self {
        Self {
            card_id,
            element_id,
            body,
        }
    }
}

// ── Generated response data ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCardRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdConvertCardRespData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}
// ── Resources ──

pub struct CardInstanceResource<'a> {
    config: &'a Config,
}

impl<'a> CardInstanceResource<'a> {
    pub async fn create(
        &self,
        body: &CreateCardInstanceReqBody,
        option: &RequestOption,
    ) -> Result<CreateCardInstanceResp, LarkError> {
        self.create_by_query(&CreateCardInstanceQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateCardInstanceQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateCardInstanceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/cardkit/v1/card_instances",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<CardInstanceData, CreateCardInstanceResp>()
        .await
    }

    pub async fn update(
        &self,
        instance_id: &str,
        body: &UpdateCardInstanceReqBody,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        self.update_by_query(&UpdateCardInstanceQuery::new(instance_id, body), option)
            .await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateCardInstanceQuery<'_>,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let path = format!("/open-apis/cardkit/v1/card_instances/{}", query.instance_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_empty()
        .await
    }
}

pub struct CardResource<'a> {
    config: &'a Config,
}

impl CardResource<'_> {
    pub async fn create(
        &self,
        body: &CreateCardReqBody,
        option: &RequestOption,
    ) -> Result<CreateCardResp, LarkError> {
        self.create_by_query(&CreateCardQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateCardQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateCardResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/cardkit/v1/cards",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<CreateCardRespData, CreateCardResp>()
        .await
    }

    pub async fn update(
        &self,
        card_id: &str,
        body: &UpdateCardReqBody,
        option: &RequestOption,
    ) -> Result<UpdateCardResp, LarkError> {
        self.update_by_query(&UpdateCardQuery::new(card_id, body), option)
            .await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateCardQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateCardResp, LarkError> {
        let path = format!("/open-apis/cardkit/v1/cards/{}", query.card_id);
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), UpdateCardResp>()
        .await
    }

    pub async fn batch_update(
        &self,
        card_id: &str,
        body: &BatchUpdateCardReqBody,
        option: &RequestOption,
    ) -> Result<BatchUpdateCardResp, LarkError> {
        self.batch_update_by_query(&BatchUpdateCardQuery::new(card_id, body), option)
            .await
    }

    pub async fn batch_update_by_query(
        &self,
        query: &BatchUpdateCardQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchUpdateCardResp, LarkError> {
        let path = format!("/open-apis/cardkit/v1/cards/{}/batch_update", query.card_id);
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), BatchUpdateCardResp>()
        .await
    }

    pub async fn id_convert(
        &self,
        body: &IdConvertCardReqBody,
        option: &RequestOption,
    ) -> Result<IdConvertCardResp, LarkError> {
        self.id_convert_by_query(&IdConvertCardQuery::new(body), option)
            .await
    }

    pub async fn id_convert_by_query(
        &self,
        query: &IdConvertCardQuery<'_>,
        option: &RequestOption,
    ) -> Result<IdConvertCardResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/cardkit/v1/cards/id_convert",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<IdConvertCardRespData, IdConvertCardResp>()
        .await
    }

    pub async fn settings(
        &self,
        card_id: &str,
        body: &SettingsCardReqBody,
        option: &RequestOption,
    ) -> Result<SettingsCardResp, LarkError> {
        self.settings_by_query(&SettingsCardQuery::new(card_id, body), option)
            .await
    }

    pub async fn settings_by_query(
        &self,
        query: &SettingsCardQuery<'_>,
        option: &RequestOption,
    ) -> Result<SettingsCardResp, LarkError> {
        let path = format!("/open-apis/cardkit/v1/cards/{}/settings", query.card_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), SettingsCardResp>()
        .await
    }
}

pub struct CardElementResource<'a> {
    config: &'a Config,
}

impl CardElementResource<'_> {
    pub async fn create(
        &self,
        card_id: &str,
        body: &CreateCardElementReqBody,
        option: &RequestOption,
    ) -> Result<CreateCardElementResp, LarkError> {
        self.create_by_query(&CreateCardElementQuery::new(card_id, body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateCardElementQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateCardElementResp, LarkError> {
        let path = format!("/open-apis/cardkit/v1/cards/{}/elements", query.card_id);
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), CreateCardElementResp>()
        .await
    }

    pub async fn update(
        &self,
        card_id: &str,
        element_id: &str,
        body: &UpdateCardElementReqBody,
        option: &RequestOption,
    ) -> Result<UpdateCardElementResp, LarkError> {
        self.update_by_query(
            &UpdateCardElementQuery::new(card_id, element_id, body),
            option,
        )
        .await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateCardElementQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateCardElementResp, LarkError> {
        let path = format!(
            "/open-apis/cardkit/v1/cards/{}/elements/{}",
            query.card_id, query.element_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), UpdateCardElementResp>()
        .await
    }

    pub async fn delete(
        &self,
        card_id: &str,
        element_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCardElementResp, LarkError> {
        self.delete_by_query(&DeleteCardElementQuery::new(card_id, element_id), option)
            .await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteCardElementQuery<'_>,
        option: &RequestOption,
    ) -> Result<DeleteCardElementResp, LarkError> {
        let path = format!(
            "/open-apis/cardkit/v1/cards/{}/elements/{}",
            query.card_id, query.element_id
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteCardElementResp>()
        .await
    }

    pub async fn patch(
        &self,
        card_id: &str,
        element_id: &str,
        body: &PatchCardElementReqBody,
        option: &RequestOption,
    ) -> Result<PatchCardElementResp, LarkError> {
        self.patch_by_query(
            &PatchCardElementQuery::new(card_id, element_id, body),
            option,
        )
        .await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchCardElementQuery<'_>,
        option: &RequestOption,
    ) -> Result<PatchCardElementResp, LarkError> {
        let path = format!(
            "/open-apis/cardkit/v1/cards/{}/elements/{}",
            query.card_id, query.element_id
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), PatchCardElementResp>()
        .await
    }

    pub async fn content(
        &self,
        card_id: &str,
        element_id: &str,
        body: &ContentCardElementReqBody,
        option: &RequestOption,
    ) -> Result<ContentCardElementResp, LarkError> {
        self.content_by_query(
            &ContentCardElementQuery::new(card_id, element_id, body),
            option,
        )
        .await
    }

    pub async fn content_by_query(
        &self,
        query: &ContentCardElementQuery<'_>,
        option: &RequestOption,
    ) -> Result<ContentCardElementResp, LarkError> {
        let path = format!(
            "/open-apis/cardkit/v1/cards/{}/elements/{}/content",
            query.card_id, query.element_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), ContentCardElementResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub card_instance: CardInstanceResource<'a>,
    pub card: CardResource<'a>,
    pub card_element: CardElementResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            card_instance: CardInstanceResource { config },
            card: CardResource { config },
            card_element: CardElementResource { config },
        }
    }
}
