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

#[tokio::test]
async fn hire_talent_folder_and_combined_write_responses() {
    let add_to_folder = r#"{"code":0,"msg":"ok","data":{"talent_id_list":["talent-1","talent-2"],"folder_id":"folder-1"}}"#;
    let combined_create = r#"{"code":0,"msg":"ok","data":{"talent_id":"talent-1","creator_id":"ou_creator","creator_account_type":1}}"#;
    let combined_update = r#"{"code":0,"msg":"ok","data":{"talent_id":"talent-1","operator_id":"ou_operator","operator_account_type":2}}"#;
    let remove_to_folder =
        r#"{"code":0,"msg":"ok","data":{"talent_id_list":["talent-1"],"folder_id":"folder-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, add_to_folder),
        http_response(200, combined_create),
        http_response(200, combined_update),
        http_response(200, remove_to_folder),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let add_to_folder = Box::pin(hire.talent.add_to_folder(
        json!({"talent_id_list":["talent-1","talent-2"]}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();
    let combined_create = Box::pin(hire.talent.combined_create(
        json!({"basic_info":{"name":"Candidate"}}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();
    let combined_update = Box::pin(hire.talent.combined_update(
        json!({"talent_id":"talent-1","operator_id":"ou_operator"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();
    let remove_to_folder = Box::pin(hire.talent.remove_to_folder(
        json!({"talent_id_list":["talent-1"]}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();

    assert_eq!(
        add_to_folder.talent_id_list,
        vec!["talent-1".to_string(), "talent-2".to_string()]
    );
    assert_eq!(add_to_folder.folder_id.as_deref(), Some("folder-1"));
    assert_eq!(combined_create.talent_id.as_deref(), Some("talent-1"));
    assert_eq!(combined_create.creator_id.as_deref(), Some("ou_creator"));
    assert_eq!(combined_create.creator_account_type, Some(1));
    assert_eq!(combined_update.talent_id.as_deref(), Some("talent-1"));
    assert_eq!(combined_update.operator_id.as_deref(), Some("ou_operator"));
    assert_eq!(combined_update.operator_account_type, Some(2));
    assert_eq!(
        remove_to_folder.talent_id_list,
        vec!["talent-1".to_string()]
    );
    assert_eq!(remove_to_folder.folder_id.as_deref(), Some("folder-1"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/talents/add_to_folder "));
    assert!(request.contains("POST /open-apis/hire/v1/talents/combined_create "));
    assert!(request.contains("POST /open-apis/hire/v1/talents/combined_update "));
    assert!(request.contains("POST /open-apis/hire/v1/talents/remove_to_folder "));
    assert!(request.contains(r#""basic_info":{"name":"Candidate"}"#));
    assert!(request.contains(r#""operator_id":"ou_operator""#));
}

#[tokio::test]
async fn hire_talent_pool_and_exam_write_responses() {
    let moved_talent =
        r#"{"code":0,"msg":"ok","data":{"talent_pool_id":"pool-2","talent_id":"talent-1"}}"#;
    let exam = r#"{"code":0,"msg":"ok","data":{"exam_id":"exam-1","application_id":"application-1","exam_resource_name":"Rust Quiz","score":92.5,"uuid":"file-1","operator_id":"ou_operator","operate_time":"1710000000000"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, moved_talent),
        http_response(200, exam),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    let moved_talent = Box::pin(hire.talent_pool.move_talent(
        "pool-1",
        json!({"talent_id":"talent-1","target_talent_pool_id":"pool-2"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();
    let exam = Box::pin(hire.exam.create(
        json!({"application_id":"application-1","exam_resource_name":"Rust Quiz"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap()
    .data
    .unwrap();

    assert_eq!(moved_talent.talent_pool_id.as_deref(), Some("pool-2"));
    assert_eq!(moved_talent.talent_id.as_deref(), Some("talent-1"));
    assert_eq!(exam.exam_id.as_deref(), Some("exam-1"));
    assert_eq!(exam.application_id.as_deref(), Some("application-1"));
    assert_eq!(exam.exam_resource_name.as_deref(), Some("Rust Quiz"));
    assert_eq!(exam.score, Some(92.5));
    assert_eq!(exam.uuid.as_deref(), Some("file-1"));
    assert_eq!(exam.operator_id.as_deref(), Some("ou_operator"));
    assert_eq!(exam.operate_time.as_deref(), Some("1710000000000"));

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/talent_pools/pool-1/talent_relationship "));
    assert!(request.contains("POST /open-apis/hire/v1/exams "));
    assert!(request.contains(r#""target_talent_pool_id":"pool-2""#));
    assert!(request.contains(r#""exam_resource_name":"Rust Quiz""#));
}
