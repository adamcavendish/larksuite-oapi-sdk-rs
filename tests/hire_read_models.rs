mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    GetByApplicationEmployeeQuery, ListBackgroundCheckOrderQuery, ListEvaluationQuery,
    ListInterviewRecordQuery, ListJobProcessQuery, ListJobSchemaQuery,
    ListOfferApprovalTemplateQuery, ListQuestionnaireQuery, ListTalentTagQuery,
    ListTripartiteAgreementQuery,
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
async fn hire_application_offer_response_deserializes_and_sends_user_id_type() {
    let offer = r#"{"code":0,"msg":"ok","data":{"offer":{"id":"offer-1","application_id":"application-1","schema_id":"offer-schema-1","offer_status":2,"basic_info":{"offer_type":1,"employee_type":{"id":"employee-type-1","zh_name":"正式"},"contract_period":{"period_type":1,"period":3},"customize_info_list":[{"object_id":"field-1","customize_value":"value-1"}],"common_attachment_id_list":["attachment-1"]},"salary_plan":{"currency":"CNY","basic_salary":"10000","customize_info_list":[{"object_id":"salary-field-1","customize_value":"value-2"}]},"job_info":{"job_id":"job-1","job_name":"Engineer"},"customized_module_list":[{"ID":"module-1","object_list":[{"object_id":"module-field-1","customize_value":"value-3"}]}],"job_requirement_id":"requirement-1","offer_send_record_list":[{"offer_send_record_id":"send-1","operator_user_id":"ou_operator","offer_letter_status":1,"email_info":{"subject":"Offer"}}]}}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, offer)]).await;

    let client = client_for(addr);
    let offer = client
        .hire()
        .application
        .offer("application-1", Some("open_id"), &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .offer
        .unwrap();

    assert_eq!(offer.id.as_deref(), Some("offer-1"));
    assert_eq!(offer.offer_status, Some(2));
    assert_eq!(
        offer
            .basic_info
            .as_ref()
            .unwrap()
            .contract_period
            .as_ref()
            .unwrap()
            .period,
        Some(3)
    );
    assert_eq!(
        offer
            .salary_plan
            .as_ref()
            .unwrap()
            .customize_info_list
            .as_ref()
            .unwrap()[0]
            .object_id
            .as_deref(),
        Some("salary-field-1")
    );
    assert_eq!(
        offer.job_info.as_ref().unwrap().job_name.as_deref(),
        Some("Engineer")
    );
    assert_eq!(
        offer.customized_module_list.as_ref().unwrap()[0]
            .id
            .as_deref(),
        Some("module-1")
    );
    assert_eq!(
        offer.offer_send_record_list.as_ref().unwrap()[0].offer_letter_status,
        Some(1)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/application-1/offer?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_job_response_models_deserialize_and_preserve_requests() {
    let config = r#"{"code":0,"msg":"ok","data":{"job_config":{"id":"job-1","offer_apply_schema":{"id":"offer-schema-1","name":{"en_us":"Offer"}},"interview_round_list":[{"round":1,"interviewer_list":[{"id":"ou_interviewer"}]}],"interview_registration":{"schema_id":"interview-schema-1","name":"Interview"},"interview_round_type_list":[{"assessment_round":{"id":"round-type-1"}}],"interview_appointment_config":{"enable_interview_appointment_by_interviewer":true,"config":{"interview_type":1,"cc":["ou_cc"]}}}}}"#;
    let combined_create = r#"{"code":0,"msg":"ok","data":{"default_job_post":{"id":"post-1"},"job":{"id":"job-1","title":"Engineer"},"job_manager":{"id":"manager-1","recruiter_id":"ou_recruiter"},"interview_registration_schema_info":{"schema_id":"interview-schema-1","name":"Interview"},"target_major_list":[{"id":"major-1","zh_name":"Computer Science","en_name":"Computer Science"}]}}"#;
    let combined_update = r#"{"code":0,"msg":"ok","data":{"default_job_post":{"id":"post-1"},"job":{"id":"job-1","title":"Staff Engineer"},"job_manager":{"id":"manager-1","assistant_id_list":["ou_assistant"]},"onboard_registration_schema_info":{"schema_id":"onboard-schema-1","name":"Onboard"}}}"#;
    let update_config = r#"{"code":0,"msg":"ok","data":{"job_config":{"id":"job-1","job_attribute":2,"onboard_registration":{"schema_id":"onboard-schema-1","name":"Onboard"}}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, config),
        http_response(200, combined_create),
        http_response(200, combined_update),
        http_response(200, update_config),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let config = Box::pin(hire.job.config("job-1", &RequestOption::default()))
        .await
        .unwrap()
        .data
        .unwrap()
        .job_config
        .unwrap();
    let combined_create = Box::pin(hire.job.combined_create(
        json!({"job":{"title":"Engineer"}}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();
    let combined_update = Box::pin(hire.job.combined_update(
        "job-1",
        json!({"job":{"title":"Staff Engineer"}}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();
    let update_config = Box::pin(hire.job.update_config(
        "job-1",
        json!({"job_attribute":2}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap()
    .job_config
    .unwrap();

    assert_eq!(config.id.as_deref(), Some("job-1"));
    assert_eq!(
        config.interview_round_list.as_ref().unwrap()[0].round,
        Some(1)
    );
    assert_eq!(
        config
            .interview_appointment_config
            .as_ref()
            .unwrap()
            .config
            .as_ref()
            .unwrap()
            .cc
            .as_ref()
            .unwrap()[0],
        "ou_cc"
    );
    assert_eq!(
        combined_create.default_job_post.unwrap().id.as_deref(),
        Some("post-1")
    );
    assert_eq!(
        combined_create.target_major_list.unwrap()[0]
            .zh_name
            .as_deref(),
        Some("Computer Science")
    );
    assert_eq!(
        combined_update
            .onboard_registration_schema_info
            .unwrap()
            .schema_id
            .as_deref(),
        Some("onboard-schema-1")
    );
    assert_eq!(update_config.job_attribute, Some(2));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs/job-1/config "));
    assert!(request.contains("POST /open-apis/hire/v1/jobs/combined_create "));
    assert!(request.contains("POST /open-apis/hire/v1/jobs/job-1/combined_update "));
    assert!(request.contains("POST /open-apis/hire/v1/jobs/job-1/update_config "));
    assert!(request.contains(r#""title":"Engineer""#));
    assert!(request.contains("job_attribute"));
}

#[tokio::test]
async fn hire_config_read_models_deserialize_and_send_filters() {
    let job_process = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"process-1","zh_name":"流程","type":1,"stage_list":[{"id":"stage-1","type":4}]}],"has_more":false,"page_token":"p2"}}"#;
    let job_schema = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-1","scenario_type":2,"name":{"en_us":"Job schema"},"object_list":[{"id":"field-1","name":{"zh_cn":"字段"}}]}],"has_more":false}}"#;
    let offer_form = r#"{"code":0,"msg":"ok","data":{"offer_apply_form":{"id":"form-1","name":{"en_us":"Offer Form"},"schema":{"id":"schema-v1","module_list":[{"id":"module-1","object_list":[{"id":"object-1","object_type_v2":"text","config":{"max_length":100}}]}]}}}}"#;
    let offer_template = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"template-1","name":{"zh_cn":"审批"},"department_list":[{"id":"dept-1","name":"研发"}]}],"has_more":false}}"#;
    let questionnaire = r#"{"code":0,"msg":"ok","data":{"items":[{"questionnaire_id":"q-1","application_id":"app-1","version":3,"questions":[{"id":"question-1"}],"has_answers":true}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, job_process),
        http_response(200, job_schema),
        http_response(200, offer_form),
        http_response(200, offer_template),
        http_response(200, questionnaire),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    let process_resp = hire
        .job_process
        .list_by_query(
            &ListJobProcessQuery::new().page(page),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let schema_resp = hire
        .job_schema
        .list_by_query(
            &ListJobSchemaQuery::new().page(page).scenario(2),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let form_resp = hire
        .offer_application_form
        .get("form-1", &RequestOption::default())
        .await
        .unwrap();
    let template_resp = hire
        .offer_approval_template
        .list_by_query(
            &ListOfferApprovalTemplateQuery::new()
                .page(page)
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let questionnaire_resp = hire
        .questionnaire
        .list_by_query(
            &ListQuestionnaireQuery::new()
                .page(page)
                .application_id("app-1")
                .interview_id("interview-1")
                .update_start_time("1710000000")
                .update_end_time("1710009999"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        process_resp.data.unwrap().items[0]
            .stage_list
            .as_ref()
            .unwrap()[0]
            .type_,
        Some(4)
    );
    assert_eq!(
        schema_resp.data.unwrap().items[0]
            .object_list
            .as_ref()
            .unwrap()[0]
            .id
            .as_deref(),
        Some("field-1")
    );
    assert_eq!(
        form_resp
            .data
            .unwrap()
            .offer_apply_form
            .unwrap()
            .schema
            .unwrap()
            .module_list
            .unwrap()[0]
            .object_list
            .as_ref()
            .unwrap()[0]
            .object_type_v2
            .as_deref(),
        Some("text")
    );
    assert_eq!(
        template_resp.data.unwrap().items[0]
            .department_list
            .as_ref()
            .unwrap()[0]
            .name
            .as_deref(),
        Some("研发")
    );
    assert_eq!(
        questionnaire_resp.data.unwrap().items[0].has_answers,
        Some(true)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/job_processes?"));
    assert!(request.contains("GET /open-apis/hire/v1/job_schemas?"));
    assert!(request.contains("scenario=2"));
    assert!(request.contains("GET /open-apis/hire/v1/offer_application_forms/form-1 "));
    assert!(request.contains("department_id_type=open_department_id"));
    assert!(request.contains("GET /open-apis/hire/v1/questionnaires?"));
    assert!(request.contains("application_id=app-1"));
    assert_eq!(request.matches("page_token=seed-token").count(), 4);
}

#[tokio::test]
async fn hire_activity_read_models_deserialize_and_send_filters() {
    let employee = r#"{"code":0,"msg":"ok","data":{"employee":{"id":"emp-1","application_id":"app-1","onboard_status":1,"external_employment_id":"ehr-1"}}}"#;
    let evaluation = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"eval-1","application_id":"app-1","conclusion":2,"content":"Strong"}],"page_token":"eval-next","has_more":false}}"#;
    let note_get = r#"{"code":0,"msg":"ok","data":{"note":{"id":"note-1","talent_id":"talent-1","content":"hello","mention_entity_list":[{"id":"user-1","name":"Ada"}]}}}"#;
    let note_list =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"note-2","privacy":1}],"has_more":false}}"#;
    let record_get = r#"{"code":0,"msg":"ok","data":{"interview_record":{"id":"record-1","user_id":"ou_1","interviewer":{"id":"ou_1","en_name":"Interviewer"},"assessment_score":{"score":5}}}}"#;
    let record_list = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"record-2","feedback_submit_time":1710000000}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, employee),
        http_response(200, evaluation),
        http_response(200, note_get),
        http_response(200, note_list),
        http_response(200, record_get),
        http_response(200, record_list),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(10).page_token("seed-token");

    let employee_resp = hire
        .employee
        .get_by_application_query(
            &GetByApplicationEmployeeQuery::new()
                .application_id("app-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let evaluation_resp = hire
        .evaluation
        .list_by_query(
            &ListEvaluationQuery::new()
                .page(page)
                .application_id("app-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let note = hire
        .note
        .get("note-1", &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .note
        .unwrap();
    let note_list_resp = hire
        .note
        .list_by_query(
            &larksuite_oapi_sdk_rs::service::hire::v1::ListNoteQuery::new()
                .page(page)
                .talent_id("talent-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let record = hire
        .interview_record
        .get("record-1", &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .interview_record
        .unwrap();
    let record_ids = ["record-1", "record-2"];
    let record_list_resp = hire
        .interview_record
        .list_by_query(
            &ListInterviewRecordQuery::new()
                .page(page)
                .ids(Some(&record_ids[..]))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        employee_resp
            .data
            .unwrap()
            .employee
            .unwrap()
            .external_employment_id
            .as_deref(),
        Some("ehr-1")
    );
    assert_eq!(evaluation_resp.data.unwrap().items[0].conclusion, Some(2));
    assert_eq!(
        note.mention_entity_list.unwrap()[0].name.as_deref(),
        Some("Ada")
    );
    assert_eq!(note_list_resp.data.unwrap().items[0].privacy, Some(1));
    assert_eq!(
        record
            .assessment_score
            .as_ref()
            .and_then(|value| value.get("score"))
            .and_then(|value| value.as_i64()),
        Some(5)
    );
    assert_eq!(
        record_list_resp.data.unwrap().items[0].feedback_submit_time,
        Some(1710000000)
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/employees/get_by_application?"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("GET /open-apis/hire/v1/evaluations?"));
    assert!(request.contains("GET /open-apis/hire/v1/notes/note-1 "));
    assert!(request.contains("GET /open-apis/hire/v1/notes?"));
    assert!(request.contains("talent_id=talent-1"));
    assert!(request.contains("GET /open-apis/hire/v1/interview_records/record-1 "));
    assert!(request.contains("GET /open-apis/hire/v1/interview_records?"));
    assert!(request.contains("ids=record-1"));
    assert!(request.contains("ids=record-2"));
}

#[tokio::test]
async fn hire_new_read_iterators_and_background_models_use_page_state() {
    let tag_page1 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"tag-1","type":1}],"page_token":"tag-next","has_more":true}}"#;
    let tag_page2 = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"tag-2","active_status":1}],"has_more":false}}"#;
    let evaluation_page =
        r#"{"code":0,"msg":"ok","data":{"items":[{"id":"eval-1"}],"has_more":false}}"#;
    let agreement_page = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"agreement-1","application_id":"app-1","state":1}],"has_more":false}}"#;
    let background_list = r#"{"code":0,"msg":"ok","data":{"items":[{"order_id":"order-1","application_id":"app-1","package":"standard","candidate_info":{"name":"Ada"}}],"has_more":false}}"#;
    let background_batch = r#"{"code":0,"msg":"ok","data":{"items":[{"order_id":"order-2","order_status":2,"provider_info":{"id":"provider-1"}}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, tag_page1),
        http_response(200, tag_page2),
        http_response(200, evaluation_page),
        http_response(200, agreement_page),
        http_response(200, background_list),
        http_response(200, background_batch),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let tag_query = ListTalentTagQuery::new()
        .page_size(1)
        .page_token("tag-seed");
    let mut tags = hire.talent_tag.list_iterator_by_query(&tag_query);
    let first_tag = tags.next(&RequestOption::default()).await.unwrap().unwrap();
    let second_tag = tags.next(&RequestOption::default()).await.unwrap().unwrap();

    let evaluation_query = ListEvaluationQuery::new()
        .page_size(1)
        .application_id("app-1");
    let mut evaluations = hire.evaluation.list_iterator_by_query(&evaluation_query);
    let evaluation = evaluations
        .next(&RequestOption::default())
        .await
        .unwrap()
        .unwrap();

    let agreement_query = ListTripartiteAgreementQuery::new()
        .page_size(1)
        .application_id("app-1");
    let mut agreements = hire
        .tripartite_agreement
        .list_iterator_by_query(&agreement_query);
    let agreement = agreements
        .next(&RequestOption::default())
        .await
        .unwrap()
        .unwrap();

    let background = hire
        .background_check_order
        .list_by_query(
            &ListBackgroundCheckOrderQuery::new()
                .page_size(20)
                .application_id("app-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);
    let batch = hire
        .background_check_order
        .batch_query(json!({"application_id":"app-1"}), &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .items
        .remove(0);

    assert_eq!(first_tag.id.as_deref(), Some("tag-1"));
    assert_eq!(second_tag.id.as_deref(), Some("tag-2"));
    assert_eq!(evaluation.id.as_deref(), Some("eval-1"));
    assert_eq!(agreement.state, Some(1));
    assert_eq!(
        background
            .candidate_info
            .as_ref()
            .and_then(|value| value.get("name"))
            .and_then(|value| value.as_str()),
        Some("Ada")
    );
    assert_eq!(batch.order_status, Some(2));

    let reqs = requests.lock().unwrap();
    assert!(reqs[0].contains("GET /open-apis/hire/v1/talent_tags?"));
    assert!(reqs[0].contains("page_token=tag-seed"));
    assert!(reqs[1].contains("page_token=tag-next"));
    assert!(reqs[2].contains("GET /open-apis/hire/v1/evaluations?"));
    assert!(reqs[3].contains("GET /open-apis/hire/v1/tripartite_agreements?"));
    assert!(reqs[4].contains("GET /open-apis/hire/v1/background_check_orders?"));
    assert!(reqs[4].contains("user_id_type=open_id"));
    assert!(reqs[5].contains("POST /open-apis/hire/v1/background_check_orders/batch_query "));
    assert!(reqs[5].contains(r#""application_id":"app-1""#));
}
