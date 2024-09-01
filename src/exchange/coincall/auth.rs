use std::borrow::Cow;
use std::error::Error;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use hmac::Hmac;
use hmac::Mac;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;
use tracing::debug;

pub const COINCALL_AUTH_URL: &str = "https://www.coincall.com/api/auth/start/v1";
pub const COINCALL_ORDERBOOK_V1: &str = "/trade/order/orderBook/v1";

pub type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoincallOptionAuthResponse {
    pub code: i32,
    pub msg: String,
    pub i18n_args: Option<serde_json::Value>,
    pub data: CoincallOptionAuthData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoincallOptionAuthData {
    pub server_ts: i64,
    pub uuid: String,
    pub key: String,
    pub token: String,
}

pub async fn get_cc_token() -> Result<String, Box<dyn Error>> {
    debug!("Fetching Coincall token");
    let resp = get_cc_auth_parms().await?;
    Ok(resp.token)
}

#[derive(Debug)]
pub struct SigGenParams<'a> {
    pub key: String,
    pub uuid: String,
    pub ts: i64,
    pub tsdiff: i64,
    pub instrument_name: Cow<'a, str>,
}

pub async fn get_cc_auth_parms() -> Result<CoincallOptionAuthData, Box<dyn Error>> {
    debug!("Fetching auth parameters from URL: {}", COINCALL_AUTH_URL);
    let response = reqwest::get(COINCALL_AUTH_URL).await?;
    let response_json: CoincallOptionAuthResponse = response.json().await?;
    Ok(response_json.data)
}

pub fn calc_cc_ts(server_ts: i64) -> (i64, i64) {
    debug!("Calculating timestamp difference for Coincall");
    let start = SystemTime::now();
    let ts = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64;
    let tsdiff = ts - server_ts;
    (ts, tsdiff)
}

pub fn gen_cc_secret(sig_params: &SigGenParams<'_>, method: &str, path: &str) -> String {
    debug!("Generating secret for Coincall");
    format!(
        "{method}{path}/{instrument_name}?uuid={uuid}&ts={ts}&tsdiff={tsdiff}",
        instrument_name = sig_params.instrument_name,
        uuid = sig_params.uuid,
        ts = sig_params.ts,
        tsdiff = sig_params.tsdiff,
    )
}

pub fn gen_cc_sig(secret: String, key: &str) -> Result<String, Box<dyn Error>> {
    debug!("Generating signature for Coincall");
    let mut mac = HmacSha256::new_from_slice(key.as_bytes())?;
    mac.update(secret.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_cc_ts() {
        let server_ts = 1723754167709;
        let (ts, tsdiff) = calc_cc_ts(server_ts);

        assert!(
            ts > server_ts,
            "The current timestamp (ts) should be greater than server_ts"
        );
        assert_eq!(
            tsdiff,
            ts - server_ts,
            "Tsdiff should be the difference between ts and server_ts"
        );
    }

    #[test]
    fn test_gen_cc_sig() {
        let secret = "GET/trade/order/orderBook/v1/BTC-30AUG24?uuid=e0afd95474f34ee4b4befbfde6a9f581&ts=1723754167709&tsdiff=-916";
        let key = "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDMR2VYN/E5jkQDtPLYaZoY3eRq8d0vRw0YJLCA321D4DjiPxTGrcFwor/PGM3zskfpdHmaUXUPdr6pmtTdEuywTaCSr5uSnCjUsU5Kc7ttZP+kDz8GA28lTUFY6spVjUm/VuYUFPLq4icB2oWyWfUN1qkolgKqvEfPRQGnralKBQIDAQAB";

        let signature = gen_cc_sig(secret.to_string(), key).expect("Failed to generate signature");
        let expected_sig = "12be88be61692cafe428ee48f25c727708addf29589dee74760c723b48b1fe83";

        assert_eq!(
            signature, expected_sig,
            "Signature does not match the expected value"
        );
    }

    #[test]
    fn test_gen_cc_secret() {
        let sig_params = SigGenParams {
            key: "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDMR2VYN/E5jkQDtPLYaZoY3eRq8d0vRw0YJLCA321D4DjiPxTGrcFwor/PGM3zskfpdHmaUXUPdr6pmtTdEuywTaCSr5uSnCjUsU5Kc7ttZP+kDz8GA28lTUFY6spVjUm/VuYUFPLq4icB2oWyWfUN1qkolgKqvEfPRQGnralKBQIDAQAB".to_string(),
            uuid: "e0afd95474f34ee4b4befbfde6a9f581".to_string(),
            ts: 1723754167709,
            tsdiff: -916,
            instrument_name: Cow::Borrowed("BTC-30AUG24"),
        };

        let secret = gen_cc_secret(
            &sig_params,
            reqwest::Method::GET.as_str(),
            COINCALL_ORDERBOOK_V1,
        );
        assert_eq!(
            secret,
            "GET/trade/order/orderBook/v1/BTC-30AUG24?uuid=e0afd95474f34ee4b4befbfde6a9f581&ts=1723754167709&tsdiff=-916",
            "Secret does not match the expected value"
        );
    }
}
