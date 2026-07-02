use super::prelude::*;

// ── Helpdesk ──

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
