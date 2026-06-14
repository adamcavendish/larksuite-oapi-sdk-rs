use larksuite_oapi_sdk_rs::events::performance::{P2ReviewDataChangedV2, P2StageTaskOpenResultV2};

#[test]
fn performance_review_data_changed_event_is_typed() {
    let event: P2ReviewDataChangedV2 = serde_json::from_value(serde_json::json!({
        "items": [
            {
                "user_id": { "open_id": "ou_user" },
                "semester_id": "semester_1",
                "activity_id": "activity_1",
                "stage_changes": [
                    {
                        "stage_id": "stage_1",
                        "stage_type": "leader_review",
                        "review_stage_role": "solid_line_leader"
                    }
                ]
            }
        ]
    }))
    .unwrap();

    let item = &event.items[0];
    let stage = &item.stage_changes[0];
    assert_eq!(item.user_id.as_ref().unwrap().open_id(), Some("ou_user"));
    assert_eq!(item.semester_id.as_deref(), Some("semester_1"));
    assert_eq!(item.activity_id.as_deref(), Some("activity_1"));
    assert_eq!(stage.stage_id.as_deref(), Some("stage_1"));
    assert_eq!(stage.stage_type.as_deref(), Some("leader_review"));
    assert_eq!(
        stage.review_stage_role.as_deref(),
        Some("solid_line_leader")
    );
}

#[test]
fn performance_stage_task_open_result_event_is_typed() {
    let event: P2StageTaskOpenResultV2 = serde_json::from_value(serde_json::json!({
        "items": [
            {
                "user_id": { "union_id": "on_user" },
                "semester_id": "semester_1",
                "activity_id": "activity_1",
                "open_time": "1704038400000"
            }
        ]
    }))
    .unwrap();

    let item = &event.items[0];
    assert_eq!(item.user_id.as_ref().unwrap().union_id(), Some("on_user"));
    assert_eq!(item.semester_id.as_deref(), Some("semester_1"));
    assert_eq!(item.activity_id.as_deref(), Some("activity_1"));
    assert_eq!(item.open_time.as_deref(), Some("1704038400000"));
}

#[test]
fn performance_event_structs_accept_empty_and_null_payloads() {
    let review_data: P2ReviewDataChangedV2 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(review_data.items.is_empty());

    let review_data_null_fields: P2ReviewDataChangedV2 =
        serde_json::from_value(serde_json::json!({
            "items": [
                {
                    "user_id": null,
                    "stage_changes": []
                }
            ]
        }))
        .unwrap();
    assert!(review_data_null_fields.items[0].user_id.is_none());
    assert!(review_data_null_fields.items[0].stage_changes.is_empty());

    let open_result: P2StageTaskOpenResultV2 = serde_json::from_value(serde_json::json!({
        "items": []
    }))
    .unwrap();
    assert!(open_result.items.is_empty());
}
