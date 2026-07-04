use super::prelude::*;

#[tokio::test]
async fn hire_catalog_list_by_query_smoke() {
    let registration_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-1","name":"登记表"}],"page_token":"next-page","has_more":false}}"#;
    let resume_source_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"source-1","zh_name":"内推","en_name":"Referral"}],"page_token":"next-page","has_more":false}}"#;
    let i18n_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"item-1","name":{"zh_cn":"名称","en_us":"Name"}}],"page_token":"next-page","has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, registration_body),
        http_response(200, resume_source_body),
        http_response(200, i18n_body),
        http_response(200, i18n_body),
        http_response(200, i18n_body),
        http_response(200, i18n_body),
        http_response(200, i18n_body),
    ])
    .await;

    let client = client_for(addr);
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    client
        .hire()
        .registration_schema
        .list_by_query(
            &ListHireRegistrationSchemaQuery::new().page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .resume_source
        .list_by_query(
            &ListHireResumeSourceQuery::new().page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .job_function
        .list_by_query(
            &ListHireJobFunctionQuery::new().page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .job_type
        .list_by_query(
            &ListHireJobTypeQuery::new().page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .location
        .list_by_query(
            &ListHireLocationQuery::new().page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .role
        .list_by_query(
            &ListHireRoleQuery::new().page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .hire()
        .website
        .list_by_query(
            &ListHireWebsiteQuery::new().page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/registration_schemas?"));
    assert!(request.contains("GET /open-apis/hire/v1/resume_sources?"));
    assert!(request.contains("GET /open-apis/hire/v1/job_functions?"));
    assert!(request.contains("GET /open-apis/hire/v1/job_types?"));
    assert!(request.contains("GET /open-apis/hire/v1/locations?"));
    assert!(request.contains("GET /open-apis/hire/v1/roles?"));
    assert!(request.contains("GET /open-apis/hire/v1/websites?"));
    assert_eq!(request.matches("page_size=20").count(), 7);
    assert_eq!(request.matches("page_token=seed-token").count(), 7);
}
