use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct RecognizeFileReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<serde_json::Value>,
}

// ── Response wrappers ──

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
    ) -> Result<RecognizeVatInvoiceResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/vat_invoice/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<VatInvoiceData>()
        .await?;
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
    ) -> Result<RecognizeBusinessCardResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/business_card/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<BusinessCardData>()
        .await?;
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
    ) -> Result<RecognizeIdCardResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/id_card/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<IdCardData>()
        .await?;
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
    ) -> Result<RecognizeBankCardResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/bank_card/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<BankCardData>()
        .await?;
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
    ) -> Result<RecognizeVehicleLicenseResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/vehicle_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<VehicleLicenseData>()
        .await?;
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
    ) -> Result<RecognizeTrainInvoiceResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/train_invoice/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<TrainInvoiceData>()
        .await?;
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
    ) -> Result<RecognizeTaxiInvoiceResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/taxi_invoice/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<TaxiInvoiceData>()
        .await?;
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
    ) -> Result<RecognizeContractResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/contract/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<ContractData>()
        .await?;
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
    ) -> Result<RecognizeChinesePassportRespV2, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/chinese_passport/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<ChinesePassportData>()
        .await?;
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
    ) -> Result<RecognizeDrivingLicenseRespV2, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/driving_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<DrivingLicenseData>()
        .await?;
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
    ) -> Result<RecognizeFoodManageLicenseResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/food_manage_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<FoodManageLicenseData>()
        .await?;
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
    ) -> Result<RecognizeFoodProduceLicenseResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/food_produce_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<FoodProduceLicenseData>()
        .await?;
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
    ) -> Result<RecognizeHealthCertificateRespV2, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/health_certificate/recognize",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<HealthCertificateData>()
        .await?;
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
    ) -> Result<RecognizeHkmMainlandTravelPermitRespV2, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/hkm_mainland_travel_permit/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<HkmMainlandTravelPermitData>()
        .await?;
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
    ) -> Result<RecognizeTwMainlandTravelPermitRespV2, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/tw_mainland_travel_permit/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<TwMainlandTravelPermitData>()
        .await?;
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
    ) -> Result<RecognizeVehicleInvoiceResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/vehicle_invoice/recognize",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2::<VehicleInvoiceData>()
        .await?;
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
    ) -> Result<ParseResumeResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/resume/parse",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<ResumeData>()
        .await?;
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
    ) -> Result<RecognizeVatInvoiceRespV2, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/vat_invoice/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<VatInvoiceData>()
        .await?;
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
    ) -> Result<FieldExtractionContractResp, LarkError> {
        let (api_resp, raw) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/contract/field_extraction",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send::<FieldExtractionContractData>()
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
    ) -> Result<RecognizeBusinessLicenseResp, LarkError> {
        let (api_resp, code_error, data) = RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/business_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2::<BusinessLicenseData>()
        .await?;
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
