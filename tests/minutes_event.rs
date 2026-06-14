use larksuite_oapi_sdk_rs::events::minutes::P2MinuteGeneratedV1;

#[test]
fn minute_generated_event_is_typed() {
    let event: P2MinuteGeneratedV1 = serde_json::from_value(serde_json::json!({
        "minute_token": "minute_1",
        "minute_source": {
            "source_type": "meeting",
            "source_entity_id": "meeting_1"
        },
        "subscriber_ids": [
            { "open_id": "ou_1" },
            { "user_id": "user_2" },
            { "union_id": "on_3" }
        ]
    }))
    .unwrap();

    let source = event.minute_source.as_ref().unwrap();
    assert_eq!(event.minute_token.as_deref(), Some("minute_1"));
    assert_eq!(source.source_type.as_deref(), Some("meeting"));
    assert_eq!(source.source_entity_id.as_deref(), Some("meeting_1"));
    assert_eq!(event.subscriber_ids[0].open_id(), Some("ou_1"));
    assert_eq!(event.subscriber_ids[1].user_id(), Some("user_2"));
    assert_eq!(event.subscriber_ids[2].union_id(), Some("on_3"));
}

#[test]
fn minute_generated_event_accepts_empty_and_null_payloads() {
    let empty: P2MinuteGeneratedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(empty.minute_token.is_none());
    assert!(empty.minute_source.is_none());
    assert!(empty.subscriber_ids.is_empty());

    let null_source: P2MinuteGeneratedV1 = serde_json::from_value(serde_json::json!({
        "minute_source": null
    }))
    .unwrap();
    assert!(null_source.minute_source.is_none());
}
