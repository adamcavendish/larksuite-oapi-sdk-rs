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
