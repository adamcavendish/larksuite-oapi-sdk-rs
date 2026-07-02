use super::prelude::*;

// ── Minutes ──

#[tokio::test]
async fn minutes_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"minutes":{"minutes_token":"min-1","topic":"Weekly review"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetMinutesMinutesQuery::new("min-1").user_id_type("open_id");
    let resp = client
        .minutes()
        .minutes
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let minutes = resp.data.unwrap().minutes.unwrap();
    assert_eq!(minutes.topic.as_deref(), Some("Weekly review"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/minutes/v1/minutes/min-1?"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn minutes_participant_list_positional_adapter_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"participants":[{"user_id":"u-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .minutes()
        .participant
        .list(
            "min-1",
            Some("open_id"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/minutes/v1/minutes/min-1/participants?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn minutes_participant_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"participants":[{"user_id":"u-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListMinutesParticipantQuery::new("min-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .minutes()
        .participant
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.participants.len(), 1);
    assert_eq!(data.participants[0].user_id.as_deref(), Some("u-1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/minutes/v1/minutes/min-1/participants?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn minutes_transcript_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"transcript":{"phrases":[{"pid":"p1"}]}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = GetMinutesTranscriptQuery::new("min-1")
        .need_speaker(true)
        .need_timestamp(true)
        .file_format("srt");
    let resp = client
        .minutes()
        .transcript
        .get_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let transcript = resp.data.unwrap().transcript.unwrap();
    assert_eq!(transcript.phrases.unwrap().len(), 1);
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/minutes/v1/minutes/min-1/transcript?"));
    assert!(request.contains("need_speaker=true"));
    assert!(request.contains("need_timestamp=true"));
    assert!(request.contains("file_format=srt"));
}

#[tokio::test]
async fn moments_post_by_query_smoke() {
    let get_body = r#"{"code":0,"msg":"ok","data":{"post":{"post_id":"post-1","user_id":"ou-1","category_id":"cat-1"}}}"#;
    let list_body = r#"{"code":0,"msg":"ok","data":{"items":[{"post_id":"post-1","user_id":"ou-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, get_body),
        http_response(200, list_body),
    ])
    .await;

    let client = client_for(addr);
    client
        .moments()
        .post
        .get_by_query(
            &GetPostQuery::new("post-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    client
        .moments()
        .post
        .list_by_query(
            &ListPostQuery::new()
                .category_id("cat-1")
                .user_id_type("open_id")
                .page(PageQuery::new().page_size(20).page_token("next-page")),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/moments/v1/posts/post-1?"));
    assert!(request.contains("GET /open-apis/moments/v1/posts?"));
    assert!(request.contains("category_id=cat-1"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}
