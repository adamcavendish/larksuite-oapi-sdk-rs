use super::*;

// ── Helpdesk ──

#[tokio::test]
async fn helpdesk_get_faq_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"faq":{"id":"faq-1","answer":"Try restarting"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .get("faq-1", &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert!(resp.data.is_some());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/faq-1"));
}

#[tokio::test]
async fn helpdesk_ticket_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ticket":{"ticket_id":"ticket-1"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket
        .get_by_query(&GetTicketQuery::new("ticket-1"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.ticket.as_ref())
            .and_then(|ticket| ticket.ticket_id.as_deref()),
        Some("ticket-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/tickets/ticket-1"));
}

#[tokio::test]
async fn helpdesk_ticket_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"tickets":[{"ticket_id":"ticket-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket
        .list_by_query(
            &ListTicketQuery::new()
                .ticket_id("ticket-1")
                .agent_id("agent-1")
                .closed_by_id("agent-2")
                .status(3)
                .guest_id("guest-1")
                .keyword("refund")
                .create_time_start(1782910000)
                .create_time_end(1782913600)
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.tickets.first())
            .and_then(|ticket| ticket.ticket_id.as_deref()),
        Some("ticket-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/tickets?"));
    assert!(request.contains("ticket_id=ticket-1"));
    assert!(request.contains("agent_id=agent-1"));
    assert!(request.contains("closed_by_id=agent-2"));
    assert!(request.contains("status=3"));
    assert!(request.contains("guest_id=guest-1"));
    assert!(request.contains("keyword=refund"));
    assert!(request.contains("create_time_start=1782910000"));
    assert!(request.contains("create_time_end=1782913600"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn helpdesk_ticket_customized_fields_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"ticket_customized_fields":[{"ticket_customized_field_id":"field-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket
        .customized_fields_by_query(
            &CustomizedFieldsTicketQuery::new().visible_only(true),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/customized_fields?"));
    assert!(request.contains("visible_only=true"));
}

#[tokio::test]
async fn helpdesk_ticket_message_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"messages":[{"message_id":"msg-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket_message
        .list_by_query(
            &ListTicketMessageQuery::new("ticket-1")
                .time_start(1782910000)
                .time_end(1782913600)
                .page_token(1782910100)
                .page_size(20),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.messages.first())
            .and_then(|message| message.message_id.as_deref()),
        Some("msg-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/tickets/ticket-1/messages?"));
    assert!(request.contains("time_start=1782910000"));
    assert!(request.contains("time_end=1782913600"));
    assert!(request.contains("page_token=1782910100"));
    assert!(request.contains("page_size=20"));
}

#[tokio::test]
async fn helpdesk_agent_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"agents":[{"agent_id":"agent-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .agent
        .list_by_query(&ListAgentQuery::new().status(1), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.agents.first())
            .and_then(|agent| agent.agent_id.as_deref()),
        Some("agent-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/agents?"));
    assert!(request.contains("status=1"));
}

#[tokio::test]
async fn helpdesk_agent_schedules_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"schedules":[{"schedule_id":"schedule-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .agent_schedules
        .get_by_query(
            &GetAgentSchedulesQuery::new("agent-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/agents/agent-1/schedules"));
}

#[tokio::test]
async fn helpdesk_agent_schedule_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"schedule_id":"schedule-1"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let statuses = [1, 2];
    let resp = client
        .helpdesk()
        .agent_schedule
        .list_by_query(
            &ListAgentScheduleQuery::new().status(statuses.as_slice()),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/agent_schedules?"));
    assert!(request.contains("status=1"));
    assert!(request.contains("status=2"));
}

#[tokio::test]
async fn helpdesk_category_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"category":{"id":"cat-1","name":"Billing"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .category
        .get_by_query(&GetCategoryQuery::new("cat-1"), &RequestOption::default())
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.category.as_ref())
            .and_then(|category| category.name.as_deref()),
        Some("Billing")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/categories/cat-1"));
}

#[tokio::test]
async fn helpdesk_category_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"categories":[{"id":"cat-1","name":"Billing"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .category
        .list_by_query(
            &ListCategoryQuery::new().language("en-US"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/categories?"));
    assert!(request.contains("language=en-US"));
}

#[tokio::test]
async fn helpdesk_faq_get_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"faq":{"id":"faq-1","answer":"Try restarting"}}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .get_by_query(
            &GetHelpdeskFaqQuery::new("faq-1"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.faq.as_ref())
            .and_then(|faq| faq.id.as_deref()),
        Some("faq-1")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/faq-1"));
}

#[tokio::test]
async fn helpdesk_faq_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"faq-1","question":"How?"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .list_by_query(
            &ListFaqQuery::new()
                .category_id("cat-1")
                .keyword("restart")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs?"));
    assert!(request.contains("category_id=cat-1"));
    assert!(request.contains("keyword=restart"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn helpdesk_faq_search_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"items":[{"id":"faq-1","question":"How?"}]}}"#;
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response(200, body)]).await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .search_by_query(
            &SearchFaqQuery::new()
                .query("restart")
                .base64("cmVzdGFydA==")
                .page_size(20)
                .page_token("next-page"),
            &RequestOption::default(),
        )
        .await
        .unwrap();

    assert!(resp.success());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/search?"));
    assert!(request.contains("query=restart"));
    assert!(request.contains("base64=cmVzdGFydA%3D%3D"));
    assert!(request.contains("page_size=20"));
    assert!(request.contains("page_token=next-page"));
}

#[tokio::test]
async fn helpdesk_ticket_image_download_smoke() {
    let body = "ticket-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"ticket-image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket
        .ticket_image("ticket-1", "msg-1", Some(2), &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("ticket-image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/ticket_images?"));
    assert!(request.contains("ticket_id=ticket-1"));
    assert!(request.contains("msg_id=msg-1"));
    assert!(request.contains("index=2"));
}

#[tokio::test]
async fn helpdesk_ticket_message_image_download_smoke() {
    let body = "ticket-message-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"ticket-message-image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .ticket_message
        .image("ticket-1", "msg-1", Some(3), &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("ticket-message-image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/ticket_images?"));
    assert!(request.contains("ticket_id=ticket-1"));
    assert!(request.contains("msg_id=msg-1"));
    assert!(request.contains("index=3"));
}

#[tokio::test]
async fn helpdesk_faq_image_download_smoke() {
    let body = "faq-image-bytes";
    let (addr, _handle, requests) = mock_server_with_requests(vec![http_response_with_headers(
        200,
        "Content-Disposition: attachment; filename=\"faq-image.bin\"\r\nContent-Type: application/octet-stream\r\n",
        body,
    )])
    .await;

    let client = client_for(addr);
    let resp = client
        .helpdesk()
        .faq
        .image("faq-1", "image-key-1", &RequestOption::default())
        .await
        .unwrap();

    assert_eq!(resp.file_name.as_deref(), Some("faq-image.bin"));
    assert_eq!(resp.data, body.as_bytes());
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/faqs/faq-1/image/image-key-1"));
}
