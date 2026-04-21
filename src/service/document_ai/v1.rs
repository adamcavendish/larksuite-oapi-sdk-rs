use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::resp::{ApiResp, CodeError, RawResponse};
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

fn parse_v2<T>(api_resp: ApiResp, raw: RawResponse<T>) -> (ApiResp, Option<CodeError>, Option<T>) {
    if raw.code_error.code != 0 {
        (api_resp, Some(raw.code_error), None)
    } else {
        (api_resp, None, raw.data)
    }
}

macro_rules! impl_resp_v2 {
    ($name:ident, $data:ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub api_resp: ApiResp,
            pub code_error: Option<CodeError>,
            pub data: Option<$data>,
        }
        impl $name {
            pub fn success(&self) -> bool {
                self.code_error.as_ref().is_none_or(|e| e.code == 0)
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

// ── New data types (v2 pattern) ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoodManageLicenseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub food_manage_license: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoodProduceLicenseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub food_produce_license: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleInvoiceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vehicle_invoice: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resumes: Option<Vec<serde_json::Value>>,
}

impl_resp_v2!(RecognizeChinesePassportRespV2, ChinesePassportData);
impl_resp_v2!(RecognizeDrivingLicenseRespV2, DrivingLicenseData);
impl_resp_v2!(RecognizeFoodManageLicenseResp, FoodManageLicenseData);
impl_resp_v2!(RecognizeFoodProduceLicenseResp, FoodProduceLicenseData);
impl_resp_v2!(RecognizeHealthCertificateRespV2, HealthCertificateData);
impl_resp_v2!(
    RecognizeHkmMainlandTravelPermitRespV2,
    HkmMainlandTravelPermitData
);
impl_resp_v2!(
    RecognizeTwMainlandTravelPermitRespV2,
    TwMainlandTravelPermitData
);
impl_resp_v2!(RecognizeVehicleInvoiceResp, VehicleInvoiceData);
impl_resp_v2!(ParseResumeResp, ResumeData);
impl_resp_v2!(RecognizeVatInvoiceRespV2, VatInvoiceData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldExtractionContractData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_info: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_info: Option<Vec<serde_json::Value>>,
}

impl_resp!(FieldExtractionContractResp, FieldExtractionContractData);

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

// ── New resource structs ──

pub struct ChinesePassportResource<'a> {
    config: &'a Config,
}

impl<'a> ChinesePassportResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeChinesePassportRespV2> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/chinese_passport/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ChinesePassportData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeChinesePassportRespV2 {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct DrivingLicenseResource<'a> {
    config: &'a Config,
}

impl<'a> DrivingLicenseResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeDrivingLicenseRespV2> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/driving_license/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<DrivingLicenseData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeDrivingLicenseRespV2 {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct FoodManageLicenseResource<'a> {
    config: &'a Config,
}

impl<'a> FoodManageLicenseResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeFoodManageLicenseResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/food_manage_license/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FoodManageLicenseData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeFoodManageLicenseResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct FoodProduceLicenseResource<'a> {
    config: &'a Config,
}

impl<'a> FoodProduceLicenseResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeFoodProduceLicenseResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/food_produce_license/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FoodProduceLicenseData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeFoodProduceLicenseResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct HealthCertificateResource<'a> {
    config: &'a Config,
}

impl<'a> HealthCertificateResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeHealthCertificateRespV2> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/health_certificate/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<HealthCertificateData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeHealthCertificateRespV2 {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct HkmMainlandTravelPermitResource<'a> {
    config: &'a Config,
}

impl<'a> HkmMainlandTravelPermitResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeHkmMainlandTravelPermitRespV2> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/hkm_mainland_travel_permit/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<HkmMainlandTravelPermitData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeHkmMainlandTravelPermitRespV2 {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct TwMainlandTravelPermitResource<'a> {
    config: &'a Config,
}

impl<'a> TwMainlandTravelPermitResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeTwMainlandTravelPermitRespV2> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/tw_mainland_travel_permit/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<TwMainlandTravelPermitData>(self.config, &api_req, option)
                .await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeTwMainlandTravelPermitRespV2 {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct VehicleInvoiceResource<'a> {
    config: &'a Config,
}

impl<'a> VehicleInvoiceResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeVehicleInvoiceResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/vehicle_invoice/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<VehicleInvoiceData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeVehicleInvoiceResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ResumeResource<'a> {
    config: &'a Config,
}

impl<'a> ResumeResource<'a> {
    pub async fn parse(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<ParseResumeResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/document_ai/v1/resume/parse");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<ResumeData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ParseResumeResp {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct VatInvoiceResource<'a> {
    config: &'a Config,
}

impl<'a> VatInvoiceResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeVatInvoiceRespV2> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/vat_invoice/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<VatInvoiceData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeVatInvoiceRespV2 {
            api_resp,
            code_error,
            data,
        })
    }
}

pub struct ContractResource<'a> {
    config: &'a Config,
}

impl<'a> ContractResource<'a> {
    pub async fn field_extraction(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<FieldExtractionContractResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/contract/field_extraction",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<FieldExtractionContractData>(self.config, &api_req, option)
                .await?;
        Ok(FieldExtractionContractResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }
}

// ── BusinessLicense resource ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessLicenseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_license: Option<serde_json::Value>,
}

impl_resp_v2!(RecognizeBusinessLicenseResp, BusinessLicenseData);

pub struct BusinessLicenseResource<'a> {
    config: &'a Config,
}

impl<'a> BusinessLicenseResource<'a> {
    pub async fn recognize(
        &self,
        body: &RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeBusinessLicenseResp> {
        let mut api_req = ApiReq::new(
            http::Method::POST,
            "/open-apis/document_ai/v1/business_license/recognize",
        );
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<BusinessLicenseData>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecognizeBusinessLicenseResp {
            api_resp,
            code_error,
            data,
        })
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub ai: AiResource<'a>,
    pub business_license: BusinessLicenseResource<'a>,
    pub contract: ContractResource<'a>,
    pub chinese_passport: ChinesePassportResource<'a>,
    pub driving_license: DrivingLicenseResource<'a>,
    pub food_manage_license: FoodManageLicenseResource<'a>,
    pub food_produce_license: FoodProduceLicenseResource<'a>,
    pub health_certificate: HealthCertificateResource<'a>,
    pub hkm_mainland_travel_permit: HkmMainlandTravelPermitResource<'a>,
    pub tw_mainland_travel_permit: TwMainlandTravelPermitResource<'a>,
    pub vehicle_invoice: VehicleInvoiceResource<'a>,
    pub resume: ResumeResource<'a>,
    pub vat_invoice: VatInvoiceResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            ai: AiResource { config },
            business_license: BusinessLicenseResource { config },
            contract: ContractResource { config },
            chinese_passport: ChinesePassportResource { config },
            driving_license: DrivingLicenseResource { config },
            food_manage_license: FoodManageLicenseResource { config },
            food_produce_license: FoodProduceLicenseResource { config },
            health_certificate: HealthCertificateResource { config },
            hkm_mainland_travel_permit: HkmMainlandTravelPermitResource { config },
            tw_mainland_travel_permit: TwMainlandTravelPermitResource { config },
            vehicle_invoice: VehicleInvoiceResource { config },
            resume: ResumeResource { config },
            vat_invoice: VatInvoiceResource { config },
        }
    }
}
