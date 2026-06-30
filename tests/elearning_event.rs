use larksuite_oapi_sdk_rs::EventDispatcher;
use larksuite_oapi_sdk_rs::events::elearning::{
    P2CourseRegistrationCreatedV2, P2CourseRegistrationDeletedV2, P2CourseRegistrationUpdatedV2,
};

#[test]
fn elearning_course_registration_events_are_typed() {
    let created: P2CourseRegistrationCreatedV2 = serde_json::from_value(serde_json::json!({
        "event": {
            "course_id": "course_1",
            "learner": {
                "user_id": { "open_id": "ou_1" }
            },
            "enroll_at": 1710000000,
            "enroll_type": 1,
            "learning_duration": 120,
            "finished_at": 1710000120,
            "learning_state": 2,
            "compulsory_lesson_ids": ["lesson_1"],
            "learned_compulsory_lesson_ids": ["lesson_1"],
            "optional_lesson_ids": ["lesson_2"],
            "learned_optional_lesson_ids": []
        }
    }))
    .unwrap();

    let event = created.event.as_ref().unwrap();
    assert_eq!(event.course_id.as_deref(), Some("course_1"));
    assert_eq!(
        event
            .learner
            .as_ref()
            .and_then(|learner| learner.user_id.as_ref())
            .and_then(|user| user.open_id()),
        Some("ou_1")
    );
    assert_eq!(event.compulsory_lesson_ids, ["lesson_1"]);

    let deleted: P2CourseRegistrationDeletedV2 = serde_json::from_value(serde_json::json!({
        "event": {
            "course_id": "course_1",
            "learner": {
                "user_id": { "user_id": "user_1" }
            }
        }
    }))
    .unwrap();
    assert_eq!(
        deleted
            .event
            .as_ref()
            .and_then(|event| event.learner.as_ref())
            .and_then(|learner| learner.user_id.as_ref())
            .and_then(|user| user.user_id()),
        Some("user_1")
    );

    let _dispatcher = EventDispatcher::new("", "")
        .on_p2_elearning_course_registration_created_v2(|_| async { Ok(()) })
        .on_p2_elearning_course_registration_deleted_v2(|_| async { Ok(()) })
        .on_p2_elearning_course_registration_updated_v2(|_| async { Ok(()) });
}

#[test]
fn elearning_course_registration_events_accept_empty_payloads() {
    let created: P2CourseRegistrationCreatedV2 =
        serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(created.event.is_none());

    let updated: P2CourseRegistrationUpdatedV2 =
        serde_json::from_value(serde_json::json!({ "event": null })).unwrap();
    assert!(updated.event.is_none());
}
