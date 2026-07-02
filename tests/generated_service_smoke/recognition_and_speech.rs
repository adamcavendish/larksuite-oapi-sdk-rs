use super::prelude::*;

// ── Recognition and Speech ──

#[tokio::test]
async fn recognition_and_speech_by_query_smoke() {
    let face_body = r#"{"code":0,"msg":"ok","data":{"face_list":[{"face_token":"face-1"}]}}"#;
    let ocr_body = r#"{"code":0,"msg":"ok","data":{"recognition_text":"hello"}}"#;
    let speech_body = r#"{"code":0,"msg":"ok","data":{"recognition_text":"hello world"}}"#;
    let stream_body = r#"{"code":0,"msg":"ok","data":{"stream_id":"stream-1","sequence_id":1}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, face_body),
        http_response(200, ocr_body),
        http_response(200, speech_body),
        http_response(200, stream_body),
    ])
    .await;

    let client = client_for(addr);
    let face_req = FaceRecognizeReqBody {
        image: Some(serde_json::json!({"content":"face-bytes"})),
    };
    let ocr_req = RecognizeBasicImageReqBody {
        image: Some(serde_json::json!({"content":"ocr-bytes"})),
    };
    let speech_req = RecognizeBasicSpeechReqBody {
        speech: Some(serde_json::json!({"url":"https://example.test/audio.wav"})),
        config: Some(serde_json::json!({"language":"en_us"})),
    };
    let stream_req = RecognizeSpeechStreamReqBody {
        speech: Some(serde_json::json!({"content":"chunk-1"})),
        config: Some(serde_json::json!({"stream_id":"stream-1"})),
    };

    client
        .face_detection()
        .image
        .detect_face_attributes_by_query(
            &DetectFaceAttributesQuery::new(&face_req),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .optical_char_recognition()
        .image
        .basic_recognize_by_query(
            &BasicRecognizeImageQuery::new(&ocr_req),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .speech_to_text()
        .speech
        .file_recognize_by_query(
            &FileRecognizeSpeechQuery::new(&speech_req),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .speech_to_text()
        .speech
        .stream_recognize_by_query(
            &StreamRecognizeSpeechQuery::new(&stream_req),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/face_detection/v1/image/detect_face_attributes "));
    assert!(request.contains("POST /open-apis/optical_char_recognition/v1/image/basic_recognize "));
    assert!(request.contains("POST /open-apis/speech_to_text/v1/speech/file_recognize "));
    assert!(request.contains("POST /open-apis/speech_to_text/v1/speech/stream_recognize "));
    assert!(request.contains(r#""content":"face-bytes""#));
    assert!(request.contains(r#""content":"ocr-bytes""#));
    assert!(request.contains(r#""language":"en_us""#));
    assert!(request.contains(r#""stream_id":"stream-1""#));
}
