//! Typed CardKit helpers for validated Card JSON 2.0 documents.
//!
//! The generated CardKit resource remains available through
//! [`LarkClient::cardkit`](crate::LarkClient::cardkit). This module removes the
//! stringly-typed `card_json` envelope for a Card JSON 2.0 document and owns
//! sequence progression for text-content streaming updates.

use crate::JsonValue;
use crate::LarkClient;
use crate::card::v2;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::cardkit::v1::{
    BatchUpdateCardReqBody, BatchUpdateCardResp, ContentCardElementReqBody, ContentCardElementResp,
    CreateCardElementReqBody, CreateCardElementResp, CreateCardInstanceReqBody,
    CreateCardInstanceResp, CreateCardReqBody, CreateCardResp, DeleteCardElementReqBody,
    DeleteCardElementResp, PatchCardElementReqBody, PatchCardElementResp, SettingsCardReqBody,
    SettingsCardResp, UpdateCardElementReqBody, UpdateCardElementResp, UpdateCardInstanceReqBody,
    UpdateCardReqBody, UpdateCardResp,
};
use crate::service::common::EmptyResp;

/// A validated Card JSON 2.0 document that can be sent through CardKit.
#[derive(Debug, Clone)]
pub struct CardDocument {
    card: v2::Card,
}

impl CardDocument {
    /// Validate and retain a Card JSON 2.0 document for CardKit transport.
    pub fn new(card: v2::Card) -> Result<Self, v2::ValidationError> {
        card.validate()?;
        Ok(Self { card })
    }

    /// Access the validated Card JSON 2.0 document.
    pub fn card(&self) -> &v2::Card {
        &self.card
    }

    /// Consume this wrapper and return its Card JSON 2.0 document.
    pub fn into_card(self) -> v2::Card {
        self.card
    }

    /// Build the raw request body expected by CardKit's create-card endpoint.
    pub fn create_request(&self) -> Result<CreateCardReqBody, LarkError> {
        Ok(CreateCardReqBody {
            r#type: Some("card_json".to_string()),
            data: Some(self.encoded_data()?),
        })
    }

    /// Build the raw request body expected by CardKit's full-update endpoint.
    pub fn update_request(
        &self,
        metadata: &CardUpdateMetadata,
    ) -> Result<UpdateCardReqBody, LarkError> {
        Ok(UpdateCardReqBody {
            card: Some(crate::service::cardkit::v1::CardContent {
                r#type: Some("card_json".to_string()),
                data: Some(self.encoded_data()?),
            }),
            uuid: Some(metadata.idempotency_key.as_str().to_string()),
            sequence: Some(metadata.sequence.get()),
        })
    }

    fn encoded_data(&self) -> Result<String, LarkError> {
        serde_json::to_string(&self.card).map_err(LarkError::Json)
    }
}

impl TryFrom<v2::Card> for CardDocument {
    type Error = v2::ValidationError;

    fn try_from(card: v2::Card) -> Result<Self, Self::Error> {
        Self::new(card)
    }
}

/// An idempotency key supplied with one CardKit mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// Create a non-empty CardKit idempotency key.
    ///
    /// Lark documents UUIDs for this field. The SDK preserves the caller's key
    /// verbatim so applications can use their existing idempotency-key format.
    pub fn new(value: impl Into<String>) -> Result<Self, LarkError> {
        let value = value.into();
        if value.is_empty() {
            return Err(LarkError::IllegalParam(
                "CardKit idempotency key must not be empty".to_string(),
            ));
        }
        Ok(Self(value))
    }

    /// Return the key sent on the wire as `uuid`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A positive, CardKit update sequence number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpdateSequence(i32);

impl UpdateSequence {
    /// The first valid update sequence number.
    pub const FIRST: Self = Self(1);

    /// Validate a CardKit sequence number.
    pub fn new(value: i32) -> Result<Self, LarkError> {
        if value < 1 {
            return Err(LarkError::IllegalParam(
                "CardKit sequence must be a positive i32".to_string(),
            ));
        }
        Ok(Self(value))
    }

    /// Return the integer sent on the wire as `sequence`.
    pub fn get(self) -> i32 {
        self.0
    }

    fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }
}

/// Idempotency and ordering metadata for one CardKit full-card update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardUpdateMetadata {
    /// Unique caller-supplied idempotency key for this update.
    pub idempotency_key: IdempotencyKey,
    /// Positive sequence number for this update.
    pub sequence: UpdateSequence,
}

impl CardUpdateMetadata {
    /// Combine an idempotency key with a positive update sequence.
    pub fn new(idempotency_key: IdempotencyKey, sequence: UpdateSequence) -> Self {
        Self {
            idempotency_key,
            sequence,
        }
    }
}

/// The Card JSON 2.0 root fields accepted by CardKit's settings endpoint.
#[derive(Debug, Clone, Default)]
pub struct CardSettings {
    config: Option<v2::Config>,
    card_link: Option<v2::MultiUrl>,
}

impl CardSettings {
    /// Start an update for Card JSON 2.0 `config` and `card_link` fields.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace the card configuration.
    pub fn config(mut self, config: v2::Config) -> Self {
        self.config = Some(config);
        self
    }

    /// Replace the card's multi-platform link.
    pub fn card_link(mut self, card_link: v2::MultiUrl) -> Self {
        self.card_link = Some(card_link);
        self
    }

    /// Validate the settings update before sending it to CardKit.
    pub fn validate(&self) -> Result<(), LarkError> {
        if self.config.is_none() && self.card_link.is_none() {
            return Err(LarkError::IllegalParam(
                "CardKit settings must update config or card_link".to_string(),
            ));
        }
        if self
            .card_link
            .as_ref()
            .is_some_and(|link| link.url.as_deref().is_none_or(str::is_empty))
        {
            return Err(LarkError::IllegalParam(
                "CardKit settings card_link requires a non-empty url".to_string(),
            ));
        }
        if self.config.as_ref().is_some_and(|config| {
            config.streaming_config.is_some() && config.streaming_mode != Some(true)
        }) {
            return Err(LarkError::IllegalParam(
                "CardKit settings streaming_config requires streaming_mode".to_string(),
            ));
        }
        if self
            .config
            .as_ref()
            .is_some_and(|config| config.update_multi == Some(false))
        {
            return Err(LarkError::IllegalParam(
                "CardKit settings do not support update_multi: false".to_string(),
            ));
        }
        Ok(())
    }

    fn encoded_data(&self) -> Result<String, LarkError> {
        serde_json::to_string(&self.json_data()?).map_err(LarkError::Json)
    }

    fn json_data(&self) -> Result<serde_json::Value, LarkError> {
        self.validate()?;
        serde_json::to_value(CardSettingsPayload {
            config: self.config.as_ref(),
            card_link: self.card_link.as_ref(),
        })
        .map_err(LarkError::Json)
    }
}

#[derive(serde::Serialize)]
struct CardSettingsPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<&'a v2::Config>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_link: Option<&'a v2::MultiUrl>,
}

/// A typed Card JSON 2.0 element encoded for a CardKit mutation.
///
/// Element placement is determined by the destination card or container, so the
/// server remains responsible for placement-only constraints.
#[derive(Debug, Clone)]
pub struct CardElement {
    element: v2::Element,
}

impl CardElement {
    /// Retain a typed Card JSON 2.0 element for CardKit transport.
    pub fn new(element: v2::Element) -> Self {
        Self { element }
    }

    /// Access the typed element.
    pub fn element(&self) -> &v2::Element {
        &self.element
    }

    fn encoded_data(&self) -> Result<String, LarkError> {
        serde_json::to_string(&self.json_data()?).map_err(LarkError::Json)
    }

    fn json_data(&self) -> Result<serde_json::Value, LarkError> {
        serde_json::to_value(&self.element).map_err(LarkError::Json)
    }
}

/// A partial Card JSON 2.0 element update.
#[derive(Debug, Clone)]
pub struct PartialCardElement {
    fields: JsonValue,
}

impl PartialCardElement {
    /// Validate a non-empty partial element update that leaves its `tag` unchanged.
    pub fn new(fields: JsonValue) -> Result<Self, LarkError> {
        let object = fields.as_value().as_object().ok_or_else(|| {
            LarkError::IllegalParam("CardKit partial element must be a JSON object".to_string())
        })?;
        if object.is_empty() {
            return Err(LarkError::IllegalParam(
                "CardKit partial element must not be empty".to_string(),
            ));
        }
        if object.contains_key("tag") {
            return Err(LarkError::IllegalParam(
                "CardKit partial element must not update tag".to_string(),
            ));
        }
        Ok(Self { fields })
    }

    fn encoded_data(&self) -> Result<String, LarkError> {
        serde_json::to_string(&self.json_data()?).map_err(LarkError::Json)
    }

    fn json_data(&self) -> Result<serde_json::Value, LarkError> {
        Ok(self.fields.as_value().clone())
    }
}

/// Placement for elements added through CardKit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementInsertionPosition {
    /// Insert before the target element.
    InsertBefore,
    /// Insert after the target element.
    InsertAfter,
    /// Append to the card body or target container.
    Append,
}

impl ElementInsertionPosition {
    fn wire_value(self) -> &'static str {
        match self {
            Self::InsertBefore => "insert_before",
            Self::InsertAfter => "insert_after",
            Self::Append => "append",
        }
    }
}

/// A non-empty ordered insertion of Card JSON 2.0 elements.
#[derive(Debug, Clone)]
pub struct ElementInsertion {
    position: ElementInsertionPosition,
    target_element_id: Option<String>,
    elements: Vec<CardElement>,
}

impl ElementInsertion {
    /// Insert elements at the requested position.
    pub fn new(
        position: ElementInsertionPosition,
        elements: impl IntoIterator<Item = CardElement>,
    ) -> Result<Self, LarkError> {
        let elements: Vec<_> = elements.into_iter().collect();
        if elements.is_empty() {
            return Err(LarkError::IllegalParam(
                "CardKit element insertion requires an element".to_string(),
            ));
        }
        Ok(Self {
            position,
            target_element_id: None,
            elements,
        })
    }

    /// Target an element for before/after insertion or append to a container.
    pub fn target_element_id(mut self, element_id: impl Into<String>) -> Result<Self, LarkError> {
        let element_id = element_id.into();
        validate_element_id("CardKit target_element_id", &element_id)?;
        self.target_element_id = Some(element_id);
        Ok(self)
    }

    /// Validate the placement requirements for this insertion.
    pub fn validate(&self) -> Result<(), LarkError> {
        if matches!(
            self.position,
            ElementInsertionPosition::InsertBefore | ElementInsertionPosition::InsertAfter
        ) && self.target_element_id.is_none()
        {
            return Err(LarkError::IllegalParam(
                "CardKit insert_before and insert_after require target_element_id".to_string(),
            ));
        }
        Ok(())
    }

    fn encoded_elements(&self) -> Result<String, LarkError> {
        serde_json::to_string(&self.json_elements()?).map_err(LarkError::Json)
    }

    fn json_elements(&self) -> Result<Vec<serde_json::Value>, LarkError> {
        self.validate()?;
        self.elements.iter().map(CardElement::json_data).collect()
    }
}

/// One validated typed CardKit action for an ordered batch update.
#[derive(Debug, Clone)]
pub struct CardBatchAction {
    action: CardBatchActionKind,
}

#[derive(Debug, Clone)]
enum CardBatchActionKind {
    /// Update Card JSON 2.0 `config` or `card_link` fields.
    UpdateSettings(Box<CardSettings>),
    /// Add one or more typed elements.
    AddElements(ElementInsertion),
    /// Delete elements by their identifiers.
    DeleteElements(Vec<String>),
    /// Patch one element without changing its `tag`.
    PatchElement {
        /// The target element identifier.
        element_id: String,
        /// The fields to replace.
        patch: PartialCardElement,
    },
    /// Replace one element with a typed full element.
    ReplaceElement {
        /// The target element identifier.
        element_id: String,
        /// The replacement element.
        element: Box<CardElement>,
    },
}

impl CardBatchAction {
    /// Create a batch settings update.
    pub fn update_settings(settings: CardSettings) -> Result<Self, LarkError> {
        settings.validate()?;
        Ok(Self {
            action: CardBatchActionKind::UpdateSettings(Box::new(settings)),
        })
    }

    /// Create a batch element insertion.
    pub fn add_elements(insertion: ElementInsertion) -> Result<Self, LarkError> {
        insertion.validate()?;
        Ok(Self {
            action: CardBatchActionKind::AddElements(insertion),
        })
    }

    /// Create a batch deletion for non-empty element identifiers.
    pub fn delete_elements(
        element_ids: impl IntoIterator<Item = String>,
    ) -> Result<Self, LarkError> {
        let element_ids: Vec<_> = element_ids.into_iter().collect();
        if element_ids.is_empty() {
            return Err(LarkError::IllegalParam(
                "CardKit batch deletion requires an element_id".to_string(),
            ));
        }
        for element_id in &element_ids {
            validate_element_id("CardKit element_id", element_id)?;
        }
        Ok(Self {
            action: CardBatchActionKind::DeleteElements(element_ids),
        })
    }

    /// Create a batch partial-element update.
    pub fn patch_element(
        element_id: impl Into<String>,
        patch: PartialCardElement,
    ) -> Result<Self, LarkError> {
        let element_id = element_id.into();
        validate_element_id("CardKit element_id", &element_id)?;
        Ok(Self {
            action: CardBatchActionKind::PatchElement { element_id, patch },
        })
    }

    /// Create a batch full-element replacement.
    pub fn replace_element(
        element_id: impl Into<String>,
        element: CardElement,
    ) -> Result<Self, LarkError> {
        let element_id = element_id.into();
        validate_element_id("CardKit element_id", &element_id)?;
        Ok(Self {
            action: CardBatchActionKind::ReplaceElement {
                element_id,
                element: Box::new(element),
            },
        })
    }

    fn encoded_data(&self) -> Result<serde_json::Value, LarkError> {
        match &self.action {
            CardBatchActionKind::UpdateSettings(settings) => Ok(serde_json::json!({
                "action": "partial_update_setting",
                "params": {"settings": settings.json_data()?},
            })),
            CardBatchActionKind::AddElements(insertion) => {
                let mut params = serde_json::json!({
                    "type": insertion.position.wire_value(),
                    "elements": insertion.json_elements()?,
                });
                if let Some(target_element_id) = &insertion.target_element_id {
                    params["target_element_id"] = serde_json::json!(target_element_id);
                }
                Ok(serde_json::json!({
                    "action": "add_elements",
                    "params": params,
                }))
            }
            CardBatchActionKind::DeleteElements(element_ids) => Ok(serde_json::json!({
                "action": "delete_elements",
                "params": {"element_ids": element_ids},
            })),
            CardBatchActionKind::PatchElement { element_id, patch } => Ok(serde_json::json!({
                "action": "partial_update_element",
                "params": {
                    "element_id": element_id,
                    "partial_element": patch.json_data()?,
                },
            })),
            CardBatchActionKind::ReplaceElement {
                element_id,
                element,
            } => Ok(serde_json::json!({
                "action": "update_element",
                "params": {
                    "element_id": element_id,
                    "element": element.json_data()?,
                },
            })),
        }
    }
}

/// A CardKit template and its open-ended template variables.
#[derive(Debug, Clone)]
pub struct CardTemplate {
    template_id: String,
    variables: Option<TemplateVariables>,
}

/// An object of open-ended values supplied to a CardKit template.
#[derive(Debug, Clone)]
pub struct TemplateVariables(JsonValue);

impl TemplateVariables {
    /// Retain a name-keyed template-variable object.
    pub fn new(value: JsonValue) -> Result<Self, LarkError> {
        if !value.as_value().is_object() {
            return Err(LarkError::IllegalParam(
                "CardKit template variables must be a JSON object".to_string(),
            ));
        }
        Ok(Self(value))
    }

    /// Access the open-ended JSON object sent as `template_variable`.
    pub fn as_json(&self) -> &JsonValue {
        &self.0
    }

    /// Consume this wrapper and return its JSON object.
    pub fn into_json(self) -> JsonValue {
        self.0
    }
}

impl CardTemplate {
    /// Select a non-empty CardKit template.
    pub fn new(template_id: impl Into<String>) -> Result<Self, LarkError> {
        let template_id = template_id.into();
        validate_identifier("CardKit template_id", &template_id)?;
        Ok(Self {
            template_id,
            variables: None,
        })
    }

    /// Supply the template's documented open-ended variables.
    pub fn variables(mut self, variables: TemplateVariables) -> Self {
        self.variables = Some(variables);
        self
    }

    fn create_request(&self) -> CreateCardInstanceReqBody {
        CreateCardInstanceReqBody {
            template_id: Some(self.template_id.clone()),
            template_variable: self
                .variables
                .as_ref()
                .map(TemplateVariables::as_json)
                .cloned(),
        }
    }
}

/// Typed CardKit operations for Card JSON 2.0 documents.
pub struct CardKitCards<'a> {
    client: &'a LarkClient,
}

impl<'a> CardKitCards<'a> {
    fn new(client: &'a LarkClient) -> Self {
        Self { client }
    }

    /// Create a CardKit card entity from a validated Card JSON 2.0 document.
    pub async fn create(
        &self,
        document: &CardDocument,
        option: &RequestOption,
    ) -> Result<CreateCardResp, LarkError> {
        let request = document.create_request()?;
        self.client.cardkit().card.create(&request, option).await
    }

    /// Create a CardKit template instance.
    pub async fn create_instance(
        &self,
        template: &CardTemplate,
        option: &RequestOption,
    ) -> Result<CreateCardInstanceResp, LarkError> {
        let request = template.create_request();
        self.client
            .cardkit()
            .card_instance
            .create(&request, option)
            .await
    }

    /// Update a CardKit template instance's open-ended variables.
    pub async fn update_instance(
        &self,
        instance_id: impl Into<String>,
        variables: TemplateVariables,
        option: &RequestOption,
    ) -> Result<EmptyResp, LarkError> {
        let instance_id = instance_id.into();
        validate_identifier("CardKit instance_id", &instance_id)?;
        let request = UpdateCardInstanceReqBody {
            template_variable: Some(variables.into_json()),
        };
        self.client
            .cardkit()
            .card_instance
            .update(&instance_id, &request, option)
            .await
    }

    /// Start ordered CardKit mutations for one card entity.
    pub fn update_session(
        &self,
        card_id: impl Into<String>,
    ) -> Result<CardKitUpdateSession<'a>, LarkError> {
        CardKitUpdateSession::new(self.client, card_id, UpdateSequence::FIRST)
    }

    /// Resume ordered CardKit mutations at a caller-supplied next sequence number.
    pub fn resume_update_session(
        &self,
        card_id: impl Into<String>,
        next_sequence: UpdateSequence,
    ) -> Result<CardKitUpdateSession<'a>, LarkError> {
        CardKitUpdateSession::new(self.client, card_id, next_sequence)
    }
}

/// Stateful ordered CardKit mutations for one card entity.
///
/// Both full-document and content updates advance the same sequence only after
/// their request succeeds. Use one session for all updates to one card; resume
/// it with the next known sequence after a process restart or external update.
pub struct CardKitUpdateSession<'a> {
    client: &'a LarkClient,
    card_id: String,
    next_sequence: Option<UpdateSequence>,
}

impl<'a> CardKitUpdateSession<'a> {
    fn new(
        client: &'a LarkClient,
        card_id: impl Into<String>,
        next_sequence: UpdateSequence,
    ) -> Result<Self, LarkError> {
        let card_id = card_id.into();
        validate_identifier("CardKit card_id", &card_id)?;
        Ok(Self {
            client,
            card_id,
            next_sequence: Some(next_sequence),
        })
    }

    /// Return the sequence that the next successful update will use.
    pub fn next_sequence(&self) -> Option<UpdateSequence> {
        self.next_sequence
    }

    /// Fully replace this card with a validated Card JSON 2.0 document.
    pub async fn replace_document(
        &mut self,
        document: &CardDocument,
        idempotency_key: &IdempotencyKey,
        option: &RequestOption,
    ) -> Result<UpdateCardResp, LarkError> {
        let sequence = self.current_sequence()?;
        let metadata = CardUpdateMetadata::new(idempotency_key.clone(), sequence);
        let request = document.update_request(&metadata)?;
        let response = self
            .client
            .cardkit()
            .card
            .update(&self.card_id, &request, option)
            .await?;
        self.advance_sequence(sequence);
        Ok(response)
    }

    /// Update the Card JSON 2.0 settings fields supported by CardKit.
    pub async fn update_settings(
        &mut self,
        settings: &CardSettings,
        idempotency_key: &IdempotencyKey,
        option: &RequestOption,
    ) -> Result<SettingsCardResp, LarkError> {
        let sequence = self.current_sequence()?;
        let request = SettingsCardReqBody {
            settings: Some(settings.encoded_data()?),
            uuid: Some(idempotency_key.as_str().to_string()),
            sequence: Some(sequence.get()),
        };
        let response = self
            .client
            .cardkit()
            .card
            .settings(&self.card_id, &request, option)
            .await?;
        self.advance_sequence(sequence);
        Ok(response)
    }

    /// Insert typed Card JSON 2.0 elements at the requested position.
    pub async fn insert_elements(
        &mut self,
        insertion: &ElementInsertion,
        idempotency_key: &IdempotencyKey,
        option: &RequestOption,
    ) -> Result<CreateCardElementResp, LarkError> {
        let sequence = self.current_sequence()?;
        let request = CreateCardElementReqBody {
            r#type: Some(insertion.position.wire_value().to_string()),
            target_element_id: insertion.target_element_id.clone(),
            uuid: Some(idempotency_key.as_str().to_string()),
            sequence: Some(sequence.get()),
            elements: Some(insertion.encoded_elements()?),
        };
        let response = self
            .client
            .cardkit()
            .card_element
            .create(&self.card_id, &request, option)
            .await?;
        self.advance_sequence(sequence);
        Ok(response)
    }

    /// Replace one element with a typed Card JSON 2.0 element.
    pub async fn replace_element(
        &mut self,
        element_id: impl Into<String>,
        element: &CardElement,
        idempotency_key: &IdempotencyKey,
        option: &RequestOption,
    ) -> Result<UpdateCardElementResp, LarkError> {
        let element_id = element_id.into();
        validate_element_id("CardKit element_id", &element_id)?;
        let sequence = self.current_sequence()?;
        let request = UpdateCardElementReqBody {
            uuid: Some(idempotency_key.as_str().to_string()),
            element: Some(element.encoded_data()?),
            sequence: Some(sequence.get()),
        };
        let response = self
            .client
            .cardkit()
            .card_element
            .update(&self.card_id, &element_id, &request, option)
            .await?;
        self.advance_sequence(sequence);
        Ok(response)
    }

    /// Patch one element without changing its Card JSON tag.
    pub async fn patch_element(
        &mut self,
        element_id: impl Into<String>,
        patch: &PartialCardElement,
        idempotency_key: &IdempotencyKey,
        option: &RequestOption,
    ) -> Result<PatchCardElementResp, LarkError> {
        let element_id = element_id.into();
        validate_element_id("CardKit element_id", &element_id)?;
        let sequence = self.current_sequence()?;
        let request = PatchCardElementReqBody {
            partial_element: Some(patch.encoded_data()?),
            uuid: Some(idempotency_key.as_str().to_string()),
            sequence: Some(sequence.get()),
        };
        let response = self
            .client
            .cardkit()
            .card_element
            .patch(&self.card_id, &element_id, &request, option)
            .await?;
        self.advance_sequence(sequence);
        Ok(response)
    }

    /// Delete one element as an ordered CardKit mutation.
    pub async fn delete_element(
        &mut self,
        element_id: impl Into<String>,
        idempotency_key: &IdempotencyKey,
        option: &RequestOption,
    ) -> Result<DeleteCardElementResp, LarkError> {
        let element_id = element_id.into();
        validate_element_id("CardKit element_id", &element_id)?;
        let sequence = self.current_sequence()?;
        let request = DeleteCardElementReqBody {
            uuid: Some(idempotency_key.as_str().to_string()),
            sequence: Some(sequence.get()),
        };
        let response = self
            .client
            .cardkit()
            .card_element
            .delete_with_body(&self.card_id, &element_id, &request, option)
            .await?;
        self.advance_sequence(sequence);
        Ok(response)
    }

    /// Apply typed settings and element actions atomically at one sequence number.
    pub async fn batch_update(
        &mut self,
        actions: impl IntoIterator<Item = CardBatchAction>,
        idempotency_key: &IdempotencyKey,
        option: &RequestOption,
    ) -> Result<BatchUpdateCardResp, LarkError> {
        let actions: Vec<_> = actions.into_iter().collect();
        if actions.is_empty() {
            return Err(LarkError::IllegalParam(
                "CardKit batch update requires an action".to_string(),
            ));
        }
        let sequence = self.current_sequence()?;
        let actions: Result<Vec<_>, _> =
            actions.iter().map(CardBatchAction::encoded_data).collect();
        let request = BatchUpdateCardReqBody {
            uuid: Some(idempotency_key.as_str().to_string()),
            sequence: Some(sequence.get()),
            actions: Some(serde_json::to_string(&actions?)?),
        };
        let response = self
            .client
            .cardkit()
            .card
            .batch_update(&self.card_id, &request, option)
            .await?;
        self.advance_sequence(sequence);
        Ok(response)
    }

    /// Start ordered full-content updates for one CardKit text or Markdown element.
    pub fn content_stream(
        &mut self,
        element_id: impl Into<String>,
    ) -> Result<CardContentStream<'_, 'a>, LarkError> {
        CardContentStream::new(self, element_id)
    }

    fn current_sequence(&self) -> Result<UpdateSequence, LarkError> {
        self.next_sequence
            .ok_or_else(|| LarkError::IllegalParam("CardKit sequence is exhausted".to_string()))
    }

    fn advance_sequence(&mut self, sequence: UpdateSequence) {
        self.next_sequence = sequence.next();
    }
}

/// Ordered full-content updates borrowed from one [`CardKitUpdateSession`].
pub struct CardContentStream<'session, 'client> {
    session: &'session mut CardKitUpdateSession<'client>,
    element_id: String,
}

impl<'session, 'client> CardContentStream<'session, 'client> {
    fn new(
        session: &'session mut CardKitUpdateSession<'client>,
        element_id: impl Into<String>,
    ) -> Result<Self, LarkError> {
        let element_id = element_id.into();
        validate_element_id("CardKit element_id", &element_id)?;
        Ok(Self {
            session,
            element_id,
        })
    }

    /// Return the sequence that the next successful update will use.
    pub fn next_sequence(&self) -> Option<UpdateSequence> {
        self.session.next_sequence()
    }

    /// Send a complete new text value for the configured element.
    ///
    /// If the new content extends the prior content by prefix, Lark renders the
    /// suffix as a typewriter update; otherwise it replaces the content.
    pub async fn replace(
        &mut self,
        content: impl Into<String>,
        idempotency_key: &IdempotencyKey,
        option: &RequestOption,
    ) -> Result<ContentCardElementResp, LarkError> {
        let sequence = self.session.current_sequence()?;
        let request = ContentCardElementReqBody {
            uuid: Some(idempotency_key.as_str().to_string()),
            content: Some(content.into()),
            sequence: Some(sequence.get()),
        };
        let response = self
            .session
            .client
            .cardkit()
            .card_element
            .content(&self.session.card_id, &self.element_id, &request, option)
            .await?;
        self.session.advance_sequence(sequence);
        Ok(response)
    }
}

impl LarkClient {
    /// Access typed CardKit helpers for Card JSON 2.0 entities and content streams.
    pub fn cardkit_cards(&self) -> CardKitCards<'_> {
        CardKitCards::new(self)
    }
}

fn validate_identifier(name: &str, value: &str) -> Result<(), LarkError> {
    if value.is_empty() {
        return Err(LarkError::IllegalParam(format!("{name} must not be empty")));
    }
    Ok(())
}

fn validate_element_id(name: &str, value: &str) -> Result<(), LarkError> {
    if value.is_empty()
        || value.len() > 20
        || !value
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_alphabetic())
        || !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(LarkError::IllegalParam(format!(
            "{name} must start with a letter and contain at most 20 ASCII letters, digits, or underscores"
        )));
    }
    Ok(())
}
