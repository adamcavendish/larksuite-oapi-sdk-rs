use larksuite_oapi_sdk_rs::events::drive::{
    P2DriveFileBitableFieldChangedV1, P2DriveFileBitableRecordChangedV1,
    P2DriveFileCreatedInFolderV1, P2DriveFileDeletedV1, P2DriveFileEditedV1,
    P2DriveFilePermissionMemberAddedV1, P2DriveFilePermissionMemberAppliedV1,
    P2DriveFilePermissionMemberRemovedV1, P2DriveFileReadV1, P2DriveFileTitleUpdatedV1,
    P2DriveFileTrashedV1, P2DriveNoticeCommentAddV1,
};

#[test]
fn drive_file_read_edit_and_lifecycle_events_are_typed() {
    let read: P2DriveFileReadV1 = serde_json::from_value(serde_json::json!({
        "file_type": "docx",
        "file_token": "doc_1",
        "operator_id_list": [
            { "open_id": "ou_reader" },
            { "user_id": "user_reader" }
        ],
        "subscriber_id_list": [{ "union_id": "on_subscriber" }]
    }))
    .unwrap();
    assert_eq!(read.file_token.as_deref(), Some("doc_1"));
    assert_eq!(read.operator_id_list[0].open_id(), Some("ou_reader"));
    assert_eq!(read.operator_id_list[1].user_id(), Some("user_reader"));
    assert_eq!(read.subscriber_id_list[0].union_id(), Some("on_subscriber"));

    let edited: P2DriveFileEditedV1 = serde_json::from_value(serde_json::json!({
        "file_type": "sheet",
        "file_token": "sheet_1",
        "operator_id_list": [{ "open_id": "ou_editor" }],
        "subscriber_id_list": [{ "open_id": "ou_subscriber" }],
        "sheet_id": "sh_1"
    }))
    .unwrap();
    assert_eq!(edited.operator_id_list[0].open_id(), Some("ou_editor"));
    assert_eq!(edited.sheet_id.as_deref(), Some("sh_1"));

    let deleted: P2DriveFileDeletedV1 = serde_json::from_value(serde_json::json!({
        "file_token": "doc_1",
        "operator_id": { "open_id": "ou_deleter" },
        "subscriber_id_list": [{ "user_id": "user_subscriber" }]
    }))
    .unwrap();
    assert_eq!(
        deleted.operator_id.as_ref().unwrap().open_id(),
        Some("ou_deleter")
    );
    assert_eq!(
        deleted.subscriber_id_list[0].user_id(),
        Some("user_subscriber")
    );

    let title: P2DriveFileTitleUpdatedV1 = serde_json::from_value(serde_json::json!({
        "file_token": "doc_1",
        "operator_id": { "union_id": "on_title" }
    }))
    .unwrap();
    assert_eq!(
        title.operator_id.as_ref().unwrap().union_id(),
        Some("on_title")
    );

    let trashed: P2DriveFileTrashedV1 = serde_json::from_value(serde_json::json!({
        "file_token": "doc_1",
        "operator_id": { "open_id": "ou_trash" }
    }))
    .unwrap();
    assert_eq!(
        trashed.operator_id.as_ref().unwrap().open_id(),
        Some("ou_trash")
    );
}

#[test]
fn drive_permission_member_events_are_typed() {
    let added: P2DriveFilePermissionMemberAddedV1 = serde_json::from_value(serde_json::json!({
        "file_type": "docx",
        "file_token": "doc_1",
        "operator_id": { "open_id": "ou_operator" },
        "user_list": [{ "open_id": "ou_added" }],
        "chat_list": ["oc_1"],
        "open_department_id_list": ["od_1"],
        "subscriber_id_list": [{ "union_id": "on_subscriber" }]
    }))
    .unwrap();
    assert_eq!(
        added.operator_id.as_ref().unwrap().open_id(),
        Some("ou_operator")
    );
    assert_eq!(added.user_list[0].open_id(), Some("ou_added"));
    assert_eq!(added.chat_list, vec!["oc_1"]);
    assert_eq!(added.open_department_id_list, vec!["od_1"]);

    let applied: P2DriveFilePermissionMemberAppliedV1 = serde_json::from_value(serde_json::json!({
        "file_token": "doc_1",
        "operator_id": { "open_id": "ou_applicant" },
        "approver_id": { "user_id": "approver_1" },
        "application_user_list": [{ "union_id": "on_target" }],
        "application_chat_list": ["oc_2"],
        "application_department_list": ["od_2"],
        "application_remark": "please approve",
        "permission": "read",
        "subscriber_ids": [{ "open_id": "ou_subscriber" }]
    }))
    .unwrap();
    assert_eq!(
        applied.approver_id.as_ref().unwrap().user_id(),
        Some("approver_1")
    );
    assert_eq!(
        applied.application_user_list[0].union_id(),
        Some("on_target")
    );
    assert_eq!(applied.permission.as_deref(), Some("read"));

    let removed: P2DriveFilePermissionMemberRemovedV1 = serde_json::from_value(serde_json::json!({
        "file_token": "doc_1",
        "operator_id": { "open_id": "ou_operator" },
        "user_list": [{ "user_id": "user_removed" }],
        "chat_list": ["oc_3"],
        "open_department_id_list": ["od_3"],
        "subscriber_id_list": [{ "open_id": "ou_subscriber" }]
    }))
    .unwrap();
    assert_eq!(removed.user_list[0].user_id(), Some("user_removed"));
    assert_eq!(
        removed.subscriber_id_list[0].open_id(),
        Some("ou_subscriber")
    );
}

#[test]
fn drive_bitable_events_have_typed_actions() {
    let field: P2DriveFileBitableFieldChangedV1 = serde_json::from_value(serde_json::json!({
        "file_type": "bitable",
        "file_token": "base_1",
        "table_id": "tbl_1",
        "operator_id": { "open_id": "ou_operator" },
        "action_list": [{
            "action": "field_edited",
            "field_id": "fld_1",
            "before_value": {
                "id": "fld_1",
                "name": "Old",
                "type": 1,
                "property": {
                    "formatter": "1,000",
                    "auto_serial": {
                        "type": "custom",
                        "options": [{ "type": "created_time", "value": "yyyyMMdd" }]
                    },
                    "options": [{ "id": "opt_1", "name": "Open", "color": 1 }]
                }
            },
            "after_value": { "id": "fld_1", "name": "New", "type": 2 }
        }],
        "revision": 12,
        "subscriber_id_list": [{ "open_id": "ou_subscriber" }],
        "update_time": 1710000000
    }))
    .unwrap();
    assert_eq!(field.action_list[0].action.as_deref(), Some("field_edited"));
    assert_eq!(
        field.action_list[0].before_value.as_ref().unwrap().type_,
        Some(1)
    );
    assert_eq!(
        field.action_list[0]
            .before_value
            .as_ref()
            .unwrap()
            .property
            .as_ref()
            .unwrap()
            .auto_serial
            .as_ref()
            .unwrap()
            .options[0]
            .type_
            .as_deref(),
        Some("created_time")
    );

    let record: P2DriveFileBitableRecordChangedV1 = serde_json::from_value(serde_json::json!({
        "file_type": "bitable",
        "file_token": "base_1",
        "table_id": "tbl_1",
        "revision": 13,
        "operator_id": { "union_id": "on_operator" },
        "action_list": [{
            "record_id": "rec_1",
            "action": "record_edited",
            "before_value": [{ "field_id": "fld_1", "field_value": "old" }],
            "after_value": [{
                "field_id": "fld_1",
                "field_value": "new",
                "field_identity_value": {
                    "users": [{
                        "user_id": { "open_id": "ou_user" },
                        "name": "Ada",
                        "en_name": "Ada",
                        "avatar_url": "https://example.test/avatar.png"
                    }]
                }
            }]
        }],
        "subscriber_id_list": [{ "open_id": "ou_subscriber" }],
        "update_time": 1710000001
    }))
    .unwrap();
    assert_eq!(
        record.operator_id.as_ref().unwrap().union_id(),
        Some("on_operator")
    );
    assert_eq!(record.action_list[0].record_id.as_deref(), Some("rec_1"));
    assert_eq!(
        record.action_list[0].after_value[0]
            .field_identity_value
            .as_ref()
            .unwrap()
            .users[0]
            .user_id
            .as_ref()
            .unwrap()
            .open_id(),
        Some("ou_user")
    );
}

#[test]
fn drive_created_in_folder_and_notice_comment_events_are_typed() {
    let created: P2DriveFileCreatedInFolderV1 = serde_json::from_value(serde_json::json!({
        "file_type": "docx",
        "file_token": "doc_1",
        "folder_token": "fld_1",
        "operator_id": { "open_id": "ou_operator" },
        "subscriber_ids": [{ "union_id": "on_subscriber" }]
    }))
    .unwrap();
    assert_eq!(created.folder_token.as_deref(), Some("fld_1"));
    assert_eq!(created.subscriber_ids[0].union_id(), Some("on_subscriber"));

    let notice: P2DriveNoticeCommentAddV1 = serde_json::from_value(serde_json::json!({
        "notice_meta": {
            "file_type": "docx",
            "file_token": "doc_1",
            "from_user_id": { "open_id": "ou_from" },
            "to_user_id": { "user_id": "user_to" },
            "notice_type": "add_comment"
        },
        "comment_id": "comment_1",
        "reply_id": "reply_1",
        "is_mentioned": true
    }))
    .unwrap();
    assert_eq!(
        notice
            .notice_meta
            .as_ref()
            .unwrap()
            .from_user_id
            .as_ref()
            .unwrap()
            .open_id(),
        Some("ou_from")
    );
    assert_eq!(
        notice
            .notice_meta
            .as_ref()
            .unwrap()
            .to_user_id
            .as_ref()
            .unwrap()
            .user_id(),
        Some("user_to")
    );
    assert_eq!(notice.is_mentioned, Some(true));
}

#[test]
fn drive_event_structs_accept_empty_and_null_payloads() {
    let read: P2DriveFileReadV1 = serde_json::from_value(serde_json::json!({})).unwrap();
    assert!(read.file_token.is_none());
    assert!(read.operator_id_list.is_empty());

    let notice: P2DriveNoticeCommentAddV1 = serde_json::from_value(serde_json::json!({
        "notice_meta": null
    }))
    .unwrap();
    assert!(notice.notice_meta.is_none());
}
