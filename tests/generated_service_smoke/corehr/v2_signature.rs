use super::prelude::*;

// CoreHR v2 signature smoke tests

#[tokio::test]
async fn corehr_v2_signature_file_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"signature-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureFileV2Query::new()
        .signature_file_id("signature-1")
        .states("sign_finished")
        .update_time_start("2026-01-01")
        .update_time_end("2026-12-31")
        .user_id_type("people_corehr_id")
        .template_ids("template-1")
        .select_sign_url(true)
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .signature_file
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("signature-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_files?"));
    assert!(request.contains("signature_file_id=signature-1"));
    assert!(request.contains("states=sign_finished"));
    assert!(request.contains("update_time_start=2026-01-01"));
    assert!(request.contains("update_time_end=2026-12-31"));
    assert!(request.contains("user_id_type=people_corehr_id"));
    assert!(request.contains("template_ids=template-1"));
    assert!(request.contains("select_sign_url=true"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn corehr_v2_signature_file_list_by_biz_id_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"signature-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureFileByBizIdV2Query::new()
        .biz_process_id("biz-process-1")
        .biz_type("OpenAPI")
        .user_id_type("people_corehr_id")
        .select_sign_url(true);
    let resp = client
        .corehr_v2()
        .signature_file
        .list_by_biz_id_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("signature-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_files/list_by_biz_id?"));
    assert!(request.contains("biz_process_id=biz-process-1"));
    assert!(request.contains("biz_type=OpenAPI"));
    assert!(request.contains("user_id_type=people_corehr_id"));
    assert!(request.contains("select_sign_url=true"));
}

#[tokio::test]
async fn corehr_v2_signature_node_list_by_file_id_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"node-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureNodeByFileIdV2Query::new()
        .file_id("file-1")
        .user_id_type("people_corehr_id");
    let resp = client
        .corehr_v2()
        .signature_node
        .list_by_file_id_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("node-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_nodes/list_by_file_id?"));
    assert!(request.contains("file_id=file-1"));
    assert!(request.contains("user_id_type=people_corehr_id"));
}

#[tokio::test]
async fn corehr_v2_signature_template_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"template-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = SearchCorehrSignatureTemplateV2Query::new()
        .template_ids("template-1")
        .select_custom_field(true);
    let resp = client
        .corehr_v2()
        .signature_template
        .search_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("template-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_templates/search?"));
    assert!(request.contains("template_ids=template-1"));
    assert!(request.contains("select_custom_field=true"));
}

#[tokio::test]
async fn corehr_v2_signature_template_info_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"template-info-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListCorehrSignatureTemplateInfoWithThumbnailV2Query::new()
        .name("Onboarding")
        .category_apiname("contract_agreement")
        .usage_apiname("general")
        .active(false)
        .need_region_info(true)
        .user_id_type("people_corehr_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .corehr_v2()
        .signature_template_info_with_thumbnail
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0]["id"].as_str(), Some("template-info-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/corehr/v2/signature_template_info_with_thumbnails?"));
    assert!(request.contains("name=Onboarding"));
    assert!(request.contains("category_apiname=contract_agreement"));
    assert!(request.contains("usage_apiname=general"));
    assert!(request.contains("active=false"));
    assert!(request.contains("need_region_info=true"));
    assert!(request.contains("user_id_type=people_corehr_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
