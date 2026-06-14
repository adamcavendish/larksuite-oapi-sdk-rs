use larksuite_oapi_sdk_rs::events::moments::{
    P2MomentsCommentCreatedV1, P2MomentsCommentDeletedV1, P2MomentsPostCreatedV1,
    P2MomentsPostDeletedV1, P2MomentsPostStatisticsUpdatedV1, P2MomentsReactionCreatedV1,
    P2MomentsReactionDeletedV1,
};

#[test]
fn moments_post_events_match_go_shape() {
    let created: P2MomentsPostCreatedV1 = serde_json::from_value(serde_json::json!({
        "id": "post_1",
        "user_id": { "open_id": "ou_user" },
        "create_time": "1710000000",
        "category_ids": ["category_1", "category_2"],
        "link": "https://example.com/post_1",
        "user_type": 1
    }))
    .unwrap();

    assert_eq!(created.id.as_deref(), Some("post_1"));
    assert_eq!(created.user_id.as_ref().unwrap().open_id(), Some("ou_user"));
    assert_eq!(created.create_time.as_deref(), Some("1710000000"));
    assert_eq!(created.category_ids, vec!["category_1", "category_2"]);
    assert_eq!(created.link.as_deref(), Some("https://example.com/post_1"));
    assert_eq!(created.user_type, Some(1));

    let deleted: P2MomentsPostDeletedV1 = serde_json::from_value(serde_json::json!({
        "id": "post_1"
    }))
    .unwrap();
    assert_eq!(deleted.id.as_deref(), Some("post_1"));
}

#[test]
fn moments_reaction_events_match_go_shape() {
    let created: P2MomentsReactionCreatedV1 = serde_json::from_value(serde_json::json!({
        "type": "LIKE",
        "user_id": { "user_id": "user_1" },
        "entity_id": "post_1",
        "id": "reaction_1",
        "entity_type": 1,
        "user_type": 2,
        "create_time": "1710000001"
    }))
    .unwrap();

    assert_eq!(created.type_.as_deref(), Some("LIKE"));
    assert_eq!(created.user_id.as_ref().unwrap().user_id(), Some("user_1"));
    assert_eq!(created.entity_id.as_deref(), Some("post_1"));
    assert_eq!(created.id.as_deref(), Some("reaction_1"));
    assert_eq!(created.entity_type, Some(1));
    assert_eq!(created.user_type, Some(2));
    assert_eq!(created.create_time.as_deref(), Some("1710000001"));

    let deleted: P2MomentsReactionDeletedV1 = serde_json::from_value(serde_json::json!({
        "type": "LIKE",
        "user_id": { "union_id": "on_user" },
        "entity_id": "comment_1",
        "id": "reaction_2",
        "entity_type": 2,
        "user_type": 1
    }))
    .unwrap();

    assert_eq!(deleted.type_.as_deref(), Some("LIKE"));
    assert_eq!(
        deleted.user_id.as_ref().unwrap().union_id(),
        Some("on_user")
    );
    assert_eq!(deleted.entity_id.as_deref(), Some("comment_1"));
    assert_eq!(deleted.id.as_deref(), Some("reaction_2"));
    assert_eq!(deleted.entity_type, Some(2));
    assert_eq!(deleted.user_type, Some(1));
}

#[test]
fn moments_comment_events_match_go_shape() {
    let created: P2MomentsCommentCreatedV1 = serde_json::from_value(serde_json::json!({
        "user_id": { "open_id": "ou_commenter" },
        "id": "comment_1",
        "create_time": "1710000002",
        "post_id": "post_1",
        "reply_comment_id": "comment_parent",
        "root_comment_id": "comment_root",
        "user_type": 1
    }))
    .unwrap();

    assert_eq!(
        created.user_id.as_ref().unwrap().open_id(),
        Some("ou_commenter")
    );
    assert_eq!(created.id.as_deref(), Some("comment_1"));
    assert_eq!(created.create_time.as_deref(), Some("1710000002"));
    assert_eq!(created.post_id.as_deref(), Some("post_1"));
    assert_eq!(created.reply_comment_id.as_deref(), Some("comment_parent"));
    assert_eq!(created.root_comment_id.as_deref(), Some("comment_root"));
    assert_eq!(created.user_type, Some(1));

    let deleted: P2MomentsCommentDeletedV1 = serde_json::from_value(serde_json::json!({
        "id": "comment_1",
        "post_id": "post_1"
    }))
    .unwrap();
    assert_eq!(deleted.id.as_deref(), Some("comment_1"));
    assert_eq!(deleted.post_id.as_deref(), Some("post_1"));
}

#[test]
fn moments_post_statistics_updated_event_matches_go_shape() {
    let event: P2MomentsPostStatisticsUpdatedV1 = serde_json::from_value(serde_json::json!({
        "post_id": "post_1",
        "statistics_type": 1,
        "statistics": { "share_count": 3 }
    }))
    .unwrap();

    assert_eq!(event.post_id.as_deref(), Some("post_1"));
    assert_eq!(event.statistics_type, Some(1));
    assert_eq!(event.statistics.as_ref().unwrap().share_count, Some(3));
}

#[test]
fn moments_events_accept_empty_and_null_payloads() {
    let created: P2MomentsPostCreatedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(created.id.is_none());
    assert!(created.user_id.is_none());
    assert!(created.category_ids.is_empty());

    let reaction: P2MomentsReactionCreatedV1 = serde_json::from_value(serde_json::json!({
        "user_id": null,
        "type": null
    }))
    .unwrap();
    assert!(reaction.user_id.is_none());
    assert!(reaction.type_.is_none());

    let stats: P2MomentsPostStatisticsUpdatedV1 = serde_json::from_value(serde_json::json!({
        "statistics": null
    }))
    .unwrap();
    assert!(stats.statistics.is_none());
}
