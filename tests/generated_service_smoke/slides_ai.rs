use super::prelude::*;
use larksuite_oapi_sdk_rs::service::slides_ai::v1::{
    AddSlideQuery, DeleteSlideQuery, GetSlideQuery, GetXmlPresentationHistoryRevertStatusQuery,
    GetXmlPresentationQuery, ListXmlPresentationHistoryQuery, ReplaceSlideQuery,
};

// ── Slides AI ──

#[tokio::test]
async fn slides_ai_core_contract_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
        http_response(200, body),
    ])
    .await;

    let client = client_for(addr);
    let user_option = RequestOption {
        user_access_token: Some("user-token".to_string()),
        ..RequestOption::default()
    };
    let tenant_option = RequestOption {
        tenant_access_token: Some("tenant-token".to_string()),
        ..RequestOption::default()
    };
    let presentation_id = "pres /A";

    let create = client
        .slides_ai()
        .presentation
        .create(
            json_value!({"xml_presentation": {"content": "<presentation/>"}}),
            &user_option,
        )
        .await
        .unwrap();
    let presentation = client
        .slides_ai()
        .presentation
        .get(
            &GetXmlPresentationQuery::new(presentation_id)
                .revision_id(7)
                .remove_attr_id(true),
            &tenant_option,
        )
        .await
        .unwrap();
    let slide = client
        .slides_ai()
        .slide
        .get(
            &GetSlideQuery::by_number(presentation_id, 2).revision_id(8),
            &user_option,
        )
        .await
        .unwrap();
    let slide_by_id = client
        .slides_ai()
        .slide
        .get(
            &GetSlideQuery::by_id(presentation_id, "slide /1").revision_id(11),
            &tenant_option,
        )
        .await
        .unwrap();
    let add = client
        .slides_ai()
        .slide
        .add(
            &AddSlideQuery::new(presentation_id),
            json_value!({"slide": {"content": "<slide id=\"new\"/>"}}),
            &tenant_option,
        )
        .await
        .unwrap();
    let delete = client
        .slides_ai()
        .slide
        .delete(
            &DeleteSlideQuery::new(presentation_id, "slide /1").revision_id(9),
            &user_option,
        )
        .await
        .unwrap();
    let replace = client
        .slides_ai()
        .slide
        .replace(
            &ReplaceSlideQuery::new(presentation_id, "slide /1")
                .revision_id(10)
                .tid("tid /1"),
            json_value!({
                "parts": [{
                    "action": "block_replace",
                    "block_id": "slide /1",
                    "replacement": "<slide id=\"slide /1\"/>",
                }],
            }),
            &tenant_option,
        )
        .await
        .unwrap();
    let history = client
        .slides_ai()
        .history
        .list(
            &ListXmlPresentationHistoryQuery::new(presentation_id)
                .page(PageQuery::new().page_size(20).page_token("next page")),
            &user_option,
        )
        .await
        .unwrap();
    let revert = client
        .slides_ai()
        .history
        .revert(
            presentation_id,
            json_value!({"history_version_id": "42"}),
            &tenant_option,
        )
        .await
        .unwrap();
    let status = client
        .slides_ai()
        .history
        .revert_status(
            &GetXmlPresentationHistoryRevertStatusQuery::new(presentation_id, "task id"),
            &user_option,
        )
        .await
        .unwrap();

    assert!(create.success());
    assert!(presentation.success());
    assert!(slide.success());
    assert!(slide_by_id.success());
    assert!(add.success());
    assert!(delete.success());
    assert!(replace.success());
    assert!(history.success());
    assert!(revert.success());
    assert!(status.success());

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/slides_ai/v1/xml_presentations "));
    assert!(request.contains("GET /open-apis/slides_ai/v1/xml_presentations/pres%20%2FA?"));
    assert!(request.contains("GET /open-apis/slides_ai/v1/xml_presentations/pres%20%2FA/slide?"));
    assert!(request.contains("POST /open-apis/slides_ai/v1/xml_presentations/pres%20%2FA/slide?"));
    assert!(
        request.contains("DELETE /open-apis/slides_ai/v1/xml_presentations/pres%20%2FA/slide?")
    );
    assert!(
        request
            .contains("POST /open-apis/slides_ai/v1/xml_presentations/pres%20%2FA/slide/replace?")
    );
    assert!(
        request.contains("GET /open-apis/slides_ai/v1/xml_presentations/pres%20%2FA/histories?")
    );
    assert!(
        request
            .contains("POST /open-apis/slides_ai/v1/xml_presentations/pres%20%2FA/history/revert ")
    );
    assert!(request.contains(
        "GET /open-apis/slides_ai/v1/xml_presentations/pres%20%2FA/history/revert_status?"
    ));
    assert!(request.contains("revision_id=7"));
    assert!(request.contains("remove_attr_id=true"));
    assert!(request.contains("slide_number=2"));
    assert!(request.contains("revision_id=8"));
    assert!(request.contains("revision_id=11"));
    assert!(request.contains("revision_id=-1"));
    assert!(request.contains("slide_id=slide+%2F1"));
    assert!(request.contains("revision_id=9"));
    assert!(request.contains("revision_id=10"));
    assert!(request.contains("tid=tid+%2F1"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next+page"));
    assert!(request.contains("task_id=task+id"));
    assert!(request.contains("authorization: Bearer user-token"));
    assert!(request.contains("authorization: Bearer tenant-token"));
    assert!(request.contains(r#""xml_presentation":{"content":"<presentation/>"}"#));
    assert!(request.contains(r#""block_replace"#));
    assert!(request.contains(r#""history_version_id":"42"#));
}
