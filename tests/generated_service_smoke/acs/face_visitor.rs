use super::prelude::*;

// ── ACS ──

#[tokio::test]
async fn acs_face_visitor_by_query_smoke() {
    let photo_body = r#"{"code":0,"msg":"ok","data":{"file_content":"photo-bytes"}}"#;
    let face_body = r#"{"code":0,"msg":"ok","data":{"file_content":"face-bytes"}}"#;
    let empty_body = r#"{"code":0,"msg":"ok"}"#;
    let visitor_body = r#"{"code":0,"msg":"ok","data":{"visitor_id":"visitor-1"}}"#;
    let delete_body = r#"{"code":0,"msg":"ok"}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![
        http_response(200, photo_body),
        http_response(200, face_body),
        http_response(200, empty_body),
        http_response(200, visitor_body),
        http_response(200, delete_body),
    ])
    .await;

    let client = client_for(addr);
    let face_update_body: UpdateUserFaceReqBody = FormDataBuilder::new()
        .file(
            "files",
            "face.jpg",
            b"face-bytes".to_vec(),
            Some("image/jpeg"),
        )
        .field("file_type", "jpg")
        .field("file_name", "face.jpg")
        .build();
    let visitor_create_body = CreateVisitorReqBody {
        user: Some(UserExternal {
            user_name: Some("Visitor".into()),
            user_id: Some("ou-1".into()),
            ..Default::default()
        }),
    };
    let photo_resp = client
        .acs()
        .access_record_access_photo
        .get_by_query(
            &GetAccessRecordAccessPhotoQuery::new("rec-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let face_resp = client
        .acs()
        .user_face
        .get_by_query(
            &GetUserFaceQuery::new("ou-1")
                .is_cropped(true)
                .user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let update_face_resp = client
        .acs()
        .user_face
        .update_by_query(
            &UpdateUserFaceQuery::new("ou-1", &face_update_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let create_visitor_resp = client
        .acs()
        .visitor
        .create_by_query(
            &CreateVisitorQuery::new(&visitor_create_body).user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();
    let delete_visitor_resp = client
        .acs()
        .visitor
        .delete_by_query(
            &DeleteVisitorQuery::new("visitor-1").user_id_type("open_id"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(photo_resp.success());
    assert!(face_resp.success());
    assert!(update_face_resp.success());
    assert!(create_visitor_resp.success());
    assert!(delete_visitor_resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/acs/v1/access_records/rec-1/access_photo"));
    assert!(request.contains("GET /open-apis/acs/v1/users/ou-1/face?"));
    assert!(request.contains("PUT /open-apis/acs/v1/users/ou-1/face?"));
    assert!(request.contains("POST /open-apis/acs/v1/visitors?"));
    assert!(request.contains("DELETE /open-apis/acs/v1/visitors/visitor-1?"));
    assert!(request.contains("is_cropped=true"));
    assert!(request.contains("user_id_type=open_id"));
    assert!(request.contains("name=\"file_type\""));
    assert!(request.contains("name=\"file_name\""));
    assert!(request.contains("face-bytes"));
    assert!(request.contains(r#""user_name":"Visitor""#));
}
