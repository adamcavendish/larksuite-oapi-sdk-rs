use super::prelude::*;

// ── Helpdesk ──

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
