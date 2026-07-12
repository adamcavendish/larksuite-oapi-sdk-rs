use super::LarkClient;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, RequestOption};
use crate::resp::{ApiResp, RawResponse};
use crate::transport;

impl LarkClient {
    /// Execute a custom OpenAPI request with SDK auth and token handling.
    ///
    /// This is the raw escape hatch for endpoints that do not have a generated
    /// service wrapper yet. Prefer generated service methods when available.
    /// Set [`ApiReq::supported_access_token_types`] to the token accepted by
    /// the endpoint, or use [`LarkClient::raw_request_with_token`] for the common
    /// single-token case.
    pub async fn raw_request(
        &self,
        api_req: &ApiReq,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        transport::request(&self.config, api_req, option).await
    }

    /// Execute a custom OpenAPI request with the given supported token type.
    pub async fn raw_request_with_token(
        &self,
        mut api_req: ApiReq,
        token_type: AccessTokenType,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        api_req.supported_access_token_types = vec![token_type];
        transport::request(&self.config, &api_req, option).await
    }

    /// Execute a custom OpenAPI request and deserialize its `data` field.
    ///
    /// Set [`ApiReq::supported_access_token_types`] to the token accepted by
    /// the endpoint, or use [`LarkClient::raw_request_typed_with_token`] for the
    /// common single-token case.
    pub async fn raw_request_typed<T: for<'de> serde::Deserialize<'de>>(
        &self,
        api_req: &ApiReq,
        option: &RequestOption,
    ) -> Result<(ApiResp, RawResponse<T>), LarkError> {
        transport::request_typed(&self.config, api_req, option).await
    }

    /// Execute a custom OpenAPI request with the given supported token type and
    /// deserialize its `data` field.
    pub async fn raw_request_typed_with_token<T: for<'de> serde::Deserialize<'de>>(
        &self,
        mut api_req: ApiReq,
        token_type: AccessTokenType,
        option: &RequestOption,
    ) -> Result<(ApiResp, RawResponse<T>), LarkError> {
        api_req.supported_access_token_types = vec![token_type];
        transport::request_typed(&self.config, &api_req, option).await
    }

    pub async fn download_file(&self, url: &str) -> Result<Vec<u8>, LarkError> {
        let resp = self
            .config
            .http_client
            .request(http::Method::GET, url)?
            .send()
            .await
            .map_err(|e| LarkError::Http(e.into_error()))?;

        let status = resp.status();
        let bytes = resp.bytes().await?;

        if !status.is_success() {
            return Err(LarkError::Http(aioduct::Error::Status(status)));
        }

        Ok(bytes.to_vec())
    }
}
