use super::prelude::*;

// Contact job smoke tests

#[tokio::test]
async fn contact_job_level_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"job_level_id":"level-1","name":"L1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactJobLevelQuery::new()
        .name("L1")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .job_level
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|level| level.job_level_id.as_deref()),
        Some("level-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/job_levels?"));
    assert!(request.contains("name=L1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_job_family_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"job_family_id":"family-1","name":"Engineering"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactJobFamilyQuery::new()
        .name("Engineering")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .contact()
        .job_family
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|family| family.job_family_id.as_deref()),
        Some("family-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/job_families?"));
    assert!(request.contains("name=Engineering"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn contact_job_title_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"job_title_id":"title-1","name":"Engineer"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListContactJobTitleQuery::new()
        .page_size(20)
        .page_token("next-page");
    let resp = client
        .contact()
        .job_title
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|title| title.job_title_id.as_deref()),
        Some("title-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/contact/v3/job_titles?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
