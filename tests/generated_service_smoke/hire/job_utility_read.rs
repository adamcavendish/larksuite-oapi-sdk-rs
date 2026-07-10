use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_job_utility_read_query_smoke() {
    let publish_records =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"post-1"}],"has_more":false}}"#;
    let job_requirements = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"jr-1","customized_data_list":[{"object_id":"field-1","value":{"time_range":{"start_time":"1710000000","end_time":"1710003600"}}}]}]}}"#;
    let locations = r#"{"code":0,"msg":"ok","data":{"items":[{"country":{"country_code":"CN_1"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, publish_records),
        http_response(200, job_requirements),
        http_response(200, locations),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    hire.job_publish_record
        .search_by_query(
            &SearchHireJobPublishRecordQuery::new()
                .page(page)
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .job_level_id_type("job_level_id")
                .job_family_id_type("job_family_id"),
            json!({"job_channel_id":"channel-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let job_requirements = hire
        .job_requirement
        .list_by_id_by_query(
            &ListByIdHireJobRequirementQuery::new()
                .user_id_type("open_id")
                .department_id_type("open_department_id")
                .job_level_id_type("job_level_id")
                .job_family_id_type("job_family_id")
                .employee_type_id_type("employee_type_id"),
            json!({"id_list":["jr-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    assert_eq!(
        job_requirements.data.unwrap().items[0]
            .customized_data_list
            .as_ref()
            .unwrap()[0]
            .value
            .as_ref()
            .unwrap()
            .time_range
            .as_ref()
            .unwrap()
            .end_time
            .as_deref(),
        Some("1710003600")
    );
    hire.location
        .query_by_query(
            &QueryHireLocationQuery::new().page(page),
            json!({"code_list":["CN_1"],"location_type":1}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/job_publish_records/search?"));
    assert!(request.contains("POST /open-apis/hire/v1/job_requirements/search?"));
    assert!(request.contains("POST /open-apis/hire/v1/locations/query?"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("employee_type_id_type=employee_type_id"));
    assert!(request.contains(r#""job_channel_id":"channel-1""#));
    assert!(request.contains(r#""id_list":["jr-1"]"#));
    assert!(request.contains(r#""location_type":1"#));
}
