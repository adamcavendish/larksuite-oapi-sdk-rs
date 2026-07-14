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
    let body = r#"{"code":0,"msg":"ok","data":{"agent_schedule":{"status":1,"agent":{"id":"agent-1","name":"Alice"},"schedule":[{"start_time":"09:00","end_time":"18:00","weekday":1}],"agent_skills":[{"id":"skill-1","name":"Support","is_default":true}]}}}"#;
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
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.agent_schedule.as_ref())
            .and_then(|schedule| schedule.agent.as_ref())
            .and_then(|agent| agent.name.as_deref()),
        Some("Alice")
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/agents/agent-1/schedules"));
}

#[tokio::test]
async fn helpdesk_agent_schedule_list_by_query_smoke() {
    let body = r#"{"code":0,"msg":"ok","data":{"agent_schedules":[{"status":2,"agent":{"id":"agent-1"}}]}}"#;
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
    assert_eq!(
        resp.data
            .as_ref()
            .and_then(|data| data.agent_schedules.as_ref())
            .and_then(|schedules| schedules.first())
            .and_then(|schedule| schedule.status),
        Some(2)
    );
    let request = requests.lock().unwrap().join("\n");
    assert!(request.contains("GET /open-apis/helpdesk/v1/agent_schedules?"));
    assert!(request.contains("status=1"));
    assert!(request.contains("status=2"));
}
