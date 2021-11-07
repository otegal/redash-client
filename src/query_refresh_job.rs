use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RootQueryRefreshJob {
    job: Job,
}

#[derive(Serialize, Deserialize, Debug)]
struct Job {
    status: i64,
    error: String,
    id: String,
    query_result_id: Option<i64>,
    updated_at: i64,
}

pub async fn fetch_query_refresh_job(job_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let resp = client
        .get(dotenv::var("USERS_QUERY_REFRESH_JOB_URL").unwrap() + job_id)
        .header(
            reqwest::header::AUTHORIZATION,
            dotenv::var("USER_API_KEY").unwrap(),
        )
        .send()
        .await?
        .json::<RootQueryRefreshJob>()
        .await?;
    println!("{:?}", resp);

    Ok(())
}
