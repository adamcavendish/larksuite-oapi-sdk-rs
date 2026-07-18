use super::prelude::*;

// ── Hire ──

#[tokio::test]
async fn hire_application_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"application":{"id":"app-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetHireApplicationQuery::new("app-1").user_id_type("open_id");
    let resp = client
        .hire()
        .application
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().application.unwrap().id.as_deref(),
        Some("app-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_application_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"app-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListHireApplicationQuery::new()
        .process_id("process-1")
        .job_id("job-1")
        .stage_id("stage-1")
        .talent_id("talent-1")
        .active_status(1)
        .user_id_type("open_id")
        .update_start_time("1618500278663")
        .update_end_time("1618500279999")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .hire()
        .application
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("app-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications?"));
    assert!(request.contains("process_id=process-1"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("stage_id=stage-1"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("active_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("update_start_time=1618500278663"));
    assert!(request.contains("update_end_time=1618500279999"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_application_list_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"app-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .application
        .list(
            Some(20),
            Some("next-page"),
            Some("job-1"),
            Some("stage-1"),
            Some("talent-1"),
            Some(1),
            Some("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items[0].id.as_deref(), Some("app-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications?"));
    assert!(request.contains("job_id=job-1"));
    assert!(request.contains("stage_id=stage-1"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("active_status=1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn hire_application_offer_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"offer":{"id":"offer-1","application_id":"app-1","basic_info":{"onboard_address":{"city":{"en_name":"Shanghai"}},"work_location_address_info":{"location_info":{"id":"location-1"}}},"job_info":{"job_id":"job-1","job_name":"Engineer"},"offer_send_record_list":[{"email_info":{"receiver_email_list":["candidate@example.com"]},"acceptance_list":[{"conclusion":1}],"offer_file_list":[{"id":"file-1"}],"offer_signature_info":{"attachment_list":[{"file_name":"Signed Offer.pdf"}]}}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .hire()
        .application
        .offer("app-1", Some("open_id"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let offer = resp.data.unwrap().offer.unwrap();
    assert_eq!(
        offer.job_info.unwrap().job_name.as_deref(),
        Some("Engineer")
    );
    assert_eq!(
        offer
            .basic_info
            .as_ref()
            .unwrap()
            .onboard_address
            .as_ref()
            .unwrap()
            .city
            .as_ref()
            .unwrap()
            .en_name
            .as_deref(),
        Some("Shanghai")
    );
    let send_record = &offer.offer_send_record_list.as_ref().unwrap()[0];
    assert_eq!(
        send_record
            .email_info
            .as_ref()
            .unwrap()
            .receiver_email_list
            .as_ref()
            .unwrap()[0],
        "candidate@example.com"
    );
    assert_eq!(
        send_record.acceptance_list.as_ref().unwrap()[0].conclusion,
        Some(1)
    );
    assert_eq!(
        send_record
            .offer_signature_info
            .as_ref()
            .unwrap()
            .attachment_list
            .as_ref()
            .unwrap()[0]
            .file_name
            .as_deref(),
        Some("Signed Offer.pdf")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1/offer?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_application_lifecycle_smoke() {
    let detail = r#"{"code":0,"msg":"ok","data":{"application_detail":{"basic_info":{"id":"app-1","stage":{"id":"stage-1","type":2}},"job":{"id":"job-1","name":"Engineer"},"talent":{"id":"talent-1","name":"Taylor"}}}}"#;
    let empty = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, detail),
        http_response(200, empty),
        http_response(200, empty),
    ])
    .await;

    let client = client_for(addr);
    let detail = client
        .hire()
        .application
        .get_detail("app-1", &RequestOption::default())
        .await
        .unwrap();
    let cancel_onboard = client
        .hire()
        .application
        .cancel_onboard(
            "app-1",
            json_value!({"termination_type": 1}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let recover = client
        .hire()
        .application
        .recover("app-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(
        detail
            .data
            .unwrap()
            .application_detail
            .unwrap()
            .job
            .unwrap()
            .name
            .as_deref(),
        Some("Engineer")
    );
    assert!(cancel_onboard.success());
    assert!(recover.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1/get_detail "));
    assert!(request.contains("POST /open-apis/hire/v1/applications/app-1/cancel_onboard "));
    assert!(request.contains("POST /open-apis/hire/v1/applications/app-1/recover "));
}

#[tokio::test]
async fn hire_application_detail_graph_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"application_detail":{"evaluations":[{"id":"evaluation-1","conclusion":1}],"interview_aggregation":{"interviews":[{"id":"interview-1","interview_record_list":[{"record_score":{"score":9.0},"attachments":[{"file_id":"file-1"}],"module_assessments":[{"module_score":4.5,"dimension_assessments":[{"dimension_score":5,"ability_assessments":[{"content":"Strong"}]}]}]}],"meeting_room_list":[{"room_id":"room-1"}]}]},"offer":{"offer_basic":{"id":"offer-1","attachment_list":[{"id":"attachment-1","name":"Offer.pdf"}]}},"employee":{"id":"employee-1","department_id":"dept-1"},"agency":{"basic_info":{"hunter_company_name":"Search Co"}},"portal":{"campus_volunteer_info":{"volunteer_seq":1}},"referral":{"basic_info":{"id":"referral-1","user_info":{"id":"ou_referrer"}},"recommend_info":{"specific_relationship":{"extra":"Former colleague"}}}}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let detail = client
        .hire()
        .application
        .get_detail("app-1", &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .application_detail
        .unwrap();

    assert_eq!(detail.evaluations.unwrap()[0].conclusion, Some(1));
    let interviews = detail.interview_aggregation.unwrap().interviews.unwrap();
    assert_eq!(
        interviews[0].meeting_room_list.as_ref().unwrap()[0]
            .room_id
            .as_deref(),
        Some("room-1")
    );
    let record = &interviews[0].interview_record_list.as_ref().unwrap()[0];
    assert_eq!(record.record_score.as_ref().unwrap().score, Some(9.0));
    assert_eq!(
        record.module_assessments.as_ref().unwrap()[0]
            .dimension_assessments
            .as_ref()
            .unwrap()[0]
            .ability_assessments
            .as_ref()
            .unwrap()[0]
            .content
            .as_deref(),
        Some("Strong")
    );
    assert_eq!(
        detail
            .offer
            .unwrap()
            .offer_basic
            .unwrap()
            .attachment_list
            .unwrap()[0]
            .name
            .as_deref(),
        Some("Offer.pdf")
    );
    assert_eq!(
        detail.employee.unwrap().department_id.as_deref(),
        Some("dept-1")
    );
    assert_eq!(
        detail
            .agency
            .unwrap()
            .basic_info
            .unwrap()
            .hunter_company_name
            .as_deref(),
        Some("Search Co")
    );
    assert_eq!(
        detail
            .portal
            .unwrap()
            .campus_volunteer_info
            .unwrap()
            .volunteer_seq,
        Some(1)
    );
    assert_eq!(
        detail
            .referral
            .as_ref()
            .unwrap()
            .basic_info
            .as_ref()
            .unwrap()
            .user_info
            .as_ref()
            .unwrap()
            .id
            .as_deref(),
        Some("ou_referrer")
    );
    assert_eq!(
        detail
            .referral
            .unwrap()
            .recommend_info
            .unwrap()
            .specific_relationship
            .unwrap()
            .extra
            .as_deref(),
        Some("Former colleague")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1/get_detail "));
}
