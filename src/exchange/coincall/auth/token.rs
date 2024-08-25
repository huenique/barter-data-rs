use std::error::Error;

use serde::Deserialize;
use serde::Serialize;
use tracing::debug;

const COINCALL_AUTH_URL: &str = "https://www.coincall.com/api/auth/start/v1";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CoincallOptionAuthResponse {
    code: i32,
    msg: String,
    i18n_args: Option<serde_json::Value>,
    data: CoincallOptionAuthData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CoincallOptionAuthData {
    server_ts: u64,
    uuid: String,
    key: String,
    token: String,
}

pub async fn fetch_token_from_url() -> Result<String, Box<dyn Error>> {
    debug!("Fetching token from URL: {}", COINCALL_AUTH_URL);
    let response = reqwest::get(COINCALL_AUTH_URL).await?;
    let response_json: CoincallOptionAuthResponse = response.json().await?;
    Ok(response_json.data.token)
}
