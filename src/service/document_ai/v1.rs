use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError};
use crate::transport;

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecognizeFileReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<serde_json::Value>,
}

// ── Response wrappers ──

macro_rules! impl_resp {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: CodeError,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.success()
            }
        }
    };
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VatInvoiceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vat_invoices: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessCardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_cards: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdCardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_card: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankCardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_card: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleLicenseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vehicle_license: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainInvoiceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub train_invoices: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaxiInvoiceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxi_invoices: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthCertificateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_certificate: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HkmMainlandTravelPermitData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hkm_mainland_travel_permit: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TwMainlandTravelPermitData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tw_mainland_travel_permit: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChinesePassportData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chinese_passport: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DrivingLicenseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driving_license: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NurseQualificationCertificateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nurse_qualification_certificate: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts: Option<Vec<serde_json::Value>>,
}

impl_resp!(RecognizeVatInvoiceResp, VatInvoiceData);
impl_resp!(RecognizeBusinessCardResp, BusinessCardData);
impl_resp!(RecognizeIdCardResp, IdCardData);
impl_resp!(RecognizeBankCardResp, BankCardData);
impl_resp!(RecognizeVehicleLicenseResp, VehicleLicenseData);
impl_resp!(RecognizeTrainInvoiceResp, TrainInvoiceData);
impl_resp!(RecognizeTaxiInvoiceResp, TaxiInvoiceData);
impl_resp!(RecognizeHealthCertificateResp, HealthCertificateData);
impl_resp!(
    RecognizeHkmMainlandTravelPermitResp,
    HkmMainlandTravelPermitData
);
impl_resp!(
    RecognizeTwMainlandTravelPermitResp,
    TwMainlandTravelPermitData
);
impl_resp!(RecognizeChinesePassportResp, ChinesePassportData);
impl_resp!(RecognizeDrivingLicenseResp, DrivingLicenseData);
impl_resp!(
    RecognizeNurseQualificationCertificateResp,
    NurseQualificationCertificateData
);
impl_resp!(RecognizeContractResp, ContractData);

// ── Resources ──

pub struct AiResource<'a> {
    config: &'a Config,
}

impl<'a> AiResource<'a> {
    pub async fn recognize_vat_invoice(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeVatInvoiceResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/vat_invoice/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<VatInvoiceData>(self.config, &api_req, option).await?;
        Ok(RecognizeVatInvoiceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn recognize_business_card(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeBusinessCardResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/business_card/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BusinessCardData>(self.config, &api_req, option).await?;
        Ok(RecognizeBusinessCardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn recognize_id_card(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeIdCardResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/id_card/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<IdCardData>(self.config, &api_req, option).await?;
        Ok(RecognizeIdCardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn recognize_bank_card(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeBankCardResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/bank_card/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BankCardData>(self.config, &api_req, option).await?;
        Ok(RecognizeBankCardResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn recognize_vehicle_license(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeVehicleLicenseResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/vehicle_license/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<VehicleLicenseData>(self.config, &api_req, option).await?;
        Ok(RecognizeVehicleLicenseResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn recognize_train_invoice(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeTrainInvoiceResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/train_invoice/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TrainInvoiceData>(self.config, &api_req, option).await?;
        Ok(RecognizeTrainInvoiceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn recognize_taxi_invoice(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeTaxiInvoiceResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/taxi_invoice/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TaxiInvoiceData>(self.config, &api_req, option).await?;
        Ok(RecognizeTaxiInvoiceResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    pub async fn recognize_contract(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeContractResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/contract/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ContractData>(self.config, &api_req, option).await?;
        Ok(RecognizeContractResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub ai: AiResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            ai: AiResource { config },
        }
    }
}
