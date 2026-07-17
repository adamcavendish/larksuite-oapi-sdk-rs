use super::prelude::*;
use serde_json::json;

#[tokio::test]
async fn hire_talent_write_smoke() {
    let add_to_folder =
        r#"{"code":0,"msg":"ok","data":{"talent_id_list":["talent-1"],"folder_id":"folder-1"}}"#;
    let combined_create = r#"{"code":0,"msg":"ok","data":{"talent_id":"talent-1"}}"#;
    let combined_update = r#"{"code":0,"msg":"ok","data":{"talent_id":"talent-1"}}"#;
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

    Box::pin(hire.talent.add_to_folder(
        AddToFolderTalentReqBody {
            talent_id_list: Some(vec!["talent-1".into()]),
            ..Default::default()
        },
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.talent.combined_create(
        json!({"basic_info":{"name":"Candidate"}}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.talent.combined_update(
        json!({"talent_id":"talent-1","operator_id":"ou_operator"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.talent.remove_to_folder(
        RemoveToFolderTalentReqBody {
            talent_id_list: Some(vec!["talent-1".into()]),
            ..Default::default()
        },
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/talents/add_to_folder "));
    assert!(request.contains("POST /open-apis/hire/v1/talents/combined_create "));
    assert!(request.contains("POST /open-apis/hire/v1/talents/combined_update "));
    assert!(request.contains("POST /open-apis/hire/v1/talents/remove_to_folder "));
}

#[tokio::test]
async fn hire_talent_pool_and_exam_write_smoke() {
    let moved_talent =
        r#"{"code":0,"msg":"ok","data":{"talent_pool_id":"pool-2","talent_id":"talent-1"}}"#;
    let exam = r#"{"code":0,"msg":"ok","data":{"exam_id":"exam-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, moved_talent),
        http_response(200, exam),
    ])
    .await;

    let client = client_for(addr);
    let hire = client.hire();

    Box::pin(hire.talent_pool.move_talent(
        "pool-1",
        json!({"talent_id":"talent-1","target_talent_pool_id":"pool-2"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();
    Box::pin(hire.exam.create(
        json!({"application_id":"application-1","exam_resource_name":"Rust Quiz"}),
        &RequestOption::default(),
    ))
    .await
    .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/hire/v1/talent_pools/pool-1/talent_relationship "));
    assert!(request.contains("POST /open-apis/hire/v1/exams "));
}
