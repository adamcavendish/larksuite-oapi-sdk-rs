use larksuite_oapi_sdk_rs::service::calendar::v4::{ListCalendarQuery, ListEventQuery};
use larksuite_oapi_sdk_rs::{Client, RequestOption};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").expect("APP_ID env var required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET env var required");
    let calendar_id = std::env::var("CALENDAR_ID").expect("CALENDAR_ID env var required");
    let start_time = std::env::var("START_TIME").ok();
    let end_time = std::env::var("END_TIME").ok();

    let client = Client::builder(app_id, app_secret).build()?;
    let option = RequestOption::default();

    let calendars = client
        .calendar()
        .calendar
        .list_by_query(&ListCalendarQuery::new().page_size(20), &option)
        .await?;
    if calendars.success() {
        println!(
            "calendars: {}",
            calendars
                .data
                .as_ref()
                .map(|data| data.calendar_list.len())
                .unwrap_or(0)
        );
    } else {
        println!("calendar list error: {}", calendars.code_error);
    }

    let events = client
        .calendar()
        .event
        .list_by_query(
            &ListEventQuery::new(&calendar_id)
                .page_size(20)
                .start_time(start_time.as_deref())
                .end_time(end_time.as_deref())
                .user_id_type("open_id"),
            &option,
        )
        .await?;
    if events.success() {
        println!(
            "events: {}",
            events
                .data
                .as_ref()
                .map(|data| data.items.len())
                .unwrap_or(0)
        );
    } else {
        println!("event list error: {}", events.code_error);
    }

    Ok(())
}
