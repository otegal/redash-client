pub mod query_refresh;
pub mod query_refresh_job;
pub mod query_result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let job_id = query_refresh::call_query_refresh().await?;
    query_refresh_job::fetch_query_refresh_job(&job_id).await?;
    query_result::fetch_query_result().await?;
    Ok(())
}
