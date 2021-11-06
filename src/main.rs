pub mod query_result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    query_result::fetch_query_result().await?;
    Ok(())
}
