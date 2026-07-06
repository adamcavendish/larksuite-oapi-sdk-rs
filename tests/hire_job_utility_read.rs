mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    ListByIdJobRequirementQuery, QueryLocationQuery, SearchJobPublishRecordQuery,
};
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn hire_job_utility_read_responses_deserialize_and_send_filters() {
    let publish_records = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"post-1","title":"Backend","job_id":"job-1"}],"has_more":false,"page_token":"publish-next"}}"#;
    let job_requirements = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"jr-1","short_code":"JR-1","recruiter_list":[{"id":"ou_1","en_name":"Recruiter"}],"job_id_list":["job-1"],"count_data":{"offer_count":1}}]}}"#;
    let locations = r#"{"code":0,"msg":"ok","data":{"items":[{"city":{"city_code":"CT_1","city_name_info":{"zh_name":"成都"}}}],"has_more":false,"page_token":"location-next"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, publish_records),
        http_response(200, job_requirements),
        http_response(200, locations),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    let publish_record = hire
        .job_publish_record
        .search_by_query(
            &SearchJobPublishRecordQuery::new()
                .page(page)
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .job_level_id_type("job_level_id")
                .job_family_id_type("job_family_id"),
            json!({"job_channel_id":"channel-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let job_requirement = hire
        .job_requirement
        .list_by_id_by_query(
            &ListByIdJobRequirementQuery::new()
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .job_level_id_type("job_level_id")
                .job_family_id_type("job_family_id")
                .employee_type_id_type("employee_type_id"),
            json!({"id_list":["jr-1"],"short_code_list":["JR-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let location = hire
        .location
        .query_by_query(
            &QueryLocationQuery::new().page(page),
            json!({"code_list":["CN_1"],"location_type":3}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);

    assert_eq!(publish_record.title.as_deref(), Some("Backend"));
    assert_eq!(
        job_requirement.recruiter_list.as_ref().unwrap()[0]
            .en_name
            .as_deref(),
        Some("Recruiter")
    );
    assert_eq!(job_requirement.job_id_list.as_ref().unwrap()[0], "job-1");
    assert_eq!(
        job_requirement
            .count_data
            .as_ref()
            .and_then(|count| count.offer_count),
        Some(1)
    );
    assert_eq!(
        location
            .city
            .as_ref()
            .and_then(|city| city.city_name_info.as_ref())
            .and_then(|name| name.zh_name.as_deref()),
        Some("成都")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/job_publish_records/search?"));
    assert!(request.contains("POST /open-apis/hire/v1/job_requirements/search?"));
    assert!(request.contains("POST /open-apis/hire/v1/locations/query?"));
    assert_eq!(request.matches("page_token=seed-token").count(), 2);
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("job_family_id_type=job_family_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
    assert!(request.contains(r#""job_channel_id":"channel-1""#));
    assert!(request.contains(r#""id_list":["jr-1"]"#));
    assert!(request.contains(r#""short_code_list":["JR-1"]"#));
    assert!(request.contains(r#""code_list":["CN_1"]"#));
    assert!(request.contains(r#""location_type":3"#));
}
