use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{FormDataField, RequestOption};
use crate::service::common::RestRequest;

// ── Request body types ──

pub type RecognizeFileReqBody = Vec<FormDataField>;

// ── Recognition models ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankCard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<BankCardEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankCardEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BankEntity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BodyEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_representative: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BodyInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BodyEntity>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessLicense {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<BusinessEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChinesePassport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<ChinesePassportEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChinesePassportEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DrivingEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DrivingLicense {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<DrivingEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractCopy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_num: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_copy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractCurrency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractPrice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_price: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_price_original: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractTerm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_time_start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_time_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_term: Option<ExtractTerm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_initial_term: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoodManageEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoodManageLicense {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<FoodManageEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoodProduceEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoodProduceLicense {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<FoodProduceEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthCertificate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<HealthCertificateEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthCertificateEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HkmMainlandTravelPermit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<HkmMainlandTravelPermitEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HkmMainlandTravelPermitEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdCard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<IdEntity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conners: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_conners: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecognizedEntities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<RecognizedEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecognizedEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Resume {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_md5: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_is_virtual: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub educations: Option<Vec<ResumeEducation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub careers: Option<Vec<ResumeCareer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ResumeProject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_year: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub willing_positions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub willing_locations: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<ResumeLanguage>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awards: Option<Vec<ResumeAward>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<ResumeCertificate>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub competitions: Option<Vec<ResumeCompetition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_evaluation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub social_links: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeAward {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeCareer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_str: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeCertificate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeCompetition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeEducation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeLanguage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeProject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaxiEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaxiInvoice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<TaxiEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainInvoice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<TrainEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TwMainlandTravelPermit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<TwMainlandTravelPermitEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TwMainlandTravelPermitEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VatEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Vec<KvEntity>>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KvEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VatInvoice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<VatEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleInvoice {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<VehicleInvoiceEntity>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleInvoiceEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleLicense {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<VehicleEntity>>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VatInvoiceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vat_invoices: Option<Vec<VatInvoice>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessCardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_cards: Option<Vec<RecognizedEntities>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdCardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_card: Option<IdCard>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankCardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_card: Option<BankCard>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleLicenseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vehicle_license: Option<VehicleLicense>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainInvoiceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub train_invoices: Option<Vec<TrainInvoice>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaxiInvoiceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxi_invoices: Option<Vec<TaxiInvoice>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthCertificateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_certificate: Option<HealthCertificate>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HkmMainlandTravelPermitData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hkm_mainland_travel_permit: Option<HkmMainlandTravelPermit>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TwMainlandTravelPermitData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tw_mainland_travel_permit: Option<TwMainlandTravelPermit>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChinesePassportData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chinese_passport: Option<ChinesePassport>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DrivingLicenseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub driving_license: Option<DrivingLicense>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NurseQualificationCertificateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nurse_qualification_certificate: Option<crate::JsonValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts: Option<Vec<crate::JsonValue>>,
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
    pub food_manage_license: Option<FoodManageLicense>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoodProduceLicenseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub food_produce_license: Option<FoodProduceLicense>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VehicleInvoiceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vehicle_invoice: Option<VehicleInvoice>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResumeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resumes: Option<Vec<Resume>>,
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
    pub price: Option<ExtractPrice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<ExtractTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy: Option<ExtractCopy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<ExtractCurrency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_info: Option<Vec<BodyInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_info: Option<Vec<BankInfo>>,
}

impl_resp!(FieldExtractionContractResp, FieldExtractionContractData);

// ── Resources ──

pub struct AiResource<'a> {
    config: &'a Config,
}

impl<'a> AiResource<'a> {
    pub async fn recognize_vat_invoice(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeVatInvoiceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/vat_invoice/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_response::<VatInvoiceData, RecognizeVatInvoiceResp>()
        .await
    }

    pub async fn recognize_business_card(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeBusinessCardResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/business_card/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_response::<BusinessCardData, RecognizeBusinessCardResp>()
        .await
    }

    pub async fn recognize_id_card(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeIdCardResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/id_card/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_response::<IdCardData, RecognizeIdCardResp>()
        .await
    }

    pub async fn recognize_bank_card(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeBankCardResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/bank_card/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_response::<BankCardData, RecognizeBankCardResp>()
        .await
    }

    pub async fn recognize_vehicle_license(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeVehicleLicenseResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/vehicle_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_response::<VehicleLicenseData, RecognizeVehicleLicenseResp>()
        .await
    }

    pub async fn recognize_train_invoice(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeTrainInvoiceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/train_invoice/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_response::<TrainInvoiceData, RecognizeTrainInvoiceResp>()
        .await
    }

    pub async fn recognize_taxi_invoice(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeTaxiInvoiceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/taxi_invoice/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_response::<TaxiInvoiceData, RecognizeTaxiInvoiceResp>()
        .await
    }

    pub async fn recognize_contract(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeContractResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/contract/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_response::<ContractData, RecognizeContractResp>()
        .await
    }
}

// ── New resource structs ──

pub struct ChinesePassportResource<'a> {
    config: &'a Config,
}

impl<'a> ChinesePassportResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeChinesePassportRespV2, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/chinese_passport/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_v2_response::<ChinesePassportData, RecognizeChinesePassportRespV2>()
        .await
    }
}

pub struct DrivingLicenseResource<'a> {
    config: &'a Config,
}

impl<'a> DrivingLicenseResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeDrivingLicenseRespV2, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/driving_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_v2_response::<DrivingLicenseData, RecognizeDrivingLicenseRespV2>()
        .await
    }
}

pub struct FoodManageLicenseResource<'a> {
    config: &'a Config,
}

impl<'a> FoodManageLicenseResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeFoodManageLicenseResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/food_manage_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_v2_response::<FoodManageLicenseData, RecognizeFoodManageLicenseResp>()
        .await
    }
}

pub struct FoodProduceLicenseResource<'a> {
    config: &'a Config,
}

impl<'a> FoodProduceLicenseResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeFoodProduceLicenseResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/food_produce_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_v2_response::<FoodProduceLicenseData, RecognizeFoodProduceLicenseResp>()
        .await
    }
}

pub struct HealthCertificateResource<'a> {
    config: &'a Config,
}

impl<'a> HealthCertificateResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeHealthCertificateRespV2, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/health_certificate/recognize",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .form_body(body)
        .send_v2_response::<HealthCertificateData, RecognizeHealthCertificateRespV2>()
        .await
    }
}

pub struct HkmMainlandTravelPermitResource<'a> {
    config: &'a Config,
}

impl<'a> HkmMainlandTravelPermitResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeHkmMainlandTravelPermitRespV2, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/hkm_mainland_travel_permit/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_v2_response::<HkmMainlandTravelPermitData, RecognizeHkmMainlandTravelPermitRespV2>()
        .await
    }
}

pub struct TwMainlandTravelPermitResource<'a> {
    config: &'a Config,
}

impl<'a> TwMainlandTravelPermitResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeTwMainlandTravelPermitRespV2, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/tw_mainland_travel_permit/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_v2_response::<TwMainlandTravelPermitData, RecognizeTwMainlandTravelPermitRespV2>()
        .await
    }
}

pub struct VehicleInvoiceResource<'a> {
    config: &'a Config,
}

impl<'a> VehicleInvoiceResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeVehicleInvoiceResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/vehicle_invoice/recognize",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .form_body(body)
        .send_v2_response::<VehicleInvoiceData, RecognizeVehicleInvoiceResp>()
        .await
    }
}

pub struct ResumeResource<'a> {
    config: &'a Config,
}

impl<'a> ResumeResource<'a> {
    pub async fn parse(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<ParseResumeResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/resume/parse",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_v2_response::<ResumeData, ParseResumeResp>()
        .await
    }
}

pub struct VatInvoiceResource<'a> {
    config: &'a Config,
}

impl<'a> VatInvoiceResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeVatInvoiceRespV2, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/vat_invoice/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_v2_response::<VatInvoiceData, RecognizeVatInvoiceRespV2>()
        .await
    }
}

pub struct ContractResource<'a> {
    config: &'a Config,
}

impl<'a> ContractResource<'a> {
    pub async fn field_extraction(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<FieldExtractionContractResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/contract/field_extraction",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_response::<FieldExtractionContractData, FieldExtractionContractResp>()
        .await
    }
}

// ── BusinessLicense resource ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessLicenseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_license: Option<BusinessLicense>,
}

impl_resp_v2!(RecognizeBusinessLicenseResp, BusinessLicenseData);

pub struct BusinessLicenseResource<'a> {
    config: &'a Config,
}

impl<'a> BusinessLicenseResource<'a> {
    pub async fn recognize(
        &self,
        body: RecognizeFileReqBody,
        option: &RequestOption,
    ) -> Result<RecognizeBusinessLicenseResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/document_ai/v1/business_license/recognize",
            vec![AccessTokenType::Tenant],
            option,
        )
        .form_body(body)
        .send_v2_response::<BusinessLicenseData, RecognizeBusinessLicenseResp>()
        .await
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
