use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::RestRequest;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApaasApp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct InvokeOpenApiReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<crate::JsonValue>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvokeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_body: Option<crate::JsonValue>,
}

impl_resp!(InvokeOpenApiResp, InvokeData);

// ── v2 response types ──

impl_resp_v2!(AuditLogListResp, ());
impl_resp_v2!(DataChangeLogDetailResp, ());
impl_resp_v2!(DataChangeLogsListResp, ());
impl_resp_v2!(GetAuditLogResp, ());
impl_resp_v2!(GetEnvironmentVariableResp, ());
impl_resp_v2!(QueryEnvironmentVariableResp, ());
impl_resp_v2!(ExecuteFlowResp, ());
impl_resp_v2!(InvokeFunctionResp, ());
impl_resp_v2!(OqlQueryObjectResp, ());
impl_resp_v2!(SearchObjectResp, ());
impl_resp_v2!(BatchCreateRecordResp, ());
impl_resp_v2!(BatchDeleteRecordResp, ());
impl_resp_v2!(BatchQueryRecordResp, ());
impl_resp_v2!(BatchUpdateRecordResp, ());
impl_resp_v2!(CreateRecordResp, ());
impl_resp_v2!(DeleteRecordResp, ());
impl_resp_v2!(PatchRecordResp, ());
impl_resp_v2!(QueryRecordResp, ());
impl_resp_v2!(BatchCreateAuthorizationRecordPermissionResp, ());
impl_resp_v2!(BatchRemoveAuthorizationRecordPermissionResp, ());
impl_resp_v2!(BatchCreateAuthorizationRoleMemberResp, ());
impl_resp_v2!(BatchRemoveAuthorizationRoleMemberResp, ());
impl_resp_v2!(GetRoleMemberResp, ());
impl_resp_v2!(CancelApprovalInstanceResp, ());
impl_resp_v2!(AddAssigneeApprovalTaskResp, ());
impl_resp_v2!(AgreeApprovalTaskResp, ());
impl_resp_v2!(RejectApprovalTaskResp, ());
impl_resp_v2!(TransferApprovalTaskResp, ());
impl_resp_v2!(ListSeatActivityResp, ListSeatActivityData);
impl_resp_v2!(ListSeatAssignmentResp, ListSeatAssignmentData);
impl_resp_v2!(CcUserTaskResp, ());
impl_resp_v2!(ChatGroupUserTaskResp, ChatGroupUserTaskData);
impl_resp_v2!(ExpeditingUserTaskResp, ());
impl_resp_v2!(QueryUserTaskResp, QueryUserTaskData);
impl_resp_v2!(RollbackUserTaskResp, ());
impl_resp_v2!(RollbackPointsUserTaskResp, RollbackPointsUserTaskData);
impl_resp_v2!(SqlCommandsWorkspaceResp, SqlCommandsWorkspaceData);
impl_resp_v2!(EnumGetWorkspaceEnumResp, EnumGetWorkspaceEnumData);
impl_resp_v2!(ListWorkspaceEnumResp, ListWorkspaceEnumData);
impl_resp_v2!(ListWorkspaceTableResp, ListWorkspaceTableData);
impl_resp_v2!(RecordsBatchUpdateWorkspaceTableResp, RecordIdsData);
impl_resp_v2!(RecordsDeleteWorkspaceTableResp, ());
impl_resp_v2!(RecordsGetWorkspaceTableResp, WorkspaceRecordsData);
impl_resp_v2!(RecordsPatchWorkspaceTableResp, RecordIdsData);
impl_resp_v2!(RecordsPostWorkspaceTableResp, RecordIdsData);
impl_resp_v2!(TableGetWorkspaceTableResp, WorkspaceTableData);
impl_resp_v2!(ViewsGetWorkspaceViewResp, WorkspaceRecordsData);
impl_resp_v2!(ListAppResp, ApaasAppListData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSeatActivityData {
    #[serde(default)]
    pub items: Vec<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListSeatAssignmentData {
    #[serde(default)]
    pub items: Vec<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatGroupUserTaskData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryUserTaskData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    #[serde(default)]
    pub tasks: Vec<crate::JsonValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RollbackPointsUserTaskData {
    #[serde(default)]
    pub tasks: Vec<crate::JsonValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SqlCommandsWorkspaceData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnumGetWorkspaceEnumData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<crate::JsonValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListWorkspaceEnumData {
    #[serde(default)]
    pub items: Vec<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListWorkspaceTableData {
    #[serde(default)]
    pub items: Vec<crate::JsonValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordIdsData {
    #[serde(default)]
    pub record_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceRecordsData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceTableData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub columns: Vec<crate::JsonValue>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApaasAppListData {
    #[serde(default)]
    pub items: Vec<ApaasApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// ── Resources ──

pub struct AppResource<'a> {
    config: &'a Config,
}

impl<'a> AppResource<'a> {
    /// Invoke open API (existing method)
    pub async fn invoke_open_api(
        &self,
        namespace: &str,
        body: &InvokeOpenApiReqBody,
        option: &RequestOption,
    ) -> Result<InvokeOpenApiResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/apps/{namespace}/open_api/invoke");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_response::<InvokeData, InvokeOpenApiResp>()
        .await
    }

    /// List apps
    /// GET /open-apis/apaas/v1/apps
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/apaas/v1/apps",
            vec![AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<ApaasAppListData, ListAppResp>()
        .await
    }
}

// ── ApplicationAuditLog ──

pub struct ApplicationAuditLogResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationAuditLogResource<'a> {
    /// 获取审计日志列表
    /// GET /open-apis/apaas/v1/applications/:namespace/audit_log/audit_log_list
    pub async fn audit_log_list(
        &self,
        namespace: &str,
        body: Option<&crate::JsonValue>,
        option: &RequestOption,
    ) -> Result<AuditLogListResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/audit_log/audit_log_list");
        let mut request = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        );
        if let Some(b) = body {
            request = request.json_body(b)?;
        }
        request.send_v2_response::<(), AuditLogListResp>().await
    }

    /// 获取数据变更日志详情
    /// GET /open-apis/apaas/v1/applications/:namespace/audit_log/data_change_log_detail
    pub async fn data_change_log_detail(
        &self,
        namespace: &str,
        body: Option<&crate::JsonValue>,
        option: &RequestOption,
    ) -> Result<DataChangeLogDetailResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/audit_log/data_change_log_detail"
        );
        let mut request = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        );
        if let Some(b) = body {
            request = request.json_body(b)?;
        }
        request
            .send_v2_response::<(), DataChangeLogDetailResp>()
            .await
    }

    /// 获取数据变更日志列表
    /// GET /open-apis/apaas/v1/applications/:namespace/audit_log/data_change_logs_list
    pub async fn data_change_logs_list(
        &self,
        namespace: &str,
        body: Option<&crate::JsonValue>,
        option: &RequestOption,
    ) -> Result<DataChangeLogsListResp, LarkError> {
        let path =
            format!("/open-apis/apaas/v1/applications/{namespace}/audit_log/data_change_logs_list");
        let mut request = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        );
        if let Some(b) = body {
            request = request.json_body(b)?;
        }
        request
            .send_v2_response::<(), DataChangeLogsListResp>()
            .await
    }

    /// 获取审计日志详情
    /// GET /open-apis/apaas/v1/applications/:namespace/audit_log
    pub async fn get(
        &self,
        namespace: &str,
        body: Option<&crate::JsonValue>,
        option: &RequestOption,
    ) -> Result<GetAuditLogResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/audit_log");
        let mut request = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        );
        if let Some(b) = body {
            request = request.json_body(b)?;
        }
        request.send_v2_response::<(), GetAuditLogResp>().await
    }
}

// ── ApplicationEnvironmentVariable ──

pub struct ApplicationEnvironmentVariableResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationEnvironmentVariableResource<'a> {
    /// 查询环境变量详情
    /// GET /open-apis/apaas/v1/applications/:namespace/environment_variables/:environment_variable_api_name
    pub async fn get(
        &self,
        namespace: &str,
        environment_variable_api_name: &str,
        option: &RequestOption,
    ) -> Result<GetEnvironmentVariableResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/environment_variables/{environment_variable_api_name}"
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), GetEnvironmentVariableResp>()
        .await
    }

    /// 查询环境变量列表
    /// POST /open-apis/apaas/v1/applications/:namespace/environment_variables/query
    pub async fn query(
        &self,
        namespace: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<QueryEnvironmentVariableResp, LarkError> {
        let path =
            format!("/open-apis/apaas/v1/applications/{namespace}/environment_variables/query");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), QueryEnvironmentVariableResp>()
        .await
    }
}

// ── ApplicationFlow ──

pub struct ApplicationFlowResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationFlowResource<'a> {
    /// 流程执行接口
    /// POST /open-apis/apaas/v1/applications/:namespace/flows/:flow_id/execute
    pub async fn execute(
        &self,
        namespace: &str,
        flow_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<ExecuteFlowResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/flows/{flow_id}/execute");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), ExecuteFlowResp>()
        .await
    }
}

// ── ApplicationFunction ──

pub struct ApplicationFunctionResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationFunctionResource<'a> {
    /// 执行函数
    /// POST /open-apis/apaas/v1/applications/:namespace/functions/:function_api_name/invoke
    pub async fn invoke(
        &self,
        namespace: &str,
        function_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<InvokeFunctionResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/functions/{function_api_name}/invoke"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), InvokeFunctionResp>()
        .await
    }
}

// ── ApplicationObject ──

pub struct ApplicationObjectResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationObjectResource<'a> {
    /// 执行 OQL
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/oql_query
    pub async fn oql_query(
        &self,
        namespace: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<OqlQueryObjectResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/objects/oql_query");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), OqlQueryObjectResp>()
        .await
    }

    /// 搜索记录
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/search
    pub async fn search(
        &self,
        namespace: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<SearchObjectResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/objects/search");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), SearchObjectResp>()
        .await
    }
}

// ── ApplicationObjectRecord ──

pub struct ApplicationObjectRecordResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationObjectRecordResource<'a> {
    /// 记录批量创建
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_create
    pub async fn batch_create(
        &self,
        namespace: &str,
        object_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<BatchCreateRecordResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/batch_create"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), BatchCreateRecordResp>()
        .await
    }

    /// 记录批量删除
    /// DELETE /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_delete
    pub async fn batch_delete(
        &self,
        namespace: &str,
        object_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<BatchDeleteRecordResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/batch_delete"
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), BatchDeleteRecordResp>()
        .await
    }

    /// 批量查询对象记录
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_query
    pub async fn batch_query(
        &self,
        namespace: &str,
        object_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<BatchQueryRecordResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/batch_query"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), BatchQueryRecordResp>()
        .await
    }

    /// 记录批量更新
    /// PATCH /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_update
    pub async fn batch_update(
        &self,
        namespace: &str,
        object_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<BatchUpdateRecordResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/batch_update"
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), BatchUpdateRecordResp>()
        .await
    }

    /// 创建记录
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records
    pub async fn create(
        &self,
        namespace: &str,
        object_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateRecordResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), CreateRecordResp>()
        .await
    }

    /// 删除记录
    /// DELETE /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id
    pub async fn delete(
        &self,
        namespace: &str,
        object_api_name: &str,
        id: &str,
        option: &RequestOption,
    ) -> Result<DeleteRecordResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/{id}"
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteRecordResp>()
        .await
    }

    /// 更新记录
    /// PATCH /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id
    pub async fn patch(
        &self,
        namespace: &str,
        object_api_name: &str,
        id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<PatchRecordResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/{id}"
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), PatchRecordResp>()
        .await
    }

    /// 获取记录
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id/query
    pub async fn query(
        &self,
        namespace: &str,
        object_api_name: &str,
        id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<QueryRecordResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/{id}/query"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), QueryRecordResp>()
        .await
    }
}

// ── ApplicationRecordPermissionMember ──

pub struct ApplicationRecordPermissionMemberResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationRecordPermissionMemberResource<'a> {
    /// POST /open-apis/apaas/v1/applications/:namespace/record_permissions/:record_permission_api_name/member/batch_create_authorization
    pub async fn batch_create_authorization(
        &self,
        namespace: &str,
        record_permission_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<BatchCreateAuthorizationRecordPermissionResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/record_permissions/{record_permission_api_name}/member/batch_create_authorization"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), BatchCreateAuthorizationRecordPermissionResp>()
        .await
    }

    /// POST /open-apis/apaas/v1/applications/:namespace/record_permissions/:record_permission_api_name/member/batch_remove_authorization
    pub async fn batch_remove_authorization(
        &self,
        namespace: &str,
        record_permission_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<BatchRemoveAuthorizationRecordPermissionResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/record_permissions/{record_permission_api_name}/member/batch_remove_authorization"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), BatchRemoveAuthorizationRecordPermissionResp>()
        .await
    }
}

// ── ApplicationRoleMember ──

pub struct ApplicationRoleMemberResource<'a> {
    config: &'a Config,
}

impl<'a> ApplicationRoleMemberResource<'a> {
    /// 批量添加角色成员用户和部门
    /// POST /open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member/batch_create_authorization
    pub async fn batch_create_authorization(
        &self,
        namespace: &str,
        role_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<BatchCreateAuthorizationRoleMemberResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/roles/{role_api_name}/member/batch_create_authorization"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), BatchCreateAuthorizationRoleMemberResp>()
        .await
    }

    /// 批量删除角色成员用户和部门
    /// POST /open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member/batch_remove_authorization
    pub async fn batch_remove_authorization(
        &self,
        namespace: &str,
        role_api_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<BatchRemoveAuthorizationRoleMemberResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/roles/{role_api_name}/member/batch_remove_authorization"
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), BatchRemoveAuthorizationRoleMemberResp>()
        .await
    }

    /// GET /open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member
    pub async fn get(
        &self,
        namespace: &str,
        role_api_name: &str,
        option: &RequestOption,
    ) -> Result<GetRoleMemberResp, LarkError> {
        let path =
            format!("/open-apis/apaas/v1/applications/{namespace}/roles/{role_api_name}/member");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), GetRoleMemberResp>()
        .await
    }
}

// ── ApprovalInstance ──

pub struct ApprovalInstanceResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalInstanceResource<'a> {
    /// POST /open-apis/apaas/v1/approval_instances/:approval_instance_id/cancel
    pub async fn cancel(
        &self,
        approval_instance_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<CancelApprovalInstanceResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/approval_instances/{approval_instance_id}/cancel");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), CancelApprovalInstanceResp>()
        .await
    }
}

// ── ApprovalTask ──

pub struct ApprovalTaskResource<'a> {
    config: &'a Config,
}

impl<'a> ApprovalTaskResource<'a> {
    /// POST /open-apis/apaas/v1/approval_tasks/:approval_task_id/add_assignee
    pub async fn add_assignee(
        &self,
        approval_task_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<AddAssigneeApprovalTaskResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/approval_tasks/{approval_task_id}/add_assignee");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), AddAssigneeApprovalTaskResp>()
        .await
    }

    /// POST /open-apis/apaas/v1/approval_tasks/:approval_task_id/agree
    pub async fn agree(
        &self,
        approval_task_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<AgreeApprovalTaskResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/approval_tasks/{approval_task_id}/agree");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), AgreeApprovalTaskResp>()
        .await
    }

    /// POST /open-apis/apaas/v1/approval_tasks/:approval_task_id/reject
    pub async fn reject(
        &self,
        approval_task_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<RejectApprovalTaskResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/approval_tasks/{approval_task_id}/reject");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), RejectApprovalTaskResp>()
        .await
    }

    /// POST /open-apis/apaas/v1/approval_tasks/:approval_task_id/transfer
    pub async fn transfer(
        &self,
        approval_task_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<TransferApprovalTaskResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/approval_tasks/{approval_task_id}/transfer");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), TransferApprovalTaskResp>()
        .await
    }
}

// ── SeatActivity ──

pub struct SeatActivityResource<'a> {
    config: &'a Config,
}

impl<'a> SeatActivityResource<'a> {
    /// GET /open-apis/apaas/v1/seat_activities
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSeatActivityResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/apaas/v1/seat_activities",
            vec![AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<ListSeatActivityData, ListSeatActivityResp>()
        .await
    }
}

// ── SeatAssignment ──

pub struct SeatAssignmentResource<'a> {
    config: &'a Config,
}

impl<'a> SeatAssignmentResource<'a> {
    /// GET /open-apis/apaas/v1/seat_assignments
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListSeatAssignmentResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/apaas/v1/seat_assignments",
            vec![AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<ListSeatAssignmentData, ListSeatAssignmentResp>()
        .await
    }
}

// ── UserTask ──

pub struct UserTaskResource<'a> {
    config: &'a Config,
}

impl<'a> UserTaskResource<'a> {
    /// POST /open-apis/apaas/v1/user_tasks/:task_id/cc
    pub async fn cc(
        &self,
        task_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<CcUserTaskResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/cc");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), CcUserTaskResp>()
        .await
    }

    /// POST /open-apis/apaas/v1/user_tasks/:task_id/chat_group
    pub async fn chat_group(
        &self,
        task_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<ChatGroupUserTaskResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/chat_group");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<ChatGroupUserTaskData, ChatGroupUserTaskResp>()
        .await
    }

    /// POST /open-apis/apaas/v1/user_tasks/:task_id/expediting
    pub async fn expediting(
        &self,
        task_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<ExpeditingUserTaskResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/expediting");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), ExpeditingUserTaskResp>()
        .await
    }

    /// 获取任务列表
    /// POST /open-apis/apaas/v1/user_task/query
    pub async fn query(
        &self,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<QueryUserTaskResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/apaas/v1/user_task/query",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<QueryUserTaskData, QueryUserTaskResp>()
        .await
    }

    /// POST /open-apis/apaas/v1/user_tasks/:task_id/rollback
    pub async fn rollback(
        &self,
        task_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<RollbackUserTaskResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/rollback");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), RollbackUserTaskResp>()
        .await
    }

    /// POST /open-apis/apaas/v1/user_tasks/:task_id/rollback_points
    pub async fn rollback_points(
        &self,
        task_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<RollbackPointsUserTaskResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/rollback_points");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(body)?
        .send_v2_response::<RollbackPointsUserTaskData, RollbackPointsUserTaskResp>()
        .await
    }
}

// ── Workspace ──

pub struct WorkspaceResource<'a> {
    config: &'a Config,
}

impl<'a> WorkspaceResource<'a> {
    /// 执行 SQL
    /// POST /open-apis/apaas/v1/workspaces/:workspace_id/sql_commands
    pub async fn sql_commands(
        &self,
        workspace_id: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<SqlCommandsWorkspaceResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/sql_commands");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2_response::<SqlCommandsWorkspaceData, SqlCommandsWorkspaceResp>()
        .await
    }
}

// ── WorkspaceEnum ──

pub struct WorkspaceEnumResource<'a> {
    config: &'a Config,
}

impl<'a> WorkspaceEnumResource<'a> {
    /// 获取自定义枚举详细信息
    /// GET /open-apis/apaas/v1/workspaces/:workspace_id/enums/:enum_name
    pub async fn enum_get(
        &self,
        workspace_id: &str,
        enum_name: &str,
        option: &RequestOption,
    ) -> Result<EnumGetWorkspaceEnumResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/enums/{enum_name}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_v2_response::<EnumGetWorkspaceEnumData, EnumGetWorkspaceEnumResp>()
        .await
    }

    /// 获取工作空间下的自定义枚举列表
    /// GET /open-apis/apaas/v1/workspaces/:workspace_id/enums
    pub async fn list(
        &self,
        workspace_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWorkspaceEnumResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/enums");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<ListWorkspaceEnumData, ListWorkspaceEnumResp>()
        .await
    }
}

// ── WorkspaceTable ──

pub struct WorkspaceTableResource<'a> {
    config: &'a Config,
}

impl<'a> WorkspaceTableResource<'a> {
    /// 获取工作空间下的数据表列表
    /// GET /open-apis/apaas/v1/workspaces/:workspace_id/tables
    pub async fn list(
        &self,
        workspace_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWorkspaceTableResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<ListWorkspaceTableData, ListWorkspaceTableResp>()
        .await
    }

    /// 批量更新数据表中的记录
    /// PATCH /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records_batch_update
    pub async fn records_batch_update(
        &self,
        workspace_id: &str,
        table_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<RecordsBatchUpdateWorkspaceTableResp, LarkError> {
        let path = format!(
            "/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records_batch_update"
        );
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2_response::<RecordIdsData, RecordsBatchUpdateWorkspaceTableResp>()
        .await
    }

    /// 删除数据表中的记录
    /// DELETE /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
    pub async fn records_delete(
        &self,
        workspace_id: &str,
        table_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<RecordsDeleteWorkspaceTableResp, LarkError> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records");
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2_response::<(), RecordsDeleteWorkspaceTableResp>()
        .await
    }

    /// 查询数据表数据记录
    /// GET /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
    pub async fn records_get(
        &self,
        workspace_id: &str,
        table_name: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<RecordsGetWorkspaceTableResp, LarkError> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<WorkspaceRecordsData, RecordsGetWorkspaceTableResp>()
        .await
    }

    /// 按条件更新数据表中的记录
    /// PATCH /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
    pub async fn records_patch(
        &self,
        workspace_id: &str,
        table_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<RecordsPatchWorkspaceTableResp, LarkError> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records");
        RestRequest::new(
            self.config,
            http::Method::PATCH,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2_response::<RecordIdsData, RecordsPatchWorkspaceTableResp>()
        .await
    }

    /// 向数据表中添加或更新记录
    /// POST /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
    pub async fn records_post(
        &self,
        workspace_id: &str,
        table_name: &str,
        body: &impl Serialize,
        option: &RequestOption,
    ) -> Result<RecordsPostWorkspaceTableResp, LarkError> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records");
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .json_body(body)?
        .send_v2_response::<RecordIdsData, RecordsPostWorkspaceTableResp>()
        .await
    }

    /// 获取数据表详细信息
    /// GET /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name
    pub async fn table_get(
        &self,
        workspace_id: &str,
        table_name: &str,
        option: &RequestOption,
    ) -> Result<TableGetWorkspaceTableResp, LarkError> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .send_v2_response::<WorkspaceTableData, TableGetWorkspaceTableResp>()
        .await
    }
}

// ── WorkspaceView ──

pub struct WorkspaceViewResource<'a> {
    config: &'a Config,
}

impl<'a> WorkspaceViewResource<'a> {
    /// GET /open-apis/apaas/v1/workspaces/:workspace_id/views/:view_name/records
    pub async fn views_get(
        &self,
        workspace_id: &str,
        view_name: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ViewsGetWorkspaceViewResp, LarkError> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/views/{view_name}/records");
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User],
            option,
        )
        .query("page_size", page_size)
        .query("page_token", page_token)
        .send_v2_response::<WorkspaceRecordsData, ViewsGetWorkspaceViewResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub app: AppResource<'a>,
    pub application_audit_log: ApplicationAuditLogResource<'a>,
    pub application_environment_variable: ApplicationEnvironmentVariableResource<'a>,
    pub application_flow: ApplicationFlowResource<'a>,
    pub application_function: ApplicationFunctionResource<'a>,
    pub application_object: ApplicationObjectResource<'a>,
    pub application_object_record: ApplicationObjectRecordResource<'a>,
    pub application_record_permission_member: ApplicationRecordPermissionMemberResource<'a>,
    pub application_role_member: ApplicationRoleMemberResource<'a>,
    pub approval_instance: ApprovalInstanceResource<'a>,
    pub approval_task: ApprovalTaskResource<'a>,
    pub seat_activity: SeatActivityResource<'a>,
    pub seat_assignment: SeatAssignmentResource<'a>,
    pub user_task: UserTaskResource<'a>,
    pub workspace: WorkspaceResource<'a>,
    pub workspace_enum: WorkspaceEnumResource<'a>,
    pub workspace_table: WorkspaceTableResource<'a>,
    pub workspace_view: WorkspaceViewResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            app: AppResource { config },
            application_audit_log: ApplicationAuditLogResource { config },
            application_environment_variable: ApplicationEnvironmentVariableResource { config },
            application_flow: ApplicationFlowResource { config },
            application_function: ApplicationFunctionResource { config },
            application_object: ApplicationObjectResource { config },
            application_object_record: ApplicationObjectRecordResource { config },
            application_record_permission_member: ApplicationRecordPermissionMemberResource {
                config,
            },
            application_role_member: ApplicationRoleMemberResource { config },
            approval_instance: ApprovalInstanceResource { config },
            approval_task: ApprovalTaskResource { config },
            seat_activity: SeatActivityResource { config },
            seat_assignment: SeatAssignmentResource { config },
            user_task: UserTaskResource { config },
            workspace: WorkspaceResource { config },
            workspace_enum: WorkspaceEnumResource { config },
            workspace_table: WorkspaceTableResource { config },
            workspace_view: WorkspaceViewResource { config },
        }
    }
}
