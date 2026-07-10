mod common;

use common::{http_response, mock_server_with_requests};

use larksuite_oapi_sdk_rs::Client;
use larksuite_oapi_sdk_rs::req::RequestOption;
use serde_json::json;

fn client_for(addr: std::net::SocketAddr) -> Client {
    Client::builder("test_app_id", "test_secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

fn empty_responses(count: usize) -> Vec<common::MockResponse> {
    (0..count)
        .map(|_| http_response(200, r#"{"code":0,"msg":"ok"}"#))
        .collect()
}

#[tokio::test]
async fn hire_agency_and_talent_code_only_sweep_preserves_requests() {
    let (addr, _handle, requests) = mock_server_with_requests(empty_responses(3)).await;
    let client = client_for(addr);

    let operate = client
        .hire()
        .agency
        .operate_agency_account(
            json!({"account_id": "account-1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let block = client
        .hire()
        .talent_blocklist
        .change(json!({"talent_id": "talent-1"}), &RequestOption::default())
        .await
        .unwrap();
    let pool = client
        .hire()
        .talent_pool
        .batch_change_talent_pool(
            "pool-1",
            json!({"talent_id_list": ["talent-1"]}),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(operate.success());
    assert!(block.success());
    assert!(pool.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/agencies/operate_agency_account "));
    assert!(request.contains("POST /open-apis/hire/v1/talent_blocklist/change_talent_block "));
    assert!(
        request.contains("POST /open-apis/hire/v1/talent_pools/pool-1/batch_change_talent_pool ")
    );
}

#[tokio::test]
async fn hire_eco_account_custom_field_code_only_sweep_preserves_requests() {
    let (addr, _handle, requests) = mock_server_with_requests(empty_responses(3)).await;
    let client = client_for(addr);

    let create = client
        .hire()
        .eco_account_custom_field
        .create(json!({"name": "Account field"}), &RequestOption::default())
        .await
        .unwrap();
    let delete = client
        .hire()
        .eco_account_custom_field
        .batch_delete(json!({"id_list": ["field-1"]}), &RequestOption::default())
        .await
        .unwrap();
    let update = client
        .hire()
        .eco_account_custom_field
        .batch_update(json!({"field_list": []}), &RequestOption::default())
        .await
        .unwrap();

    assert!(create.success());
    assert!(delete.success());
    assert!(update.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_account_custom_fields "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_account_custom_fields/batch_delete "));
    assert!(request.contains("PATCH /open-apis/hire/v1/eco_account_custom_fields/batch_update "));
}

#[tokio::test]
async fn hire_eco_background_check_code_only_sweep_preserves_requests() {
    let (addr, _handle, requests) = mock_server_with_requests(empty_responses(3)).await;
    let client = client_for(addr);

    let cancel = client
        .hire()
        .eco_background_check
        .cancel(json!({"order_id": "order-1"}), &RequestOption::default())
        .await
        .unwrap();
    let progress = client
        .hire()
        .eco_background_check
        .update_progress(json!({"order_id": "order-1"}), &RequestOption::default())
        .await
        .unwrap();
    let result = client
        .hire()
        .eco_background_check
        .update_result(json!({"order_id": "order-1"}), &RequestOption::default())
        .await
        .unwrap();

    assert!(cancel.success());
    assert!(progress.success());
    assert!(result.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_background_checks/cancel "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_background_checks/update_progress "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_background_checks/update_result "));
}

#[tokio::test]
async fn hire_eco_background_check_custom_field_code_only_sweep_preserves_requests() {
    let (addr, _handle, requests) = mock_server_with_requests(empty_responses(3)).await;
    let client = client_for(addr);

    let create = client
        .hire()
        .eco_background_check_custom_field
        .create(
            json!({"name": "Background field"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete = client
        .hire()
        .eco_background_check_custom_field
        .batch_delete(json!({"id_list": ["field-1"]}), &RequestOption::default())
        .await
        .unwrap();
    let update = client
        .hire()
        .eco_background_check_custom_field
        .batch_update(json!({"field_list": []}), &RequestOption::default())
        .await
        .unwrap();

    assert!(create.success());
    assert!(delete.success());
    assert!(update.success());

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
async fn hire_eco_background_check_package_code_only_sweep_preserves_requests() {
    let (addr, _handle, requests) = mock_server_with_requests(empty_responses(3)).await;
    let client = client_for(addr);

    let create = client
        .hire()
        .eco_background_check_package
        .create(
            json!({"name": "Background package"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete = client
        .hire()
        .eco_background_check_package
        .batch_delete(json!({"id_list": ["package-1"]}), &RequestOption::default())
        .await
        .unwrap();
    let update = client
        .hire()
        .eco_background_check_package
        .batch_update(json!({"package_list": []}), &RequestOption::default())
        .await
        .unwrap();

    assert!(create.success());
    assert!(delete.success());
    assert!(update.success());

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
async fn hire_eco_exam_code_only_sweep_preserves_requests() {
    let (addr, _handle, requests) = mock_server_with_requests(empty_responses(2)).await;
    let client = client_for(addr);

    let login = client
        .hire()
        .eco_exam
        .login_info(
            "exam-1",
            json!({"user_id": "ou_1"}),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let result = client
        .hire()
        .eco_exam
        .update_result("exam-1", json!({"score": 90}), &RequestOption::default())
        .await
        .unwrap();

    assert!(login.success());
    assert!(result.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_exams/exam-1/login_info "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_exams/exam-1/update_result "));
}

#[tokio::test]
async fn hire_eco_exam_paper_code_only_sweep_preserves_requests() {
    let (addr, _handle, requests) = mock_server_with_requests(empty_responses(3)).await;
    let client = client_for(addr);

    let create = client
        .hire()
        .eco_exam_paper
        .create(json!({"name": "Exam paper"}), &RequestOption::default())
        .await
        .unwrap();
    let delete = client
        .hire()
        .eco_exam_paper
        .batch_delete(json!({"id_list": ["paper-1"]}), &RequestOption::default())
        .await
        .unwrap();
    let update = client
        .hire()
        .eco_exam_paper
        .batch_update(json!({"paper_list": []}), &RequestOption::default())
        .await
        .unwrap();

    assert!(create.success());
    assert!(delete.success());
    assert!(update.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/eco_exam_papers "));
    assert!(request.contains("POST /open-apis/hire/v1/eco_exam_papers/batch_delete "));
    assert!(request.contains("PATCH /open-apis/hire/v1/eco_exam_papers/batch_update "));
}
