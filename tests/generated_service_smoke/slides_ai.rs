use super::prelude::*;
use larksuite_oapi_sdk_rs::LarkError;
use larksuite_oapi_sdk_rs::service::slides_ai::v1::{
    AddSlideQuery, DeleteSlideQuery, GetSlideImagesRequest, GetSlideQuery,
    GetXmlPresentationHistoryRevertStatusQuery, GetXmlPresentationQuery,
    ListXmlPresentationHistoryQuery, ReplaceSlideQuery, SlideImage, XmlLintBody,
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
            XmlLintBody::new(json_value!({
                "lint_xml": false,
                "slide": {"content": "<slide id=\"new\"/>"},
            }))
            .unwrap(),
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
            XmlLintBody::without_lint(json_value!({
                "parts": [{
                    "action": "block_replace",
                    "block_id": "slide /1",
                    "replacement": "<slide id=\"slide /1\"/>",
                }],
            }))
            .unwrap(),
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
    assert!(request.contains(r#""lint_xml":true"#));
    assert!(request.contains(r#""lint_xml":false"#));
    assert!(!request.contains("lint_xml=true"));
    assert!(!request.contains("lint_xml=false"));
}

#[test]
fn xml_lint_body_requires_an_object_and_owns_the_reserved_field() {
    let body = XmlLintBody::new(json_value!({"lint_xml": false, "parts": []})).unwrap();
    assert_eq!(serde_json::to_value(body).unwrap()["lint_xml"], true);

    let unlinted = XmlLintBody::without_lint(json_value!({"parts": []})).unwrap();
    assert_eq!(serde_json::to_value(unlinted).unwrap()["lint_xml"], false);

    let error = XmlLintBody::new(json_value!([])).unwrap_err();
    assert!(matches!(error, LarkError::IllegalParam(_)));
}

#[tokio::test]
async fn slides_ai_image_contract_smoke() {
    let selected_body = r#"{"code":0,"msg":"ok","data":{"slide_images":[{"slide_id":"slide /1","slide_number":1,"format":"jpeg","data":"c2VsZWN0ZWQ="}]}}"#;
    let rendered_body =
        r#"{"code":0,"msg":"ok","data":{"slide_image":{"format":"png","data":"cmVuZGVyZWQ="}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, selected_body),
        http_response(200, selected_body),
        http_response(200, rendered_body),
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
    let slide_numbers = [1, 2];
    let slide_ids = ["slide /1"];

    let selected_by_number = client
        .slides_ai()
        .image
        .get(
            &GetSlideImagesRequest::by_numbers(presentation_id, &slide_numbers),
            &user_option,
        )
        .await
        .unwrap();
    let selected_by_id = client
        .slides_ai()
        .image
        .get(
            &GetSlideImagesRequest::by_ids(presentation_id, &slide_ids),
            &tenant_option,
        )
        .await
        .unwrap();
    let rendered = client
        .slides_ai()
        .image
        .render("<slide id=\"render\"/>", &user_option)
        .await
        .unwrap();

    assert!(selected_by_number.success());
    assert!(selected_by_id.success());
    assert!(rendered.success());
    assert_eq!(
        selected_by_number.data.unwrap().slide_images[0]
            .decode()
            .unwrap(),
        b"selected"
    );
    assert_eq!(
        rendered
            .data
            .unwrap()
            .slide_image
            .unwrap()
            .decode()
            .unwrap(),
        b"rendered"
    );

    let empty_ids: [&str; 0] = [];
    let err = client
        .slides_ai()
        .image
        .get(
            &GetSlideImagesRequest::by_ids(presentation_id, &empty_ids),
            &user_option,
        )
        .await
        .unwrap_err();
    assert!(err.to_string().contains("requires at least one"));

    let too_many_ids = ["slide"; 11];
    let err = client
        .slides_ai()
        .image
        .get(
            &GetSlideImagesRequest::by_ids(presentation_id, &too_many_ids),
            &user_option,
        )
        .await
        .unwrap_err();
    assert!(err.to_string().contains("at most 10"));

    let err = client
        .slides_ai()
        .image
        .render("  ", &tenant_option)
        .await
        .unwrap_err();
    assert!(err.to_string().contains("content cannot be empty"));

    let invalid_image: SlideImage = serde_json::from_str(r#"{"data":"not-base64"}"#).unwrap();
    assert!(invalid_image.decode().is_err());

    let request = requests.lock().unwrap().join("\n");
    assert!(
        request
            .contains("POST /open-apis/slides_ai/v1/xml_presentations/pres%20%2FA/slide_images ")
    );
    assert!(request.contains("POST /open-apis/slides_ai/v1/slide_image/render "));
    assert!(request.contains("authorization: Bearer user-token"));
    assert!(request.contains("authorization: Bearer tenant-token"));
    assert!(request.contains(r#""slide_numbers":[1,2]"#));
    assert!(request.contains(r#""slide_ids":["slide /1"]"#));
    assert!(request.contains(r#""content":"<slide id=\"render\"/>"#));
}
