use serde::{Deserialize, Serialize};

const REDASH_USERS_QUERY_RESULT_URL: &str = "put your redash endpoint with api key params";

#[derive(Serialize, Deserialize, Debug)]
struct RootQueryResult {
    query_result: QueryResult,
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryResult {
    id: i64,
    query_hash: String,
    query: String,
    data: Box<Data>,
    data_source_id: i64,
    runtime: f64,
    retrieved_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    columns: Box<[Column]>,
    rows: Box<[Row]>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Column {
    name: String,
    friendly_name: String,
    r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    updated_at: String,
    created_at: String,
    id: i64,
    org_id: i64,
    name: String,
    email: String,
    profile_image_url: Option<String>,
    password_hash: String,
    groups: Box<[i64]>,
    api_key: String,
    disabled_at: Option<String>,
    details: Detail,
}

#[derive(Serialize, Deserialize, Debug)]
struct Detail {
    active_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(REDASH_USERS_QUERY_RESULT_URL)
        .await?
        .json::<RootQueryResult>()
        .await?;
    println!("{:?}", resp);
    println!("query_result.id is {:?}", resp.query_result.id);
    Ok(())
}
