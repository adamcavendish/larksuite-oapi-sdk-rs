mod common;

use common::{http_response, mock_server_with_requests};
use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::okr::v2::{
    CreateObjectiveAlignmentReqBody, CreateObjectiveKeyResultReqBody, CreateObjectiveReqBody,
    KeyResultWeight, KeyResultsPositionReqBody, KeyResultsWeightReqBody, ObjectiveWeight,
    ObjectivesPositionReqBody, ObjectivesWeightReqBody, OkrPageQuery, PatchIndicatorReqBody,
    PatchKeyResultReqBody, PatchObjectiveReqBody,
};

fn client_for(addr: std::net::SocketAddr) -> LarkClient {
    LarkClient::builder("app_id", "secret")
        .base_url(format!("http://{addr}"))
        .disable_token_cache()
        .build()
        .unwrap()
}

fn tenant_option() -> RequestOption {
    RequestOption {
        tenant_access_token: Some("tenant-token".into()),
        ..Default::default()
    }
}

#[tokio::test]
async fn okr_v2_exposes_every_go_resource_route() {
    let ok = http_response(200, r#"{"code":0,"msg":"ok","data":{}}"#);
    let (addr, _handle, requests) = mock_server_with_requests(vec![ok; 25]).await;
    let client = client_for(addr);
    let okr = client.okr_v2();
    let option = tenant_option();
    let page = OkrPageQuery::new()
        .page_size(Some(20))
        .user_id_type(Some("open_id"));
    let create_objective = CreateObjectiveReqBody {
        deadline: Some("123".into()),
        ..Default::default()
    };
    let patch_objective = PatchObjectiveReqBody {
        score: Some(0.8),
        ..Default::default()
    };
    let mut objective_weight = ObjectiveWeight::default();
    objective_weight.objective_id = Some("obj-1".into());
    objective_weight.weight = Some(1.0);
    let mut key_result_weight = KeyResultWeight::default();
    key_result_weight.key_result_id = Some("kr-1".into());
    key_result_weight.weight = Some(1.0);
    let create_key_result = CreateObjectiveKeyResultReqBody {
        deadline: Some("456".into()),
        ..Default::default()
    };

    okr.okr_alignment.delete("align-1", &option).await.unwrap();
    okr.okr_alignment.get("align-1", &option).await.unwrap();
    okr.okr_category.list(&page, &option).await.unwrap();
    okr.okr_cycle.list(&page, &option).await.unwrap();
    okr.okr_cycle
        .objectives_position(
            "cycle-1",
            &ObjectivesPositionReqBody {
                objective_ids: vec!["obj-1".into()],
            },
            &option,
        )
        .await
        .unwrap();
    okr.okr_cycle
        .objectives_weight(
            "cycle-1",
            &ObjectivesWeightReqBody {
                objective_weights: vec![objective_weight],
            },
            &option,
        )
        .await
        .unwrap();
    okr.okr_cycle_objective
        .create("cycle-1", &create_objective, &option)
        .await
        .unwrap();
    okr.okr_cycle_objective
        .list("cycle-1", &page, &option)
        .await
        .unwrap();
    okr.okr_indicator
        .patch(
            "indicator-1",
            &PatchIndicatorReqBody {
                current_value: Some(2.0),
                ..Default::default()
            },
            &option,
        )
        .await
        .unwrap();
    okr.okr_key_result.delete("kr-1", &option).await.unwrap();
    okr.okr_key_result.get("kr-1", &option).await.unwrap();
    okr.okr_key_result
        .patch(
            "kr-1",
            &PatchKeyResultReqBody {
                score: Some(0.5),
                ..Default::default()
            },
            &option,
        )
        .await
        .unwrap();
    okr.okr_key_result_indicator
        .list("kr-1", &option)
        .await
        .unwrap();
    okr.okr_key_result_progress
        .list("kr-1", &page, &option)
        .await
        .unwrap();
    okr.okr_objective.delete("obj-1", &option).await.unwrap();
    okr.okr_objective.get("obj-1", &option).await.unwrap();
    okr.okr_objective
        .key_results_position(
            "obj-1",
            &KeyResultsPositionReqBody {
                key_result_ids: vec!["kr-1".into()],
                ..Default::default()
            },
            &option,
        )
        .await
        .unwrap();
    okr.okr_objective
        .key_results_weight(
            "obj-1",
            &KeyResultsWeightReqBody {
                key_result_weights: vec![key_result_weight],
                ..Default::default()
            },
            &option,
        )
        .await
        .unwrap();
    okr.okr_objective
        .patch("obj-1", &patch_objective, &option)
        .await
        .unwrap();
    okr.okr_objective_alignment
        .create(
            "obj-1",
            &CreateObjectiveAlignmentReqBody {
                to_entity_type: Some(1),
                to_entity_id: Some("obj-2".into()),
                ..Default::default()
            },
            &option,
        )
        .await
        .unwrap();
    okr.okr_objective_alignment
        .list("obj-1", &page, &option)
        .await
        .unwrap();
    okr.okr_objective_indicator
        .list("obj-1", &option)
        .await
        .unwrap();
    okr.okr_objective_key_result
        .create("obj-1", &create_key_result, &option)
        .await
        .unwrap();
    okr.okr_objective_key_result
        .list("obj-1", &page, &option)
        .await
        .unwrap();
    okr.okr_objective_progress
        .list("obj-1", &page, &option)
        .await
        .unwrap();

    let requests = requests.lock().unwrap().join("\n");
    assert!(!requests.contains("prefer_draft"));
    for path in [
        "/open-apis/okr/v2/alignments/align-1",
        "/open-apis/okr/v2/categories",
        "/open-apis/okr/v2/cycles/cycle-1/objectives",
        "/open-apis/okr/v2/indicators/indicator-1",
        "/open-apis/okr/v2/key_results/kr-1/progresses",
        "/open-apis/okr/v2/objectives/obj-1/alignments",
        "/open-apis/okr/v2/objectives/obj-1/key_results",
        "/open-apis/okr/v2/objectives/obj-1/progresses",
    ] {
        assert!(requests.contains(path), "missing request path {path}");
    }
    assert!(requests.contains("user_id_type=open_id"));
    assert!(requests.contains(r#""objective_ids":["obj-1"]"#));
    assert!(requests.contains(r#""key_result_ids":["kr-1"]"#));
}

#[tokio::test]
async fn okr_v2_iterators_resume_from_page_token() {
    let first = r#"{"code":0,"msg":"ok","data":{"items":[{"objective_id":"obj-1"}],"page_token":"next","has_more":true}}"#;
    let second =
        r#"{"code":0,"msg":"ok","data":{"items":[{"objective_id":"obj-2"}],"has_more":false}}"#;
    let (addr, _handle, requests) =
        mock_server_with_requests(vec![http_response(200, first), http_response(200, second)])
            .await;
    let client = client_for(addr);
    let option = tenant_option();
    let okr = client.okr_v2();
    let mut iterator = okr
        .okr_cycle_objective
        .list_by_iterator("cycle-1", &OkrPageQuery::new().page_size(Some(1)))
        .limit(2);

    assert_eq!(
        iterator
            .next(&option)
            .await
            .unwrap()
            .unwrap()
            .objective_id
            .as_deref(),
        Some("obj-1")
    );
    assert_eq!(
        iterator
            .next(&option)
            .await
            .unwrap()
            .unwrap()
            .objective_id
            .as_deref(),
        Some("obj-2")
    );
    assert!(iterator.next(&option).await.unwrap().is_none());
    assert!(
        requests
            .lock()
            .unwrap()
            .join("\n")
            .contains("page_token=next")
    );
}

// The wire contract distinguishes an omitted draft choice from explicit false.
#[tokio::test]
async fn okr_draft_lists_preserve_optional_query() {
    for choice in [None, Some(false), Some(true)] {
        let ok = http_response(
            200,
            r#"{"code":0,"data":{"indicator":{"indicator_id":"i"}}}"#,
        );
        let (addr, _handle, requests) = mock_server_with_requests(vec![ok; 5]).await;
        let client = client_for(addr);
        let okr = client.okr_v2();
        let option = if choice == Some(true) {
            RequestOption {
                user_access_token: Some("user-token".into()),
                ..Default::default()
            }
        } else {
            tenant_option()
        };
        let page = OkrPageQuery::new().page_size(Some(2));
        okr.okr_cycle_objective
            .list_with_draft("c", &page, choice, &option)
            .await
            .unwrap();
        okr.okr_objective_alignment
            .list_with_draft("o", &page, choice, &option)
            .await
            .unwrap();
        okr.okr_objective_key_result
            .list_with_draft("o", &page, choice, &option)
            .await
            .unwrap();
        assert!(
            okr.okr_key_result_indicator
                .list_with_draft("k", choice, &option)
                .await
                .unwrap()
                .data
                .unwrap()
                .indicator
                .is_some()
        );
        assert!(
            okr.okr_objective_indicator
                .list_with_draft("o", choice, &option)
                .await
                .unwrap()
                .data
                .unwrap()
                .indicator
                .is_some()
        );
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 5);
        for (request, path) in requests.iter().zip([
            "/cycles/c/objectives",
            "/objectives/o/alignments",
            "/objectives/o/key_results",
            "/key_results/k/indicators",
            "/objectives/o/indicators",
        ]) {
            let line = request.lines().next().unwrap();
            assert!(line.starts_with("GET "));
            assert!(line.contains(path));
            match choice {
                None => assert!(!line.contains("prefer_draft")),
                Some(value) => assert!(line.contains(&format!("prefer_draft={value}"))),
            }
            assert!(
                request
                    .to_ascii_lowercase()
                    .contains(if choice == Some(true) {
                        "authorization: bearer user-token"
                    } else {
                        "authorization: bearer tenant-token"
                    })
            );
        }
    }
}

#[tokio::test]
async fn cycle_objective_draft_iterator_keeps_choice_across_pages() {
    for choice in [None, Some(false), Some(true)] {
        let first = http_response(
            200,
            r#"{"code":0,"data":{"items":[{"objective_id":"one"}],"page_token":"next","has_more":true}}"#,
        );
        let second = http_response(
            200,
            r#"{"code":0,"data":{"items":[{"objective_id":"two"}],"has_more":false}}"#,
        );
        let (addr, _handle, requests) = mock_server_with_requests(vec![first, second]).await;
        let client = client_for(addr);
        let okr = client.okr_v2();
        let page = OkrPageQuery::new()
            .page_size(Some(1))
            .page_token(Some("start"))
            .user_id(Some("user"))
            .user_id_type(Some("open_id"));
        let mut iterator = match choice {
            None => okr.okr_cycle_objective.list_by_iterator("c", &page),
            _ => okr
                .okr_cycle_objective
                .list_by_iterator_with_draft("c", &page, choice),
        };
        let option = tenant_option();
        assert_eq!(
            iterator
                .next(&option)
                .await
                .unwrap()
                .unwrap()
                .objective_id
                .as_deref(),
            Some("one")
        );
        assert_eq!(
            iterator
                .next(&option)
                .await
                .unwrap()
                .unwrap()
                .objective_id
                .as_deref(),
            Some("two")
        );
        assert!(iterator.next(&option).await.unwrap().is_none());
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        for (request, token) in requests.iter().zip(["start", "next"]) {
            let line = request.lines().next().unwrap();
            for value in ["page_size=1", "user_id=user", "user_id_type=open_id"] {
                assert!(line.contains(value));
            }
            assert!(line.contains(&format!("page_token={token}")));
            match choice {
                None => assert!(!line.contains("prefer_draft")),
                Some(value) => assert!(line.contains(&format!("prefer_draft={value}"))),
            }
        }
    }
}

#[tokio::test]
async fn objective_alignment_draft_iterator_keeps_choice_across_pages() {
    for choice in [None, Some(false), Some(true)] {
        let first = http_response(
            200,
            r#"{"code":0,"data":{"items":[{"alignment_id":"one"}],"page_token":"next","has_more":true}}"#,
        );
        let second = http_response(
            200,
            r#"{"code":0,"data":{"items":[{"alignment_id":"two"}],"has_more":false}}"#,
        );
        let (addr, _handle, requests) = mock_server_with_requests(vec![first, second]).await;
        let client = client_for(addr);
        let okr = client.okr_v2();
        let page = OkrPageQuery::new()
            .page_size(Some(1))
            .page_token(Some("start"))
            .user_id(Some("user"))
            .user_id_type(Some("open_id"));
        let mut iterator = match choice {
            None => okr.okr_objective_alignment.list_by_iterator("o", &page),
            _ => okr
                .okr_objective_alignment
                .list_by_iterator_with_draft("o", &page, choice),
        };
        let option = tenant_option();
        assert_eq!(
            iterator
                .next(&option)
                .await
                .unwrap()
                .unwrap()
                .alignment_id
                .as_deref(),
            Some("one")
        );
        assert_eq!(
            iterator
                .next(&option)
                .await
                .unwrap()
                .unwrap()
                .alignment_id
                .as_deref(),
            Some("two")
        );
        assert!(iterator.next(&option).await.unwrap().is_none());
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        for (request, token) in requests.iter().zip(["start", "next"]) {
            let line = request.lines().next().unwrap();
            for value in ["page_size=1", "user_id=user", "user_id_type=open_id"] {
                assert!(line.contains(value));
            }
            assert!(line.contains(&format!("page_token={token}")));
            match choice {
                None => assert!(!line.contains("prefer_draft")),
                Some(value) => assert!(line.contains(&format!("prefer_draft={value}"))),
            }
        }
    }
}

#[tokio::test]
async fn objective_key_result_draft_iterator_keeps_choice_across_pages() {
    for choice in [None, Some(false), Some(true)] {
        let first = http_response(
            200,
            r#"{"code":0,"data":{"items":[{"key_result_id":"one"}],"page_token":"next","has_more":true}}"#,
        );
        let second = http_response(
            200,
            r#"{"code":0,"data":{"items":[{"key_result_id":"two"}],"has_more":false}}"#,
        );
        let (addr, _handle, requests) = mock_server_with_requests(vec![first, second]).await;
        let client = client_for(addr);
        let okr = client.okr_v2();
        let page = OkrPageQuery::new()
            .page_size(Some(1))
            .page_token(Some("start"))
            .user_id(Some("user"))
            .user_id_type(Some("open_id"));
        let mut iterator = match choice {
            None => okr.okr_objective_key_result.list_by_iterator("o", &page),
            _ => okr
                .okr_objective_key_result
                .list_by_iterator_with_draft("o", &page, choice),
        };
        let option = tenant_option();
        assert_eq!(
            iterator
                .next(&option)
                .await
                .unwrap()
                .unwrap()
                .key_result_id
                .as_deref(),
            Some("one")
        );
        assert_eq!(
            iterator
                .next(&option)
                .await
                .unwrap()
                .unwrap()
                .key_result_id
                .as_deref(),
            Some("two")
        );
        assert!(iterator.next(&option).await.unwrap().is_none());
        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        for (request, token) in requests.iter().zip(["start", "next"]) {
            let line = request.lines().next().unwrap();
            for value in ["page_size=1", "user_id=user", "user_id_type=open_id"] {
                assert!(line.contains(value));
            }
            assert!(line.contains(&format!("page_token={token}")));
            match choice {
                None => assert!(!line.contains("prefer_draft")),
                Some(value) => assert!(line.contains(&format!("prefer_draft={value}"))),
            }
        }
    }
}

#[tokio::test]
async fn okr_draft_list_preserves_service_errors() {
    let (addr, _handle, _) = mock_server_with_requests(vec![http_response(
        200,
        r#"{"code":99991672,"msg":"draft access denied"}"#,
    )])
    .await;
    let client = client_for(addr);
    let error = client
        .okr_v2()
        .okr_objective_indicator
        .list_with_draft("o", Some(true), &tenant_option())
        .await
        .unwrap_err();
    assert!(error.to_string().contains("draft access denied"));
}
