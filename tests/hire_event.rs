use larksuite_oapi_sdk_rs::events::hire::{
    P2HireApplicationDeletedV1, P2HireApplicationStageChangedV1, P2HireEcoBackgroundCheckCreatedV1,
    P2HireEcoExamCreatedV1, P2HireEhrImportTaskForInternshipOfferImportedV1,
    P2HireEhrImportTaskImportedV1, P2HireOfferStatusChangedV1, P2HireReferralAccountAssetsUpdateV1,
    P2HireTalentTagSubscriptionV1,
};

#[test]
fn hire_application_and_offer_events_match_go_shape() {
    let offer: P2HireOfferStatusChangedV1 = serde_json::from_value(serde_json::json!({
        "offer_id": "offer_1",
        "offer_status": 3,
        "offer_type": 99
    }))
    .unwrap();
    assert_eq!(offer.offer_id.as_deref(), Some("offer_1"));
    assert_eq!(offer.offer_status, Some(3));

    let stage: P2HireApplicationStageChangedV1 = serde_json::from_value(serde_json::json!({
        "application_id": "app_1",
        "origin_stage_id": "stage_old",
        "target_stage_id": "stage_new",
        "update_time": 1710000000000i64
    }))
    .unwrap();
    assert_eq!(stage.origin_stage_id.as_deref(), Some("stage_old"));
    assert_eq!(stage.target_stage_id.as_deref(), Some("stage_new"));
    assert_eq!(stage.update_time, Some(1710000000000));

    let deleted: P2HireApplicationDeletedV1 = serde_json::from_value(serde_json::json!({
        "application_ids": ["app_1", "app_2"]
    }))
    .unwrap();
    assert_eq!(deleted.application_ids, vec!["app_1", "app_2"]);
}

#[test]
fn hire_ehr_import_events_have_typed_operator_and_department_ids() {
    let imported: P2HireEhrImportTaskImportedV1 = serde_json::from_value(serde_json::json!({
        "task_id": "task_1",
        "application_id": "app_1",
        "ehr_department_id": "dept_custom",
        "ehr_requirement_id": "req_1",
        "operator_id": "hire_user",
        "operator_user_id": { "open_id": "ou_operator" },
        "ehr_department": { "department_id": "dept_1", "open_department_id": "od_1" }
    }))
    .unwrap();
    assert_eq!(
        imported.operator_user_id.as_ref().unwrap().open_id(),
        Some("ou_operator")
    );
    assert_eq!(
        imported
            .ehr_department
            .as_ref()
            .unwrap()
            .open_department_id
            .as_deref(),
        Some("od_1")
    );

    let internship: P2HireEhrImportTaskForInternshipOfferImportedV1 =
        serde_json::from_value(serde_json::json!({
            "task_id": "task_2",
            "offer_id": "offer_1",
            "pre_onboard_id": "pre_1",
            "operator_user_id": { "union_id": "on_operator" }
        }))
        .unwrap();
    assert_eq!(
        internship.operator_user_id.as_ref().unwrap().union_id(),
        Some("on_operator")
    );
    assert_eq!(internship.offer_id.as_deref(), Some("offer_1"));
}

#[test]
fn hire_eco_background_and_exam_events_are_typed() {
    let background: P2HireEcoBackgroundCheckCreatedV1 = serde_json::from_value(serde_json::json!({
        "background_check_id": "bg_1",
        "account_id": "acct_1",
        "package_id": "pkg_1",
        "additional_item_id_list": ["item_1"],
        "comment": "check",
        "candidate_info": {
            "name": "Ada",
            "mobile": { "code": "+1", "number": "5550100" },
            "email": "ada@example.test"
        },
        "client_contact_info": {
            "name": "Recruiter",
            "mobile": { "code": "+1", "number": "5550101" }
        },
        "custom_field_list": [{ "key": "field", "value": "value" }]
    }))
    .unwrap();
    assert_eq!(background.background_check_id.as_deref(), Some("bg_1"));
    assert_eq!(background.additional_item_id_list, vec!["item_1"]);
    assert_eq!(
        background
            .candidate_info
            .as_ref()
            .unwrap()
            .mobile
            .as_ref()
            .unwrap()
            .number
            .as_deref(),
        Some("5550100")
    );
    assert_eq!(
        background.custom_field_list.as_ref().unwrap()[0]
            .key
            .as_deref(),
        Some("field")
    );

    let exam: P2HireEcoExamCreatedV1 = serde_json::from_value(serde_json::json!({
        "exam_id": "exam_1",
        "candidate_info": {
            "name": "Ada",
            "mobile": { "code": "+1", "number": "5550102" }
        },
        "talent_id": "talent_1",
        "application_id": "app_1"
    }))
    .unwrap();
    assert_eq!(exam.exam_id.as_deref(), Some("exam_1"));
    assert_eq!(
        exam.candidate_info.as_ref().unwrap().name.as_deref(),
        Some("Ada")
    );
}

#[test]
fn hire_referral_and_talent_tag_events_are_typed() {
    let referral: P2HireReferralAccountAssetsUpdateV1 = serde_json::from_value(serde_json::json!({
        "account_id": "account_1",
        "assets": {
            "confirmed_bonus": {
                "bonus_type": 1,
                "point_bonus": 100,
                "cash": { "currency_type": "USD", "amount": 12.5 }
            }
        },
        "modify_time": "1710000000000"
    }))
    .unwrap();
    assert_eq!(referral.account_id.as_deref(), Some("account_1"));
    assert_eq!(
        referral
            .assets
            .as_ref()
            .unwrap()
            .confirmed_bonus
            .as_ref()
            .unwrap()
            .cash
            .as_ref()
            .unwrap()
            .amount,
        Some(12.5)
    );

    let tag: P2HireTalentTagSubscriptionV1 = serde_json::from_value(serde_json::json!({
        "talent_id": "talent_1",
        "application_id": "app_1",
        "type": 1,
        "tag": {
            "id": "tag_1",
            "name": { "zh_cn": "标签", "en_us": "Tag" },
            "active_status": 1
        },
        "lock_status": 0,
        "application_stage": { "id": "stage_1", "zh_name": "初筛" }
    }))
    .unwrap();
    assert_eq!(tag.type_, Some(1));
    assert_eq!(tag.tag.as_ref().unwrap().id.as_deref(), Some("tag_1"));
    assert_eq!(
        tag.application_stage.as_ref().unwrap().zh_name.as_deref(),
        Some("初筛")
    );
}

#[test]
fn hire_event_structs_accept_empty_and_null_payloads() {
    let offer: P2HireOfferStatusChangedV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(offer.offer_id.is_none());

    let background: P2HireEcoBackgroundCheckCreatedV1 = serde_json::from_value(serde_json::json!({
        "candidate_info": null,
        "custom_field_list": null
    }))
    .unwrap();
    assert!(background.candidate_info.is_none());
    assert!(background.custom_field_list.is_none());
}
