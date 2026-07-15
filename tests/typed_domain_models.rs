use larksuite_oapi_sdk_rs::service::{
    aily::v1 as aily_v1, board::v1 as board_v1, document_ai::v1, docx::v1 as docx_v1,
};

#[test]
fn board_models_deserialize_full_typed_node_graph() {
    let response: board_v1::WhiteboardNodeListData = serde_json::from_str(
        r##"{
            "nodes": [{
                "id": "node_1",
                "type": "connector",
                "text": {
                    "text": "Roadmap",
                    "rich_text": {"paragraphs": [{"elements": [{
                        "element_type": 1,
                        "text_element": {"text": "Milestone", "text_style": {"font_size": 14}}
                    }]}]}
                },
                "style": {
                    "fill_gradient": {"type": "linear", "handle_positions": [{"x": 0.0, "y": 1.0}], "stops": [{"position": 0.5, "color": "#00ff00"}]},
                    "border_radius": {"top_left": 8}
                },
                "connector": {"turning_points": [{"x": 1.0, "y": 2.0}], "start": {"position": {"x": 1.0, "y": 2.0}}},
                "table": {"meta": {"row_num": 1, "col_num": 1}, "cells": [{"row_index": 1, "col_index": 1, "text": {"text": "Cell"}}]},
                "mind_map_node": {"type": "topic", "children": ["node_2"]},
                "syntax": {"syntax_type": 1, "code": "@startuml"}
            }]
        }"##,
    )
    .unwrap();
    let node = &response.nodes[0];

    assert_eq!(node.r#type.as_deref(), Some("connector"));
    assert_eq!(
        node.text
            .as_ref()
            .unwrap()
            .rich_text
            .as_ref()
            .unwrap()
            .paragraphs[0]
            .elements[0]
            .text_element
            .as_ref()
            .unwrap()
            .text
            .as_deref(),
        Some("Milestone")
    );
    assert_eq!(
        node.style
            .as_ref()
            .unwrap()
            .fill_gradient
            .as_ref()
            .unwrap()
            .stops[0]
            .color
            .as_deref(),
        Some("#00ff00")
    );
    assert_eq!(
        node.connector.as_ref().unwrap().turning_points[0].x,
        Some(1.0)
    );
    assert_eq!(node.table.as_ref().unwrap().cells[0].row_index, Some(1));

    let body = board_v1::CreateWhiteboardNodeReqBody {
        nodes: vec![node.clone()],
        overwrite: Some(true),
    };
    assert_eq!(
        serde_json::to_value(body).unwrap()["nodes"][0]["type"],
        serde_json::json!("connector")
    );
}

#[test]
fn acs_models_deserialize_typed_rule_and_user_graphs() {
    use larksuite_oapi_sdk_rs::service::acs::v1 as acs_v1;

    let rules: acs_v1::GetRuleExternalData = serde_json::from_str(
        r#"{
            "rules": [{
                "id": "rule_1",
                "devices": [{"id": "device_1", "name": "Front door"}],
                "users": [{"user_id": "ou_1", "user_name": "Ada"}],
                "opening_time": {"valid_day": {"start_day": 20260715}, "day_times": [{"start_hhmm": 900, "end_hhmm": 1800}]}
            }]
        }"#,
    )
    .unwrap();
    let record: acs_v1::AccessRecord =
        serde_json::from_str(r#"{"access_type":"face","is_door_open":true}"#).unwrap();

    assert_eq!(
        rules.rules[0].devices[0].name.as_deref(),
        Some("Front door")
    );
    assert_eq!(
        rules.rules[0].opening_time.as_ref().unwrap().day_times[0].end_hhmm,
        Some(1800)
    );
    assert_eq!(record.access_type.as_deref(), Some("face"));
}

#[test]
fn board_and_acs_model_defaults_serialize() {
    use larksuite_oapi_sdk_rs::service::acs::v1 as acs_v1;

    macro_rules! assert_serializes {
        ($($value:expr),+ $(,)?) => {{
            $(assert!(serde_json::to_value($value).is_ok());)+
        }};
    }

    assert_serializes!(
        board_v1::AuthInfo::default(),
        board_v1::BorderRadius::default(),
        board_v1::ClientInfo::default(),
        board_v1::CompositeShape::default(),
        board_v1::Connector::default(),
        board_v1::ConnectorAttachedObject::default(),
        board_v1::ConnectorCaption::default(),
        board_v1::ConnectorInfo::default(),
        board_v1::Cube::default(),
        board_v1::DepartmentId::default(),
        board_v1::FillGradient::default(),
        board_v1::GradientStop::default(),
        board_v1::Head::default(),
        board_v1::Image::default(),
        board_v1::Lifeline::default(),
        board_v1::MindMap::default(),
        board_v1::MindMapNode::default(),
        board_v1::MindMapRoot::default(),
        board_v1::Paint::default(),
        board_v1::Pie::default(),
        board_v1::Point::default(),
        board_v1::RichText::default(),
        board_v1::RichTextElement::default(),
        board_v1::RichTextElementLink::default(),
        board_v1::RichTextElementMentionDoc::default(),
        board_v1::RichTextElementMentionUser::default(),
        board_v1::RichTextElementText::default(),
        board_v1::RichTextElementTextStyle::default(),
        board_v1::RichTextParagraph::default(),
        board_v1::Section::default(),
        board_v1::Shadow::default(),
        board_v1::StickyNote::default(),
        board_v1::Style::default(),
        board_v1::Svg::default(),
        board_v1::Syntax::default(),
        board_v1::Table::default(),
        board_v1::TableCell::default(),
        board_v1::TableCellMergeInfo::default(),
        board_v1::TableMeta::default(),
        board_v1::Text::default(),
        board_v1::Trapezoid::default(),
        board_v1::WhiteboardNode::default(),
        board_v1::BatchDeleteWhiteboardNodeReqBody::default(),
        board_v1::CreatePlantumlWhiteboardNodeReqBody::default(),
        acs_v1::Feature::default(),
        acs_v1::Property::default(),
        acs_v1::DeviceExternal::default(),
        acs_v1::UserExternal::default(),
        acs_v1::OpeningTimePeriodExternal::default(),
        acs_v1::OpeningTimeValidDayExternal::default(),
        acs_v1::OpeningTimeExternal::default(),
        acs_v1::Rule::default(),
        acs_v1::User::default(),
        acs_v1::CreateRuleExternalReqBody::default(),
        acs_v1::DeviceBindRuleExternalReqBody::default(),
        acs_v1::CreateVisitorReqBody::default(),
        acs_v1::CreateRuleExternalData::default(),
    );
}

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

#[test]
fn corehr_models_deserialize_contact_hierarchy_and_permission_fields() {
    use larksuite_oapi_sdk_rs::service::corehr::v1;

    let employee: v1::Employee = serde_json::from_str(
        r#"{
            "phone_list": [{"phone_number": "123456", "is_primary": true}],
            "address_list": [{"address_line1": "1 Main Street"}],
            "email_list": [{"email": "ada@example.test"}],
            "cost_center_rate": [{"cost_center_id": "cc_1", "rate": 100}]
        }"#,
    )
    .unwrap();
    let department: v1::Department =
        serde_json::from_str(r#"{"hiberarchy_common": {"parent_id": "od_root"}}"#).unwrap();
    let permission: v1::PermissionDetail = serde_json::from_str(
        r#"{
            "assigned_organization_list": [[{
                "org_key": "department",
                "org_name": {"en_us": "Engineering"},
                "org_id_list": ["od_1"]
            }]]
        }"#,
    )
    .unwrap();
    let field: v1::CustomFieldData =
        serde_json::from_str(r#"{"name": {"en_us": "Work mode"}}"#).unwrap();

    assert_eq!(
        employee.phone_list.unwrap()[0].phone_number.as_deref(),
        Some("123456")
    );
    assert_eq!(
        employee.email_list.unwrap()[0].email.as_deref(),
        Some("ada@example.test")
    );
    assert_eq!(
        employee.cost_center_rate.unwrap()[0]
            .cost_center_id
            .as_deref(),
        Some("cc_1")
    );
    assert_eq!(
        department.hiberarchy_common.unwrap().parent_id.as_deref(),
        Some("od_root")
    );
    assert_eq!(
        permission.assigned_organization_list[0][0]
            .org_name
            .as_ref()
            .unwrap()
            .en_us
            .as_deref(),
        Some("Engineering")
    );
    assert_eq!(field.name.unwrap().en_us.as_deref(), Some("Work mode"));
}

#[test]
fn lingo_models_deserialize_entity_metadata_and_search_filter() {
    use larksuite_oapi_sdk_rs::service::lingo::v1;

    let entity: v1::LingoEntity = serde_json::from_str(
        r#"{
            "related_meta": {
                "users": [{"id": "ou_1", "title": "Ada"}],
                "classifications": [{"id": "cls_1"}]
            },
            "statistics": {"like_count": 3},
            "outer_info": {"provider": "catalog", "outer_id": "external_1"},
            "display_status": {"allow_search": true},
            "classification": [{"id": "cls_1"}],
            "images": [{"token": "img_1"}]
        }"#,
    )
    .unwrap();
    let body = v1::SearchLingoEntityReqBody {
        classification_filter: Some(v1::ClassificationFilter {
            include: vec!["cls_1".into()],
            exclude: vec!["cls_2".into()],
        }),
        ..Default::default()
    };

    assert_eq!(
        entity.related_meta.unwrap().users[0].title.as_deref(),
        Some("Ada")
    );
    assert_eq!(entity.statistics.unwrap().like_count, Some(3));
    assert_eq!(
        entity.outer_info.unwrap().outer_id.as_deref(),
        Some("external_1")
    );
    assert_eq!(entity.display_status.unwrap().allow_search, Some(true));
    assert_eq!(
        entity.classification.unwrap()[0].id.as_deref(),
        Some("cls_1")
    );
    assert_eq!(entity.images.unwrap()[0].token.as_deref(), Some("img_1"));
    assert_eq!(
        serde_json::to_value(body).unwrap()["classification_filter"]["include"],
        serde_json::json!(["cls_1"])
    );
}

#[test]
fn application_v7_patch_models_serialize_typed_configuration() {
    use larksuite_oapi_sdk_rs::service::application::v7;

    let body = v7::PatchApplicationConfigReqBody::new()
        .scope(v7::AppConfigScope {
            add_scopes: vec![v7::AppConfigScopeItem {
                scope_name: Some("im:message".into()),
                token_type: Some("tenant".into()),
            }],
            ..Default::default()
        })
        .visibility(v7::AppConfigVisibility {
            is_visible_to_all: Some(false),
            visible_list: Some(v7::AppVisibilityIdList {
                user_ids: vec!["ou_1".into()],
                ..Default::default()
            }),
        })
        .callback(v7::AppConfigCallback {
            request_url: Some("https://example.test/callback".into()),
            ..Default::default()
        });

    let value = serde_json::to_value(body).unwrap();
    assert_eq!(value["scope"]["add_scopes"][0]["scope_name"], "im:message");
    assert_eq!(
        value["visibility"]["visible_list"]["user_ids"],
        serde_json::json!(["ou_1"])
    );
    assert_eq!(
        value["callback"]["request_url"],
        "https://example.test/callback"
    );
    assert!(value["callback"].get("add_callbacks").is_none());
}

#[test]
fn baike_and_attendance_models_serialize_typed_nested_values() {
    use larksuite_oapi_sdk_rs::service::{attendance::v1 as attendance_v1, baike::v1 as baike_v1};

    let entity: baike_v1::BaikeEntity = serde_json::from_str(
        r#"{
            "related_meta": {
                "users": [{"id": "ou_1", "title": "Ada"}],
                "classifications": [{"id": "cls_1"}],
                "images": [{"token": "img_1"}]
            }
        }"#,
    )
    .unwrap();
    let report = attendance_v1::UploadReportArchiveRuleReqBody {
        archive_report_datas: Some(vec![attendance_v1::ArchiveReportData {
            member_id: Some("ou_1".into()),
            field_datas: Some(vec![attendance_v1::ArchiveFieldData {
                code: Some("work_hours".into()),
                value: Some("8".into()),
            }]),
            ..Default::default()
        }]),
        ..Default::default()
    };

    assert_eq!(
        entity.related_meta.unwrap().users.unwrap()[0]
            .title
            .as_deref(),
        Some("Ada")
    );
    assert_eq!(
        serde_json::to_value(report).unwrap()["archive_report_datas"][0]["field_datas"][0]["code"],
        "work_hours"
    );
}

#[test]
fn docx_vc_and_task_models_use_existing_typed_graphs() {
    use larksuite_oapi_sdk_rs::service::{
        docx::v1 as docx_v1, task::v1 as task_v1, vc::v1 as vc_v1,
    };

    let update = docx_v1::UpdateBlockReqBody {
        update_text_elements: Some(docx_v1::UpdateTextElementsRequest {
            elements: vec![docx_v1::TextElement {
                text_run: Some(docx_v1::TextRun {
                    content: Some("updated".into()),
                    ..Default::default()
                }),
                ..Default::default()
            }],
        }),
        merge_table_cells: Some(docx_v1::MergeTableCellsRequest {
            row_start_index: Some(0),
            row_end_index: Some(2),
            column_start_index: Some(0),
            column_end_index: Some(2),
        }),
        ..Default::default()
    };
    let config = vc_v1::SetRoomConfigReqBody {
        digital_signage: Some(vc_v1::RoomDigitalSignage {
            enable: Some(true),
            ..Default::default()
        }),
        ..Default::default()
    };
    let task = task_v1::PatchTaskReqBody {
        task: Some(task_v1::Task {
            summary: Some("Follow up".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    assert_eq!(
        serde_json::to_value(update).unwrap()["update_text_elements"]["elements"][0]["text_run"]["content"],
        "updated"
    );
    assert_eq!(config.digital_signage.unwrap().enable, Some(true));
    assert_eq!(
        serde_json::to_value(task).unwrap()["task"]["summary"],
        "Follow up"
    );
}
