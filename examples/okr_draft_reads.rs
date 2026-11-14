//! Read draft objectives without publishing or dropping them.
use larksuite_oapi_sdk_rs::LarkClient;
use larksuite_oapi_sdk_rs::req::RequestOption;
use larksuite_oapi_sdk_rs::service::okr::v2::OkrPageQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client =
        LarkClient::builder(std::env::var("APP_ID")?, std::env::var("APP_SECRET")?).build()?;
    let option = RequestOption {
        user_access_token: Some(std::env::var("USER_ACCESS_TOKEN")?),
        ..Default::default()
    };
    let cycle_id = std::env::var("OKR_CYCLE_ID")?;
    let okr = client.okr_v2();
    let query = OkrPageQuery::new().page_size(Some(20));
    let mut objectives =
        okr.okr_cycle_objective
            .list_by_iterator_with_draft(&cycle_id, &query, Some(true));
    while let Some(objective) = objectives.next(&option).await? {
        println!("{:?}: {:?}", objective.objective_id, objective.is_draft);
    }
    Ok(())
}
