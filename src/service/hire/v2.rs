use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

pub use super::shared::{
    I18n, IdNameObject, RegistrationBasicInfo, TalentInterviewRegistrationSimple,
    TalentResumeSource,
};

// ── Response data types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AbilityAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendedJobLevel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_limit_job_level_name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub higher_limit_job_level_name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DimensionOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_val: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuestionAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abilities: Option<Vec<Ability>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DimensionAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_feedback_form_dimension_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_option: Option<DimensionOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_options: Option<Vec<DimensionOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_score: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended_job_level: Option<RecommendedJobLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question_assessments: Option<Vec<QuestionAssessment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ability_assessments: Option<Vec<AbilityAssessment>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModuleAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_feedback_form_module_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_weight: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_assessments: Option<Vec<DimensionAssessment>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordScore {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_score: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_form_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_score: Option<RecordScore>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interviewer: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_assessments: Option<Vec<ModuleAssessment>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentCustomizedOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentCustomizedTimeRange {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentCustomizedAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentCustomizedValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<TalentCustomizedOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option_list: Option<Vec<TalentCustomizedOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TalentCustomizedTimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_attachment: Option<Vec<TalentCustomizedAttachment>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentCustomizedDataChild {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<TalentCustomizedValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompositeTalentCustomizedData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TalentCustomizedDataChild>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompositeTalentBasicInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experience_years: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_location_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hometown_location_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_location_code_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identification_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identification_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birthday: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<TalentCustomizedDataChild>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hukou_location_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidential: Option<i32>,
}

macro_rules! composite_talent_text_section {
    ($name:ident { $($field:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, Default, Serialize, Deserialize)]
        pub struct $name {
            $(
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub $field: Option<String>,
            )+
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub customized_data_list: Option<Vec<TalentCustomizedDataChild>>,
        }
    };
}

composite_talent_text_section!(CompositeTalentCareerInfo {
    company_name,
    description,
    end_time,
    start_time,
    title,
});
composite_talent_text_section!(CompositeTalentInternshipInfo {
    company_name,
    description,
    end_time,
    start_time,
    title,
});
composite_talent_text_section!(CompositeTalentProjectInfo {
    project_name,
    role,
    link,
    description,
    start_time,
    end_time,
});
composite_talent_text_section!(CompositeTalentAwardInfo {
    award_name,
    award_time,
    description,
});
composite_talent_text_section!(CompositeTalentWorksInfo {
    id,
    link,
    description,
});

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompositeTalentEducationInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub academic_ranking: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<TalentCustomizedDataChild>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompositeTalentLanguageInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proficiency: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<TalentCustomizedDataChild>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompositeTalentSnsInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sns_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<TalentCustomizedDataChild>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentResumeAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentFolder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentSimilar {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_similar_talent: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similar_talent_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentPool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18n>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentNote {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompositeTalent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<CompositeTalentBasicInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education_list: Option<Vec<CompositeTalentEducationInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub career_list: Option<Vec<CompositeTalentCareerInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_list: Option<Vec<CompositeTalentProjectInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub works_list: Option<Vec<CompositeTalentWorksInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub award_list: Option<Vec<CompositeTalentAwardInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_list: Option<Vec<CompositeTalentLanguageInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sns_list: Option<Vec<CompositeTalentSnsInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_source_list: Option<Vec<TalentResumeSource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internship_list: Option<Vec<CompositeTalentInternshipInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customized_data_list: Option<Vec<CompositeTalentCustomizedData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_attachment_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_attachment_list: Option<Vec<TalentResumeAttachment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_registration_list: Option<Vec<TalentInterviewRegistrationSimple>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_list: Option<Vec<RegistrationBasicInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_onboarded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_in_agency_period: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_degree: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_pool_id_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_folder_ref_list_v2: Option<Vec<TalentFolder>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<TalentTag>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similar_info_v2: Option<TalentSimilar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_info: Option<TalentBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent_pool_ref_list_v2: Option<Vec<TalentPool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note_list_v2: Option<Vec<TalentNote>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewRecordData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interview_record: Option<InterviewRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterviewRecordListData {
    #[serde(default)]
    pub items: Vec<InterviewRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TalentV2Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talent: Option<CompositeTalent>,
}

// ── Response types ────────────────────────────────────────────────────────────

impl_resp_v2!(GetInterviewRecordV2Resp, InterviewRecordData);
impl_resp_v2!(ListInterviewRecordV2Resp, InterviewRecordListData);
impl_resp_v2!(GetTalentV2Resp, TalentV2Data);

#[derive(Debug, Clone, Copy)]
pub struct GetInterviewRecordV2Query<'a> {
    pub interview_record_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetInterviewRecordV2Query<'a> {
    pub fn new(interview_record_id: &'a str) -> Self {
        Self {
            interview_record_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListInterviewRecordV2Query<'a> {
    pub interview_id: Option<&'a str>,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListInterviewRecordV2Query<'a> {
    pub fn new() -> Self {
        Self {
            interview_id: None,
            page: PageQuery::default(),
            user_id_type: None,
        }
    }

    pub fn interview_id(mut self, interview_id: impl Into<Option<&'a str>>) -> Self {
        self.interview_id = interview_id.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

impl<'a> Default for ListInterviewRecordV2Query<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetTalentV2Query<'a> {
    pub talent_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetTalentV2Query<'a> {
    pub fn new(talent_id: &'a str) -> Self {
        Self {
            talent_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = user_id_type.into();
        self
    }
}

// ── V2 service entry ──────────────────────────────────────────────────────────

pub struct V2<'a> {
    pub interview_record: InterviewRecordV2Resource<'a>,
    pub talent: TalentV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            interview_record: InterviewRecordV2Resource { config },
            talent: TalentV2Resource { config },
        }
    }
}

// ── InterviewRecord resource ──────────────────────────────────────────────────

pub struct InterviewRecordV2Resource<'a> {
    config: &'a Config,
}

impl InterviewRecordV2Resource<'_> {
    pub async fn get(
        &self,
        interview_record_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetInterviewRecordV2Resp, LarkError> {
        self.get_by_query(
            &GetInterviewRecordV2Query::new(interview_record_id).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetInterviewRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetInterviewRecordV2Resp, LarkError> {
        let path = format!(
            "/open-apis/hire/v2/interview_records/{}",
            query.interview_record_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<InterviewRecordData, GetInterviewRecordV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        interview_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListInterviewRecordV2Resp, LarkError> {
        let query = ListInterviewRecordV2Query::new()
            .interview_id(interview_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListInterviewRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListInterviewRecordV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/hire/v2/interview_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .query("interview_id", query.interview_id)
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<InterviewRecordListData, ListInterviewRecordV2Resp>()
        .await
    }
}

// ── Talent resource ───────────────────────────────────────────────────────────

pub struct TalentV2Resource<'a> {
    config: &'a Config,
}

impl TalentV2Resource<'_> {
    pub async fn get(
        &self,
        talent_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetTalentV2Resp, LarkError> {
        self.get_by_query(
            &GetTalentV2Query::new(talent_id).user_id_type(user_id_type),
            option,
        )
        .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetTalentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetTalentV2Resp, LarkError> {
        let path = format!("/open-apis/hire/v2/talents/{}", query.talent_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<TalentV2Data, GetTalentV2Resp>()
        .await
    }
}
