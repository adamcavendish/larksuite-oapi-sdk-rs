//! Typed CardKit helpers for validated Card JSON 2.0 documents.
//!
//! The generated CardKit resource remains available through
//! [`LarkClient::cardkit`](crate::LarkClient::cardkit). This module removes the
//! stringly-typed `card_json` envelope for a Card JSON 2.0 document and owns
//! sequence progression for text-content streaming updates.

use crate::LarkClient;
use crate::card::v2;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::cardkit::v1::{
    ContentCardElementReqBody, ContentCardElementResp, CreateCardReqBody, CreateCardResp,
    UpdateCardReqBody, UpdateCardResp,
};

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
        validate_identifier("CardKit element_id", &element_id)?;
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
