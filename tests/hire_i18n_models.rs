use larksuite_oapi_sdk_rs::service::hire::{v1, v2};

#[test]
fn hire_v1_id_name_object_uses_i18n_name() {
    let value: v1::IdNameObject =
        serde_json::from_str(r#"{"id":"job-type-1","name":{"zh_cn":"正式","en_us":"Full time"}}"#)
            .unwrap();

    assert_eq!(value.id.as_deref(), Some("job-type-1"));
    assert_eq!(value.name.as_ref().unwrap().zh_cn.as_deref(), Some("正式"));
    assert_eq!(
        value.name.as_ref().unwrap().en_us.as_deref(),
        Some("Full time")
    );
}

#[test]
fn hire_v2_nested_i18n_models_deserialize() {
    let value: v2::CompositeTalent = serde_json::from_str(
        r#"{"talent_id":"talent-1","tag_list":[{"id":"tag-1","name":{"en_us":"Priority"},"description":{"zh_cn":"优先"}}],"note_list_v2":[{"id":"note-1","privacy":1}]}"#,
    )
    .unwrap();

    assert_eq!(value.talent_id.as_deref(), Some("talent-1"));
    assert_eq!(
        value.tag_list.as_ref().unwrap()[0]
            .description
            .as_ref()
            .unwrap()
            .zh_cn
            .as_deref(),
        Some("优先")
    );
    assert_eq!(value.note_list_v2.as_ref().unwrap()[0].privacy, Some(1));
}
