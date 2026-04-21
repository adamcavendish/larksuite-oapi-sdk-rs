use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::Result;
use crate::req::{ApiReq, ReqBody, RequestOption};
use crate::service::common::parse_v2;
use crate::transport;

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
    pub body: Option<serde_json::Value>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvokeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_body: Option<serde_json::Value>,
}

impl_resp!(InvokeOpenApiResp, InvokeData);

// ── v2 response types (all use serde_json::Value data) ──

impl_resp_v2!(AuditLogListResp, serde_json::Value);
impl_resp_v2!(DataChangeLogDetailResp, serde_json::Value);
impl_resp_v2!(DataChangeLogsListResp, serde_json::Value);
impl_resp_v2!(GetAuditLogResp, serde_json::Value);
impl_resp_v2!(GetEnvironmentVariableResp, serde_json::Value);
impl_resp_v2!(QueryEnvironmentVariableResp, serde_json::Value);
impl_resp_v2!(ExecuteFlowResp, serde_json::Value);
impl_resp_v2!(InvokeFunctionResp, serde_json::Value);
impl_resp_v2!(OqlQueryObjectResp, serde_json::Value);
impl_resp_v2!(SearchObjectResp, serde_json::Value);
impl_resp_v2!(BatchCreateRecordResp, serde_json::Value);
impl_resp_v2!(BatchDeleteRecordResp, ());
impl_resp_v2!(BatchQueryRecordResp, serde_json::Value);
impl_resp_v2!(BatchUpdateRecordResp, serde_json::Value);
impl_resp_v2!(CreateRecordResp, serde_json::Value);
impl_resp_v2!(DeleteRecordResp, ());
impl_resp_v2!(PatchRecordResp, serde_json::Value);
impl_resp_v2!(QueryRecordResp, serde_json::Value);
impl_resp_v2!(
    BatchCreateAuthorizationRecordPermissionResp,
    serde_json::Value
);
impl_resp_v2!(
    BatchRemoveAuthorizationRecordPermissionResp,
    serde_json::Value
);
impl_resp_v2!(BatchCreateAuthorizationRoleMemberResp, serde_json::Value);
impl_resp_v2!(BatchRemoveAuthorizationRoleMemberResp, serde_json::Value);
impl_resp_v2!(GetRoleMemberResp, serde_json::Value);
impl_resp_v2!(CancelApprovalInstanceResp, serde_json::Value);
impl_resp_v2!(AddAssigneeApprovalTaskResp, serde_json::Value);
impl_resp_v2!(AgreeApprovalTaskResp, serde_json::Value);
impl_resp_v2!(RejectApprovalTaskResp, serde_json::Value);
impl_resp_v2!(TransferApprovalTaskResp, serde_json::Value);
impl_resp_v2!(ListSeatActivityResp, serde_json::Value);
impl_resp_v2!(ListSeatAssignmentResp, serde_json::Value);
impl_resp_v2!(CcUserTaskResp, serde_json::Value);
impl_resp_v2!(ChatGroupUserTaskResp, serde_json::Value);
impl_resp_v2!(ExpeditingUserTaskResp, serde_json::Value);
impl_resp_v2!(QueryUserTaskResp, serde_json::Value);
impl_resp_v2!(RollbackUserTaskResp, serde_json::Value);
impl_resp_v2!(RollbackPointsUserTaskResp, serde_json::Value);
impl_resp_v2!(SqlCommandsWorkspaceResp, serde_json::Value);
impl_resp_v2!(EnumGetWorkspaceEnumResp, serde_json::Value);
impl_resp_v2!(ListWorkspaceEnumResp, serde_json::Value);
impl_resp_v2!(ListWorkspaceTableResp, serde_json::Value);
impl_resp_v2!(RecordsBatchUpdateWorkspaceTableResp, serde_json::Value);
impl_resp_v2!(RecordsDeleteWorkspaceTableResp, ());
impl_resp_v2!(RecordsGetWorkspaceTableResp, serde_json::Value);
impl_resp_v2!(RecordsPatchWorkspaceTableResp, serde_json::Value);
impl_resp_v2!(RecordsPostWorkspaceTableResp, serde_json::Value);
impl_resp_v2!(TableGetWorkspaceTableResp, serde_json::Value);
impl_resp_v2!(ViewsGetWorkspaceViewResp, serde_json::Value);
impl_resp_v2!(ListAppResp, serde_json::Value);

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
    ) -> Result<InvokeOpenApiResp> {
        let path = format!("/open-apis/apaas/v1/apps/{namespace}/open_api/invoke");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<InvokeData>(self.config, &api_req, option).await?;
        Ok(InvokeOpenApiResp {
            api_resp,
            code_error: raw.code_error,
            data: raw.data,
        })
    }

    /// List apps
    /// GET /open-apis/apaas/v1/apps
    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListAppResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/apaas/v1/apps");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListAppResp {
            api_resp,
            code_error,
            data,
        })
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
        body: Option<&serde_json::Value>,
        option: &RequestOption,
    ) -> Result<AuditLogListResp> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/audit_log/audit_log_list");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(b) = body {
            api_req.body = Some(ReqBody::json(b)?);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(AuditLogListResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 获取数据变更日志详情
    /// GET /open-apis/apaas/v1/applications/:namespace/audit_log/data_change_log_detail
    pub async fn data_change_log_detail(
        &self,
        namespace: &str,
        body: Option<&serde_json::Value>,
        option: &RequestOption,
    ) -> Result<DataChangeLogDetailResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/audit_log/data_change_log_detail"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(b) = body {
            api_req.body = Some(ReqBody::json(b)?);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DataChangeLogDetailResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 获取数据变更日志列表
    /// GET /open-apis/apaas/v1/applications/:namespace/audit_log/data_change_logs_list
    pub async fn data_change_logs_list(
        &self,
        namespace: &str,
        body: Option<&serde_json::Value>,
        option: &RequestOption,
    ) -> Result<DataChangeLogsListResp> {
        let path =
            format!("/open-apis/apaas/v1/applications/{namespace}/audit_log/data_change_logs_list");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(b) = body {
            api_req.body = Some(ReqBody::json(b)?);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DataChangeLogsListResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 获取审计日志详情
    /// GET /open-apis/apaas/v1/applications/:namespace/audit_log
    pub async fn get(
        &self,
        namespace: &str,
        body: Option<&serde_json::Value>,
        option: &RequestOption,
    ) -> Result<GetAuditLogResp> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/audit_log");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(b) = body {
            api_req.body = Some(ReqBody::json(b)?);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetAuditLogResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<GetEnvironmentVariableResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/environment_variables/{environment_variable_api_name}"
        );
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetEnvironmentVariableResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 查询环境变量列表
    /// POST /open-apis/apaas/v1/applications/:namespace/environment_variables/query
    pub async fn query(
        &self,
        namespace: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryEnvironmentVariableResp> {
        let path =
            format!("/open-apis/apaas/v1/applications/{namespace}/environment_variables/query");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryEnvironmentVariableResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<ExecuteFlowResp> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/flows/{flow_id}/execute");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ExecuteFlowResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<InvokeFunctionResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/functions/{function_api_name}/invoke"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(InvokeFunctionResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<OqlQueryObjectResp> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/objects/oql_query");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(OqlQueryObjectResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 搜索记录
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/search
    pub async fn search(
        &self,
        namespace: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<SearchObjectResp> {
        let path = format!("/open-apis/apaas/v1/applications/{namespace}/objects/search");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SearchObjectResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchCreateRecordResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/batch_create"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchCreateRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 记录批量删除
    /// DELETE /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_delete
    pub async fn batch_delete(
        &self,
        namespace: &str,
        object_api_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchDeleteRecordResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/batch_delete"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchDeleteRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 批量查询对象记录
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_query
    pub async fn batch_query(
        &self,
        namespace: &str,
        object_api_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchQueryRecordResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/batch_query"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchQueryRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 记录批量更新
    /// PATCH /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/batch_update
    pub async fn batch_update(
        &self,
        namespace: &str,
        object_api_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchUpdateRecordResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/batch_update"
        );
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchUpdateRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 创建记录
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records
    pub async fn create(
        &self,
        namespace: &str,
        object_api_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CreateRecordResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CreateRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 删除记录
    /// DELETE /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id
    pub async fn delete(
        &self,
        namespace: &str,
        object_api_name: &str,
        id: &str,
        option: &RequestOption,
    ) -> Result<DeleteRecordResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/{id}"
        );
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(DeleteRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 更新记录
    /// PATCH /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id
    pub async fn patch(
        &self,
        namespace: &str,
        object_api_name: &str,
        id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<PatchRecordResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/{id}"
        );
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(PatchRecordResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 获取记录
    /// POST /open-apis/apaas/v1/applications/:namespace/objects/:object_api_name/records/:id/query
    pub async fn query(
        &self,
        namespace: &str,
        object_api_name: &str,
        id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryRecordResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/objects/{object_api_name}/records/{id}/query"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryRecordResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchCreateAuthorizationRecordPermissionResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/record_permissions/{record_permission_api_name}/member/batch_create_authorization"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchCreateAuthorizationRecordPermissionResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/apaas/v1/applications/:namespace/record_permissions/:record_permission_api_name/member/batch_remove_authorization
    pub async fn batch_remove_authorization(
        &self,
        namespace: &str,
        record_permission_api_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchRemoveAuthorizationRecordPermissionResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/record_permissions/{record_permission_api_name}/member/batch_remove_authorization"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchRemoveAuthorizationRecordPermissionResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchCreateAuthorizationRoleMemberResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/roles/{role_api_name}/member/batch_create_authorization"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchCreateAuthorizationRoleMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 批量删除角色成员用户和部门
    /// POST /open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member/batch_remove_authorization
    pub async fn batch_remove_authorization(
        &self,
        namespace: &str,
        role_api_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<BatchRemoveAuthorizationRoleMemberResp> {
        let path = format!(
            "/open-apis/apaas/v1/applications/{namespace}/roles/{role_api_name}/member/batch_remove_authorization"
        );
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(BatchRemoveAuthorizationRoleMemberResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// GET /open-apis/apaas/v1/applications/:namespace/roles/:role_api_name/member
    pub async fn get(
        &self,
        namespace: &str,
        role_api_name: &str,
        option: &RequestOption,
    ) -> Result<GetRoleMemberResp> {
        let path =
            format!("/open-apis/apaas/v1/applications/{namespace}/roles/{role_api_name}/member");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(GetRoleMemberResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CancelApprovalInstanceResp> {
        let path = format!("/open-apis/apaas/v1/approval_instances/{approval_instance_id}/cancel");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CancelApprovalInstanceResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<AddAssigneeApprovalTaskResp> {
        let path = format!("/open-apis/apaas/v1/approval_tasks/{approval_task_id}/add_assignee");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(AddAssigneeApprovalTaskResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/apaas/v1/approval_tasks/:approval_task_id/agree
    pub async fn agree(
        &self,
        approval_task_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<AgreeApprovalTaskResp> {
        let path = format!("/open-apis/apaas/v1/approval_tasks/{approval_task_id}/agree");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(AgreeApprovalTaskResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/apaas/v1/approval_tasks/:approval_task_id/reject
    pub async fn reject(
        &self,
        approval_task_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RejectApprovalTaskResp> {
        let path = format!("/open-apis/apaas/v1/approval_tasks/{approval_task_id}/reject");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RejectApprovalTaskResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/apaas/v1/approval_tasks/:approval_task_id/transfer
    pub async fn transfer(
        &self,
        approval_task_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<TransferApprovalTaskResp> {
        let path = format!("/open-apis/apaas/v1/approval_tasks/{approval_task_id}/transfer");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(TransferApprovalTaskResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<ListSeatActivityResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/apaas/v1/seat_activities");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListSeatActivityResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<ListSeatAssignmentResp> {
        let mut api_req = ApiReq::new(http::Method::GET, "/open-apis/apaas/v1/seat_assignments");
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListSeatAssignmentResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<CcUserTaskResp> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/cc");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(CcUserTaskResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/apaas/v1/user_tasks/:task_id/chat_group
    pub async fn chat_group(
        &self,
        task_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<ChatGroupUserTaskResp> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/chat_group");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ChatGroupUserTaskResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/apaas/v1/user_tasks/:task_id/expediting
    pub async fn expediting(
        &self,
        task_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<ExpeditingUserTaskResp> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/expediting");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ExpeditingUserTaskResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 获取任务列表
    /// POST /open-apis/apaas/v1/user_task/query
    pub async fn query(
        &self,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<QueryUserTaskResp> {
        let mut api_req = ApiReq::new(http::Method::POST, "/open-apis/apaas/v1/user_task/query");
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(QueryUserTaskResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/apaas/v1/user_tasks/:task_id/rollback
    pub async fn rollback(
        &self,
        task_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RollbackUserTaskResp> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/rollback");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RollbackUserTaskResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// POST /open-apis/apaas/v1/user_tasks/:task_id/rollback_points
    pub async fn rollback_points(
        &self,
        task_id: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RollbackPointsUserTaskResp> {
        let path = format!("/open-apis/apaas/v1/user_tasks/{task_id}/rollback_points");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RollbackPointsUserTaskResp {
            api_resp,
            code_error,
            data,
        })
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
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<SqlCommandsWorkspaceResp> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/sql_commands");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(SqlCommandsWorkspaceResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<EnumGetWorkspaceEnumResp> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/enums/{enum_name}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(EnumGetWorkspaceEnumResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 获取工作空间下的自定义枚举列表
    /// GET /open-apis/apaas/v1/workspaces/:workspace_id/enums
    pub async fn list(
        &self,
        workspace_id: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListWorkspaceEnumResp> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/enums");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListWorkspaceEnumResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<ListWorkspaceTableResp> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ListWorkspaceTableResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 批量更新数据表中的记录
    /// PATCH /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records_batch_update
    pub async fn records_batch_update(
        &self,
        workspace_id: &str,
        table_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RecordsBatchUpdateWorkspaceTableResp> {
        let path = format!(
            "/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records_batch_update"
        );
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecordsBatchUpdateWorkspaceTableResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 删除数据表中的记录
    /// DELETE /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
    pub async fn records_delete(
        &self,
        workspace_id: &str,
        table_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RecordsDeleteWorkspaceTableResp> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records");
        let mut api_req = ApiReq::new(http::Method::DELETE, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) = transport::request_typed::<()>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecordsDeleteWorkspaceTableResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<RecordsGetWorkspaceTableResp> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecordsGetWorkspaceTableResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 按条件更新数据表中的记录
    /// PATCH /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
    pub async fn records_patch(
        &self,
        workspace_id: &str,
        table_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RecordsPatchWorkspaceTableResp> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records");
        let mut api_req = ApiReq::new(http::Method::PATCH, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecordsPatchWorkspaceTableResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 向数据表中添加或更新记录
    /// POST /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
    pub async fn records_post(
        &self,
        workspace_id: &str,
        table_name: &str,
        body: &serde_json::Value,
        option: &RequestOption,
    ) -> Result<RecordsPostWorkspaceTableResp> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}/records");
        let mut api_req = ApiReq::new(http::Method::POST, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        api_req.body = Some(ReqBody::json(body)?);
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(RecordsPostWorkspaceTableResp {
            api_resp,
            code_error,
            data,
        })
    }

    /// 获取数据表详细信息
    /// GET /open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name
    pub async fn table_get(
        &self,
        workspace_id: &str,
        table_name: &str,
        option: &RequestOption,
    ) -> Result<TableGetWorkspaceTableResp> {
        let path = format!("/open-apis/apaas/v1/workspaces/{workspace_id}/tables/{table_name}");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(TableGetWorkspaceTableResp {
            api_resp,
            code_error,
            data,
        })
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
    ) -> Result<ViewsGetWorkspaceViewResp> {
        let path =
            format!("/open-apis/apaas/v1/workspaces/{workspace_id}/views/{view_name}/records");
        let mut api_req = ApiReq::new(http::Method::GET, &path);
        api_req.supported_access_token_types = vec![AccessTokenType::User];
        if let Some(v) = page_size {
            api_req.query_params.set("page_size", v.to_string());
        }
        if let Some(v) = page_token {
            api_req.query_params.set("page_token", v);
        }
        let (api_resp, raw) =
            transport::request_typed::<serde_json::Value>(self.config, &api_req, option).await?;
        let (api_resp, code_error, data) = parse_v2(api_resp, raw);
        Ok(ViewsGetWorkspaceViewResp {
            api_resp,
            code_error,
            data,
        })
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
