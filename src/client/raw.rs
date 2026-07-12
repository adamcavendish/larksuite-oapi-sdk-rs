use super::LarkClient;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, RawResponse};
use crate::transport;

impl LarkClient {
    pub async fn do_req(
        &self,
        api_req: &ApiReq,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        transport::request(&self.config, api_req, option).await
    }

    pub async fn do_req_typed<T: for<'de> serde::Deserialize<'de>>(
        &self,
        api_req: &ApiReq,
        option: &RequestOption,
    ) -> Result<(ApiResp, RawResponse<T>), LarkError> {
        transport::request_typed(&self.config, api_req, option).await
    }

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
        self.do_req(api_req, option).await
    }

    /// Execute a custom OpenAPI request with the given supported token type.
    pub async fn raw_request_with_token(
        &self,
        mut api_req: ApiReq,
        token_type: AccessTokenType,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        api_req.supported_access_token_types = vec![token_type];
        self.do_req(&api_req, option).await
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
        self.do_req_typed(api_req, option).await
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
        self.do_req_typed(&api_req, option).await
    }

    pub async fn get(&self, path: &str, option: &RequestOption) -> Result<ApiResp, LarkError> {
        self.do_req(&Self::tenant_request(http::Method::GET, path, None), option)
            .await
    }

    pub async fn post(
        &self,
        path: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        self.do_req(
            &Self::tenant_request(http::Method::POST, path, Some(body)),
            option,
        )
        .await
    }

    pub async fn put(
        &self,
        path: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        self.do_req(
            &Self::tenant_request(http::Method::PUT, path, Some(body)),
            option,
        )
        .await
    }

    pub async fn patch(
        &self,
        path: &str,
        body: serde_json::Value,
        option: &RequestOption,
    ) -> Result<ApiResp, LarkError> {
        self.do_req(
            &Self::tenant_request(http::Method::PATCH, path, Some(body)),
            option,
        )
        .await
    }

    pub async fn delete(&self, path: &str, option: &RequestOption) -> Result<ApiResp, LarkError> {
        self.do_req(
            &Self::tenant_request(http::Method::DELETE, path, None),
            option,
        )
        .await
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

    fn tenant_request(method: http::Method, path: &str, body: Option<serde_json::Value>) -> ApiReq {
        let mut api_req = ApiReq::new(method, path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = body.map(ReqBody::Json);
        api_req
    }
}
