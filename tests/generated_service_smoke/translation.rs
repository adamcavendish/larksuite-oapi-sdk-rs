use super::prelude::*;

// ── Translation ──

#[tokio::test]
async fn translation_text_by_query_smoke() {
    let translate_body = r#"{"code":0,"msg":"ok","data":{"text":"ni hao"}}"#;
    let detect_body = r#"{"code":0,"msg":"ok","data":{"language":"fr"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, translate_body),
        http_response(200, detect_body),
    ])
    .await;

    let client = client_for(addr);
    let translate_body = TranslateReqBody {
        source_language: Some("en".into()),
        target_language: Some("zh".into()),
        text: Some("hello".into()),
        glossary: Some(vec![
            serde_json::json!({"source":"hello","target":"ni hao"}),
        ]),
    };
    let detect_body = DetectLanguageReqBody {
        text: Some("bonjour".into()),
    };

    client
        .translation()
        .text
        .translate_by_query(
            &TranslateTextQuery::new(&translate_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .translation()
        .text
        .detect_language_by_query(
            &DetectLanguageQuery::new(&detect_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/translation/v1/text/translate "));
    assert!(request.contains("POST /open-apis/translation/v1/text/detect "));
    assert!(request.contains(r#""source_language":"en""#));
    assert!(request.contains(r#""target_language":"zh""#));
    assert!(request.contains(r#""text":"hello""#));
    assert!(request.contains(r#""text":"bonjour""#));
}

#[tokio::test]
async fn translation_detect_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"language":"en"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let req_body = DetectLanguageReqBody {
        text: Some("hello".into()),
    };
    let resp = client
        .translation()
        .text
        .detect_language(&req_body, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.language.as_deref(), Some("en"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/translation/v1/text/detect"));
}
