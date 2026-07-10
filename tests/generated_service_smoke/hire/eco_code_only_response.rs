use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_eco_account_custom_field_code_only_response_smoke() {
    let responses = (0..3)
        .map(|_| http_response(200, r#"{"code":0,"msg":"ok"}"#))
        .collect();
    let (addr, _handle, requests) = mock_server_with_requests(responses).await;
    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(
        hire.eco_account_custom_field
            .create(json!({"name": "Account field"}), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(
        hire.eco_account_custom_field
            .batch_delete(json!({"id_list": ["field-1"]}), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(
        hire.eco_account_custom_field
            .batch_update(json!({"field_list": []}), &RequestOption::default()),
    )
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_account_custom_fields "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_account_custom_fields/batch_delete "));
    assert!(request.contains("PATCH /open-apis/hire/v1/eco_account_custom_fields/batch_update "));
}

#[tokio::test]
async fn hire_eco_background_check_code_only_response_smoke() {
    let responses = (0..3)
        .map(|_| http_response(200, r#"{"code":0,"msg":"ok"}"#))
        .collect();
    let (addr, _handle, requests) = mock_server_with_requests(responses).await;
    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(
        hire.eco_background_check
            .cancel(json!({"order_id": "order-1"}), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(
        hire.eco_background_check
            .update_progress(json!({"order_id": "order-1"}), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(
        hire.eco_background_check
            .update_result(json!({"order_id": "order-1"}), &RequestOption::default()),
    )
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_background_checks/cancel "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_background_checks/update_progress "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_background_checks/update_result "));
}

#[tokio::test]
async fn hire_eco_background_check_custom_field_code_only_response_smoke() {
    let responses = (0..3)
        .map(|_| http_response(200, r#"{"code":0,"msg":"ok"}"#))
        .collect();
    let (addr, _handle, requests) = mock_server_with_requests(responses).await;
    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.eco_background_check_custom_field.create(
        json!({"name": "Background field"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(
        hire.eco_background_check_custom_field
            .batch_delete(json!({"id_list": ["field-1"]}), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(
        hire.eco_background_check_custom_field
            .batch_update(json!({"field_list": []}), &RequestOption::default()),
    )
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_background_check_custom_fields "));
    assert!(
        request
            .contains("POST /open-apis/hire/v1/eco_background_check_custom_fields/batch_delete ")
    );
    assert!(
        request
            .contains("PATCH /open-apis/hire/v1/eco_background_check_custom_fields/batch_update ")
    );
}

#[tokio::test]
async fn hire_eco_background_check_package_code_only_response_smoke() {
    let responses = (0..3)
        .map(|_| http_response(200, r#"{"code":0,"msg":"ok"}"#))
        .collect();
    let (addr, _handle, requests) = mock_server_with_requests(responses).await;
    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.eco_background_check_package.create(
        json!({"name": "Background package"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(
        hire.eco_background_check_package
            .batch_delete(json!({"id_list": ["package-1"]}), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(
        hire.eco_background_check_package
            .batch_update(json!({"package_list": []}), &RequestOption::default()),
    )
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_background_check_packages "));
    assert!(
        request.contains("POST /open-apis/hire/v1/eco_background_check_packages/batch_delete ")
    );
    assert!(
        request.contains("PATCH /open-apis/hire/v1/eco_background_check_packages/batch_update ")
    );
}

#[tokio::test]
async fn hire_eco_exam_code_only_response_smoke() {
    let responses = (0..2)
        .map(|_| http_response(200, r#"{"code":0,"msg":"ok"}"#))
        .collect();
    let (addr, _handle, requests) = mock_server_with_requests(responses).await;
    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.eco_exam.login_info(
        "exam-1",
        json!({"user_id": "ou_1"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.eco_exam.update_result(
        "exam-1",
        json!({"score": 90}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_exams/exam-1/login_info "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_exams/exam-1/update_result "));
}

#[tokio::test]
async fn hire_eco_exam_paper_code_only_response_smoke() {
    let responses = (0..3)
        .map(|_| http_response(200, r#"{"code":0,"msg":"ok"}"#))
        .collect();
    let (addr, _handle, requests) = mock_server_with_requests(responses).await;
    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(
        hire.eco_exam_paper
            .create(json!({"name": "Exam paper"}), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(
        hire.eco_exam_paper
            .batch_delete(json!({"id_list": ["paper-1"]}), &RequestOption::default()),
    )
    .await
    .unwrap();
    Box::pin(
        hire.eco_exam_paper
            .batch_update(json!({"paper_list": []}), &RequestOption::default()),
    )
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_exam_papers "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_exam_papers/batch_delete "));
    assert!(request.contains("PATCH /open-apis/hire/v1/eco_exam_papers/batch_update "));
}
