use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::service::common::{PageQuery, RestRequest};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_system: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_ownership: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certification_level: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_terminal_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_managed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdm_device_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdm_provider_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_verified_at_unix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_issuer: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceRecordData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_record: Option<DeviceRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceRecordListData {
    #[serde(default)]
    pub items: Vec<DeviceRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDeviceRecordData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MineDeviceRecordData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_ownership: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_status: Option<i32>,
}

impl_resp_v2!(UpdateDeviceApplyRecordV2Resp, ());
impl_resp_v2!(CreateDeviceRecordV2Resp, CreateDeviceRecordData);
impl_resp_v2!(DeleteDeviceRecordV2Resp, ());
impl_resp_v2!(GetDeviceRecordV2Resp, DeviceRecordData);
impl_resp_v2!(ListDeviceRecordV2Resp, DeviceRecordListData);
impl_resp_v2!(MineDeviceRecordV2Resp, MineDeviceRecordData);
impl_resp_v2!(UpdateDeviceRecordV2Resp, ());

#[derive(Debug, Clone, Copy, Default)]
pub struct ListDeviceRecordV2Query<'a> {
    pub page: PageQuery<'a>,
}

impl<'a> ListDeviceRecordV2Query<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: PageQuery<'a>) -> Self {
        self.page = page;
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateDeviceApplyRecordV2Query<'a> {
    pub device_apply_record_id: &'a str,
    pub body: &'a crate::JsonValue,
}

impl<'a> UpdateDeviceApplyRecordV2Query<'a> {
    pub fn new(device_apply_record_id: &'a str, body: &'a crate::JsonValue) -> Self {
        Self {
            device_apply_record_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct CreateDeviceRecordV2Query<'a> {
    pub body: &'a crate::JsonValue,
}

impl<'a> CreateDeviceRecordV2Query<'a> {
    pub fn new(body: &'a crate::JsonValue) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct DeleteDeviceRecordV2Query<'a> {
    pub device_record_id: &'a str,
}

impl<'a> DeleteDeviceRecordV2Query<'a> {
    pub fn new(device_record_id: &'a str) -> Self {
        Self { device_record_id }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct GetDeviceRecordV2Query<'a> {
    pub device_record_id: &'a str,
}

impl<'a> GetDeviceRecordV2Query<'a> {
    pub fn new(device_record_id: &'a str) -> Self {
        Self { device_record_id }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct MineDeviceRecordV2Query;

impl MineDeviceRecordV2Query {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct UpdateDeviceRecordV2Query<'a> {
    pub device_record_id: &'a str,
    pub body: &'a crate::JsonValue,
}

impl<'a> UpdateDeviceRecordV2Query<'a> {
    pub fn new(device_record_id: &'a str, body: &'a crate::JsonValue) -> Self {
        Self {
            device_record_id,
            body,
        }
    }
}

pub struct V2<'a> {
    pub device_apply_record: DeviceApplyRecordV2Resource<'a>,
    pub device_record: DeviceRecordV2Resource<'a>,
}

impl<'a> V2<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            device_apply_record: DeviceApplyRecordV2Resource { config },
            device_record: DeviceRecordV2Resource { config },
        }
    }
}

pub struct DeviceApplyRecordV2Resource<'a> {
    config: &'a Config,
}

impl DeviceApplyRecordV2Resource<'_> {
    pub async fn update(
        &self,
        device_apply_record_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateDeviceApplyRecordV2Resp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        let query = UpdateDeviceApplyRecordV2Query::new(device_apply_record_id, &body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateDeviceApplyRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<UpdateDeviceApplyRecordV2Resp, LarkError> {
        let path = format!(
            "/open-apis/security_and_compliance/v2/device_apply_records/{}",
            query.device_apply_record_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), UpdateDeviceApplyRecordV2Resp>()
        .await
    }
}

pub struct DeviceRecordV2Resource<'a> {
    config: &'a Config,
}

impl DeviceRecordV2Resource<'_> {
    pub async fn create(
        &self,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<CreateDeviceRecordV2Resp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        let query = CreateDeviceRecordV2Query::new(&body);
        self.create_by_query(&query, option).await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateDeviceRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<CreateDeviceRecordV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/security_and_compliance/v2/device_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<CreateDeviceRecordData, CreateDeviceRecordV2Resp>()
        .await
    }

    pub async fn delete(
        &self,
        device_record_id: &str,
        option: &RequestOption,
    ) -> Result<DeleteDeviceRecordV2Resp, LarkError> {
        let query = DeleteDeviceRecordV2Query::new(device_record_id);
        self.delete_by_query(&query, option).await
    }

    pub async fn delete_by_query(
        &self,
        query: &DeleteDeviceRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<DeleteDeviceRecordV2Resp, LarkError> {
        let path = format!(
            "/open-apis/security_and_compliance/v2/device_records/{}",
            query.device_record_id
        );
        RestRequest::new(
            self.config,
            http::Method::DELETE,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<(), DeleteDeviceRecordV2Resp>()
        .await
    }

    pub async fn get(
        &self,
        device_record_id: &str,
        option: &RequestOption,
    ) -> Result<GetDeviceRecordV2Resp, LarkError> {
        let query = GetDeviceRecordV2Query::new(device_record_id);
        self.get_by_query(&query, option).await
    }

    pub async fn get_by_query(
        &self,
        query: &GetDeviceRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<GetDeviceRecordV2Resp, LarkError> {
        let path = format!(
            "/open-apis/security_and_compliance/v2/device_records/{}",
            query.device_record_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .send_v2_response::<DeviceRecordData, GetDeviceRecordV2Resp>()
        .await
    }

    pub async fn list(
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<ListDeviceRecordV2Resp, LarkError> {
        let query =
            ListDeviceRecordV2Query::new().page(PageQuery::from_parts(page_size, page_token));
        self.list_by_query(&query, option).await
    }

    pub async fn list_by_query(
        &self,
        query: &ListDeviceRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<ListDeviceRecordV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/security_and_compliance/v2/device_records",
            vec![AccessTokenType::Tenant],
            option,
        )
        .page_query(query.page)
        .send_v2_response::<DeviceRecordListData, ListDeviceRecordV2Resp>()
        .await
    }

    pub async fn mine(&self, option: &RequestOption) -> Result<MineDeviceRecordV2Resp, LarkError> {
        let query = MineDeviceRecordV2Query::new();
        self.mine_by_query(&query, option).await
    }

    pub async fn mine_by_query(
        &self,
        _query: &MineDeviceRecordV2Query,
        option: &RequestOption,
    ) -> Result<MineDeviceRecordV2Resp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::GET,
            "/open-apis/security_and_compliance/v2/device_records/mine",
            vec![AccessTokenType::User],
            option,
        )
        .send_v2_response::<MineDeviceRecordData, MineDeviceRecordV2Resp>()
        .await
    }

    pub async fn update(
        &self,
        device_record_id: &str,
        body: impl Serialize,
        option: &RequestOption,
    ) -> Result<UpdateDeviceRecordV2Resp, LarkError> {
        let body = crate::JsonValue::from_serializable(body)?;
        let query = UpdateDeviceRecordV2Query::new(device_record_id, &body);
        self.update_by_query(&query, option).await
    }

    pub async fn update_by_query(
        &self,
        query: &UpdateDeviceRecordV2Query<'_>,
        option: &RequestOption,
    ) -> Result<UpdateDeviceRecordV2Resp, LarkError> {
        let path = format!(
            "/open-apis/security_and_compliance/v2/device_records/{}",
            query.device_record_id
        );
        RestRequest::new(
            self.config,
            http::Method::PUT,
            path,
            vec![AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_v2_response::<(), UpdateDeviceRecordV2Resp>()
        .await
    }
}
