use serde::{Deserialize, Serialize};

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

pub async fn fetch_query_result() -> Result<(), Box<dyn std::error::Error>> {
    let redash_users_query_result_url: String = dotenv::var("USERS_QUERY_RESULT_URL").unwrap();
    let resp = reqwest::get(redash_users_query_result_url)
        .await?
        .json::<RootQueryResult>()
        .await?;
    println!("{:?}", resp);
    println!("query_result.id is {:?}", resp.query_result.id);
    Ok(())
}
