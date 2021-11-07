use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RootQueryRefresh {
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

pub async fn call_query_refresh() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let resp = client
        .post(dotenv::var("USERS_QUERY_REFRESH_URL").unwrap())
        .header(
            reqwest::header::AUTHORIZATION,
            dotenv::var("USER_API_KEY").unwrap(),
        )
        .send()
        .await?
        .json::<RootQueryRefresh>()
        .await?;
    println!("{:?}", resp);

    Ok(())
}
