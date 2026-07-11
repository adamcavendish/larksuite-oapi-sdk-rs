mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    ListApplicationInterviewQuery, ListInterviewFeedbackFormQuery,
    ListInterviewRegistrationSchemaQuery, ListInterviewRoundTypeQuery,
};

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

fn tenant_option() -> RequestOption {
    RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..Default::default()
    }
}

#[tokio::test]
async fn hire_interview_support_lists_deserialize_typed_items() {
    let app_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"interview-1","begin_time":1710000000,"stage_id":"stage-1"}],"page_token":"next-1","has_more":false}}"#;
    let feedback_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"form-1","version":1,"name":{"en_us":"Default"},"type":1,"score_calculation_config":{"enabled":true,"calculation_mode":1},"modules":[{"id":"module-1","name":{"en_us":"Technical"},"description":{"en_us":"Technical interview"},"type":1,"sequence":1,"weight":1.0,"dimensions":[{"id":"dimension-1","name":{"en_us":"Coding"},"description":{"en_us":"Coding skill"},"type":2,"enabled":true,"sequence":1,"is_required":true,"weight":0.5,"score_dimension_config":{"score_dimension_type":1,"lower_limit_score":1,"upper_limit_score":5},"option_items":[{"id":"option-1","name":{"en_us":"Strong"},"description":{"en_us":"Strong answer"},"score_val":5,"alias_name":{"en_us":"Excellent"}}],"display_not_evident":true,"ability_list":[{"id":"ability-1","name":{"en_us":"Communication"},"description":{"en_us":"Communication skill"}}],"related_dimension_config":{"type":1,"related_dimension_settings":[{"dimension_id":"dimension-2","related_operator_type":1,"dimension_option_ids":["option-1"]}]},"dimension_ability_args":[{"ability_id":"ability-1","place_holder":"评价","en_place_holder":"Assessment"}]}]}]}],"page_token":"next-form","has_more":false}}"#;
    let round_body = r#"{"code":0,"msg":"ok","data":{"active_status":1,"items":[{"id":"round-1","biz_id":"biz-1","name":{"en_us":"Technical"},"process_type":1,"active_status":1,"interview_assessment_template_info":{"id":"form-1","biz_id":"form-biz-1","name":{"en_us":"Technical Assessment"}}}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, app_body),
        http_response(200, feedback_body),
        http_response(200, round_body),
    ])
    .await;

    let client = client_for(addr);
    let app_resp = client
        .hire()
        .application_interview
        .list_by_query(
            &ListApplicationInterviewQuery::new("app-1")
                .page(PageQuery::new().page_size(20).page_token("seed-token"))
                .user_id_type("open_id")
                .job_level_id_type("job_level_id"),
            &tenant_option(),
        )
        .await
        .unwrap();
    let feedback_resp = client
        .hire()
        .interview_feedback_form
        .list_by_query(
            &ListInterviewFeedbackFormQuery::new().page_size(Some(20)),
            &tenant_option(),
        )
        .await
        .unwrap();
    let round_resp = client
        .hire()
        .interview_round_type
        .list_by_query(
            &ListInterviewRoundTypeQuery::new().process_type(1),
            &tenant_option(),
        )
        .await
        .unwrap();

    let app_data = app_resp.data.unwrap();
    assert_eq!(app_data.items[0].id.as_deref(), Some("interview-1"));
    assert_eq!(app_data.items[0].stage_id.as_deref(), Some("stage-1"));
    assert_eq!(app_data.page_token.as_deref(), Some("next-1"));

    let feedback = &feedback_resp.data.unwrap().items[0];
    assert_eq!(
        feedback
            .score_calculation_config
            .as_ref()
            .unwrap()
            .calculation_mode,
        Some(1)
    );
    let dimension = &feedback.modules.as_ref().unwrap()[0]
        .dimensions
        .as_ref()
        .unwrap()[0];
    assert_eq!(
        dimension
            .score_dimension_config
            .as_ref()
            .unwrap()
            .upper_limit_score,
        Some(5)
    );
    assert_eq!(
        dimension.option_items.as_ref().unwrap()[0]
            .alias_name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Excellent")
    );
    assert_eq!(
        dimension
            .related_dimension_config
            .as_ref()
            .unwrap()
            .related_dimension_settings
            .as_ref()
            .unwrap()[0]
            .dimension_option_ids
            .as_ref()
            .unwrap()[0],
        "option-1"
    );
    assert_eq!(
        dimension.dimension_ability_args.as_ref().unwrap()[0]
            .en_place_holder
            .as_deref(),
        Some("Assessment")
    );

    let round_data = round_resp.data.unwrap();
    assert_eq!(round_data.active_status, Some(1));
    assert_eq!(round_data.items[0].id.as_deref(), Some("round-1"));
    assert_eq!(
        round_data.items[0]
            .name
            .as_ref()
            .and_then(|name| name.en_us.as_deref()),
        Some("Technical")
    );
    assert_eq!(
        round_data.items[0]
            .interview_assessment_template_info
            .as_ref()
            .unwrap()
            .name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Technical Assessment")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/app-1/interviews?"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("job_level_id_type=job_level_id"));
    assert!(request.contains("GET /open-apis/hire/v1/interview_round_types?"));
    assert!(request.contains("GET /open-apis/hire/v1/interview_feedback_forms?"));
    assert!(request.contains("process_type=1"));
}

#[tokio::test]
async fn hire_interviewer_iterator_pages_and_filters() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou_user_1","verify_status":1},{"user_id":"ou_user_2","verify_status":1}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"user_id":"ou_user_3","verify_status":1}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let user_ids = ["ou_user_1", "ou_user_2"];
    let hire = client.hire();
    let mut iter = hire
        .interviewer
        .list_by_iterator(
            Some(2),
            Some(user_ids.as_slice()),
            Some(1),
            Some("1710000000"),
            Some("1710009999"),
            Some("open_id"),
        )
        .limit(2);

    let first = iter.next(&tenant_option()).await.unwrap().unwrap();
    let second = iter.next(&tenant_option()).await.unwrap().unwrap();
    let third = iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first.user_id.as_deref(), Some("ou_user_1"));
    assert_eq!(second.user_id.as_deref(), Some("ou_user_2"));
    assert!(third.is_none());
    assert_eq!(iter.next_page_token(), Some("next-1"));

    let reqs = requests.lock().unwrap();
    assert_eq!(reqs.len(), 1);
    assert!(reqs[0].contains("GET /open-apis/hire/v1/interviewers?"));
    assert!(reqs[0].contains("page_size=2"));
    assert!(reqs[0].contains("user_ids=ou_user_1"));
    assert!(reqs[0].contains("user_ids=ou_user_2"));
    assert!(reqs[0].contains("verify_status=1"));
    assert!(reqs[0].contains("earliest_update_time=1710000000"));
    assert!(reqs[0].contains("latest_update_time=1710009999"));
    assert!(reqs[0].contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_interview_feedback_form_iterator_sends_resume_token() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"form-1","version":1,"name":{"en_us":"Default"},"type":1}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"form-2","version":2,"type":1}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let form_ids = ["form-1"];
    let query = ListInterviewFeedbackFormQuery::new()
        .page_size(Some(1))
        .page_token(Some("seed-token"))
        .interview_feedback_form_ids(Some(form_ids.as_slice()))
        .user_id_type(Some("open_id"));
    let hire = client.hire();
    let mut iter = hire.interview_feedback_form.list_iterator_by_query(&query);

    let first = iter.next(&tenant_option()).await.unwrap().unwrap();
    let _ = iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("form-1"));
    assert_eq!(first.type_, Some(1));

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("GET /open-apis/hire/v1/interview_feedback_forms?"));
    assert!(reqs[0].contains("page_token=seed-token"));
    assert!(reqs[0].contains("interview_feedback_form_ids=form-1"));
    assert!(reqs[1].contains("page_token=next-1"));
}

#[tokio::test]
async fn hire_interview_registration_schema_iterator_limit_zero_is_unlimited() {
    let page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-1","name":"登记表","is_used_as_interview":true}],"page_token":"next-1","has_more":true}}"#;
    let page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-2","name":"登记表2"}],"page_token":"next-2","has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, page1), http_response(200, page2)]).await;

    let client = client_for(addr);
    let hire = client.hire();
    let mut iter = hire
        .interview_registration_schema
        .list_iterator_by_query(
            &ListInterviewRegistrationSchemaQuery::new()
                .page_size(Some(1))
                .user_id_type(Some("open_id")),
        )
        .limit(0);

    let first = iter.next(&tenant_option()).await.unwrap().unwrap();
    let second = iter.next(&tenant_option()).await.unwrap().unwrap();
    let third = iter.next(&tenant_option()).await.unwrap();

    assert_eq!(first.id.as_deref(), Some("schema-1"));
    assert_eq!(first.is_used_as_interview, Some(true));
    assert_eq!(second.id.as_deref(), Some("schema-2"));
    assert!(third.is_none());
    assert_eq!(requests.lock().unwrap().len(), 2);
}
