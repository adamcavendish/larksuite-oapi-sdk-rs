use base64::Engine as _;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::resp::{ApiResp, CodeError};
use crate::service::common::{FromV2Response, JsonResp, PageQuery, RestRequest};

pub type CreateXmlPresentationResp = JsonResp;
pub type GetXmlPresentationResp = JsonResp;
pub type GetSlideResp = JsonResp;
pub type AddSlideResp = JsonResp;
pub type DeleteSlideResp = JsonResp;
pub type ReplaceSlideResp = JsonResp;
pub type ListXmlPresentationHistoryResp = JsonResp;
pub type RevertXmlPresentationHistoryResp = JsonResp;
pub type GetXmlPresentationHistoryRevertStatusResp = JsonResp;

pub type GetSlideImagesResp = SlideImageResponse<SlideImagesData>;
pub type RenderSlideImageResp = SlideImageResponse<RenderedSlideImageData>;

const MAX_SLIDE_IMAGES_PER_REQUEST: usize = 10;

/// A rendered slide image returned as Base64-encoded bytes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SlideImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slide_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slide_number: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    pub data: String,
}

impl SlideImage {
    /// Decodes the service's standard-Base64 image payload.
    pub fn decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
        base64::engine::general_purpose::STANDARD.decode(self.data.trim())
    }
}

/// Image data returned for selected presentation slides.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SlideImagesData {
    #[serde(default)]
    pub slide_images: Vec<SlideImage>,
}

/// Image data returned after rendering one XML slide fragment.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RenderedSlideImageData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slide_image: Option<SlideImage>,
}

/// A typed Slides AI image response.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SlideImageResponse<T> {
    pub api_resp: ApiResp,
    pub code_error: Option<CodeError>,
    pub data: Option<T>,
}

impl<T> SlideImageResponse<T> {
    pub fn success(&self) -> bool {
        self.api_resp.status_code == 200 && self.code_error.as_ref().is_none_or(|e| e.code == 0)
    }
}

impl<T> FromV2Response<T> for SlideImageResponse<T> {
    fn from_v2_response(api_resp: ApiResp, code_error: Option<CodeError>, data: Option<T>) -> Self {
        Self {
            api_resp,
            code_error,
            data,
        }
    }
}

/// Exactly one supported selector type for presentation-slide images.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum SlideImageSelector<'a> {
    Ids(&'a [&'a str]),
    Numbers(&'a [i32]),
}

/// Request parameters for rendering existing presentation slides as images.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetSlideImagesRequest<'a> {
    pub xml_presentation_id: &'a str,
    pub selector: SlideImageSelector<'a>,
}

impl<'a> GetSlideImagesRequest<'a> {
    pub fn by_ids(xml_presentation_id: &'a str, slide_ids: &'a [&'a str]) -> Self {
        Self {
            xml_presentation_id,
            selector: SlideImageSelector::Ids(slide_ids),
        }
    }

    pub fn by_numbers(xml_presentation_id: &'a str, slide_numbers: &'a [i32]) -> Self {
        Self {
            xml_presentation_id,
            selector: SlideImageSelector::Numbers(slide_numbers),
        }
    }

    fn validate(&self) -> Result<(), LarkError> {
        let count = match self.selector {
            SlideImageSelector::Ids(slide_ids) => slide_ids.len(),
            SlideImageSelector::Numbers(slide_numbers) => slide_numbers.len(),
        };
        if count == 0 {
            return Err(LarkError::IllegalParam(
                "slide image request requires at least one slide ID or number".to_string(),
            ));
        }
        if count > MAX_SLIDE_IMAGES_PER_REQUEST {
            return Err(LarkError::IllegalParam(format!(
                "slide image request supports at most {MAX_SLIDE_IMAGES_PER_REQUEST} selections"
            )));
        }
        Ok(())
    }
}

/// Query parameters for retrieving an XML presentation.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetXmlPresentationQuery<'a> {
    pub xml_presentation_id: &'a str,
    pub revision_id: i32,
    pub remove_attr_id: Option<bool>,
}

impl<'a> GetXmlPresentationQuery<'a> {
    pub fn new(xml_presentation_id: &'a str) -> Self {
        Self {
            xml_presentation_id,
            revision_id: -1,
            remove_attr_id: None,
        }
    }

    pub fn revision_id(mut self, value: i32) -> Self {
        self.revision_id = value;
        self
    }

    pub fn remove_attr_id(mut self, value: impl Into<Option<bool>>) -> Self {
        self.remove_attr_id = value.into();
        self
    }
}

/// Exactly one supported selector for a slide read.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum SlideSelector<'a> {
    Id(&'a str),
    Number(i32),
}

/// Query parameters for retrieving one slide from an XML presentation.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetSlideQuery<'a> {
    pub xml_presentation_id: &'a str,
    pub selector: SlideSelector<'a>,
    pub revision_id: i32,
}

impl<'a> GetSlideQuery<'a> {
    pub fn by_id(xml_presentation_id: &'a str, slide_id: &'a str) -> Self {
        Self {
            xml_presentation_id,
            selector: SlideSelector::Id(slide_id),
            revision_id: -1,
        }
    }

    pub fn by_number(xml_presentation_id: &'a str, slide_number: i32) -> Self {
        Self {
            xml_presentation_id,
            selector: SlideSelector::Number(slide_number),
            revision_id: -1,
        }
    }

    pub fn revision_id(mut self, value: i32) -> Self {
        self.revision_id = value;
        self
    }
}

/// Query parameters for appending one slide to an XML presentation.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct AddSlideQuery<'a> {
    pub xml_presentation_id: &'a str,
    pub revision_id: i32,
}

impl<'a> AddSlideQuery<'a> {
    pub fn new(xml_presentation_id: &'a str) -> Self {
        Self {
            xml_presentation_id,
            revision_id: -1,
        }
    }

    pub fn revision_id(mut self, value: i32) -> Self {
        self.revision_id = value;
        self
    }
}

/// Query parameters for deleting one slide from an XML presentation.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteSlideQuery<'a> {
    pub xml_presentation_id: &'a str,
    pub slide_id: &'a str,
    pub revision_id: i32,
}

impl<'a> DeleteSlideQuery<'a> {
    pub fn new(xml_presentation_id: &'a str, slide_id: &'a str) -> Self {
        Self {
            xml_presentation_id,
            slide_id,
            revision_id: -1,
        }
    }

    pub fn revision_id(mut self, value: i32) -> Self {
        self.revision_id = value;
        self
    }
}

/// Query parameters for replacing parts of one slide.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ReplaceSlideQuery<'a> {
    pub xml_presentation_id: &'a str,
    pub slide_id: &'a str,
    pub revision_id: i32,
    pub tid: Option<&'a str>,
}

impl<'a> ReplaceSlideQuery<'a> {
    pub fn new(xml_presentation_id: &'a str, slide_id: &'a str) -> Self {
        Self {
            xml_presentation_id,
            slide_id,
            revision_id: -1,
            tid: None,
        }
    }

    pub fn revision_id(mut self, value: i32) -> Self {
        self.revision_id = value;
        self
    }

    pub fn tid(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.tid = value.into();
        self
    }
}

/// Query parameters for listing XML-presentation history versions.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListXmlPresentationHistoryQuery<'a> {
    pub xml_presentation_id: &'a str,
    pub page: PageQuery<'a>,
}

impl<'a> ListXmlPresentationHistoryQuery<'a> {
    pub fn new(xml_presentation_id: &'a str) -> Self {
        Self {
            xml_presentation_id,
            page: PageQuery::default(),
        }
    }

    pub fn page(mut self, value: PageQuery<'a>) -> Self {
        self.page = value;
        self
    }
}

/// Query parameters for an XML-presentation history-revert task.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetXmlPresentationHistoryRevertStatusQuery<'a> {
    pub xml_presentation_id: &'a str,
    pub task_id: &'a str,
}

impl<'a> GetXmlPresentationHistoryRevertStatusQuery<'a> {
    pub fn new(xml_presentation_id: &'a str, task_id: &'a str) -> Self {
        Self {
            xml_presentation_id,
            task_id,
        }
    }
}

/// Slides AI v1 operations proven by the official Lark CLI's Open Platform client.
///
/// This module is separate from Docs AI document content and structured Docx
/// block APIs. XML presentation and slide-part schemas are not present in the
/// pinned Go SDK catalog, so callers pass serializable JSON payloads and
/// receive [`JsonResp`] values unchanged. Slide-image routes use small typed
/// result models because their Base64 payloads have reusable decoding behavior.
pub struct V1<'a> {
    pub presentation: PresentationResource<'a>,
    pub slide: SlideResource<'a>,
    pub image: SlideImageResource<'a>,
    pub history: XmlPresentationHistoryResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            presentation: PresentationResource { config },
            slide: SlideResource { config },
            image: SlideImageResource { config },
            history: XmlPresentationHistoryResource { config },
        }
    }
}

/// Create and retrieve XML presentations.
pub struct PresentationResource<'a> {
    config: &'a Config,
}

impl PresentationResource<'_> {
    /// Creates an XML presentation from the upstream JSON request body.
    pub async fn create(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateXmlPresentationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/slides_ai/v1/xml_presentations",
            supported_access_tokens(),
            option,
        )
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Retrieves an XML presentation and its selected revision.
    pub async fn get(
        &self,
        query: &GetXmlPresentationQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetXmlPresentationResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/slides_ai/v1/xml_presentations/:xml_presentation_id",
            supported_access_tokens(),
            option,
        )
        .path_param("xml_presentation_id", query.xml_presentation_id)
        .query("revision_id", query.revision_id)
        .query("remove_attr_id", query.remove_attr_id)
        .send_json()
        .await
    }
}

/// Retrieve and mutate individual XML-presentation slides.
pub struct SlideResource<'a> {
    config: &'a Config,
}

impl SlideResource<'_> {
    /// Retrieves one slide by ID or by its one-based presentation number.
    pub async fn get(
        &self,
        query: &GetSlideQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetSlideResp, LarkError> {
        let request = RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/slides_ai/v1/xml_presentations/:xml_presentation_id/slide",
            supported_access_tokens(),
            option,
        )
        .path_param("xml_presentation_id", query.xml_presentation_id)
        .query("revision_id", query.revision_id);
        let request = match query.selector {
            SlideSelector::Id(slide_id) => request.query("slide_id", slide_id),
            SlideSelector::Number(slide_number) => request.query("slide_number", slide_number),
        };
        request.send_json().await
    }

    /// Appends a slide from the upstream JSON request body.
    pub async fn add(
        &self,
        query: &AddSlideQuery<'_>,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<AddSlideResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/slides_ai/v1/xml_presentations/:xml_presentation_id/slide",
            supported_access_tokens(),
            option,
        )
        .path_param("xml_presentation_id", query.xml_presentation_id)
        .query("revision_id", query.revision_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Deletes one slide. This operation has immediate server-side effects.
    pub async fn delete(
        &self,
        query: &DeleteSlideQuery<'_>,
        option: &RequestOption,
    ) -> Result<DeleteSlideResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            "/open-apis/slides_ai/v1/xml_presentations/:xml_presentation_id/slide",
            supported_access_tokens(),
            option,
        )
        .path_param("xml_presentation_id", query.xml_presentation_id)
        .query("slide_id", query.slide_id)
        .query("revision_id", query.revision_id)
        .send_json()
        .await
    }

    /// Replaces slide parts with the upstream JSON request body.
    ///
    /// The SDK forwards caller-provided parts unchanged and does not perform
    /// the CLI's XML parsing, ID injection, or local normalizations.
    pub async fn replace(
        &self,
        query: &ReplaceSlideQuery<'_>,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<ReplaceSlideResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/slides_ai/v1/xml_presentations/:xml_presentation_id/slide/replace",
            supported_access_tokens(),
            option,
        )
        .path_param("xml_presentation_id", query.xml_presentation_id)
        .query("slide_id", query.slide_id)
        .query("revision_id", query.revision_id)
        .query("tid", query.tid)
        .json_body(&body)?
        .send_json()
        .await
    }
}

/// Render Slides AI XML content as server-produced image data.
pub struct SlideImageResource<'a> {
    config: &'a Config,
}

impl SlideImageResource<'_> {
    /// Renders selected existing slides as Base64 image payloads.
    pub async fn get(
        &self,
        request: &GetSlideImagesRequest<'_>,
        option: &RequestOption,
    ) -> Result<GetSlideImagesResp, LarkError> {
        request.validate()?;
        let body = match request.selector {
            SlideImageSelector::Ids(slide_ids) => serde_json::json!({ "slide_ids": slide_ids }),
            SlideImageSelector::Numbers(slide_numbers) => {
                serde_json::json!({ "slide_numbers": slide_numbers })
            }
        };
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/slides_ai/v1/xml_presentations/:xml_presentation_id/slide_images",
            supported_access_tokens(),
            option,
        )
        .path_param("xml_presentation_id", request.xml_presentation_id)
        .json_body(&body)?
        .send_v2_response::<SlideImagesData, GetSlideImagesResp>()
        .await
    }

    /// Renders one XML slide fragment as a Base64 image payload.
    pub async fn render(
        &self,
        content: &str,
        option: &RequestOption,
    ) -> Result<RenderSlideImageResp, LarkError> {
        if content.trim().is_empty() {
            return Err(LarkError::IllegalParam(
                "slide image render content cannot be empty".to_string(),
            ));
        }
        let body = RenderSlideImageBody { content };
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/slides_ai/v1/slide_image/render",
            supported_access_tokens(),
            option,
        )
        .json_body(&body)?
        .send_v2_response::<RenderedSlideImageData, RenderSlideImageResp>()
        .await
    }
}

#[derive(Serialize)]
struct RenderSlideImageBody<'a> {
    content: &'a str,
}

/// List, revert, and inspect XML-presentation history.
pub struct XmlPresentationHistoryResource<'a> {
    config: &'a Config,
}

impl XmlPresentationHistoryResource<'_> {
    /// Lists XML-presentation history versions.
    pub async fn list(
        &self,
        query: &ListXmlPresentationHistoryQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListXmlPresentationHistoryResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/slides_ai/v1/xml_presentations/:xml_presentation_id/histories",
            supported_access_tokens(),
            option,
        )
        .path_param("xml_presentation_id", query.xml_presentation_id)
        .page_query(query.page)
        .send_json()
        .await
    }

    /// Starts an XML-presentation history revert with the upstream JSON request body.
    pub async fn revert(
        &self,
        xml_presentation_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<RevertXmlPresentationHistoryResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/slides_ai/v1/xml_presentations/:xml_presentation_id/history/revert",
            supported_access_tokens(),
            option,
        )
        .path_param("xml_presentation_id", xml_presentation_id)
        .json_body(&body)?
        .send_json()
        .await
    }

    /// Gets the status of an XML-presentation history-revert task.
    pub async fn revert_status(
        &self,
        query: &GetXmlPresentationHistoryRevertStatusQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetXmlPresentationHistoryRevertStatusResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/slides_ai/v1/xml_presentations/:xml_presentation_id/history/revert_status",
            supported_access_tokens(),
            option,
        )
        .path_param("xml_presentation_id", query.xml_presentation_id)
        .query("task_id", query.task_id)
        .send_json()
        .await
    }
}

fn supported_access_tokens() -> Vec<AccessTokenType> {
    vec![AccessTokenType::User, AccessTokenType::Tenant]
}
