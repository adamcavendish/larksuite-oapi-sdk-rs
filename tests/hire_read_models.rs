mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::common::PageQuery;
use larksuite_oapi_sdk_rs::service::hire::v1::{
    GetByApplicationEmployeeQuery, ListBackgroundCheckOrderQuery, ListEvaluationQuery,
    ListInterviewRecordQuery, ListJobProcessQuery, ListJobSchemaQuery,
    ListOfferApprovalTemplateQuery, ListQuestionnaireQuery, ListTalentTagQuery,
    ListTripartiteAgreementQuery, OpenJobReqBody,
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
async fn hire_application_offer_response_deserializes_and_sends_user_id_type() {
    let offer = r#"{"code":0,"msg":"ok","data":{"offer":{"id":"offer-1","application_id":"application-1","schema_id":"offer-schema-1","offer_status":2,"basic_info":{"offer_type":1,"employee_type":{"id":"employee-type-1","zh_name":"正式"},"contract_period":{"period_type":1,"period":3},"onboard_address":{"id":"address-1","city":{"en_name":"Shanghai","location_type":3}},"work_address":{"id":"address-2","district":{"zh_name":"徐汇","location_type":4}},"work_location_address_info":{"location_info":{"id":"location-1","en_name":"Shanghai Office"},"address_info":{"id":"location-2","zh_name":"徐汇区"}},"customize_info_list":[{"object_id":"field-1","customize_value":"value-1"}],"common_attachment_id_list":["attachment-1"]},"salary_plan":{"currency":"CNY","basic_salary":"10000","customize_info_list":[{"object_id":"salary-field-1","customize_value":"value-2"}]},"job_info":{"job_id":"job-1","job_name":"Engineer"},"customized_module_list":[{"ID":"module-1","object_list":[{"object_id":"module-field-1","customize_value":"value-3"}]}],"job_requirement_id":"requirement-1","offer_send_record_list":[{"offer_send_record_id":"send-1","operator_user_id":"ou_operator","offer_letter_status":1,"email_info":{"cc_email_list":["cc@example.com"],"receiver_email_list":["candidate@example.com"],"content":"Welcome"},"acceptance_list":[{"operator_type":1,"conclusion":2,"memo":"Accepted","operate_time":"1718959426734"}],"offer_file_list":[{"id":"file-1","file_template_name":"Offer Letter"}],"offer_signature_info":{"id":"signature-1","signature_status":1,"attachment_list":[{"id":"attachment-1","file_name":"Signed Offer.pdf"}]}}]}}}"#;
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
    let basic_info = offer.basic_info.as_ref().unwrap();
    assert_eq!(
        basic_info
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
    assert_eq!(
        basic_info
            .work_location_address_info
            .as_ref()
            .unwrap()
            .address_info
            .as_ref()
            .unwrap()
            .zh_name
            .as_deref(),
        Some("徐汇区")
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
        Some(2)
    );
    assert_eq!(
        send_record.offer_file_list.as_ref().unwrap()[0]
            .file_template_name
            .as_deref(),
        Some("Offer Letter")
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
async fn hire_job_detail_response_deserializes_and_preserves_request() {
    let body = r#"{"code":0,"msg":"ok","data":{"job_detail":{"basic_info":{"id":"job-1","title":"Engineer","recruitment_type":{"id":"employment-1","name":{"en_us":"Full time"},"active_status":1},"department":{"id":"dept-1","name":{"en_us":"Engineering"}},"highlight_list":[{"id":"highlight-1","name":{"en_us":"Remote"}}],"customized_data_list":[{"object_id":"field-1","name":{"en_us":"Work authorization"},"object_type":1,"value":{"content":"Required","option_list":[{"key":"eligible","name":{"en_us":"Eligible"}}],"time_range":{"start_time":"1710000000","end_time":"1710003600"}}}],"city_list":[{"code":"CN-SH","name":{"en_us":"Shanghai"}}],"target_major_list":[{"id":"major-1","name":{"en_us":"Computer Science"}}]},"recruiter":{"id":"ou_recruiter","name":{"en_us":"Recruiter"}},"assistant_list":[{"id":"ou_assistant","name":{"en_us":"Assistant"}}],"hiring_manager_list":[{"id":"ou_manager","name":{"en_us":"Manager"}}],"job_requirement_list":[{"id":"requirement-1","short_code":"REQ-1","name":"Platform","department_id":"dept-1"}],"address_list":[{"id":"address-1","name":{"en_us":"Shanghai"}}],"job_config":{"id":"job-1","internship_offer_apply_schema":{"id":"intern-schema-1"}},"storefront_list":[{"id":"store-1","name":{"en_us":"Store"},"department":{"id":"dept-1","name":{"en_us":"Retail"}},"manager":{"id":"ou_store_manager"},"remark":{"en_us":"Flagship"}}],"tag_list":[{"id":"tag-1","name":{"en_us":"Priority"},"order":1}],"stage_count_list":[{"count":3,"stage":{"id":"stage-1","type":2}}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let detail = client
        .hire()
        .job
        .get_detail("job-1", &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .job_detail
        .unwrap();

    let basic_info = detail.basic_info.as_ref().unwrap();
    assert_eq!(basic_info.title.as_deref(), Some("Engineer"));
    assert_eq!(
        basic_info.recruitment_type.as_ref().unwrap().active_status,
        Some(1)
    );
    assert_eq!(
        basic_info.customized_data_list.as_ref().unwrap()[0]
            .value
            .as_ref()
            .unwrap()
            .content
            .as_deref(),
        Some("Required")
    );
    assert_eq!(
        basic_info.city_list.as_ref().unwrap()[0].code.as_deref(),
        Some("CN-SH")
    );
    assert_eq!(
        detail.recruiter.as_ref().unwrap().id.as_deref(),
        Some("ou_recruiter")
    );
    assert_eq!(
        detail.job_requirement_list.as_ref().unwrap()[0]
            .short_code
            .as_deref(),
        Some("REQ-1")
    );
    assert_eq!(
        detail
            .job_config
            .as_ref()
            .unwrap()
            .internship_offer_apply_schema
            .as_ref()
            .unwrap()
            .id
            .as_deref(),
        Some("intern-schema-1")
    );
    assert_eq!(detail.tag_list.as_ref().unwrap()[0].order, Some(1));
    assert_eq!(detail.stage_count_list.as_ref().unwrap()[0].count, Some(3));
    assert_eq!(
        detail.stage_count_list.as_ref().unwrap()[0]
            .stage
            .as_ref()
            .unwrap()
            .type_,
        Some(2)
    );
    assert_eq!(
        detail.storefront_list.as_ref().unwrap()[0]
            .manager
            .as_ref()
            .unwrap()
            .id
            .as_deref(),
        Some("ou_store_manager")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/jobs/job-1/get_detail "));
}

#[tokio::test]
async fn hire_application_lifecycle_responses_deserialize_and_preserve_requests() {
    let detail = r#"{"code":0,"msg":"ok","data":{"application_detail":{"basic_info":{"id":"application-1","job_id":"job-1","talent_id":"talent-1","stage":{"id":"stage-1","zh_name":"Screening","en_name":"Screening","type":2},"active_status":1,"delivery_type":2,"resume_source_info":{"id":"source-1","name":{"en_us":"Referral"},"resume_source_type":3},"website_resume_source":{"website_id":"website-1","website_name":{"en_us":"Careers"},"channel":{"channel_id":"channel-1","channel_name":{"en_us":"Organic"}}},"stage_time_list":[{"stage_id":"stage-1","enter_time":"1710000000"}],"application_preferred_city_list":[{"code":"CN-SH","name":{"en_us":"Shanghai"}}],"termination_reason":{"id":"reason-1","name":{"en_us":"Candidate withdrew"},"children":[{"id":"reason-child-1","name":{"en_us":"Compensation"}}]}},"job":{"id":"job-1","name":"Engineer","recruitment_type":{"id":"employment-1","name":{"en_us":"Full time"},"active_status":1},"city_list":{"code":"CN-SH","name":{"en_us":"Shanghai"}}},"talent":{"id":"talent-1","name":"Taylor","mobile_code":"+86","mobile_number":"13800000000","email":"taylor@example.com"},"evaluations":[{"id":"evaluation-1","conclusion":1}],"interview_aggregation":{"interviews":[{"id":"interview-1","round":2,"interview_record_list":[{"id":"record-1","interviewer":{"id":"ou_interviewer","name":{"en_us":"Interviewer"}}}],"meeting_room_list":[{"room_id":"room-1","room_name":"Orchid"}]}]},"offer":{"offer_basic":{"id":"offer-1","offer_status":2,"leader":{"id":"ou_leader","name":{"en_us":"Leader"}},"contract_period":{"period_type":1,"period":3},"attachment_list":[{"id":"attachment-1","name":"Offer.pdf"}]},"offer_salary":{"id":"salary-1","total_annual_cash":"200000"}},"employee":{"id":"employee-1","onboard_status":1,"department_id":"dept-1"},"agency":{"basic_info":{"hunter_company_name":"Search Co"},"comment_info":[{"name":{"en_us":"Recommendation"},"value":{"en_us":"Strong"}}]},"portal":{"campus_volunteer_info":{"volunteer_seq":1}},"referral":{"basic_info":{"id":"referral-1","user_info":{"id":"ou_referrer","name":{"en_us":"Referrer"}}},"recommend_info":{"comment":"Excellent candidate"}}}}}"#;
    let empty = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, detail),
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
        http_response(200, empty),
    ])
    .await;

    let client = client_for(addr);
    let detail = client
        .hire()
        .application
        .get_detail("application-1", &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .application_detail
        .unwrap();
    let cancel_onboard = client
        .hire()
        .application
        .cancel_onboard(
            "application-1",
            json!({"termination_type": 1}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let recover = client
        .hire()
        .application
        .recover("application-1", &RequestOption::default())
        .await
        .unwrap();
    let close = client
        .hire()
        .job
        .close(
            "job-1",
            json!({"termination_type": 1}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let open = client
        .hire()
        .job
        .open(
            "job-1",
            OpenJobReqBody::default(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let basic_info = detail.basic_info.as_ref().unwrap();
    assert_eq!(basic_info.id.as_deref(), Some("application-1"));
    assert_eq!(basic_info.stage.as_ref().unwrap().type_, Some(2));
    assert_eq!(
        basic_info
            .website_resume_source
            .as_ref()
            .unwrap()
            .channel
            .as_ref()
            .unwrap()
            .channel_id
            .as_deref(),
        Some("channel-1")
    );
    assert_eq!(
        basic_info
            .termination_reason
            .as_ref()
            .unwrap()
            .children
            .as_ref()
            .unwrap()[0]
            .id
            .as_deref(),
        Some("reason-child-1")
    );
    assert_eq!(
        detail.job.as_ref().unwrap().name.as_deref(),
        Some("Engineer")
    );
    assert_eq!(
        detail.talent.as_ref().unwrap().email.as_deref(),
        Some("taylor@example.com")
    );
    assert_eq!(detail.evaluations.as_ref().unwrap()[0].conclusion, Some(1));
    assert_eq!(
        detail
            .interview_aggregation
            .as_ref()
            .unwrap()
            .interviews
            .as_ref()
            .unwrap()[0]
            .meeting_room_list
            .as_ref()
            .unwrap()[0]
            .room_name
            .as_deref(),
        Some("Orchid")
    );
    assert_eq!(
        detail
            .offer
            .as_ref()
            .unwrap()
            .offer_basic
            .as_ref()
            .unwrap()
            .attachment_list
            .as_ref()
            .unwrap()[0]
            .name
            .as_deref(),
        Some("Offer.pdf")
    );
    assert_eq!(
        detail.employee.as_ref().unwrap().department_id.as_deref(),
        Some("dept-1")
    );
    assert_eq!(
        detail
            .agency
            .as_ref()
            .unwrap()
            .basic_info
            .as_ref()
            .unwrap()
            .hunter_company_name
            .as_deref(),
        Some("Search Co")
    );
    assert_eq!(
        detail
            .portal
            .as_ref()
            .unwrap()
            .campus_volunteer_info
            .as_ref()
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
    assert!(cancel_onboard.success());
    assert!(recover.success());
    assert!(close.success());
    assert!(open.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/application-1/get_detail "));
    assert!(request.contains("POST /open-apis/hire/v1/applications/application-1/cancel_onboard "));
    assert!(request.contains("POST /open-apis/hire/v1/applications/application-1/recover "));
    assert!(request.contains("POST /open-apis/hire/v1/jobs/job-1/close "));
    assert!(request.contains("POST /open-apis/hire/v1/jobs/job-1/open "));
    assert!(request.contains("termination_type"));
}

#[tokio::test]
async fn hire_application_detail_leaf_models_deserialize() {
    let ability = json!({"id": "ability-1", "name": {"en_us": "Communication"}});
    let question = json!({
        "question_type": 1,
        "title": {"en_us": "Design"},
        "content": "Detailed answer",
        "abilities": [ability]
    });
    let dimension = json!({
        "interview_feedback_form_dimension_id": "dimension-1",
        "dimension_name": {"en_us": "Coding"},
        "dimension_type": 2,
        "weight": 0.5,
        "dimension_option": {"id": "option-1", "name": {"en_us": "Strong"}, "score_val": 5},
        "dimension_options": [{"id": "option-2", "alias_name": {"en_us": "Excellent"}}],
        "dimension_score": 5,
        "recommended_job_level": {
            "lower_limit_job_level_name": {"en_us": "L4"},
            "higher_limit_job_level_name": {"en_us": "L5"}
        },
        "question_assessments": [question],
        "ability_assessments": [{"id": "assessment-1", "ability_id": "ability-1", "content": "Strong"}]
    });
    let module = json!({
        "interview_feedback_form_module_id": "module-1",
        "module_name": {"en_us": "Technical"},
        "module_type": 1,
        "module_weight": 0.5,
        "module_score": 4.5,
        "dimension_assessments": [dimension]
    });
    let record = json!({
        "id": "record-1",
        "record_score": {"score": 8.5, "total_score": 10.0},
        "attachments": [{
            "file_id": "file-1",
            "file_name": "feedback.pdf",
            "content_type": "application/pdf",
            "create_time": "1710399930151"
        }],
        "module_assessments": [module]
    });
    let body = json!({
        "code": 0,
        "msg": "ok",
        "data": {
            "application_detail": {
                "interview_aggregation": {"interviews": [{"interview_record_list": [record]}]},
                "referral": {
                    "recommend_info": {
                        "specific_relationship": {"relation_with_candidate": 1, "extra": "Former colleague"}
                    }
                }
            }
        }
    })
    .to_string();
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, &body)]).await;

    let detail = client_for(addr)
        .hire()
        .application
        .get_detail("application-1", &RequestOption::default())
        .await
        .unwrap()
        .data
        .unwrap()
        .application_detail
        .unwrap();

    let record = &detail
        .interview_aggregation
        .as_ref()
        .unwrap()
        .interviews
        .as_ref()
        .unwrap()[0]
        .interview_record_list
        .as_ref()
        .unwrap()[0];
    assert_eq!(record.record_score.as_ref().unwrap().score, Some(8.5));
    assert_eq!(
        record.attachments.as_ref().unwrap()[0]
            .content_type
            .as_deref(),
        Some("application/pdf")
    );
    let dimension = &record.module_assessments.as_ref().unwrap()[0]
        .dimension_assessments
        .as_ref()
        .unwrap()[0];
    assert_eq!(
        dimension.dimension_option.as_ref().unwrap().score_val,
        Some(5)
    );
    assert_eq!(
        dimension
            .recommended_job_level
            .as_ref()
            .unwrap()
            .higher_limit_job_level_name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("L5")
    );
    assert_eq!(
        dimension.question_assessments.as_ref().unwrap()[0]
            .abilities
            .as_ref()
            .unwrap()[0]
            .name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Communication")
    );
    assert_eq!(
        dimension.ability_assessments.as_ref().unwrap()[0]
            .content
            .as_deref(),
        Some("Strong")
    );
    assert_eq!(
        detail
            .referral
            .as_ref()
            .unwrap()
            .recommend_info
            .as_ref()
            .unwrap()
            .specific_relationship
            .as_ref()
            .unwrap()
            .extra
            .as_deref(),
        Some("Former colleague")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/applications/application-1/get_detail "));
}

#[tokio::test]
async fn hire_intern_offer_status_response_deserializes_and_preserves_request() {
    let body = r#"{"code":0,"msg":"ok","data":{"offer_id":"offer-1","operation":"offboard","onboarding_info":{"actual_onboarding_date":"2022-01-01"},"offboarding_info":{"actual_offboarding_date":"2022-03-02","notes":"Completed internship"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let data = client
        .hire()
        .offer
        .intern_offer_status(
            "offer-1",
            json!({"operation": "offboard"}),
            &RequestOption::default(),
        )
        .await
        .unwrap()
        .data
        .unwrap();

    assert_eq!(data.offer_id.as_deref(), Some("offer-1"));
    assert_eq!(data.operation.as_deref(), Some("offboard"));
    assert_eq!(
        data.offboarding_info
            .unwrap()
            .actual_offboarding_date
            .as_deref(),
        Some("2022-03-02")
    );

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/offers/offer-1/intern_offer_status "));
    assert!(request.contains("offboard"));
}

#[tokio::test]
async fn hire_config_read_models_deserialize_and_send_filters() {
    let job_process = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"process-1","zh_name":"流程","type":1,"stage_list":[{"id":"stage-1","type":4}]}],"has_more":false,"page_token":"p2"}}"#;
    let job_schema = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"schema-1","scenario_type":2,"name":{"en_us":"Job schema"},"object_list":[{"id":"module-1","name":{"zh_cn":"模块"},"setting":{"object_type":1,"config":{"options":[{"key":"option-1","name":{"en_us":"Option"},"active_status":1}]}},"children_list":[{"id":"field-1","name":{"zh_cn":"字段"},"parent_id":"module-1","setting":{"object_type":2,"config":{"options":[{"key":"child-option"}]}}}]}]}],"has_more":false}}"#;
    let offer_form = r#"{"code":0,"msg":"ok","data":{"offer_apply_form":{"id":"form-1","name":{"en_us":"Offer Form"},"schema":{"id":"schema-v1","module_list":[{"id":"module-1","object_list":[{"id":"object-1","object_type_v2":"text","config":{"options":[{"id":"option-1","name":{"en_us":"Full time"}}],"formula":{"value":"[object-1] * 12","result":1,"extra_map":[{"key":"object-1","value":{"en_us":"Base salary"}}]},"object_display_config":{"display_condition":1,"pre_object_config_list":[{"id":"object-0","operator":1,"value":["yes"]}]}}}]}]}}}}"#;
    let offer_template = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"template-1","name":{"zh_cn":"审批"},"department_list":[{"id":"dept-1","name":"研发"}]}],"has_more":false}}"#;
    let questionnaire = r#"{"code":0,"msg":"ok","data":{"items":[{"questionnaire_id":"q-1","application_id":"app-1","version":3,"questions":[{"question_id":"question-1","question_name":"Overall experience","question_en_name":"Overall experience","question_type":4,"is_required":true,"select_option_result_list":[{"option_id":"option-1","option_name":"Good","is_selected":true}],"five_start_scoring_result":{"highest_score_desc":"Great","lowest_score_desc":"Poor","score_result":4.5},"description_result":"Helpful"}],"has_answers":true}],"has_more":false}}"#;
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
    let schema = schema_resp.data.unwrap().items.remove(0);
    assert_eq!(
        schema.object_list.as_ref().unwrap()[0].id.as_deref(),
        Some("module-1")
    );
    assert_eq!(
        schema.object_list.as_ref().unwrap()[0]
            .setting
            .as_ref()
            .unwrap()
            .config
            .as_ref()
            .unwrap()
            .options
            .as_ref()
            .unwrap()[0]
            .key
            .as_deref(),
        Some("option-1")
    );
    assert_eq!(
        schema.object_list.as_ref().unwrap()[0]
            .children_list
            .as_ref()
            .unwrap()[0]
            .parent_id
            .as_deref(),
        Some("module-1")
    );
    let offer_field = form_resp
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
        .clone();
    assert_eq!(offer_field.object_type_v2.as_deref(), Some("text"));
    assert_eq!(
        offer_field
            .config
            .as_ref()
            .unwrap()
            .object_display_config
            .as_ref()
            .unwrap()
            .pre_object_config_list
            .as_ref()
            .unwrap()[0]
            .value
            .as_ref()
            .unwrap()[0]
            .as_str(),
        "yes"
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
    let questionnaire = questionnaire_resp.data.unwrap().items.remove(0);
    assert_eq!(questionnaire.has_answers, Some(true));
    let question = &questionnaire.questions.as_ref().unwrap()[0];
    assert_eq!(question.question_id.as_deref(), Some("question-1"));
    assert_eq!(
        question.select_option_result_list.as_ref().unwrap()[0].is_selected,
        Some(true)
    );
    assert_eq!(
        question
            .five_start_scoring_result
            .as_ref()
            .unwrap()
            .score_result,
        Some(4.5)
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
            .and_then(|value| value.score),
        Some(5.0)
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
    let background_list = r#"{"code":0,"msg":"ok","data":{"items":[{"order_id":"order-1","application_id":"app-1","package":"standard","feedback_info_list":[{"id":"report-1","attachment_url":"https://example.com/report.pdf","report_preview_url":"https://example.com/preview","result":"green","report_type":1,"create_time":"1710000000","report_name":"Background report"}],"process_info_list":[{"process":"arranged","update_time":"1710000001","en_process":"arranged"}],"candidate_info":{"name":"Ada","mobile":"13800000000","email":"ada@example.com","first_name":"Ada","last_name":"Lovelace"},"creator_info":{"user_id":"ou_creator"},"contactor_info":{"name":"Grace","email":"grace@example.com"},"provider_info":{"provider_id":"provider-1","provider_name":{"en_us":"Provider"}},"custom_field_list":[{"type":"select","key":"candidate_resume","name":{"en_us":"Resume"},"is_required":true,"description":{"en_us":"Attach a resume"},"options":[{"key":"A","name":{"en_us":"Option A"}}]}],"custom_data_list":[{"key":"candidate_resume","value":"A"}],"ext_item_info_list":[{"id":"item-1","name":"Identity check"}]}],"has_more":false}}"#;
    let background_batch = r#"{"code":0,"msg":"ok","data":{"items":[{"order_id":"order-2","order_status":2,"provider_info":{"provider_id":"provider-1"}}],"has_more":false}}"#;
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
        background.candidate_info.as_ref().unwrap().name.as_deref(),
        Some("Ada")
    );
    assert_eq!(
        background.feedback_info_list.as_ref().unwrap()[0]
            .report_name
            .as_deref(),
        Some("Background report")
    );
    assert_eq!(
        background.process_info_list.as_ref().unwrap()[0]
            .en_process
            .as_deref(),
        Some("arranged")
    );
    assert_eq!(
        background
            .provider_info
            .as_ref()
            .unwrap()
            .provider_name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Provider")
    );
    assert_eq!(
        background.custom_field_list.as_ref().unwrap()[0]
            .options
            .as_ref()
            .unwrap()[0]
            .key
            .as_deref(),
        Some("A")
    );
    assert_eq!(
        background.custom_data_list.as_ref().unwrap()[0]
            .value
            .as_deref(),
        Some("A")
    );
    assert_eq!(
        background.ext_item_info_list.as_ref().unwrap()[0]
            .name
            .as_deref(),
        Some("Identity check")
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
