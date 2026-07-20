use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{JsonResp, RestRequest};

include!("go_v397_metadata.rs");

fn build_api_req<P, Q, PK, PV, QK, QV>(
    meta: &GoV397EndpointMeta,
    path_params: P,
    query_params: Q,
    body: Option<ReqBody>,
) -> ApiReq
where
    P: IntoIterator<Item = (PK, PV)>,
    PK: AsRef<str>,
    PV: AsRef<str>,
    Q: IntoIterator<Item = (QK, QV)>,
    QK: AsRef<str>,
    QV: AsRef<str>,
{
    let mut api_req = ApiReq::new(meta.method.clone(), meta.path);
    api_req.supported_access_token_types = meta.supported_access_token_types.to_vec();
    for (key, value) in path_params {
        api_req.path_params.set(key.as_ref(), value.as_ref());
    }
    for (key, value) in query_params {
        api_req.query_params.add(key.as_ref(), value.as_ref());
    }
    api_req.body = body;
    api_req
}

pub struct GoV397<'a> {
    config: &'a Config,
}

impl<'a> GoV397<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn request<P, Q, PK, PV, QK, QV>(
        &self,
        endpoint: GoV397Endpoint,
        path_params: P,
        query_params: Q,
        body: Option<ReqBody>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError>
    where
        P: IntoIterator<Item = (PK, PV)>,
        PK: AsRef<str>,
        PV: AsRef<str>,
        Q: IntoIterator<Item = (QK, QV)>,
        QK: AsRef<str>,
        QV: AsRef<str>,
    {
        let meta = endpoint.meta();
        let api_req = build_api_req(&meta, path_params, query_params, body);

        let request = RestRequest::from_api_req(self.config, api_req, option);
        let request = if meta.file_upload {
            request.file_upload()
        } else {
            request
        };
        request.send_json().await
    }

    pub async fn request_json<P, Q, PK, PV, QK, QV>(
        &self,
        endpoint: GoV397Endpoint,
        path_params: P,
        query_params: Q,
        body: Option<&crate::JsonValue>,
        option: &RequestOption,
    ) -> Result<JsonResp, LarkError>
    where
        P: IntoIterator<Item = (PK, PV)>,
        PK: AsRef<str>,
        PV: AsRef<str>,
        Q: IntoIterator<Item = (QK, QV)>,
        QK: AsRef<str>,
        QV: AsRef<str>,
    {
        self.request(
            endpoint,
            path_params,
            query_params,
            body.cloned().map(ReqBody::Json),
            option,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_builder_preserves_repeated_query_params() {
        let meta = GoV397Endpoint::GetAilyV1AppStats.meta();
        let api_req = build_api_req(
            &meta,
            std::iter::empty::<(&str, &str)>(),
            [("field", "a"), ("field", "b")],
            None,
        );
        let encoded = api_req.query_params.encode();

        assert!(encoded.contains("field=a"));
        assert!(encoded.contains("field=b"));
        assert_eq!(encoded.matches("field=").count(), 2);
    }

    #[test]
    fn generated_endpoint_metadata_is_valid() {
        assert_eq!(GoV397Endpoint::ALL.len(), 114);
        for endpoint in GoV397Endpoint::ALL {
            let meta = endpoint.meta();
            assert!(meta.path.starts_with("/open-apis/"));
            assert!(!meta.supported_access_token_types.is_empty());
        }
    }
}
