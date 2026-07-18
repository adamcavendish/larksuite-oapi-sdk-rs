mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    BatchQueryExternalBackgroundCheckQuery, BatchQueryExternalInterviewQuery,
    BatchQueryExternalOfferQuery, GetByTalentInterviewQuery, ListExternalApplicationQuery,
};
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

#[tokio::test]
async fn hire_external_read_responses_deserialize_and_send_filters() {
    let applications = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"external-app-1","external_id":"crm-app-1","job_title":"Backend","talent_id":"talent-1","modify_time":1710000000}],"has_more":false,"page_token":"app-next"}}"#;
    let background_checks = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"bg-1","external_application_id":"external-app-1","attachment_list":[{"id":"file-1","name":"bg.pdf","size":42}]}],"has_more":false,"page_token":"bg-next"}}"#;
    let interviews = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"interview-1","external_application_id":"external-app-1","interview_assessments":[{"id":"assessment-1","assessment_dimension_list":[{"title":"Skill","score":5}]}]}],"has_more":false,"page_token":"interview-next"}}"#;
    let offers = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"offer-1","external_application_id":"external-app-1","offer_status":"sent","attachment_list":[{"id":"offer-file","name":"offer.pdf","size":7}]}],"has_more":false,"page_token":"offer-next"}}"#;
    let talent_interviews = r#"{"code":0,"msg":"ok","data":{"items":[{"application_id":"app-1","interview_list":[{"id":"interview-1","address":{"id":"address-1","city":{"code":"CN-SH","name":{"en_us":"Shanghai"}}},"meeting_room_list":[{"room_id":"room-1","room_name":"Orchid"}],"interview_record_list":[{"id":"record-1","interview_score":{"id":"score-1","level":3},"assessment_score":{"calculate_type":1,"score":8.5,"full_score":10},"question_list":[{"id":"question-1","title":{"en_us":"System design"},"ability_list":[{"id":"ability-1","name":{"en_us":"Architecture"}}]}],"image_list":[{"id":"image-1","name":"whiteboard.png"}],"dimension_assessment_list":[{"id":"dimension-1","dimension_score":{"id":"dimension-score-1","score_val":5},"question_list":[{"id":"question-2","content":"Strong"}]}]}]}]}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, applications),
        http_response(200, background_checks),
        http_response(200, interviews),
        http_response(200, offers),
        http_response(200, talent_interviews),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    let application = hire
        .external_application
        .list_by_query(
            &ListExternalApplicationQuery::new()
                .page(page)
                .talent_id("talent-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let background_check = hire
        .external_background_check
        .batch_query_by_query(
            &BatchQueryExternalBackgroundCheckQuery::new()
                .page(page)
                .external_application_id("external-app-1"),
            json!({"external_background_check_id_list":["bg-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let interview = hire
        .external_interview
        .batch_query_by_query(
            &BatchQueryExternalInterviewQuery::new()
                .page(page)
                .external_application_id("external-app-1"),
            json!({"external_interview_id_list":["interview-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let offer = hire
        .external_offer
        .batch_query_by_query(
            &BatchQueryExternalOfferQuery::new()
                .page(page)
                .external_application_id("external-app-1"),
            json!({"external_offer_id_list":["offer-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let talent_interview = hire
        .interview
        .get_by_talent_query(
            &GetByTalentInterviewQuery::new()
                .talent_id("talent-1")
                .user_id_type("open_id")
                .job_level_id_type("job_level_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);

    assert_eq!(application.job_title.as_deref(), Some("Backend"));
    assert_eq!(application.modify_time, Some(1710000000));
    assert_eq!(
        background_check.attachment_list.as_ref().unwrap()[0]
            .name
            .as_deref(),
        Some("bg.pdf")
    );
    assert_eq!(
        interview.interview_assessments.as_ref().unwrap()[0]
            .assessment_dimension_list
            .as_ref()
            .unwrap()[0]
            .score,
        Some(5)
    );
    assert_eq!(
        offer.attachment_list.as_ref().unwrap()[0].id.as_deref(),
        Some("offer-file")
    );
    assert_eq!(
        talent_interview.interview_list.as_ref().unwrap()[0]
            .id
            .as_deref(),
        Some("interview-1")
    );
    let record = &talent_interview.interview_list.as_ref().unwrap()[0]
        .interview_record_list
        .as_ref()
        .unwrap()[0];
    assert_eq!(record.interview_score.as_ref().unwrap().level, Some(3));
    assert_eq!(record.assessment_score.as_ref().unwrap().score, Some(8.5));
    assert_eq!(
        record.question_list.as_ref().unwrap()[0]
            .ability_list
            .as_ref()
            .unwrap()[0]
            .id
            .as_deref(),
        Some("ability-1")
    );
    assert_eq!(
        record.dimension_assessment_list.as_ref().unwrap()[0]
            .dimension_score
            .as_ref()
            .unwrap()
            .score_val,
        Some(5)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/external_applications?"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("POST /open-apis/hire/v1/external_background_checks/batch_query?"));
    assert!(request.contains("POST /open-apis/hire/v1/external_interviews/batch_query?"));
    assert!(request.contains("POST /open-apis/hire/v1/external_offers/batch_query?"));
    assert!(request.contains("external_application_id=external-app-1"));
    assert!(request.contains(r#""external_background_check_id_list":["bg-1"]"#));
    assert!(request.contains(r#""external_interview_id_list":["interview-1"]"#));
    assert!(request.contains(r#""external_offer_id_list":["offer-1"]"#));
    assert!(request.contains("GET /open-apis/hire/v1/interviews/get_by_talent?"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert_eq!(request.matches("page_token=seed-token").count(), 4);
}

#[tokio::test]
async fn hire_external_read_iterators_page_and_preserve_filters() {
    let app_page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"external-app-1"}],"page_token":"app-next","has_more":true}}"#;
    let app_page2 =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"external-app-2"}],"has_more":false}}"#;
    let offer_page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"offer-1"}],"page_token":"offer-next","has_more":true}}"#;
    let offer_page2 =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"offer-2"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, app_page1),
        http_response(200, app_page2),
        http_response(200, offer_page1),
        http_response(200, offer_page2),
    ])
    .await;

    let client = client_for(addr);
    let app_query = ListExternalApplicationQuery::new()
        .page_size(1)
        .page_token("seed-app")
        .talent_id("talent-1");
    let hire = client.hire();
    let mut app_iter = hire
        .external_application
        .list_iterator_by_query(&app_query)
        .limit(0);

    let first_app = app_iter
        .next(&RequestOption::default())
        .await
        .unwrap()
        .unwrap();
    let second_app = app_iter
        .next(&RequestOption::default())
        .await
        .unwrap()
        .unwrap();
    let third_app = app_iter.next(&RequestOption::default()).await.unwrap();

    assert_eq!(first_app.id.as_deref(), Some("external-app-1"));
    assert_eq!(second_app.id.as_deref(), Some("external-app-2"));
    assert!(third_app.is_none());

    let offer_query = BatchQueryExternalOfferQuery::new()
        .page_size(1)
        .page_token("seed-offer")
        .external_application_id("external-app-1");
    let mut offer_iter = hire
        .external_offer
        .batch_query_iterator_by_query(
            &offer_query,
            larksuite_oapi_sdk_rs::JsonValue::from(json!({
                "external_offer_id_list":["offer-1","offer-2"]
            })),
        )
        .limit(2);

    let first_offer = offer_iter
        .next(&RequestOption::default())
        .await
        .unwrap()
        .unwrap();
    let second_offer = offer_iter
        .next(&RequestOption::default())
        .await
        .unwrap()
        .unwrap();
    let third_offer = offer_iter.next(&RequestOption::default()).await.unwrap();

    assert_eq!(first_offer.id.as_deref(), Some("offer-1"));
    assert_eq!(second_offer.id.as_deref(), Some("offer-2"));
    assert!(third_offer.is_none());
    assert_eq!(offer_iter.next_page_token(), None);

    let reqs = requests.lock().unwrap();
    assert_eq!(reqs.len(), 4);
    assert!(reqs[0].contains("page_token=seed-app"));
    assert!(reqs[1].contains("page_token=app-next"));
    assert!(reqs[0].contains("talent_id=talent-1"));
    assert!(reqs[1].contains("talent_id=talent-1"));
    assert!(reqs[2].contains("page_token=seed-offer"));
    assert!(reqs[3].contains("page_token=offer-next"));
    assert!(reqs[2].contains("external_application_id=external-app-1"));
    assert!(reqs[3].contains("external_application_id=external-app-1"));
    assert!(reqs[2].contains(r#""external_offer_id_list":["offer-1","offer-2"]"#));
    assert!(reqs[3].contains(r#""external_offer_id_list":["offer-1","offer-2"]"#));
}
