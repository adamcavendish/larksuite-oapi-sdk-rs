use larksuite_oapi_sdk_rs::client;
use larksuite_oapi_sdk_rs::service::hire::{v1, v2};
use larksuite_oapi_sdk_rs::{LarkClient, LarkClientBuilder};

#[test]
fn client_module_reexports_the_public_client_types() {
    let _: client::LarkClientBuilder = LarkClient::builder("app", "secret");
    let _: LarkClientBuilder = client::LarkClient::builder("app", "secret");
}

#[test]
fn hire_versions_share_exact_common_models() {
    fn takes_v1_i18n(_: v1::I18n) {}
    fn takes_v1_id_name(_: v1::IdNameObject) {}
    fn takes_v1_resume_source(_: v1::TalentResumeSource) {}
    fn takes_v1_registration(_: v1::TalentInterviewRegistrationSimple) {}
    fn takes_v1_registration_info(_: v1::RegistrationBasicInfo) {}

    takes_v1_i18n(v2::I18n::default());
    takes_v1_id_name(v2::IdNameObject::default());
    takes_v1_resume_source(v2::TalentResumeSource::default());
    takes_v1_registration(v2::TalentInterviewRegistrationSimple::default());
    takes_v1_registration_info(v2::RegistrationBasicInfo::default());
}
