use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_config_read_model_query_smoke() {
    let paged_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"item-1"}],"page_token":"next-page","has_more":false}}"#;
    let offer_form_get_body =
        r#"{"code":0,"msg":"ok","data":{"offer_apply_form":{"id":"form-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, paged_body),
        http_response(200, paged_body),
        http_response(200, paged_body),
        http_response(200, offer_form_get_body),
        http_response(200, paged_body),
        http_response(200, paged_body),
        http_response(200, paged_body),
        http_response(200, paged_body),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    Box::pin(hire.job_process.list_by_query(
        &ListHireJobProcessQuery::new().page(page),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.job_requirement_schema.list_by_query(
        &ListHireJobRequirementSchemaQuery::new().page(page),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.job_schema.list_by_query(
        &ListHireJobSchemaQuery::new().page(page).scenario(1),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(
        hire.offer_application_form
            .get("form-1", &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(hire.offer_application_form.list_by_query(
        &ListHireOfferApplicationFormQuery::new().page(page),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(
        hire.offer_approval_template.list_by_query(
            &ListHireOfferApprovalTemplateQuery::new()
                .page(page)
                .department_id_type("open_department_id"),
            &RequestOption::default(),
        ),
    )
    .await
    .unwrap();
    Box::pin(
        hire.questionnaire.list_by_query(
            &ListHireQuestionnaireQuery::new()
                .page(page)
                .application_id("app-1"),
            &RequestOption::default(),
        ),
    )
    .await
    .unwrap();
    Box::pin(hire.talent_tag.list_by_query(
        &ListHireTalentTagQuery::new().page(page),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/job_processes?"));
    assert!(request.contains("GET /open-apis/hire/v1/job_requirement_schemas?"));
    assert!(request.contains("GET /open-apis/hire/v1/job_schemas?"));
    assert!(request.contains("GET /open-apis/hire/v1/offer_application_forms/form-1 "));
    assert!(request.contains("GET /open-apis/hire/v1/offer_approval_templates?"));
    assert!(request.contains("GET /open-apis/hire/v1/questionnaires?"));
    assert!(request.contains("GET /open-apis/hire/v1/talent_tags?"));
    assert!(request.contains("scenario=1"));
    assert!(request.contains("department_id_type=open_department_id"));
}

#[tokio::test]
async fn hire_activity_read_model_query_smoke() {
    let paged_body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"item-1"}],"page_token":"next-page","has_more":false}}"#;
    let employee_body = r#"{"code":0,"msg":"ok","data":{"employee":{"id":"emp-1"}}}"#;
    let note_get_body = r#"{"code":0,"msg":"ok","data":{"note":{"id":"note-1"}}}"#;
    let record_get_body = r#"{"code":0,"msg":"ok","data":{"interview_record":{"id":"record-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, employee_body),
        http_response(200, paged_body),
        http_response(200, note_get_body),
        http_response(200, paged_body),
        http_response(200, record_get_body),
        http_response(200, paged_body),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();
    let page = PageQuery::new().page_size(20).page_token("seed-token");

    hire.employee
        .get_by_application_query(
            &GetHireByApplicationEmployeeQuery::new()
                .application_id("app-1")
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.evaluation
        .list_by_query(
            &ListHireEvaluationQuery::new()
                .page(page)
                .application_id("app-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.note
        .get("note-1", &RequestOption::default())
        .await
        .unwrap();
    hire.note
        .list_by_query(
            &ListHireNoteQuery::new().page(page).talent_id("talent-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.interview_record
        .get("record-1", &RequestOption::default())
        .await
        .unwrap();
    hire.interview_record
        .list_by_query(
            &ListHireInterviewRecordQuery::new()
                .page(page)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/employees/get_by_application?"));
    assert!(request.contains("GET /open-apis/hire/v1/evaluations?"));
    assert!(request.contains("GET /open-apis/hire/v1/notes/note-1 "));
    assert!(request.contains("GET /open-apis/hire/v1/notes?"));
    assert!(request.contains("GET /open-apis/hire/v1/interview_records?"));
    assert!(request.contains("application_id=app-1"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn hire_agreement_read_model_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"agreement-1","state":1}],"page_token":"next-page","has_more":false}}"#;
    let background_body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"order_id":"order-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, background_body),
        http_response(200, background_body),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    hire.tripartite_agreement
        .list_by_query(
            &ListHireTripartiteAgreementQuery::new()
                .page_size(20)
                .page_token("seed-token")
                .application_id("app-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.background_check_order
        .list_by_query(
            &ListHireBackgroundCheckOrderQuery::new()
                .page_size(20)
                .application_id("app-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    hire.background_check_order
        .batch_query(json!({"application_id":"app-1"}), &RequestOption::default())
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/hire/v1/tripartite_agreements?"));
    assert!(request.contains("page_token=seed-token"));
    assert!(request.contains("GET /open-apis/hire/v1/background_check_orders?"));
    assert!(request.contains("POST /open-apis/hire/v1/background_check_orders/batch_query "));
    assert!(request.contains(r#""application_id":"app-1""#));
}
