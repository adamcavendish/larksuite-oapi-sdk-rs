use super::LarkClient;
use crate::error::LarkError;
use crate::resp::ApiResp;
use crate::token::AppTicketManager;

impl LarkClient {
    pub fn app_ticket_manager(&self) -> AppTicketManager {
        AppTicketManager::new(self.config.token_cache.clone())
    }

    pub async fn get_app_access_token_by_self_built_app(
        &self,
        req: &crate::token::SelfBuiltAppTokenReq,
    ) -> Result<(ApiResp, crate::token::AppTokenResponse), LarkError> {
        crate::token::token_endpoint(
            &self.config,
            crate::constants::APP_ACCESS_TOKEN_INTERNAL_URL_PATH,
            req,
        )
        .await
    }

    pub async fn get_app_access_token_by_marketplace_app(
        &self,
        req: &crate::token::MarketplaceAppTokenReq,
    ) -> Result<(ApiResp, crate::token::AppTokenResponse), LarkError> {
        crate::token::token_endpoint(
            &self.config,
            crate::constants::APP_ACCESS_TOKEN_URL_PATH,
            req,
        )
        .await
    }

    pub async fn get_tenant_access_token_by_self_built_app(
        &self,
        req: &crate::token::SelfBuiltTenantTokenReq,
    ) -> Result<(ApiResp, crate::token::TenantTokenResponse), LarkError> {
        crate::token::token_endpoint(
            &self.config,
            crate::constants::TENANT_ACCESS_TOKEN_INTERNAL_URL_PATH,
            req,
        )
        .await
    }

    pub async fn get_tenant_access_token_by_marketplace_app(
        &self,
        req: &crate::token::MarketplaceTenantTokenReq,
    ) -> Result<(ApiResp, crate::token::TenantTokenResponse), LarkError> {
        crate::token::token_endpoint(
            &self.config,
            crate::constants::TENANT_ACCESS_TOKEN_URL_PATH,
            req,
        )
        .await
    }

    pub async fn resend_app_ticket(
        &self,
        req: &crate::token::ResendAppTicketRequest,
    ) -> Result<(ApiResp, crate::token::ResendAppTicketResponse), LarkError> {
        crate::token::token_endpoint(&self.config, crate::constants::APPLY_APP_TICKET_PATH, req)
            .await
    }
}
