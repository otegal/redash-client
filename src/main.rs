pub mod query_refresh;
pub mod query_result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    query_refresh::call_query_refresh().await?;
    query_result::fetch_query_result().await?;
    Ok(())
}
