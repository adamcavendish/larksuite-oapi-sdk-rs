use crate::JsonValue;
use serde_json::Value;

impl_resp_v2!(GetAppReleaseResp, AppReleaseDetail);

/// Release detail returned by the Spark release API.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AppReleaseDetail {
    pub release_id: Option<String>,
    /// The platform's release status, including values introduced after this SDK release.
    pub status: Option<String>,
    /// The platform may represent timestamps as strings or JSON numbers.
    pub created_at: Option<JsonValue>,
    pub updated_at: Option<JsonValue>,
    pub commit_id: Option<String>,
    /// The platform may provide this URL before the release finishes.
    pub online_url: Option<String>,
    pub current_node_info: Option<AppReleaseApprovalNode>,
    pub error_logs: Option<Vec<AppReleaseErrorLog>>,
}

impl AppReleaseDetail {
    /// Whether this release is in progress and the platform reports a pending approval node.
    pub fn is_waiting_for_approval(&self) -> bool {
        matches!(self.status.as_deref(), Some("pending" | "publishing"))
            && self
                .current_node_info
                .as_ref()
                .and_then(|node| node.current_status.as_deref())
                == Some("PENDING")
    }

    pub(super) fn from_data(data: &JsonValue) -> Option<Self> {
        let outer = data.as_object()?;
        let release = data
            .get("release")
            .filter(|value| value.is_object())
            .unwrap_or(data.as_value());
        // An explicit outer null supersedes a nested value.
        let auxiliary = |key: &str| outer.get(key).or_else(|| release.get(key));

        Some(Self {
            release_id: text(release, &["release_id"]).or_else(|| {
                release
                    .get("release_id")?
                    .as_number()
                    .filter(|id| id.is_i64() || id.is_u64())
                    .map(ToString::to_string)
            }),
            status: text(release, &["status"]),
            created_at: timestamp(release, &["created_at"]),
            updated_at: timestamp(release, &["updated_at"]),
            commit_id: text(release, &["commit_id"]),
            online_url: text(release, &["online_url"]),
            current_node_info: auxiliary("current_node_info")
                .and_then(AppReleaseApprovalNode::from_value),
            error_logs: auxiliary("error_logs")
                .and_then(Value::as_array)
                .map(|logs| {
                    logs.iter()
                        .filter(|log| log.is_object())
                        .map(|log| AppReleaseErrorLog {
                            step: text(log, &["step"]),
                            error_log: text(log, &["error_log"]),
                        })
                        .collect()
                }),
        })
    }
}

/// The release service's current workflow node and approval information.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AppReleaseApprovalNode {
    pub current_node: Option<String>,
    pub current_status: Option<String>,
    pub created_at: Option<JsonValue>,
    /// Approval URL supplied by the platform.
    pub approval_url: Option<String>,
    /// The person who requested the release.
    pub submitted_by: Option<AppReleaseSubmitter>,
}

impl AppReleaseApprovalNode {
    fn from_value(node: &Value) -> Option<Self> {
        node.as_object()?;
        let submitted_by = node
            .get("submitted_by")
            .filter(|value| value.is_object())
            .or_else(|| node.get("submittedBy").filter(|value| value.is_object()));

        Some(Self {
            current_node: text(node, &["current_node", "currentNode"]),
            current_status: text(node, &["current_status", "currentStatus"]),
            created_at: timestamp(node, &["created_at", "createdAt"]),
            approval_url: node
                .get("result")
                .and_then(|result| text(result, &["approval_url", "approvalURL"])),
            submitted_by: submitted_by.map(|submitter| AppReleaseSubmitter {
                username: text(submitter, &["username"]),
                email: text(submitter, &["email"]),
                open_id: text(submitter, &["open_id", "openID"]),
            }),
        })
    }
}

/// The user who submitted the release request.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AppReleaseSubmitter {
    pub username: Option<String>,
    pub email: Option<String>,
    pub open_id: Option<String>,
}

/// Failure reported for a release step.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AppReleaseErrorLog {
    pub step: Option<String>,
    pub error_log: Option<String>,
}

fn text(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| value.get(key)?.as_str().filter(|text| !text.is_empty()))
        .map(str::to_owned)
}

fn timestamp(value: &Value, keys: &[&str]) -> Option<JsonValue> {
    keys.iter()
        .filter_map(|key| value.get(key))
        .find(|value| value.is_number() || value.as_str().is_some_and(|text| !text.is_empty()))
        .cloned()
        .map(JsonValue::from)
}
