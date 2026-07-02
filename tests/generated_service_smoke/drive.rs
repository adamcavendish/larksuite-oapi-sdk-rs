use super::prelude::*;

// ── Drive ──

#[tokio::test]
async fn drive_get_export_task_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"result":{"token":"t-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .export_task
        .get("ticket-1", "file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().result.unwrap().token.as_deref(),
        Some("t-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/export_tasks/ticket-1?token=file-token-1"));
}

#[tokio::test]
async fn drive_export_task_download_smoke() {
    let body = "export-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"export.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .export_task
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("export.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/export_tasks/file/file-token-1/download"));
}

#[tokio::test]
async fn drive_list_files_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[{"token":"file-1","name":"doc.pdf","type":"file"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileQuery::new()
        .folder_token("folder-1")
        .order_by("EditedTime")
        .direction("DESC")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive()
        .file
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.files.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.files.as_ref().unwrap()[0].name.as_deref(),
        Some("doc.pdf")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files?"));
    assert!(request.contains("folder_token=folder-1"));
    assert!(request.contains("order_by=EditedTime"));
    assert!(request.contains("direction=DESC"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_list_files_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[{"token":"file-1","name":"doc.pdf","type":"file"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .list(
            None,
            None,
            None,
            None,
            None,
            None,
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.files.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.files.as_ref().unwrap()[0].name.as_deref(),
        Some("doc.pdf")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files"));
}

#[tokio::test]
async fn drive_file_comment_batch_query_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"comment_id":"comment-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let body = BatchQueryFileCommentReqBody {
        comment_ids: Some(vec!["comment-1".to_string()]),
    };
    let resp = client
        .drive()
        .file_comment
        .batch_query_by_query(
            &BatchQueryFileCommentQuery::new("file-token-1", "doc", &body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/files/file-token-1/comments/batch_query?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains(r#""comment_ids":["comment-1"]"#));
}

#[tokio::test]
async fn drive_file_comment_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"comment_id":"comment-1","user_id":"ou-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file_comment
        .get_by_query(
            &GetFileCommentQuery::new("file-token-1", "comment-1", "doc").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|comment| comment.comment_id.as_deref()),
        Some("comment-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/comments/comment-1?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn drive_file_comment_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"comment_id":"comment-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file_comment
        .list_by_query(
            &ListFileCommentQuery::new("file-token-1", "doc")
                .is_whole(true)
                .is_solved(false)
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/comments?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("is_whole=true"));
    assert!(request.contains("is_solved=false"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn drive_file_comment_reply_list_by_query_smoke() {
    let body =
        r#"{"code":0,"msg":"ok","data":{"items":[{"reply_id":"reply-1"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file_comment_reply
        .list_by_query(
            &ListFileCommentReplyQuery::new("file-token-1", "comment-1", "doc")
                .page(PageQuery::new().page_size(20).page_token("next-page"))
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(
        request.contains("GET /open-apis/drive/v1/files/file-token-1/comments/comment-1/replies?")
    );
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
    assert!(request.contains("user_id_type=open_id"));
}

#[tokio::test]
async fn drive_permission_member_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"member_id":"ou-1","member_type":"user","perm":"view"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .permission_member
        .list_by_query(
            &ListPermissionMemberQuery::new("file-token-1", "doc")
                .fields("member_id,perm")
                .perm_type("container"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.items.as_ref())
            .and_then(|items| items.first())
            .and_then(|member| member.member_id.as_deref()),
        Some("ou-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/permissions/file-token-1/members?"));
    assert!(request.contains("type=doc"));
    assert!(request.contains("fields=member_id%2Cperm"));
    assert!(request.contains("perm_type=container"));
}

#[tokio::test]
async fn drive_permission_public_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"permission_public":{"external_access":true,"share_entity":"tenant_readable"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .permission_public
        .get_by_query(
            &GetDrivePermissionPublicQuery::new("file-token-1", "doc"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.permission_public.as_ref())
            .and_then(|permission| permission.external_access),
        Some(true)
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/permissions/file-token-1/public?"));
    assert!(request.contains("type=doc"));
}

#[tokio::test]
async fn drive_v2_permission_public_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"external_access_entity":"anyone","share_entity":"tenant_readable"}}"#;
    let patch_body =
        r#"{"code":0,"msg":"ok","data":{"external_access_entity":"tenant","lock_switch":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, body),
        http_response(200, patch_body),
    ])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive_v2()
        .permission_public
        .get_by_query(
            &GetDrivePermissionPublicV2Query::new("file-token-1", "doc"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|permission| permission.external_access_entity.as_deref()),
        Some("anyone")
    );
    let update_body = PatchPermissionPublicV2ReqBody {
        external_access_entity: Some("tenant".into()),
        lock_switch: Some(true),
        ..Default::default()
    };
    let patch_resp = client
        .drive_v2()
        .permission_public
        .patch_by_query(
            &PatchDrivePermissionPublicV2Query::new("file-token-1", "doc", &update_body),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(patch_resp.success());
    assert_eq!(
        patch_resp
            .data
            .as_ref()
            .and_then(|permission| permission.lock_switch),
        Some(true)
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v2/permissions/file-token-1/public?"));
    assert!(request.contains("PATCH /open-apis/drive/v2/permissions/file-token-1/public?"));
    assert!(request.contains("type=doc"));
    assert!(request.contains(r#""external_access_entity":"tenant""#));
    assert!(request.contains(r#""lock_switch":true"#));
}

#[tokio::test]
async fn drive_file_download_smoke() {
    let body = "file-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"file.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .download("file-token-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("file.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/download"));
}

#[tokio::test]
async fn drive_file_upload_all_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_token":"file-token-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .upload_all(
            "report.txt",
            "explorer",
            "folder-token-1",
            3,
            Some("checksum-1"),
            b"abc".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().file_token.as_deref(),
        Some("file-token-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/files/upload_all"));
    assert!(request.contains("name=\"file_name\""));
    assert!(request.contains("report.txt"));
    assert!(request.contains("name=\"parent_type\""));
    assert!(request.contains("explorer"));
    assert!(request.contains("name=\"parent_node\""));
    assert!(request.contains("folder-token-1"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n3\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"report.txt\""));
    assert!(request.contains("abc"));
}

#[tokio::test]
async fn drive_file_upload_part_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .file
        .upload_part(
            "upload-id-1",
            2,
            3,
            Some("checksum-1"),
            b"abc".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/files/upload_part"));
    assert!(request.contains("name=\"upload_id\""));
    assert!(request.contains("upload-id-1"));
    assert!(request.contains("name=\"seq\""));
    assert!(request.contains("\r\n2\r\n"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n3\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"part\""));
    assert!(request.contains("abc"));
}

#[tokio::test]
async fn event_outbound_ip_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ip_list":["1.1.1.1","2.2.2.2"]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .event()
        .outbound_ip
        .list_by_query(&ListOutboundIpQuery::new(), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.ip_list.as_ref())
            .and_then(|ips| ips.first())
            .map(String::as_str),
        Some("1.1.1.1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/event/v1/outbound_ip "));
}

#[tokio::test]
async fn drive_v2_file_like_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[{"token":"like-1","name":"Alice","type":"user"}],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileLikeQuery::new("file-token-1")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive_v2()
        .file_like
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.files.len(), 1);
    assert_eq!(data.files[0].name.as_deref(), Some("Alice"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v2/files/file-token-1/likes?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_v2_file_like_positional_adapter_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"files":[],"has_more":false}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive_v2()
        .file_like
        .list(
            "file-token-1",
            Some("open_id"),
            Some(20),
            Some("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v2/files/file-token-1/likes?"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_file_version_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"name":"v1"}],"page_token":"next-version","has_more":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileVersionQuery::new("file-token-1", "doc")
        .user_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive()
        .file_version
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(data.items.as_ref().unwrap()[0].name.as_deref(), Some("v1"));
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/versions?"));
    assert!(request.contains("obj_type=doc"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_file_view_record_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"viewer_id":"u1"}],"page_token":"next-view","has_more":true}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let query = ListFileViewRecordQuery::new("file-token-1", "doc")
        .viewer_id_type("open_id")
        .page(PageQuery::new().page_size(20).page_token("next-page"));
    let resp = client
        .drive()
        .file_view_record
        .list_by_query(&query, &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    let data = resp.data.unwrap();
    assert_eq!(data.items.as_ref().unwrap().len(), 1);
    assert_eq!(
        data.items.as_ref().unwrap()[0].viewer_id.as_deref(),
        Some("u1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/files/file-token-1/view_records?"));
    assert!(request.contains("file_type=doc"));
    assert!(request.contains("viewer_id_type=open_id"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn drive_media_download_smoke() {
    let body = "media-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"media.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .media
        .download(
            "media-token-1",
            Some("extra-value"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("media.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/drive/v1/medias/media-token-1/download?"));
    assert!(request.contains("extra=extra-value"));
}

#[tokio::test]
async fn drive_media_upload_all_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"file_token":"media-token-1"}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .media
        .upload_all(
            "clip.mp4",
            "explorer",
            "folder-token-1",
            4,
            Some("checksum-1"),
            Some("extra-value"),
            b"clip".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data.unwrap().file_token.as_deref(),
        Some("media-token-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/medias/upload_all"));
    assert!(request.contains("name=\"file_name\""));
    assert!(request.contains("clip.mp4"));
    assert!(request.contains("name=\"parent_type\""));
    assert!(request.contains("explorer"));
    assert!(request.contains("name=\"parent_node\""));
    assert!(request.contains("folder-token-1"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n4\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"extra\""));
    assert!(request.contains("extra-value"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"clip.mp4\""));
    assert!(request.contains("clip"));
}

#[tokio::test]
async fn drive_media_upload_part_smoke() {
    let body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .drive()
        .media
        .upload_part(
            "upload-id-1",
            2,
            4,
            Some("checksum-1"),
            b"clip".to_vec(),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("POST /open-apis/drive/v1/medias/upload_part"));
    assert!(request.contains("name=\"upload_id\""));
    assert!(request.contains("upload-id-1"));
    assert!(request.contains("name=\"seq\""));
    assert!(request.contains("\r\n2\r\n"));
    assert!(request.contains("name=\"size\""));
    assert!(request.contains("\r\n4\r\n"));
    assert!(request.contains("name=\"checksum\""));
    assert!(request.contains("checksum-1"));
    assert!(request.contains("name=\"file\""));
    assert!(request.contains("filename=\"part\""));
    assert!(request.contains("clip"));
}
