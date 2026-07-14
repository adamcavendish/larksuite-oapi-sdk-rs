use larksuite_oapi_sdk_rs::service::{document_ai::v1, docx::v1 as docx_v1};

#[test]
fn document_ai_recognition_response_deserializes_nested_models() {
    let response: v1::VatInvoiceData = serde_json::from_str(
        r#"{
            "vat_invoices": [{
                "entities": [{
                    "type": "invoice_number",
                    "value": "12345",
                    "items": [[{"type": "amount", "value": "100"}]]
                }]
            }]
        }"#,
    )
    .unwrap();

    let invoice = response.vat_invoices.unwrap().pop().unwrap();
    let entity = invoice.entities.unwrap().pop().unwrap();
    assert_eq!(entity.r#type.as_deref(), Some("invoice_number"));
    assert_eq!(entity.items.unwrap()[0][0].value.as_deref(), Some("100"));
}

#[test]
fn docx_domain_models_deserialize_typed_block_and_document_fields() {
    let block: docx_v1::Block = serde_json::from_str(
        r#"{
            "block_id": "blk_1",
            "text": {"elements": [{"text_run": {"content": "hello"}}]},
            "table": {"cells": ["cell_1"]}
        }"#,
    )
    .unwrap();
    let document: docx_v1::Document = serde_json::from_str(
        r#"{
            "display_setting": {"show_authors": true},
            "cover": {"token": "img_1", "offset_ratio_x": 0.5}
        }"#,
    )
    .unwrap();

    assert_eq!(
        block.text.unwrap().elements[0]
            .text_run
            .as_ref()
            .unwrap()
            .content
            .as_deref(),
        Some("hello")
    );
    assert_eq!(document.display_setting.unwrap().show_authors, Some(true));
    assert_eq!(document.cover.unwrap().token.as_deref(), Some("img_1"));
}

#[test]
fn approval_shared_models_deserialize_nested_structures() {
    use larksuite_oapi_sdk_rs::service::approval::v4;

    let instance: v4::ApprovalInstance = serde_json::from_str(
        r#"{
            "comment_list": [{"id": "comment_1", "files": [{"url": "https://example.test/a"}]}],
            "i18n_resources": [{"locale": "en-US", "texts": [{"key": "@i18n@name", "value": "Leave"}]}],
            "link": {"pc_link": "https://example.test/pc"}
        }"#,
    )
    .unwrap();

    assert_eq!(
        instance.comment_list.unwrap()[0].files[0].url.as_deref(),
        Some("https://example.test/a")
    );
    assert_eq!(
        instance.i18n_resources.unwrap()[0].texts[0]
            .value
            .as_deref(),
        Some("Leave")
    );
    assert_eq!(
        instance.link.unwrap().pc_link.as_deref(),
        Some("https://example.test/pc")
    );
}

#[test]
fn attendance_shared_models_deserialize_group_and_shift_rules() {
    use larksuite_oapi_sdk_rs::service::attendance::v1;

    let group: v1::AttendanceGroup = serde_json::from_str(
        r#"{
            "locations": [{"location_id": "loc_1", "latitude": 1.5}],
            "free_punch_cfg": {"free_start_time": "09:00", "free_clock_setting": {"clock_mode": 1}},
            "need_punch_special_days": [{"punch_day": 20260715, "shift_id": "shift_1"}]
        }"#,
    )
    .unwrap();
    let shift: v1::Shift = serde_json::from_str(
        r#"{
            "punch_time_rule": [{"on_time": "09:00", "off_time": "18:00"}],
            "break_time_rule": [{"rest_begin_time": "12:00", "rest_end_time": "13:00"}]
        }"#,
    )
    .unwrap();

    assert_eq!(
        group.locations.unwrap()[0].location_id.as_deref(),
        Some("loc_1")
    );
    assert_eq!(
        group
            .free_punch_cfg
            .unwrap()
            .free_clock_setting
            .unwrap()
            .clock_mode,
        Some(1)
    );
    assert_eq!(
        shift.punch_time_rule.unwrap()[0].on_time.as_deref(),
        Some("09:00")
    );
}

#[test]
fn search_shared_models_deserialize_filters_and_result_units() {
    use larksuite_oapi_sdk_rs::service::search::v2;

    let source: v2::DataSource = serde_json::from_str(
        r#"{
            "i18n_name": {"zh_cn": "知识库", "en_us": "Knowledge"},
            "i18n_description": {"en_us": "Searchable records"}
        }"#,
    )
    .unwrap();
    let response: v2::SearchDocWikiData = serde_json::from_str(
        r#"{
            "res_units": [{
                "title_highlighted": "Guide",
                "result_meta": {"token": "doc_1", "owner_id": "ou_1"}
            }]
        }"#,
    )
    .unwrap();

    assert_eq!(
        source.i18n_name.unwrap().en_us.as_deref(),
        Some("Knowledge")
    );
    assert_eq!(
        response.res_units[0]
            .result_meta
            .as_ref()
            .unwrap()
            .token
            .as_deref(),
        Some("doc_1")
    );
}
