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
    pub template_variable: Option<serde_json::Value>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateCardInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateCardInstanceReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_variable: Option<serde_json::Value>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardInstanceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl_resp!(CreateCardInstanceResp, CardInstanceData);

impl_resp_v2!(CreateCardResp, serde_json::Value);
impl_resp_v2!(UpdateCardResp, serde_json::Value);
impl_resp_v2!(BatchUpdateCardResp, serde_json::Value);
impl_resp_v2!(IdConvertCardResp, serde_json::Value);
impl_resp_v2!(SettingsCardResp, serde_json::Value);
impl_resp_v2!(CreateCardElementResp, serde_json::Value);
impl_resp_v2!(UpdateCardElementResp, serde_json::Value);
impl_resp_v2!(DeleteCardElementResp, ());
impl_resp_v2!(PatchCardElementResp, serde_json::Value);
impl_resp_v2!(ContentCardElementResp, serde_json::Value);

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
    pub body: &'a serde_json::Value,
}

impl<'a> CreateCardQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateCardQuery<'a> {
    pub card_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> UpdateCardQuery<'a> {
    pub fn new(card_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct BatchUpdateCardQuery<'a> {
    pub card_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> BatchUpdateCardQuery<'a> {
    pub fn new(card_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct IdConvertCardQuery<'a> {
    pub body: &'a serde_json::Value,
}

impl<'a> IdConvertCardQuery<'a> {
    pub fn new(body: &'a serde_json::Value) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct SettingsCardQuery<'a> {
    pub card_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> SettingsCardQuery<'a> {
    pub fn new(card_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateCardElementQuery<'a> {
    pub card_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> CreateCardElementQuery<'a> {
    pub fn new(card_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self { card_id, body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateCardElementQuery<'a> {
    pub card_id: &'a str,
    pub element_id: &'a str,
    pub body: &'a serde_json::Value,
}

impl<'a> UpdateCardElementQuery<'a> {
    pub fn new(card_id: &'a str, element_id: &'a str, body: &'a serde_json::Value) -> Self {
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
    pub body: &'a serde_json::Value,
}

impl<'a> PatchCardElementQuery<'a> {
    pub fn new(card_id: &'a str, element_id: &'a str, body: &'a serde_json::Value) -> Self {
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
    pub body: &'a serde_json::Value,
}

impl<'a> ContentCardElementQuery<'a> {
    pub fn new(card_id: &'a str, element_id: &'a str, body: &'a serde_json::Value) -> Self {
        Self {
            card_id,
            element_id,
            body,
        }
    }
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
        body: &serde_json::Value,
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/cardkit/v1/cards",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateCardResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        card_id: &str,
        body: &serde_json::Value,
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateCardResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn batch_update(
        &self,
        card_id: &str,
        body: &serde_json::Value,
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(BatchUpdateCardResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn id_convert(
        &self,
        body: &serde_json::Value,
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/cardkit/v1/cards/id_convert",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(IdConvertCardResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn settings(
        &self,
        card_id: &str,
        body: &serde_json::Value,
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
        Ok(SettingsCardResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct CardElementResource<'a> {
    config: &'a Config,
}

impl CardElementResource<'_> {
    pub async fn create(
        &self,
        card_id: &str,
        body: &serde_json::Value,
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(CreateCardElementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn update(
        &self,
        card_id: &str,
        element_id: &str,
        body: &serde_json::Value,
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(UpdateCardElementResp {
            api_resp,
            code_error,
            data,
        })
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2::<()>()
        .await?;
        Ok(DeleteCardElementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn patch(
        &self,
        card_id: &str,
        element_id: &str,
        body: &serde_json::Value,
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
        Ok(PatchCardElementResp {
            api_resp,
            code_error,
            data,
        })
    }

    pub async fn content(
        &self,
        card_id: &str,
        element_id: &str,
        body: &serde_json::Value,
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
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2::<serde_json::Value>()
        .await?;
        Ok(ContentCardElementResp {
            api_resp,
            code_error,
            data,
        })
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
