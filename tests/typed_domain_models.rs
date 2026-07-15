use larksuite_oapi_sdk_rs::service::{aily::v1 as aily_v1, document_ai::v1, docx::v1 as docx_v1};

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

#[test]
fn aily_models_deserialize_knowledge_and_skill_inputs() {
    let asset: aily_v1::DataAsset = serde_json::from_str(
        r#"{
            "description": {"en_us": "Product docs"}
        }"#,
    )
    .unwrap();
    let answer: aily_v1::AskKnowledgeData = serde_json::from_str(
        r#"{
            "message": {"content": "Use the API reference."},
            "process_data": {"chart_dsls": ["bar chart"], "chunks": ["chunk"], "sql_data": ["[]"]},
            "faq_result": {"question": "Where are docs?", "answer": "In the portal."}
        }"#,
    )
    .unwrap();
    let body = aily_v1::CreateDataAssetReqBody {
        import_knowledge_setting: Some(aily_v1::DataAssetImportKnowledgeSetting {
            file: Some(aily_v1::DataAssetImportKnowledgeFile {
                title: Some("Guide".into()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    assert_eq!(
        asset.description.unwrap().get("en_us").map(String::as_str),
        Some("Product docs")
    );
    assert_eq!(
        answer.message.unwrap().content.as_deref(),
        Some("Use the API reference.")
    );
    assert_eq!(answer.process_data.unwrap().chunks, ["chunk"]);
    assert_eq!(
        answer.faq_result.unwrap().answer.as_deref(),
        Some("In the portal.")
    );
    assert_eq!(
        serde_json::to_value(body).unwrap()["import_knowledge_setting"]["file"]["title"],
        "Guide"
    );
}

#[test]
fn helpdesk_models_deserialize_ticket_and_notification_fields() {
    use larksuite_oapi_sdk_rs::service::helpdesk::v1;

    let ticket: v1::Ticket = serde_json::from_str(
        r#"{
            "comments": [{"content": "Investigating", "user_name": "Ada"}],
            "dissatisfaction_reason": {"en_us": "Unresolved"},
            "customized_fields": [{"id": "priority", "value": "high"}]
        }"#,
    )
    .unwrap();
    let notification: v1::Notification = serde_json::from_str(
        r#"{
            "department_list": [{"department_id": "od_1", "name": "Support"}],
            "chat_list": [{"chat_id": "oc_1", "name": "Escalations"}]
        }"#,
    )
    .unwrap();

    assert_eq!(
        ticket.comments.unwrap()[0].user_name.as_deref(),
        Some("Ada")
    );
    assert_eq!(
        ticket.dissatisfaction_reason.unwrap().en_us.as_deref(),
        Some("Unresolved")
    );
    assert_eq!(
        ticket.customized_fields.unwrap()[0].value.as_deref(),
        Some("high")
    );
    assert_eq!(
        notification.department_list.unwrap()[0].name.as_deref(),
        Some("Support")
    );
    assert_eq!(
        notification.chat_list.unwrap()[0].chat_id.as_deref(),
        Some("oc_1")
    );
}

#[test]
fn hire_models_deserialize_offer_configuration_and_salary() {
    use larksuite_oapi_sdk_rs::service::hire::v1;

    let config: v1::RecruitmentConfig = serde_json::from_str(
        r#"{
            "offer_apply_schema": {"id": "schema_1", "name": {"en_us": "Offer form"}},
            "rec_process_info": {"id": "process_1", "name": {"en_us": "Hiring"}},
            "assessment_template": {"id": "template_1", "name": {"en_us": "Interview"}}
        }"#,
    )
    .unwrap();
    let offer: v1::Offer = serde_json::from_str(
        r#"{
            "salary_plan": {"currency": "USD", "basic_salary": "100000"},
            "customize_info_list": [{"object_id": "custom_1", "customize_value": "remote"}]
        }"#,
    )
    .unwrap();

    assert_eq!(
        config
            .offer_apply_schema
            .unwrap()
            .name
            .unwrap()
            .en_us
            .as_deref(),
        Some("Offer form")
    );
    assert_eq!(
        config.rec_process_info.unwrap().id.as_deref(),
        Some("process_1")
    );
    assert_eq!(offer.salary_plan.unwrap().currency.as_deref(), Some("USD"));
    assert_eq!(
        offer.customize_info_list.unwrap()[0]
            .customize_value
            .as_deref(),
        Some("remote")
    );
}
