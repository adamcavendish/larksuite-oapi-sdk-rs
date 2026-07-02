// Code generated from larksuite-oapi-sdk-go v3.6.1..v3.9.7 service/resource.go endpoint metadata.
// It provides checked-in REST parity for post-v3.6.1 endpoints not yet modelled by dedicated Rust resources.

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::{JsonResp, RestRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GoV397Endpoint {
    /// GET /open-apis/aily/v1/app_stats (service/aily/v1/resource.go)
    GetAilyV1AppStats,
    /// GET /open-apis/application/v5/applications/favourite (service/application/v5/resource.go)
    GetApplicationV5ApplicationsFavourite,
    /// GET /open-apis/application/v5/applications/recommend (service/application/v5/resource.go)
    GetApplicationV5ApplicationsRecommend,
    /// POST /open-apis/application/v7/app_avatar/upload (service/application/v7/resource.go)
    PostApplicationV7AppAvatarUpload,
    /// PATCH /open-apis/application/v7/applications/:app_id/ability (service/application/v7/resource.go)
    PatchApplicationV7ApplicationsByAppIdAbility,
    /// PATCH /open-apis/application/v7/applications/:app_id/base (service/application/v7/resource.go)
    PatchApplicationV7ApplicationsByAppIdBase,
    /// PATCH /open-apis/application/v7/applications/:app_id/config (service/application/v7/resource.go)
    PatchApplicationV7ApplicationsByAppIdConfig,
    /// POST /open-apis/application/v7/applications/:app_id/publish (service/application/v7/resource.go)
    PostApplicationV7ApplicationsByAppIdPublish,
    /// GET /open-apis/approval/v4/districts (service/approval/v4/resource.go)
    GetApprovalV4Districts,
    /// POST /open-apis/approval/v4/districts/search (service/approval/v4/resource.go)
    PostApprovalV4DistrictsSearch,
    /// GET /open-apis/bitable/v1/apps/:app_token/block_workflows (service/bitable/v1/resource.go)
    GetBitableV1AppsByAppTokenBlockWorkflows,
    /// POST /open-apis/bitable/v1/apps/:app_token/tables/:table_id/field_groups (service/bitable/v1/resource.go)
    PostBitableV1AppsByAppTokenTablesByTableIdFieldGroups,
    /// POST /open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/upgrade (service/bitable/v1/resource.go)
    PostBitableV1AppsByAppTokenTablesByTableIdFormsByFormIdUpgrade,
    /// DELETE /open-apis/board/v1/whiteboards/:whiteboard_id/nodes/batch_delete (service/board/v1/resource.go)
    DeleteBoardV1WhiteboardsByWhiteboardIdNodesBatchDelete,
    /// POST /open-apis/contact/v3/users/basic_batch (service/contact/v3/resource.go)
    PostContactV3UsersBasicBatch,
    /// POST /open-apis/corehr/v2/companies/query_multi_timeline (service/corehr/v2/resource.go)
    PostCorehrV2CompaniesQueryMultiTimeline,
    /// POST /open-apis/corehr/v2/cost_centers/query_multi_timeline (service/corehr/v2/resource.go)
    PostCorehrV2CostCentersQueryMultiTimeline,
    /// POST /open-apis/corehr/v2/cost_centers/tree (service/corehr/v2/resource.go)
    PostCorehrV2CostCentersTree,
    /// POST /open-apis/corehr/v2/custom_org/create_emp_custom_org (service/corehr/v2/resource.go)
    PostCorehrV2CustomOrgCreateEmpCustomOrg,
    /// POST /open-apis/corehr/v2/custom_org/del (service/corehr/v2/resource.go)
    PostCorehrV2CustomOrgDel,
    /// POST /open-apis/corehr/v2/custom_org/edit_emp_custom_org (service/corehr/v2/resource.go)
    PostCorehrV2CustomOrgEditEmpCustomOrg,
    /// GET /open-apis/corehr/v2/custom_org/employment_custom_org_record (service/corehr/v2/resource.go)
    GetCorehrV2CustomOrgEmploymentCustomOrgRecord,
    /// GET /open-apis/corehr/v2/custom_org/querybyid (service/corehr/v2/resource.go)
    GetCorehrV2CustomOrgQuerybyid,
    /// POST /open-apis/corehr/v2/locations/query_multi_timeline (service/corehr/v2/resource.go)
    PostCorehrV2LocationsQueryMultiTimeline,
    /// POST /open-apis/corehr/v2/probation/edit (service/corehr/v2/resource.go)
    PostCorehrV2ProbationEdit,
    /// POST /open-apis/corehr/v2/query_flow_data_template (service/corehr/v2/resource.go)
    PostCorehrV2QueryFlowDataTemplate,
    /// POST /open-apis/corehr/v2/process_start (service/corehr/v2/resource.go)
    PostCorehrV2ProcessStart,
    /// POST /open-apis/drive/v1/files/:file_token/comments/:comment_id/replies (service/drive/v1/resource.go)
    PostDriveV1FilesByFileTokenCommentsByCommentIdReplies,
    /// DELETE /open-apis/drive/v1/user/remove_subscription (service/drive/v1/resource.go)
    DeleteDriveV1UserRemoveSubscription,
    /// POST /open-apis/drive/v1/user/subscription (service/drive/v1/resource.go)
    PostDriveV1UserSubscription,
    /// GET /open-apis/drive/v1/user/subscription_status (service/drive/v1/resource.go)
    GetDriveV1UserSubscriptionStatus,
    /// POST /open-apis/drive/v2/files/:file_token/comments/reaction (service/drive/v2/resource.go)
    PostDriveV2FilesByFileTokenCommentsReaction,
    /// POST /open-apis/im/v1/messages/reactions/batch_query (service/im/v1/resource.go)
    PostImV1MessagesReactionsBatchQuery,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/accessible_mailboxes (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdAccessibleMailboxes,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdDrafts,
    /// DELETE /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id (service/mail/v1/resource.go)
    DeleteMailV1UserMailboxesByUserMailboxIdDraftsByDraftId,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdDraftsByDraftId,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdDrafts,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id/send (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdDraftsByDraftIdSend,
    /// PUT /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id (service/mail/v1/resource.go)
    PutMailV1UserMailboxesByUserMailboxIdDraftsByDraftId,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders/:folder_id (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdFoldersByFolderId,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdLabels,
    /// DELETE /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels/:label_id (service/mail/v1/resource.go)
    DeleteMailV1UserMailboxesByUserMailboxIdLabelsByLabelId,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels/:label_id (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdLabelsByLabelId,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdLabels,
    /// PATCH /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels/:label_id (service/mail/v1/resource.go)
    PatchMailV1UserMailboxesByUserMailboxIdLabelsByLabelId,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/batch_get (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdMessagesBatchGet,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/batch_modify (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdMessagesBatchModify,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/batch_trash (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdMessagesBatchTrash,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/:thread_id/messages (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdThreadsByThreadIdMessages,
    /// PUT /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/modify (service/mail/v1/resource.go)
    PutMailV1UserMailboxesByUserMailboxIdMessagesByMessageIdModify,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/trash (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdMessagesByMessageIdTrash,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/settings/send_as (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdSettingsSendAs,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdTemplates,
    /// DELETE /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates/:template_id (service/mail/v1/resource.go)
    DeleteMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateId,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates/:template_id (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateId,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdTemplates,
    /// PUT /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates/:template_id (service/mail/v1/resource.go)
    PutMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateId,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates/:template_id/attachments/download_url (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateIdAttachmentsDownloadUrl,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/batch_modify (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdThreadsBatchModify,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/batch_trash (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdThreadsBatchTrash,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/:thread_id (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdThreadsByThreadId,
    /// GET /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads (service/mail/v1/resource.go)
    GetMailV1UserMailboxesByUserMailboxIdThreads,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/:thread_id/modify (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdThreadsByThreadIdModify,
    /// POST /open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/:thread_id/trash (service/mail/v1/resource.go)
    PostMailV1UserMailboxesByUserMailboxIdThreadsByThreadIdTrash,
    /// GET /open-apis/minutes/v1/minutes/:minute_token/artifacts (service/minutes/v1/resource.go)
    GetMinutesV1MinutesByMinuteTokenArtifacts,
    /// POST /open-apis/minutes/v1/minutes/search (service/minutes/v1/resource.go)
    PostMinutesV1MinutesSearch,
    /// POST /open-apis/minutes/v1/minutes/subscription (service/minutes/v1/resource.go)
    PostMinutesV1MinutesSubscription,
    /// POST /open-apis/minutes/v1/minutes/unsubscription (service/minutes/v1/resource.go)
    PostMinutesV1MinutesUnsubscription,
    /// POST /open-apis/performance/v1/review_datas/query (service/performance/v1/resource.go)
    PostPerformanceV1ReviewDatasQuery,
    /// GET /open-apis/performance/v1/semesters (service/performance/v1/resource.go)
    GetPerformanceV1Semesters,
    /// POST /open-apis/performance/v1/stage_tasks/find_by_page (service/performance/v1/resource.go)
    PostPerformanceV1StageTasksFindByPage,
    /// POST /open-apis/performance/v1/stage_tasks/find_by_user_list (service/performance/v1/resource.go)
    PostPerformanceV1StageTasksFindByUserList,
    /// GET /open-apis/security_and_compliance/v1/multi_geo_entity/tenant (service/security_and_compliance/v1/resource.go)
    GetSecurityAndComplianceV1MultiGeoEntityTenant,
    /// POST /open-apis/security_and_compliance/v1/user_migrations/cancel (service/security_and_compliance/v1/resource.go)
    PostSecurityAndComplianceV1UserMigrationsCancel,
    /// POST /open-apis/security_and_compliance/v1/user_migrations (service/security_and_compliance/v1/resource.go)
    PostSecurityAndComplianceV1UserMigrations,
    /// GET /open-apis/security_and_compliance/v1/user_migrations/:user_id (service/security_and_compliance/v1/resource.go)
    GetSecurityAndComplianceV1UserMigrationsByUserId,
    /// POST /open-apis/security_and_compliance/v1/user_migrations/search (service/security_and_compliance/v1/resource.go)
    PostSecurityAndComplianceV1UserMigrationsSearch,
    /// POST /open-apis/spark/v1/apps (service/spark/v1/resource.go)
    PostSparkV1Apps,
    /// GET /open-apis/spark/v1/apps/:app_id/access-scope (service/spark/v1/resource.go)
    GetSparkV1AppsByAppIdAccessScope,
    /// POST /open-apis/spark/v1/icon (service/spark/v1/resource.go)
    PostSparkV1Icon,
    /// GET /open-apis/spark/v1/apps (service/spark/v1/resource.go)
    GetSparkV1Apps,
    /// PATCH /open-apis/spark/v1/apps/:app_id (service/spark/v1/resource.go)
    PatchSparkV1AppsByAppId,
    /// POST /open-apis/spark/v1/apps/:app_id/sql_commands (service/spark/v1/resource.go)
    PostSparkV1AppsByAppIdSqlCommands,
    /// PUT /open-apis/spark/v1/apps/:app_id/access-scope (service/spark/v1/resource.go)
    PutSparkV1AppsByAppIdAccessScope,
    /// POST /open-apis/spark/v1/apps/:app_id/upload_and_release_html_code (service/spark/v1/resource.go)
    PostSparkV1AppsByAppIdUploadAndReleaseHtmlCode,
    /// GET /open-apis/spark/v1/apps/:app_id/enums/:enum_name (service/spark/v1/resource.go)
    GetSparkV1AppsByAppIdEnumsByEnumName,
    /// GET /open-apis/spark/v1/apps/:app_id/enums (service/spark/v1/resource.go)
    GetSparkV1AppsByAppIdEnums,
    /// GET /open-apis/spark/v1/apps/:app_id/storage (service/spark/v1/resource.go)
    GetSparkV1AppsByAppIdStorage,
    /// POST /open-apis/spark/v1/apps/:app_id/storage/upload (service/spark/v1/resource.go)
    PostSparkV1AppsByAppIdStorageUpload,
    /// POST /open-apis/spark/v1/apps/:app_id/storage/upload/complete (service/spark/v1/resource.go)
    PostSparkV1AppsByAppIdStorageUploadComplete,
    /// POST /open-apis/spark/v1/apps/:app_id/storage/upload/initialize (service/spark/v1/resource.go)
    PostSparkV1AppsByAppIdStorageUploadInitialize,
    /// POST /open-apis/spark/v1/apps/:app_id/storage/upload/part (service/spark/v1/resource.go)
    PostSparkV1AppsByAppIdStorageUploadPart,
    /// PATCH /open-apis/spark/v1/apps/:app_id/tables/:table_name/records_batch_update (service/spark/v1/resource.go)
    PatchSparkV1AppsByAppIdTablesByTableNameRecordsBatchUpdate,
    /// DELETE /open-apis/spark/v1/apps/:app_id/tables/:table_name/records (service/spark/v1/resource.go)
    DeleteSparkV1AppsByAppIdTablesByTableNameRecords,
    /// GET /open-apis/spark/v1/apps/:app_id/tables/:table_name (service/spark/v1/resource.go)
    GetSparkV1AppsByAppIdTablesByTableName,
    /// GET /open-apis/spark/v1/apps/:app_id/tables (service/spark/v1/resource.go)
    GetSparkV1AppsByAppIdTables,
    /// GET /open-apis/spark/v1/apps/:app_id/tables/:table_name/records (service/spark/v1/resource.go)
    GetSparkV1AppsByAppIdTablesByTableNameRecords,
    /// PATCH /open-apis/spark/v1/apps/:app_id/tables/:table_name/records (service/spark/v1/resource.go)
    PatchSparkV1AppsByAppIdTablesByTableNameRecords,
    /// POST /open-apis/spark/v1/apps/:app_id/tables/:table_name/records (service/spark/v1/resource.go)
    PostSparkV1AppsByAppIdTablesByTableNameRecords,
    /// GET /open-apis/spark/v1/apps/:app_id/views/:view_name/records (service/spark/v1/resource.go)
    GetSparkV1AppsByAppIdViewsByViewNameRecords,
    /// POST /open-apis/spark/v1/directory/user/id_convert (service/spark/v1/resource.go)
    PostSparkV1DirectoryUserIdConvert,
    /// POST /open-apis/task/v2/tasks/:task_guid/set_ancestor_task (service/task/v2/resource.go)
    PostTaskV2TasksByTaskGuidSetAncestorTask,
    /// GET /open-apis/task/v2/task_v2/list_related_task (service/task/v2/resource.go)
    GetTaskV2TaskV2ListRelatedTask,
    /// POST /open-apis/task/v2/task_v2/task_subscription (service/task/v2/resource.go)
    PostTaskV2TaskV2TaskSubscription,
    /// GET /open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key (service/trust_party/v1/resource.go)
    GetTrustPartyV1CollaborationTenantsByTargetTenantKey,
    /// GET /open-apis/trust_party/v1/collaboration_tenants (service/trust_party/v1/resource.go)
    GetTrustPartyV1CollaborationTenants,
    /// GET /open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/visible_organization (service/trust_party/v1/resource.go)
    GetTrustPartyV1CollaborationTenantsByTargetTenantKeyVisibleOrganization,
    /// GET /open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/collaboration_departments/:target_department_id (service/trust_party/v1/resource.go)
    GetTrustPartyV1CollaborationTenantsByTargetTenantKeyCollaborationDepartmentsByTargetDepartmentId,
    /// GET /open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/collaboration_users/:target_user_id (service/trust_party/v1/resource.go)
    GetTrustPartyV1CollaborationTenantsByTargetTenantKeyCollaborationUsersByTargetUserId,
    /// POST /open-apis/vc/v1/meetings/search (service/vc/v1/resource.go)
    PostVcV1MeetingsSearch,
    /// POST /open-apis/vc/v1/meetings/subscription (service/vc/v1/resource.go)
    PostVcV1MeetingsSubscription,
    /// POST /open-apis/vc/v1/meetings/unsubscription (service/vc/v1/resource.go)
    PostVcV1MeetingsUnsubscription,
    /// GET /open-apis/vc/v1/notes/:note_id (service/vc/v1/resource.go)
    GetVcV1NotesByNoteId,
}

#[derive(Debug, Clone)]
pub struct GoV397EndpointMeta {
    pub method: http::Method,
    pub path: &'static str,
    pub supported_access_token_types: &'static [AccessTokenType],
    pub file_upload: bool,
}

impl GoV397Endpoint {
    pub fn meta(self) -> GoV397EndpointMeta {
        match self {
            Self::GetAilyV1AppStats => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/aily/v1/app_stats",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetApplicationV5ApplicationsFavourite => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/application/v5/applications/favourite",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetApplicationV5ApplicationsRecommend => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/application/v5/applications/recommend",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostApplicationV7AppAvatarUpload => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/application/v7/app_avatar/upload",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: true,
            },
            Self::PatchApplicationV7ApplicationsByAppIdAbility => GoV397EndpointMeta {
                method: http::Method::PATCH,
                path: "/open-apis/application/v7/applications/:app_id/ability",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PatchApplicationV7ApplicationsByAppIdBase => GoV397EndpointMeta {
                method: http::Method::PATCH,
                path: "/open-apis/application/v7/applications/:app_id/base",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PatchApplicationV7ApplicationsByAppIdConfig => GoV397EndpointMeta {
                method: http::Method::PATCH,
                path: "/open-apis/application/v7/applications/:app_id/config",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PostApplicationV7ApplicationsByAppIdPublish => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/application/v7/applications/:app_id/publish",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::GetApprovalV4Districts => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/approval/v4/districts",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostApprovalV4DistrictsSearch => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/approval/v4/districts/search",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetBitableV1AppsByAppTokenBlockWorkflows => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/bitable/v1/apps/:app_token/block_workflows",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PostBitableV1AppsByAppTokenTablesByTableIdFieldGroups => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/field_groups",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostBitableV1AppsByAppTokenTablesByTableIdFormsByFormIdUpgrade => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/forms/:form_id/upgrade",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::DeleteBoardV1WhiteboardsByWhiteboardIdNodesBatchDelete => GoV397EndpointMeta {
                method: http::Method::DELETE,
                path: "/open-apis/board/v1/whiteboards/:whiteboard_id/nodes/batch_delete",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PostContactV3UsersBasicBatch => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/contact/v3/users/basic_batch",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostCorehrV2CompaniesQueryMultiTimeline => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/companies/query_multi_timeline",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostCorehrV2CostCentersQueryMultiTimeline => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/cost_centers/query_multi_timeline",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostCorehrV2CostCentersTree => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/cost_centers/tree",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PostCorehrV2CustomOrgCreateEmpCustomOrg => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/custom_org/create_emp_custom_org",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostCorehrV2CustomOrgDel => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/custom_org/del",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostCorehrV2CustomOrgEditEmpCustomOrg => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/custom_org/edit_emp_custom_org",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetCorehrV2CustomOrgEmploymentCustomOrgRecord => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/corehr/v2/custom_org/employment_custom_org_record",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetCorehrV2CustomOrgQuerybyid => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/corehr/v2/custom_org/querybyid",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostCorehrV2LocationsQueryMultiTimeline => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/locations/query_multi_timeline",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostCorehrV2ProbationEdit => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/probation/edit",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostCorehrV2QueryFlowDataTemplate => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/query_flow_data_template",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostCorehrV2ProcessStart => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/corehr/v2/process_start",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostDriveV1FilesByFileTokenCommentsByCommentIdReplies => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/drive/v1/files/:file_token/comments/:comment_id/replies",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::DeleteDriveV1UserRemoveSubscription => GoV397EndpointMeta {
                method: http::Method::DELETE,
                path: "/open-apis/drive/v1/user/remove_subscription",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostDriveV1UserSubscription => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/drive/v1/user/subscription",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetDriveV1UserSubscriptionStatus => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/drive/v1/user/subscription_status",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostDriveV2FilesByFileTokenCommentsReaction => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/drive/v2/files/:file_token/comments/reaction",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostImV1MessagesReactionsBatchQuery => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/im/v1/messages/reactions/batch_query",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdAccessibleMailboxes => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/accessible_mailboxes",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdDrafts => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::DeleteMailV1UserMailboxesByUserMailboxIdDraftsByDraftId => GoV397EndpointMeta {
                method: http::Method::DELETE,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdDraftsByDraftId => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdDrafts => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdDraftsByDraftIdSend => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id/send",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PutMailV1UserMailboxesByUserMailboxIdDraftsByDraftId => GoV397EndpointMeta {
                method: http::Method::PUT,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/drafts/:draft_id",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdFoldersByFolderId => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/folders/:folder_id",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdLabels => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::DeleteMailV1UserMailboxesByUserMailboxIdLabelsByLabelId => GoV397EndpointMeta {
                method: http::Method::DELETE,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels/:label_id",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdLabelsByLabelId => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels/:label_id",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdLabels => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PatchMailV1UserMailboxesByUserMailboxIdLabelsByLabelId => GoV397EndpointMeta {
                method: http::Method::PATCH,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/labels/:label_id",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdMessagesBatchGet => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/batch_get",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdMessagesBatchModify => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/batch_modify",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdMessagesBatchTrash => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/batch_trash",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdThreadsByThreadIdMessages => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/:thread_id/messages",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PutMailV1UserMailboxesByUserMailboxIdMessagesByMessageIdModify => GoV397EndpointMeta {
                method: http::Method::PUT,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/modify",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdMessagesByMessageIdTrash => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/messages/:message_id/trash",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdSettingsSendAs => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/settings/send_as",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdTemplates => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::DeleteMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateId => GoV397EndpointMeta {
                method: http::Method::DELETE,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates/:template_id",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateId => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates/:template_id",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdTemplates => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PutMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateId => GoV397EndpointMeta {
                method: http::Method::PUT,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates/:template_id",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateIdAttachmentsDownloadUrl => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/templates/:template_id/attachments/download_url",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdThreadsBatchModify => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/batch_modify",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdThreadsBatchTrash => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/batch_trash",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdThreadsByThreadId => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/:thread_id",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetMailV1UserMailboxesByUserMailboxIdThreads => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdThreadsByThreadIdModify => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/:thread_id/modify",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostMailV1UserMailboxesByUserMailboxIdThreadsByThreadIdTrash => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/mail/v1/user_mailboxes/:user_mailbox_id/threads/:thread_id/trash",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetMinutesV1MinutesByMinuteTokenArtifacts => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/minutes/v1/minutes/:minute_token/artifacts",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostMinutesV1MinutesSearch => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/minutes/v1/minutes/search",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostMinutesV1MinutesSubscription => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/minutes/v1/minutes/subscription",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostMinutesV1MinutesUnsubscription => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/minutes/v1/minutes/unsubscription",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostPerformanceV1ReviewDatasQuery => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/performance/v1/review_datas/query",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetPerformanceV1Semesters => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/performance/v1/semesters",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostPerformanceV1StageTasksFindByPage => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/performance/v1/stage_tasks/find_by_page",
                supported_access_token_types: &[AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostPerformanceV1StageTasksFindByUserList => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/performance/v1/stage_tasks/find_by_user_list",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetSecurityAndComplianceV1MultiGeoEntityTenant => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/security_and_compliance/v1/multi_geo_entity/tenant",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostSecurityAndComplianceV1UserMigrationsCancel => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/security_and_compliance/v1/user_migrations/cancel",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostSecurityAndComplianceV1UserMigrations => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/security_and_compliance/v1/user_migrations",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetSecurityAndComplianceV1UserMigrationsByUserId => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/security_and_compliance/v1/user_migrations/:user_id",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostSecurityAndComplianceV1UserMigrationsSearch => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/security_and_compliance/v1/user_migrations/search",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostSparkV1Apps => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/apps",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetSparkV1AppsByAppIdAccessScope => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/spark/v1/apps/:app_id/access-scope",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostSparkV1Icon => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/icon",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: true,
            },
            Self::GetSparkV1Apps => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/spark/v1/apps",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PatchSparkV1AppsByAppId => GoV397EndpointMeta {
                method: http::Method::PATCH,
                path: "/open-apis/spark/v1/apps/:app_id",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostSparkV1AppsByAppIdSqlCommands => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/apps/:app_id/sql_commands",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PutSparkV1AppsByAppIdAccessScope => GoV397EndpointMeta {
                method: http::Method::PUT,
                path: "/open-apis/spark/v1/apps/:app_id/access-scope",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostSparkV1AppsByAppIdUploadAndReleaseHtmlCode => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/apps/:app_id/upload_and_release_html_code",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: true,
            },
            Self::GetSparkV1AppsByAppIdEnumsByEnumName => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/spark/v1/apps/:app_id/enums/:enum_name",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetSparkV1AppsByAppIdEnums => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/spark/v1/apps/:app_id/enums",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetSparkV1AppsByAppIdStorage => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/spark/v1/apps/:app_id/storage",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: true,
            },
            Self::PostSparkV1AppsByAppIdStorageUpload => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/apps/:app_id/storage/upload",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: true,
            },
            Self::PostSparkV1AppsByAppIdStorageUploadComplete => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/apps/:app_id/storage/upload/complete",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostSparkV1AppsByAppIdStorageUploadInitialize => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/apps/:app_id/storage/upload/initialize",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostSparkV1AppsByAppIdStorageUploadPart => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/apps/:app_id/storage/upload/part",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: true,
            },
            Self::PatchSparkV1AppsByAppIdTablesByTableNameRecordsBatchUpdate => GoV397EndpointMeta {
                method: http::Method::PATCH,
                path: "/open-apis/spark/v1/apps/:app_id/tables/:table_name/records_batch_update",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::DeleteSparkV1AppsByAppIdTablesByTableNameRecords => GoV397EndpointMeta {
                method: http::Method::DELETE,
                path: "/open-apis/spark/v1/apps/:app_id/tables/:table_name/records",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetSparkV1AppsByAppIdTablesByTableName => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/spark/v1/apps/:app_id/tables/:table_name",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetSparkV1AppsByAppIdTables => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/spark/v1/apps/:app_id/tables",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetSparkV1AppsByAppIdTablesByTableNameRecords => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/spark/v1/apps/:app_id/tables/:table_name/records",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PatchSparkV1AppsByAppIdTablesByTableNameRecords => GoV397EndpointMeta {
                method: http::Method::PATCH,
                path: "/open-apis/spark/v1/apps/:app_id/tables/:table_name/records",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostSparkV1AppsByAppIdTablesByTableNameRecords => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/apps/:app_id/tables/:table_name/records",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetSparkV1AppsByAppIdViewsByViewNameRecords => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/spark/v1/apps/:app_id/views/:view_name/records",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostSparkV1DirectoryUserIdConvert => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/spark/v1/directory/user/id_convert",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::PostTaskV2TasksByTaskGuidSetAncestorTask => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/task/v2/tasks/:task_guid/set_ancestor_task",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetTaskV2TaskV2ListRelatedTask => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/task/v2/task_v2/list_related_task",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostTaskV2TaskV2TaskSubscription => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/task/v2/task_v2/task_subscription",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetTrustPartyV1CollaborationTenantsByTargetTenantKey => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::GetTrustPartyV1CollaborationTenants => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/trust_party/v1/collaboration_tenants",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::GetTrustPartyV1CollaborationTenantsByTargetTenantKeyVisibleOrganization => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/visible_organization",
                supported_access_token_types: &[AccessTokenType::User, AccessTokenType::Tenant],
                file_upload: false,
            },
            Self::GetTrustPartyV1CollaborationTenantsByTargetTenantKeyCollaborationDepartmentsByTargetDepartmentId => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/collaboration_departments/:target_department_id",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::GetTrustPartyV1CollaborationTenantsByTargetTenantKeyCollaborationUsersByTargetUserId => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/trust_party/v1/collaboration_tenants/:target_tenant_key/collaboration_users/:target_user_id",
                supported_access_token_types: &[AccessTokenType::Tenant, AccessTokenType::User],
                file_upload: false,
            },
            Self::PostVcV1MeetingsSearch => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/vc/v1/meetings/search",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostVcV1MeetingsSubscription => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/vc/v1/meetings/subscription",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::PostVcV1MeetingsUnsubscription => GoV397EndpointMeta {
                method: http::Method::POST,
                path: "/open-apis/vc/v1/meetings/unsubscription",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
            Self::GetVcV1NotesByNoteId => GoV397EndpointMeta {
                method: http::Method::GET,
                path: "/open-apis/vc/v1/notes/:note_id",
                supported_access_token_types: &[AccessTokenType::User],
                file_upload: false,
            },
        }
    }
}

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
        body: Option<&serde_json::Value>,
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
    fn endpoint_metadata_covers_generated_gap_set() {
        let endpoints = [
            GoV397Endpoint::GetAilyV1AppStats,
            GoV397Endpoint::GetApplicationV5ApplicationsFavourite,
            GoV397Endpoint::GetApplicationV5ApplicationsRecommend,
            GoV397Endpoint::PostApplicationV7AppAvatarUpload,
            GoV397Endpoint::PatchApplicationV7ApplicationsByAppIdAbility,
            GoV397Endpoint::PatchApplicationV7ApplicationsByAppIdBase,
            GoV397Endpoint::PatchApplicationV7ApplicationsByAppIdConfig,
            GoV397Endpoint::PostApplicationV7ApplicationsByAppIdPublish,
            GoV397Endpoint::GetApprovalV4Districts,
            GoV397Endpoint::PostApprovalV4DistrictsSearch,
            GoV397Endpoint::GetBitableV1AppsByAppTokenBlockWorkflows,
            GoV397Endpoint::PostBitableV1AppsByAppTokenTablesByTableIdFieldGroups,
            GoV397Endpoint::PostBitableV1AppsByAppTokenTablesByTableIdFormsByFormIdUpgrade,
            GoV397Endpoint::DeleteBoardV1WhiteboardsByWhiteboardIdNodesBatchDelete,
            GoV397Endpoint::PostContactV3UsersBasicBatch,
            GoV397Endpoint::PostCorehrV2CompaniesQueryMultiTimeline,
            GoV397Endpoint::PostCorehrV2CostCentersQueryMultiTimeline,
            GoV397Endpoint::PostCorehrV2CostCentersTree,
            GoV397Endpoint::PostCorehrV2CustomOrgCreateEmpCustomOrg,
            GoV397Endpoint::PostCorehrV2CustomOrgDel,
            GoV397Endpoint::PostCorehrV2CustomOrgEditEmpCustomOrg,
            GoV397Endpoint::GetCorehrV2CustomOrgEmploymentCustomOrgRecord,
            GoV397Endpoint::GetCorehrV2CustomOrgQuerybyid,
            GoV397Endpoint::PostCorehrV2LocationsQueryMultiTimeline,
            GoV397Endpoint::PostCorehrV2ProbationEdit,
            GoV397Endpoint::PostCorehrV2QueryFlowDataTemplate,
            GoV397Endpoint::PostCorehrV2ProcessStart,
            GoV397Endpoint::PostDriveV1FilesByFileTokenCommentsByCommentIdReplies,
            GoV397Endpoint::DeleteDriveV1UserRemoveSubscription,
            GoV397Endpoint::PostDriveV1UserSubscription,
            GoV397Endpoint::GetDriveV1UserSubscriptionStatus,
            GoV397Endpoint::PostDriveV2FilesByFileTokenCommentsReaction,
            GoV397Endpoint::PostImV1MessagesReactionsBatchQuery,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdAccessibleMailboxes,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdDrafts,
            GoV397Endpoint::DeleteMailV1UserMailboxesByUserMailboxIdDraftsByDraftId,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdDraftsByDraftId,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdDrafts,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdDraftsByDraftIdSend,
            GoV397Endpoint::PutMailV1UserMailboxesByUserMailboxIdDraftsByDraftId,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdFoldersByFolderId,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdLabels,
            GoV397Endpoint::DeleteMailV1UserMailboxesByUserMailboxIdLabelsByLabelId,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdLabelsByLabelId,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdLabels,
            GoV397Endpoint::PatchMailV1UserMailboxesByUserMailboxIdLabelsByLabelId,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdMessagesBatchGet,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdMessagesBatchModify,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdMessagesBatchTrash,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdThreadsByThreadIdMessages,
            GoV397Endpoint::PutMailV1UserMailboxesByUserMailboxIdMessagesByMessageIdModify,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdMessagesByMessageIdTrash,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdSettingsSendAs,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdTemplates,
            GoV397Endpoint::DeleteMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateId,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateId,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdTemplates,
            GoV397Endpoint::PutMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateId,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdTemplatesByTemplateIdAttachmentsDownloadUrl,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdThreadsBatchModify,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdThreadsBatchTrash,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdThreadsByThreadId,
            GoV397Endpoint::GetMailV1UserMailboxesByUserMailboxIdThreads,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdThreadsByThreadIdModify,
            GoV397Endpoint::PostMailV1UserMailboxesByUserMailboxIdThreadsByThreadIdTrash,
            GoV397Endpoint::GetMinutesV1MinutesByMinuteTokenArtifacts,
            GoV397Endpoint::PostMinutesV1MinutesSearch,
            GoV397Endpoint::PostMinutesV1MinutesSubscription,
            GoV397Endpoint::PostMinutesV1MinutesUnsubscription,
            GoV397Endpoint::PostPerformanceV1ReviewDatasQuery,
            GoV397Endpoint::GetPerformanceV1Semesters,
            GoV397Endpoint::PostPerformanceV1StageTasksFindByPage,
            GoV397Endpoint::PostPerformanceV1StageTasksFindByUserList,
            GoV397Endpoint::GetSecurityAndComplianceV1MultiGeoEntityTenant,
            GoV397Endpoint::PostSecurityAndComplianceV1UserMigrationsCancel,
            GoV397Endpoint::PostSecurityAndComplianceV1UserMigrations,
            GoV397Endpoint::GetSecurityAndComplianceV1UserMigrationsByUserId,
            GoV397Endpoint::PostSecurityAndComplianceV1UserMigrationsSearch,
            GoV397Endpoint::PostSparkV1Apps,
            GoV397Endpoint::GetSparkV1AppsByAppIdAccessScope,
            GoV397Endpoint::PostSparkV1Icon,
            GoV397Endpoint::GetSparkV1Apps,
            GoV397Endpoint::PatchSparkV1AppsByAppId,
            GoV397Endpoint::PostSparkV1AppsByAppIdSqlCommands,
            GoV397Endpoint::PutSparkV1AppsByAppIdAccessScope,
            GoV397Endpoint::PostSparkV1AppsByAppIdUploadAndReleaseHtmlCode,
            GoV397Endpoint::GetSparkV1AppsByAppIdEnumsByEnumName,
            GoV397Endpoint::GetSparkV1AppsByAppIdEnums,
            GoV397Endpoint::GetSparkV1AppsByAppIdStorage,
            GoV397Endpoint::PostSparkV1AppsByAppIdStorageUpload,
            GoV397Endpoint::PostSparkV1AppsByAppIdStorageUploadComplete,
            GoV397Endpoint::PostSparkV1AppsByAppIdStorageUploadInitialize,
            GoV397Endpoint::PostSparkV1AppsByAppIdStorageUploadPart,
            GoV397Endpoint::PatchSparkV1AppsByAppIdTablesByTableNameRecordsBatchUpdate,
            GoV397Endpoint::DeleteSparkV1AppsByAppIdTablesByTableNameRecords,
            GoV397Endpoint::GetSparkV1AppsByAppIdTablesByTableName,
            GoV397Endpoint::GetSparkV1AppsByAppIdTables,
            GoV397Endpoint::GetSparkV1AppsByAppIdTablesByTableNameRecords,
            GoV397Endpoint::PatchSparkV1AppsByAppIdTablesByTableNameRecords,
            GoV397Endpoint::PostSparkV1AppsByAppIdTablesByTableNameRecords,
            GoV397Endpoint::GetSparkV1AppsByAppIdViewsByViewNameRecords,
            GoV397Endpoint::PostSparkV1DirectoryUserIdConvert,
            GoV397Endpoint::PostTaskV2TasksByTaskGuidSetAncestorTask,
            GoV397Endpoint::GetTaskV2TaskV2ListRelatedTask,
            GoV397Endpoint::PostTaskV2TaskV2TaskSubscription,
            GoV397Endpoint::GetTrustPartyV1CollaborationTenantsByTargetTenantKey,
            GoV397Endpoint::GetTrustPartyV1CollaborationTenants,
            GoV397Endpoint::GetTrustPartyV1CollaborationTenantsByTargetTenantKeyVisibleOrganization,
            GoV397Endpoint::GetTrustPartyV1CollaborationTenantsByTargetTenantKeyCollaborationDepartmentsByTargetDepartmentId,
            GoV397Endpoint::GetTrustPartyV1CollaborationTenantsByTargetTenantKeyCollaborationUsersByTargetUserId,
            GoV397Endpoint::PostVcV1MeetingsSearch,
            GoV397Endpoint::PostVcV1MeetingsSubscription,
            GoV397Endpoint::PostVcV1MeetingsUnsubscription,
            GoV397Endpoint::GetVcV1NotesByNoteId,
        ];
        assert_eq!(endpoints.len(), 114);
        for endpoint in endpoints {
            let meta = endpoint.meta();
            assert!(meta.path.starts_with("/open-apis/"));
            assert!(!meta.supported_access_token_types.is_empty());
        }
    }
}
