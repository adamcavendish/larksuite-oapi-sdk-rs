use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentV2>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_task_guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtask_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_milestone: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_task_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_task_progress: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_deliveries: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_deliveries: Option<Vec<AttachmentV2>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_task_guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskV2Due>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<TaskV2Reminder>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<TaskV2Member>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskV2Member>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<TaskV2Origin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasklists: Option<Vec<TaskV2TasklistInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_complete: Option<TaskV2CustomComplete>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<TaskV2Start>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<TaskV2CustomFieldValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<TaskV2Dependency>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee_related: Option<Vec<TaskV2Assignee>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub positive_reminders: Option<Vec<TaskV2Reminder>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cover: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<TaskV2ResourceRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploader: Option<TaskV2Member>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<TaskV2Member>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_setting: Option<TaskV2NumberSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_setting: Option<TaskV2MemberSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datetime_setting: Option<TaskV2DatetimeSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_select_setting: Option<TaskV2SelectSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_select_setting: Option<TaskV2SelectSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<TaskV2Member>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_setting: Option<TaskV2TextSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldOptionV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SectionV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<TaskV2Member>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasklist: Option<TasklistV2Summary>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasklistV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_msec: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<TaskV2Member>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaskV2Member>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskV2Member>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivitySubscriptionV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_keys: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<TaskV2Member>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Member {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Due {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Reminder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_fire_minute: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2I18nText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_hk: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_tw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_fr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub it_it: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub de_de: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ru_ru: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub th_th: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub es_es: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ko_kr: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Href {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Origin {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform_i18n_name: Option<TaskV2I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<TaskV2Href>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2TasklistInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasklist_guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_guid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2CustomComplete {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc: Option<TaskV2CustomCompleteItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ios: Option<TaskV2CustomCompleteItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android: Option<TaskV2CustomCompleteItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2CustomCompleteItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tip: Option<TaskV2I18nText>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Start {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2CustomFieldValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datetime_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_value: Option<Vec<TaskV2Member>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_select_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_select_value: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Dependency {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_guid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Assignee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2ResourceRef {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2NumberSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_symbol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_symbol_position: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2MemberSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2DatetimeSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2SelectSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomFieldOptionV2>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2TextSetting {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasklistV2Summary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// ── Response data types ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2ListData {
    #[serde(default)]
    pub items: Vec<TaskV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<AttachmentV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentListData {
    #[serde(default)]
    pub items: Vec<AttachmentV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentV2Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<CommentV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommentV2ListData {
    #[serde(default)]
    pub items: Vec<CommentV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_field: Option<CustomFieldV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldListData {
    #[serde(default)]
    pub items: Vec<CustomFieldV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomFieldOptionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<CustomFieldOptionV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SectionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<SectionV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SectionListData {
    #[serde(default)]
    pub items: Vec<SectionV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasklistData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasklist: Option<TasklistV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TasklistListData {
    #[serde(default)]
    pub items: Vec<TasklistV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivitySubscriptionData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_subscription: Option<ActivitySubscriptionV2>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivitySubscriptionListData {
    #[serde(default)]
    pub items: Vec<ActivitySubscriptionV2>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// ── Mutation request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2InputComment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2InputOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2InputSelectSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<TaskV2InputOption>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2InputCustomField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_setting: Option<TaskV2NumberSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_setting: Option<TaskV2MemberSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datetime_setting: Option<TaskV2DatetimeSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_select_setting: Option<TaskV2InputSelectSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_select_setting: Option<TaskV2InputSelectSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_setting: Option<TaskV2TextSetting>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2InputCustomFieldValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_value: Option<Vec<TaskV2Member>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datetime_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_select_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_select_value: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2DocxSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2InputTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskV2Due>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<TaskV2Origin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskV2Member>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_complete: Option<TaskV2CustomComplete>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasklists: Option<Vec<TaskV2TasklistInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<TaskV2Start>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<TaskV2Reminder>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_milestone: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<TaskV2InputCustomFieldValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docx_source: Option<TaskV2DocxSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub positive_reminders: Option<Vec<TaskV2Reminder>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_task_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_task_progress: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_deliveries: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2InputTasklist {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskV2Member>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaskV2Member>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_tasklist: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchTaskV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskV2InputTask>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchCommentV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<TaskV2InputComment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddOrRemoveCustomFieldV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchCustomFieldV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_field: Option<TaskV2InputCustomField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchCustomFieldOptionV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub option: Option<TaskV2InputOption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2InputSection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insert_before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insert_after: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchSectionV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<TaskV2InputSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependenciesTaskV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<TaskV2Dependency>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddMembersTaskV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskV2Member>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MembersTaskV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskV2Member>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemindersTaskV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<TaskV2Reminder>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveRemindersTaskV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminder_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddTasklistTaskV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasklist_guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_guid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveTasklistTaskV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasklist_guid: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAncestorTaskV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ancestor_guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchTasklistV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasklist: Option<TaskV2InputTasklist>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_owner_to_role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskV2ActivitySubscriptionInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<TaskV2Member>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_keys: Option<Vec<i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchActivitySubscriptionV2ReqBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_subscription: Option<TaskV2ActivitySubscriptionInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_fields: Option<Vec<String>>,
}

// ── Params types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default)]
pub struct TaskListParams<'a> {
    pub page_size: Option<i32>,
    pub page_token: Option<&'a str>,
    pub completed: Option<bool>,
    pub created_from: Option<&'a str>,
    pub created_to: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> TaskListParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page_size = page.page_size;
        self.page_token = page.page_token;
        self
    }

    pub fn completed(mut self, value: impl Into<Option<bool>>) -> Self {
        self.completed = value.into();
        self
    }

    pub fn created_from(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.created_from = value.into();
        self
    }

    pub fn created_to(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.created_to = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListTaskV2Query<'a> {
    pub page: PageQuery<'a>,
    pub completed: Option<bool>,
    pub task_type: Option<&'a str>,
    pub agent_task_status: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListTaskV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn completed(mut self, value: impl Into<Option<bool>>) -> Self {
        self.completed = value.into();
        self
    }

    pub fn task_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.task_type = value.into();
        self
    }

    pub fn agent_task_status(mut self, value: impl Into<Option<i32>>) -> Self {
        self.agent_task_status = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateTaskV2Query<'a> {
    pub body: &'a TaskV2InputTask,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateTaskV2Query<'a> {
    pub fn new(body: &'a TaskV2InputTask) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetTaskV2Query<'a> {
    pub task_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetTaskV2Query<'a> {
    pub fn new(task_guid: &'a str) -> Self {
        Self {
            task_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PatchTaskV2Query<'a> {
    pub task_guid: &'a str,
    pub body: &'a PatchTaskV2ReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchTaskV2Query<'a> {
    pub fn new(task_guid: &'a str, body: &'a PatchTaskV2ReqBody) -> Self {
        Self {
            task_guid,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteTaskV2Query<'a> {
    pub task_guid: &'a str,
}

impl<'a> DeleteTaskV2Query<'a> {
    pub fn new(task_guid: &'a str) -> Self {
        Self { task_guid }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TaskBodyV2Query<'a, B> {
    pub task_guid: &'a str,
    pub body: &'a B,
}

impl<'a, B> TaskBodyV2Query<'a, B> {
    pub fn new(task_guid: &'a str, body: &'a B) -> Self {
        Self { task_guid, body }
    }
}

pub type AddMembersTaskV2Query<'a> = TaskBodyV2Query<'a, AddMembersTaskV2ReqBody>;
pub type RemoveMembersTaskV2Query<'a> = TaskBodyV2Query<'a, MembersTaskV2ReqBody>;
pub type AddRemindersTaskV2Query<'a> = TaskBodyV2Query<'a, RemindersTaskV2ReqBody>;
pub type RemoveRemindersTaskV2Query<'a> = TaskBodyV2Query<'a, RemoveRemindersTaskV2ReqBody>;
pub type AddDependenciesTaskV2Query<'a> = TaskBodyV2Query<'a, DependenciesTaskV2ReqBody>;
pub type RemoveDependenciesTaskV2Query<'a> = TaskBodyV2Query<'a, DependenciesTaskV2ReqBody>;
pub type AddTasklistTaskV2Query<'a> = TaskBodyV2Query<'a, AddTasklistTaskV2ReqBody>;
pub type RemoveTasklistTaskV2Query<'a> = TaskBodyV2Query<'a, RemoveTasklistTaskV2ReqBody>;

#[derive(Debug, Clone, Copy)]
pub struct TasklistsTaskV2Query<'a> {
    pub task_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> TasklistsTaskV2Query<'a> {
    pub fn new(task_guid: &'a str) -> Self {
        Self {
            task_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateTaskSubtaskV2Query<'a> {
    pub task_guid: &'a str,
    pub body: &'a TaskV2InputTask,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateTaskSubtaskV2Query<'a> {
    pub fn new(task_guid: &'a str, body: &'a TaskV2InputTask) -> Self {
        Self {
            task_guid,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListTaskSubtaskV2Query<'a> {
    pub task_guid: &'a str,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListTaskSubtaskV2Query<'a> {
    pub fn new(task_guid: &'a str) -> Self {
        Self {
            task_guid,
            page: PageQuery::default(),
            user_id_type: None,
        }
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UploadAttachmentV2Query<'a> {
    pub body: &'a crate::JsonValue,
}

impl<'a> UploadAttachmentV2Query<'a> {
    pub fn new(body: &'a crate::JsonValue) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetAttachmentV2Query<'a> {
    pub attachment_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetAttachmentV2Query<'a> {
    pub fn new(attachment_guid: &'a str) -> Self {
        Self {
            attachment_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteAttachmentV2Query<'a> {
    pub attachment_guid: &'a str,
}

impl<'a> DeleteAttachmentV2Query<'a> {
    pub fn new(attachment_guid: &'a str) -> Self {
        Self { attachment_guid }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListAttachmentV2Query<'a> {
    pub resource_type: Option<&'a str>,
    pub resource_id: Option<&'a str>,
    pub updated_mesc: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListAttachmentV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_type = value.into();
        self
    }

    pub fn resource_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_id = value.into();
        self
    }

    pub fn updated_mesc(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.updated_mesc = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateCommentV2Query<'a> {
    pub body: &'a TaskV2InputComment,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateCommentV2Query<'a> {
    pub fn new(body: &'a TaskV2InputComment) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetCommentV2Query<'a> {
    pub comment_id: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetCommentV2Query<'a> {
    pub fn new(comment_id: &'a str) -> Self {
        Self {
            comment_id,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PatchCommentV2Query<'a> {
    pub comment_id: &'a str,
    pub body: &'a PatchCommentV2ReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchCommentV2Query<'a> {
    pub fn new(comment_id: &'a str, body: &'a PatchCommentV2ReqBody) -> Self {
        Self {
            comment_id,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteCommentV2Query<'a> {
    pub comment_id: &'a str,
}

impl<'a> DeleteCommentV2Query<'a> {
    pub fn new(comment_id: &'a str) -> Self {
        Self { comment_id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListCommentV2Query<'a> {
    pub resource_type: Option<&'a str>,
    pub resource_id: Option<&'a str>,
    pub direction: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListCommentV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_type = value.into();
        self
    }

    pub fn resource_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_id = value.into();
        self
    }

    pub fn direction(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.direction = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateCustomFieldV2Query<'a> {
    pub body: &'a TaskV2InputCustomField,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateCustomFieldV2Query<'a> {
    pub fn new(body: &'a TaskV2InputCustomField) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetCustomFieldV2Query<'a> {
    pub custom_field_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetCustomFieldV2Query<'a> {
    pub fn new(custom_field_guid: &'a str) -> Self {
        Self {
            custom_field_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PatchCustomFieldV2Query<'a> {
    pub custom_field_guid: &'a str,
    pub body: &'a PatchCustomFieldV2ReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchCustomFieldV2Query<'a> {
    pub fn new(custom_field_guid: &'a str, body: &'a PatchCustomFieldV2ReqBody) -> Self {
        Self {
            custom_field_guid,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListCustomFieldV2Query<'a> {
    pub resource_type: Option<&'a str>,
    pub resource_id: Option<&'a str>,
    pub update_msec: Option<&'a str>,
    pub user_id_type: Option<&'a str>,
    pub page: PageQuery<'a>,
}

impl<'a> ListCustomFieldV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_type = value.into();
        self
    }

    pub fn resource_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_id = value.into();
        self
    }

    pub fn update_msec(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.update_msec = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CustomFieldBodyV2Query<'a, B> {
    pub custom_field_guid: &'a str,
    pub body: &'a B,
}

impl<'a, B> CustomFieldBodyV2Query<'a, B> {
    pub fn new(custom_field_guid: &'a str, body: &'a B) -> Self {
        Self {
            custom_field_guid,
            body,
        }
    }
}

pub type AddCustomFieldV2Query<'a> = CustomFieldBodyV2Query<'a, AddOrRemoveCustomFieldV2ReqBody>;
pub type RemoveCustomFieldV2Query<'a> = CustomFieldBodyV2Query<'a, AddOrRemoveCustomFieldV2ReqBody>;
pub type CreateCustomFieldOptionV2Query<'a> = CustomFieldBodyV2Query<'a, TaskV2InputOption>;

#[derive(Debug, Clone, Copy)]
pub struct PatchCustomFieldOptionV2Query<'a> {
    pub custom_field_guid: &'a str,
    pub option_guid: &'a str,
    pub body: &'a PatchCustomFieldOptionV2ReqBody,
}

impl<'a> PatchCustomFieldOptionV2Query<'a> {
    pub fn new(
        custom_field_guid: &'a str,
        option_guid: &'a str,
        body: &'a PatchCustomFieldOptionV2ReqBody,
    ) -> Self {
        Self {
            custom_field_guid,
            option_guid,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateSectionV2Query<'a> {
    pub body: &'a TaskV2InputSection,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateSectionV2Query<'a> {
    pub fn new(body: &'a TaskV2InputSection) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetSectionV2Query<'a> {
    pub section_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetSectionV2Query<'a> {
    pub fn new(section_guid: &'a str) -> Self {
        Self {
            section_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PatchSectionV2Query<'a> {
    pub section_guid: &'a str,
    pub body: &'a PatchSectionV2ReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchSectionV2Query<'a> {
    pub fn new(section_guid: &'a str, body: &'a PatchSectionV2ReqBody) -> Self {
        Self {
            section_guid,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteSectionV2Query<'a> {
    pub section_guid: &'a str,
}

impl<'a> DeleteSectionV2Query<'a> {
    pub fn new(section_guid: &'a str) -> Self {
        Self { section_guid }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListSectionV2Query<'a> {
    pub resource_type: Option<&'a str>,
    pub resource_id: Option<&'a str>,
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListSectionV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resource_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_type = value.into();
        self
    }

    pub fn resource_id(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.resource_id = value.into();
        self
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TasksSectionV2Query<'a> {
    pub section_guid: &'a str,
    pub params: TaskListParams<'a>,
}

impl<'a> TasksSectionV2Query<'a> {
    pub fn new(section_guid: &'a str) -> Self {
        Self {
            section_guid,
            params: TaskListParams::default(),
        }
    }

    pub fn params(mut self, params: TaskListParams<'a>) -> Self {
        self.params = params;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateTasklistV2Query<'a> {
    pub body: &'a TaskV2InputTasklist,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateTasklistV2Query<'a> {
    pub fn new(body: &'a TaskV2InputTasklist) -> Self {
        Self {
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetTasklistV2Query<'a> {
    pub tasklist_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetTasklistV2Query<'a> {
    pub fn new(tasklist_guid: &'a str) -> Self {
        Self {
            tasklist_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PatchTasklistV2Query<'a> {
    pub tasklist_guid: &'a str,
    pub body: &'a PatchTasklistV2ReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchTasklistV2Query<'a> {
    pub fn new(tasklist_guid: &'a str, body: &'a PatchTasklistV2ReqBody) -> Self {
        Self {
            tasklist_guid,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteTasklistV2Query<'a> {
    pub tasklist_guid: &'a str,
}

impl<'a> DeleteTasklistV2Query<'a> {
    pub fn new(tasklist_guid: &'a str) -> Self {
        Self { tasklist_guid }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ListTasklistV2Query<'a> {
    pub page: PageQuery<'a>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListTasklistV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TasklistBodyV2Query<'a, B> {
    pub tasklist_guid: &'a str,
    pub body: &'a B,
    pub user_id_type: Option<&'a str>,
}

impl<'a, B> TasklistBodyV2Query<'a, B> {
    pub fn new(tasklist_guid: &'a str, body: &'a B) -> Self {
        Self {
            tasklist_guid,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

pub type AddMembersTasklistV2Query<'a> = TasklistBodyV2Query<'a, MembersTaskV2ReqBody>;
pub type RemoveMembersTasklistV2Query<'a> = TasklistBodyV2Query<'a, MembersTaskV2ReqBody>;

#[derive(Debug, Clone, Copy)]
pub struct TasksTasklistV2Query<'a> {
    pub tasklist_guid: &'a str,
    pub params: TaskListParams<'a>,
}

impl<'a> TasksTasklistV2Query<'a> {
    pub fn new(tasklist_guid: &'a str) -> Self {
        Self {
            tasklist_guid,
            params: TaskListParams::default(),
        }
    }

    pub fn params(mut self, params: TaskListParams<'a>) -> Self {
        self.params = params;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateActivitySubscriptionV2Query<'a> {
    pub tasklist_guid: &'a str,
    pub body: &'a TaskV2ActivitySubscriptionInput,
    pub user_id_type: Option<&'a str>,
}

impl<'a> CreateActivitySubscriptionV2Query<'a> {
    pub fn new(tasklist_guid: &'a str, body: &'a TaskV2ActivitySubscriptionInput) -> Self {
        Self {
            tasklist_guid,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetActivitySubscriptionV2Query<'a> {
    pub tasklist_guid: &'a str,
    pub activity_subscription_guid: &'a str,
    pub user_id_type: Option<&'a str>,
}

impl<'a> GetActivitySubscriptionV2Query<'a> {
    pub fn new(tasklist_guid: &'a str, activity_subscription_guid: &'a str) -> Self {
        Self {
            tasklist_guid,
            activity_subscription_guid,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PatchActivitySubscriptionV2Query<'a> {
    pub tasklist_guid: &'a str,
    pub activity_subscription_guid: &'a str,
    pub body: &'a PatchActivitySubscriptionV2ReqBody,
    pub user_id_type: Option<&'a str>,
}

impl<'a> PatchActivitySubscriptionV2Query<'a> {
    pub fn new(
        tasklist_guid: &'a str,
        activity_subscription_guid: &'a str,
        body: &'a PatchActivitySubscriptionV2ReqBody,
    ) -> Self {
        Self {
            tasklist_guid,
            activity_subscription_guid,
            body,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListActivitySubscriptionV2Query<'a> {
    pub tasklist_guid: &'a str,
    pub limit: Option<i32>,
    pub user_id_type: Option<&'a str>,
}

impl<'a> ListActivitySubscriptionV2Query<'a> {
    pub fn new(tasklist_guid: &'a str) -> Self {
        Self {
            tasklist_guid,
            limit: None,
            user_id_type: None,
        }
    }

    pub fn limit(mut self, value: impl Into<Option<i32>>) -> Self {
        self.limit = value.into();
        self
    }

    pub fn user_id_type(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.user_id_type = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DeleteActivitySubscriptionV2Query<'a> {
    pub tasklist_guid: &'a str,
    pub activity_subscription_guid: &'a str,
}

impl<'a> DeleteActivitySubscriptionV2Query<'a> {
    pub fn new(tasklist_guid: &'a str, activity_subscription_guid: &'a str) -> Self {
        Self {
            tasklist_guid,
            activity_subscription_guid,
        }
    }
}

// ── Response types ─────────────────────────────────────────────────────────────

impl_resp_v2!(CreateTaskV2Resp, TaskV2Data);
impl_resp_v2!(GetTaskV2Resp, TaskV2Data);
impl_resp_v2!(PatchTaskV2Resp, TaskV2Data);
impl_resp_v2!(ListTaskV2Resp, TaskV2ListData);
impl_resp_v2!(AddMembersTaskV2Resp, TaskV2Data);
impl_resp_v2!(RemoveMembersTaskV2Resp, TaskV2Data);
impl_resp_v2!(AddRemindersTaskV2Resp, TaskV2Data);
impl_resp_v2!(RemoveRemindersTaskV2Resp, TaskV2Data);
impl_resp_v2!(AddDependenciesTaskV2Resp, TaskV2Data);
impl_resp_v2!(RemoveDependenciesTaskV2Resp, TaskV2Data);
impl_resp_v2!(AddTasklistTaskV2Resp, TaskV2Data);
impl_resp_v2!(RemoveTasklistTaskV2Resp, TaskV2Data);
impl_resp_v2!(TasklistsTaskV2Resp, TasklistListData);
impl_resp_v2!(CreateTaskSubtaskV2Resp, TaskV2Data);
impl_resp_v2!(ListTaskSubtaskV2Resp, TaskV2ListData);
impl_resp_v2!(DeleteTaskV2Resp, ());
impl_resp_v2!(GetAttachmentV2Resp, AttachmentData);
impl_resp_v2!(ListAttachmentV2Resp, AttachmentListData);
impl_resp_v2!(DeleteAttachmentV2Resp, ());
impl_resp_v2!(UploadAttachmentV2Resp, AttachmentData);
impl_resp_v2!(CreateCommentV2Resp, CommentV2Data);
impl_resp_v2!(GetCommentV2Resp, CommentV2Data);
impl_resp_v2!(PatchCommentV2Resp, CommentV2Data);
impl_resp_v2!(ListCommentV2Resp, CommentV2ListData);
impl_resp_v2!(DeleteCommentV2Resp, ());
impl_resp_v2!(CreateCustomFieldV2Resp, CustomFieldData);
impl_resp_v2!(GetCustomFieldV2Resp, CustomFieldData);
impl_resp_v2!(PatchCustomFieldV2Resp, CustomFieldData);
impl_resp_v2!(ListCustomFieldV2Resp, CustomFieldListData);
impl_resp_v2!(AddCustomFieldV2Resp, CustomFieldData);
impl_resp_v2!(RemoveCustomFieldV2Resp, ());
impl_resp_v2!(CreateCustomFieldOptionV2Resp, CustomFieldOptionData);
impl_resp_v2!(PatchCustomFieldOptionV2Resp, CustomFieldOptionData);
impl_resp_v2!(CreateSectionV2Resp, SectionData);
impl_resp_v2!(GetSectionV2Resp, SectionData);
impl_resp_v2!(PatchSectionV2Resp, SectionData);
impl_resp_v2!(ListSectionV2Resp, SectionListData);
impl_resp_v2!(DeleteSectionV2Resp, ());
impl_resp_v2!(TasksSectionV2Resp, TaskV2ListData);
impl_resp_v2!(CreateTasklistV2Resp, TasklistData);
impl_resp_v2!(GetTasklistV2Resp, TasklistData);
impl_resp_v2!(PatchTasklistV2Resp, TasklistData);
impl_resp_v2!(ListTasklistV2Resp, TasklistListData);
impl_resp_v2!(DeleteTasklistV2Resp, ());
impl_resp_v2!(AddMembersTasklistV2Resp, TasklistData);
impl_resp_v2!(RemoveMembersTasklistV2Resp, TasklistData);
impl_resp_v2!(TasksTasklistV2Resp, TaskV2ListData);
impl_resp_v2!(CreateActivitySubscriptionV2Resp, ActivitySubscriptionData);
impl_resp_v2!(GetActivitySubscriptionV2Resp, ActivitySubscriptionData);
impl_resp_v2!(PatchActivitySubscriptionV2Resp, ActivitySubscriptionData);
impl_resp_v2!(ListActivitySubscriptionV2Resp, ActivitySubscriptionListData);
impl_resp_v2!(DeleteActivitySubscriptionV2Resp, ());

// ── V2 service entry ──────────────────────────────────────────────────────────

pub struct V2<'a> {
    pub task: TaskV2Resource<'a>,
    pub attachment: AttachmentV2Resource<'a>,
    pub comment: CommentV2Resource<'a>,
    pub custom_field: CustomFieldV2Resource<'a>,
    pub section: SectionV2Resource<'a>,
    pub tasklist: TasklistV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            task: TaskV2Resource { config },
            attachment: AttachmentV2Resource { config },
            comment: CommentV2Resource { config },
            custom_field: CustomFieldV2Resource { config },
            section: SectionV2Resource { config },
            tasklist: TasklistV2Resource { config },
        }
    }
}

// ── Task resource ─────────────────────────────────────────────────────────────

pub struct TaskV2Resource<'a> {
    config: &'a Config,
}

impl<'a> TaskV2Resource<'a> {
    pub async fn create(
        &self,
        body: &TaskV2InputTask,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTaskV2Resp, LarkError> {
        let query = CreateTaskV2Query::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateTaskV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/task/v2/tasks",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, CreateTaskV2Resp>()
        .await
    }

    pub async fn get(
        &self,
        task_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetTaskV2Resp, LarkError> {
        let query = GetTaskV2Query::new(task_guid).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}", query.task_guid);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<TaskV2Data, GetTaskV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        task_guid: &str,
        body: &PatchTaskV2ReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchTaskV2Resp, LarkError> {
        let query = PatchTaskV2Query::new(task_guid, body).user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}", query.task_guid);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, PatchTaskV2Resp>()
        .await
    }

    pub async fn delete(
        &self,
        task_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteTaskV2Resp, LarkError> {
        self.delete_by_query(&DeleteTaskV2Query::new(task_guid), option)
            .await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}", query.task_guid);
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<(), DeleteTaskV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTaskV2Resp, LarkError> {
        let query = ListTaskV2Query::new()
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListTaskV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/tasks",
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .query("completed", query.completed)
        .query("type", query.task_type)
        .query("agent_task_status", query.agent_task_status)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<TaskV2ListData, ListTaskV2Resp>()
        .await
    }

    pub async fn add_members(
        &self,
        task_guid: &str,
        body: &AddMembersTaskV2ReqBody,
        option: &RequestOption,
    ) -> Result<AddMembersTaskV2Resp, LarkError> {
        let query = AddMembersTaskV2Query::new(task_guid, body);
        self.add_members_by_query(&query, option).await
    }

    pub async fn add_members_by_query(
        &self,
        query: &AddMembersTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<AddMembersTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}/add_members", query.task_guid);
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, AddMembersTaskV2Resp>()
        .await
    }

    pub async fn remove_members(
        &self,
        task_guid: &str,
        body: &MembersTaskV2ReqBody,
        option: &RequestOption,
    ) -> Result<RemoveMembersTaskV2Resp, LarkError> {
        let query = RemoveMembersTaskV2Query::new(task_guid, body);
        self.remove_members_by_query(&query, option).await
    }

    pub async fn remove_members_by_query(
        &self,
        query: &RemoveMembersTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<RemoveMembersTaskV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasks/{}/remove_members",
            query.task_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, RemoveMembersTaskV2Resp>()
        .await
    }

    pub async fn add_reminders(
        &self,
        task_guid: &str,
        body: &RemindersTaskV2ReqBody,
        option: &RequestOption,
    ) -> Result<AddRemindersTaskV2Resp, LarkError> {
        let query = AddRemindersTaskV2Query::new(task_guid, body);
        self.add_reminders_by_query(&query, option).await
    }

    pub async fn add_reminders_by_query(
        &self,
        query: &AddRemindersTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<AddRemindersTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}/add_reminders", query.task_guid);
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, AddRemindersTaskV2Resp>()
        .await
    }

    pub async fn remove_reminders(
        &self,
        task_guid: &str,
        body: &RemoveRemindersTaskV2ReqBody,
        option: &RequestOption,
    ) -> Result<RemoveRemindersTaskV2Resp, LarkError> {
        let query = RemoveRemindersTaskV2Query::new(task_guid, body);
        self.remove_reminders_by_query(&query, option).await
    }

    pub async fn remove_reminders_by_query(
        &self,
        query: &RemoveRemindersTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<RemoveRemindersTaskV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasks/{}/remove_reminders",
            query.task_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, RemoveRemindersTaskV2Resp>()
        .await
    }

    pub async fn add_dependencies(
        &self,
        task_guid: &str,
        body: &DependenciesTaskV2ReqBody,
        option: &RequestOption,
    ) -> Result<AddDependenciesTaskV2Resp, LarkError> {
        let query = AddDependenciesTaskV2Query::new(task_guid, body);
        self.add_dependencies_by_query(&query, option).await
    }

    pub async fn add_dependencies_by_query(
        &self,
        query: &AddDependenciesTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<AddDependenciesTaskV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasks/{}/add_dependencies",
            query.task_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, AddDependenciesTaskV2Resp>()
        .await
    }

    pub async fn remove_dependencies(
        &self,
        task_guid: &str,
        body: &DependenciesTaskV2ReqBody,
        option: &RequestOption,
    ) -> Result<RemoveDependenciesTaskV2Resp, LarkError> {
        let query = RemoveDependenciesTaskV2Query::new(task_guid, body);
        self.remove_dependencies_by_query(&query, option).await
    }

    pub async fn remove_dependencies_by_query(
        &self,
        query: &RemoveDependenciesTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<RemoveDependenciesTaskV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasks/{}/remove_dependencies",
            query.task_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, RemoveDependenciesTaskV2Resp>()
        .await
    }

    pub async fn add_tasklist(
        &self,
        task_guid: &str,
        body: &AddTasklistTaskV2ReqBody,
        option: &RequestOption,
    ) -> Result<AddTasklistTaskV2Resp, LarkError> {
        let query = AddTasklistTaskV2Query::new(task_guid, body);
        self.add_tasklist_by_query(&query, option).await
    }

    pub async fn add_tasklist_by_query(
        &self,
        query: &AddTasklistTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<AddTasklistTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}/add_tasklist", query.task_guid);
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, AddTasklistTaskV2Resp>()
        .await
    }

    pub async fn remove_tasklist(
        &self,
        task_guid: &str,
        body: &RemoveTasklistTaskV2ReqBody,
        option: &RequestOption,
    ) -> Result<RemoveTasklistTaskV2Resp, LarkError> {
        let query = RemoveTasklistTaskV2Query::new(task_guid, body);
        self.remove_tasklist_by_query(&query, option).await
    }

    pub async fn remove_tasklist_by_query(
        &self,
        query: &RemoveTasklistTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<RemoveTasklistTaskV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasks/{}/remove_tasklist",
            query.task_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, RemoveTasklistTaskV2Resp>()
        .await
    }

    pub async fn tasklists(
        &self,
        task_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<TasklistsTaskV2Resp, LarkError> {
        let query = TasklistsTaskV2Query::new(task_guid).user_id_type(user_id_type);
        self.tasklists_by_query(&query, option).await
    }

    pub async fn tasklists_by_query(
        &self,
        query: &TasklistsTaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<TasklistsTaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}/tasklists", query.task_guid);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<TasklistListData, TasklistsTaskV2Resp>()
        .await
    }

    pub async fn create_subtask(
        &self,
        task_guid: &str,
        body: &TaskV2InputTask,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTaskSubtaskV2Resp, LarkError> {
        let query = CreateTaskSubtaskV2Query::new(task_guid, body).user_id_type(user_id_type);
        self.create_subtask_by_query(&query, option).await
    }

    pub async fn create_subtask_by_query(
        &self,
        query: &CreateTaskSubtaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateTaskSubtaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}/subtasks", query.task_guid);
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<TaskV2Data, CreateTaskSubtaskV2Resp>()
        .await
    }

    pub async fn list_subtasks(
        &self,
        task_guid: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTaskSubtaskV2Resp, LarkError> {
        let query = ListTaskSubtaskV2Query::new(task_guid)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_subtasks_by_query(&query, option).await
    }

    pub async fn list_subtasks_by_query(
        &self,
        query: &ListTaskSubtaskV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListTaskSubtaskV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasks/{}/subtasks", query.task_guid);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<TaskV2ListData, ListTaskSubtaskV2Resp>()
        .await
    }
}

// ── Attachment resource ───────────────────────────────────────────────────────

pub struct AttachmentV2Resource<'a> {
    config: &'a Config,
}

impl<'a> AttachmentV2Resource<'a> {
    pub async fn get(
        &self,
        attachment_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetAttachmentV2Resp, LarkError> {
        let query = GetAttachmentV2Query::new(attachment_guid).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetAttachmentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetAttachmentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/attachments/{}", query.attachment_guid);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<AttachmentData, GetAttachmentV2Resp>()
        .await
    }

    pub async fn delete(
        &self,
        attachment_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteAttachmentV2Resp, LarkError> {
        self.delete_by_query(&DeleteAttachmentV2Query::new(attachment_guid), option)
            .await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteAttachmentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteAttachmentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/attachments/{}", query.attachment_guid);
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<(), DeleteAttachmentV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAttachmentV2Resp, LarkError> {
        let query = ListAttachmentV2Query::new()
            .resource_type(resource_type)
            .resource_id(resource_id)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListAttachmentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListAttachmentV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/attachments",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("resource_type", query.resource_type)
        .query("resource_id", query.resource_id)
        .query("updated_mesc", query.updated_mesc)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2_response::<AttachmentListData, ListAttachmentV2Resp>()
        .await
    }

    pub async fn upload(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<UploadAttachmentV2Resp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        let query = UploadAttachmentV2Query::new(&body);
        self.upload_by_query(&query, option).await
    }

    pub async fn upload_by_query(
        &self,
        query: &UploadAttachmentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<UploadAttachmentV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/task/v2/attachments/upload",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<AttachmentData, UploadAttachmentV2Resp>()
        .await
    }
}

// ── Comment resource ──────────────────────────────────────────────────────────

pub struct CommentV2Resource<'a> {
    config: &'a Config,
}

impl<'a> CommentV2Resource<'a> {
    pub async fn create(
        &self,
        body: &TaskV2InputComment,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateCommentV2Resp, LarkError> {
        let query = CreateCommentV2Query::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateCommentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateCommentV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/task/v2/comments",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<CommentV2Data, CreateCommentV2Resp>()
        .await
    }

    pub async fn get(
        &self,
        comment_id: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetCommentV2Resp, LarkError> {
        let query = GetCommentV2Query::new(comment_id).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetCommentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetCommentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/comments/{}", query.comment_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<CommentV2Data, GetCommentV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        comment_id: &str,
        body: &PatchCommentV2ReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchCommentV2Resp, LarkError> {
        let query = PatchCommentV2Query::new(comment_id, body).user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchCommentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchCommentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/comments/{}", query.comment_id);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<CommentV2Data, PatchCommentV2Resp>()
        .await
    }

    pub async fn delete(
        &self,
        comment_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteCommentV2Resp, LarkError> {
        self.delete_by_query(&DeleteCommentV2Query::new(comment_id), option)
            .await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteCommentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteCommentV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/comments/{}", query.comment_id);
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<(), DeleteCommentV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCommentV2Resp, LarkError> {
        let query = ListCommentV2Query::new()
            .resource_type(resource_type)
            .resource_id(resource_id)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCommentV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListCommentV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/comments",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("resource_type", query.resource_type)
        .query("resource_id", query.resource_id)
        .query("direction", query.direction)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2_response::<CommentV2ListData, ListCommentV2Resp>()
        .await
    }
}

// ── CustomField resource ──────────────────────────────────────────────────────

pub struct CustomFieldV2Resource<'a> {
    config: &'a Config,
}

impl<'a> CustomFieldV2Resource<'a> {
    pub async fn create(
        &self,
        body: &TaskV2InputCustomField,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateCustomFieldV2Resp, LarkError> {
        let query = CreateCustomFieldV2Query::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateCustomFieldV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateCustomFieldV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/task/v2/custom_fields",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<CustomFieldData, CreateCustomFieldV2Resp>()
        .await
    }

    pub async fn get(
        &self,
        custom_field_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetCustomFieldV2Resp, LarkError> {
        let query = GetCustomFieldV2Query::new(custom_field_guid).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetCustomFieldV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetCustomFieldV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/custom_fields/{}",
            query.custom_field_guid
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<CustomFieldData, GetCustomFieldV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        custom_field_guid: &str,
        body: &PatchCustomFieldV2ReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchCustomFieldV2Resp, LarkError> {
        let query =
            PatchCustomFieldV2Query::new(custom_field_guid, body).user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchCustomFieldV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchCustomFieldV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/custom_fields/{}",
            query.custom_field_guid
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<CustomFieldData, PatchCustomFieldV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListCustomFieldV2Resp, LarkError> {
        let query = ListCustomFieldV2Query::new()
            .resource_type(resource_type)
            .resource_id(resource_id)
            .user_id_type(user_id_type)
            .page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListCustomFieldV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListCustomFieldV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/custom_fields",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("resource_type", query.resource_type)
        .query("resource_id", query.resource_id)
        .query("update_msec", query.update_msec)
        .query("user_id_type", query.user_id_type)
        .page_query(query.page)
        .send_v2_response::<CustomFieldListData, ListCustomFieldV2Resp>()
        .await
    }

    pub async fn add(
        &self,
        custom_field_guid: &str,
        body: &AddOrRemoveCustomFieldV2ReqBody,
        option: &RequestOption,
    ) -> Result<AddCustomFieldV2Resp, LarkError> {
        let query = AddCustomFieldV2Query::new(custom_field_guid, body);
        self.add_by_query(&query, option).await
    }

    pub async fn add_by_query(
        &self,
        query: &AddCustomFieldV2Query<'_>,
        option: &RequestOption,
    ) -> Result<AddCustomFieldV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/custom_fields/{}/add",
            query.custom_field_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<CustomFieldData, AddCustomFieldV2Resp>()
        .await
    }

    pub async fn remove(
        &self,
        custom_field_guid: &str,
        body: &AddOrRemoveCustomFieldV2ReqBody,
        option: &RequestOption,
    ) -> Result<RemoveCustomFieldV2Resp, LarkError> {
        let query = RemoveCustomFieldV2Query::new(custom_field_guid, body);
        self.remove_by_query(&query, option).await
    }

    pub async fn remove_by_query(
        &self,
        query: &RemoveCustomFieldV2Query<'_>,
        option: &RequestOption,
    ) -> Result<RemoveCustomFieldV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/custom_fields/{}/remove",
            query.custom_field_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), RemoveCustomFieldV2Resp>()
        .await
    }

    pub async fn create_option(
        &self,
        custom_field_guid: &str,
        body: &TaskV2InputOption,
        option: &RequestOption,
    ) -> Result<CreateCustomFieldOptionV2Resp, LarkError> {
        let query = CreateCustomFieldOptionV2Query::new(custom_field_guid, body);
        self.create_option_by_query(&query, option).await
    }

    pub async fn create_option_by_query(
        &self,
        query: &CreateCustomFieldOptionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateCustomFieldOptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/custom_fields/{}/options",
            query.custom_field_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<CustomFieldOptionData, CreateCustomFieldOptionV2Resp>()
        .await
    }

    pub async fn patch_option(
        &self,
        custom_field_guid: &str,
        option_guid: &str,
        body: &PatchCustomFieldOptionV2ReqBody,
        req_option: &RequestOption,
    ) -> Result<PatchCustomFieldOptionV2Resp, LarkError> {
        let query = PatchCustomFieldOptionV2Query::new(custom_field_guid, option_guid, body);
        self.patch_option_by_query(&query, req_option).await
    }

    pub async fn patch_option_by_query(
        &self,
        query: &PatchCustomFieldOptionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchCustomFieldOptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/custom_fields/{}/options/{}",
            query.custom_field_guid, query.option_guid
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<CustomFieldOptionData, PatchCustomFieldOptionV2Resp>()
        .await
    }
}

// ── Section resource ──────────────────────────────────────────────────────────

pub struct SectionV2Resource<'a> {
    config: &'a Config,
}

impl<'a> SectionV2Resource<'a> {
    pub async fn create(
        &self,
        body: &TaskV2InputSection,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateSectionV2Resp, LarkError> {
        let query = CreateSectionV2Query::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateSectionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateSectionV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/task/v2/sections",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<SectionData, CreateSectionV2Resp>()
        .await
    }

    pub async fn get(
        &self,
        section_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetSectionV2Resp, LarkError> {
        let query = GetSectionV2Query::new(section_guid).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetSectionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetSectionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/sections/{}", query.section_guid);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<SectionData, GetSectionV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        section_guid: &str,
        body: &PatchSectionV2ReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchSectionV2Resp, LarkError> {
        let query = PatchSectionV2Query::new(section_guid, body).user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchSectionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchSectionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/sections/{}", query.section_guid);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<SectionData, PatchSectionV2Resp>()
        .await
    }

    pub async fn delete(
        &self,
        section_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteSectionV2Resp, LarkError> {
        self.delete_by_query(&DeleteSectionV2Query::new(section_guid), option)
            .await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteSectionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteSectionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/sections/{}", query.section_guid);
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<(), DeleteSectionV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        resource_type: Option<&str>,
        resource_id: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSectionV2Resp, LarkError> {
        let query = ListSectionV2Query::new()
            .resource_type(resource_type)
            .resource_id(resource_id)
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListSectionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListSectionV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/sections",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("resource_type", query.resource_type)
        .query("resource_id", query.resource_id)
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<SectionListData, ListSectionV2Resp>()
        .await
    }

    pub async fn tasks(
        &self,
        section_guid: &str,
        params: TaskListParams<'_>,
        option: &RequestOption,
    ) -> Result<TasksSectionV2Resp, LarkError> {
        let query = TasksSectionV2Query::new(section_guid).params(params);
        self.tasks_by_query(&query, option).await
    }

    pub async fn tasks_by_query(
        &self,
        query: &TasksSectionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<TasksSectionV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/sections/{}/tasks", query.section_guid);
        let params = query.params;
        let page = PageQuery::from_parts(params.page_size, params.page_token);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(page)
        .query("completed", params.completed)
        .query("created_from", params.created_from)
        .query("created_to", params.created_to)
        .query("user_id_type", params.user_id_type)
        .send_v2_response::<TaskV2ListData, TasksSectionV2Resp>()
        .await
    }
}

// ── Tasklist resource ─────────────────────────────────────────────────────────

pub struct TasklistV2Resource<'a> {
    config: &'a Config,
}

impl<'a> TasklistV2Resource<'a> {
    pub async fn create(
        &self,
        body: &TaskV2InputTasklist,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateTasklistV2Resp, LarkError> {
        let query = CreateTasklistV2Query::new(body).user_id_type(user_id_type);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateTasklistV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateTasklistV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/task/v2/tasklists",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<TasklistData, CreateTasklistV2Resp>()
        .await
    }

    pub async fn get(
        &self,
        tasklist_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetTasklistV2Resp, LarkError> {
        let query = GetTasklistV2Query::new(tasklist_guid).user_id_type(user_id_type);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetTasklistV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{}", query.tasklist_guid);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<TasklistData, GetTasklistV2Resp>()
        .await
    }

    pub async fn patch(
        &self,
        tasklist_guid: &str,
        body: &PatchTasklistV2ReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchTasklistV2Resp, LarkError> {
        let query = PatchTasklistV2Query::new(tasklist_guid, body).user_id_type(user_id_type);
        self.patch_by_query(&query, option).await
    }

    pub async fn patch_by_query(
        &self,
        query: &PatchTasklistV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{}", query.tasklist_guid);
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<TasklistData, PatchTasklistV2Resp>()
        .await
    }

    pub async fn delete(
        &self,
        tasklist_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteTasklistV2Resp, LarkError> {
        self.delete_by_query(&DeleteTasklistV2Query::new(tasklist_guid), option)
            .await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteTasklistV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{}", query.tasklist_guid);
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<(), DeleteTasklistV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListTasklistV2Resp, LarkError> {
        let query = ListTasklistV2Query::new()
            .page(PageQuery::from_parts(page_size, page_token))
            .user_id_type(user_id_type);
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListTasklistV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListTasklistV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/task/v2/tasklists",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(query.page)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<TasklistListData, ListTasklistV2Resp>()
        .await
    }

    pub async fn add_members(
        &self,
        tasklist_guid: &str,
        body: &MembersTaskV2ReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<AddMembersTasklistV2Resp, LarkError> {
        let query = AddMembersTasklistV2Query::new(tasklist_guid, body).user_id_type(user_id_type);
        self.add_members_by_query(&query, option).await
    }

    pub async fn add_members_by_query(
        &self,
        query: &AddMembersTasklistV2Query<'_>,
        option: &RequestOption,
    ) -> Result<AddMembersTasklistV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{}/add_members",
            query.tasklist_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<TasklistData, AddMembersTasklistV2Resp>()
        .await
    }

    pub async fn remove_members(
        &self,
        tasklist_guid: &str,
        body: &MembersTaskV2ReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<RemoveMembersTasklistV2Resp, LarkError> {
        let query =
            RemoveMembersTasklistV2Query::new(tasklist_guid, body).user_id_type(user_id_type);
        self.remove_members_by_query(&query, option).await
    }

    pub async fn remove_members_by_query(
        &self,
        query: &RemoveMembersTasklistV2Query<'_>,
        option: &RequestOption,
    ) -> Result<RemoveMembersTasklistV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{}/remove_members",
            query.tasklist_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<TasklistData, RemoveMembersTasklistV2Resp>()
        .await
    }

    pub async fn tasks(
        &self,
        tasklist_guid: &str,
        params: TaskListParams<'_>,
        option: &RequestOption,
    ) -> Result<TasksTasklistV2Resp, LarkError> {
        let query = TasksTasklistV2Query::new(tasklist_guid).params(params);
        self.tasks_by_query(&query, option).await
    }

    pub async fn tasks_by_query(
        &self,
        query: &TasksTasklistV2Query<'_>,
        option: &RequestOption,
    ) -> Result<TasksTasklistV2Resp, LarkError> {
        let path = format!("/open-apis/task/v2/tasklists/{}/tasks", query.tasklist_guid);
        let params = query.params;
        let page = PageQuery::from_parts(params.page_size, params.page_token);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .page_query(page)
        .query("completed", params.completed)
        .query("created_from", params.created_from)
        .query("created_to", params.created_to)
        .query("user_id_type", params.user_id_type)
        .send_v2_response::<TaskV2ListData, TasksTasklistV2Resp>()
        .await
    }

    pub async fn create_activity_subscription(
        &self,
        tasklist_guid: &str,
        body: &TaskV2ActivitySubscriptionInput,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<CreateActivitySubscriptionV2Resp, LarkError> {
        let query =
            CreateActivitySubscriptionV2Query::new(tasklist_guid, body).user_id_type(user_id_type);
        self.create_activity_subscription_by_query(&query, option)
            .await
    }

    pub async fn create_activity_subscription_by_query(
        &self,
        query: &CreateActivitySubscriptionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateActivitySubscriptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{}/activity_subscriptions",
            query.tasklist_guid
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<ActivitySubscriptionData, CreateActivitySubscriptionV2Resp>()
        .await
    }

    pub async fn get_activity_subscription(
        &self,
        tasklist_guid: &str,
        activity_subscription_guid: &str,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<GetActivitySubscriptionV2Resp, LarkError> {
        let query = GetActivitySubscriptionV2Query::new(tasklist_guid, activity_subscription_guid)
            .user_id_type(user_id_type);
        self.get_activity_subscription_by_query(&query, option)
            .await
    }

    pub async fn get_activity_subscription_by_query(
        &self,
        query: &GetActivitySubscriptionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetActivitySubscriptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{}/activity_subscriptions/{}",
            query.tasklist_guid, query.activity_subscription_guid
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ActivitySubscriptionData, GetActivitySubscriptionV2Resp>()
        .await
    }

    pub async fn patch_activity_subscription(
        &self,
        tasklist_guid: &str,
        activity_subscription_guid: &str,
        body: &PatchActivitySubscriptionV2ReqBody,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<PatchActivitySubscriptionV2Resp, LarkError> {
        let query =
            PatchActivitySubscriptionV2Query::new(tasklist_guid, activity_subscription_guid, body)
                .user_id_type(user_id_type);
        self.patch_activity_subscription_by_query(&query, option)
            .await
    }

    pub async fn patch_activity_subscription_by_query(
        &self,
        query: &PatchActivitySubscriptionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<PatchActivitySubscriptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{}/activity_subscriptions/{}",
            query.tasklist_guid, query.activity_subscription_guid
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("user_id_type", query.user_id_type)
        .json_body(query.body)?
        .send_v2_response::<ActivitySubscriptionData, PatchActivitySubscriptionV2Resp>()
        .await
    }

    pub async fn list_activity_subscriptions(
        &self,
        tasklist_guid: &str,
        limit: Option<i32>,
        user_id_type: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListActivitySubscriptionV2Resp, LarkError> {
        let query = ListActivitySubscriptionV2Query::new(tasklist_guid)
            .limit(limit)
            .user_id_type(user_id_type);
        self.list_activity_subscriptions_by_query(&query, option)
            .await
    }

    pub async fn list_activity_subscriptions_by_query(
        &self,
        query: &ListActivitySubscriptionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListActivitySubscriptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{}/activity_subscriptions",
            query.tasklist_guid
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("limit", query.limit)
        .query("user_id_type", query.user_id_type)
        .send_v2_response::<ActivitySubscriptionListData, ListActivitySubscriptionV2Resp>()
        .await
    }

    pub async fn delete_activity_subscription(
        &self,
        tasklist_guid: &str,
        activity_subscription_guid: &str,
        option: &RequestOption,
    ) -> Result<DeleteActivitySubscriptionV2Resp, LarkError> {
        self.delete_activity_subscription_by_query(
            &DeleteActivitySubscriptionV2Query::new(tasklist_guid, activity_subscription_guid),
            option,
        )
        .await
    }

    pub async fn delete_activity_subscription_by_query(
        &self,
        query: &DeleteActivitySubscriptionV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteActivitySubscriptionV2Resp, LarkError> {
        let path = format!(
            "/open-apis/task/v2/tasklists/{}/activity_subscriptions/{}",
            query.tasklist_guid, query.activity_subscription_guid
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_v2_response::<(), DeleteActivitySubscriptionV2Resp>()
        .await
    }
}
