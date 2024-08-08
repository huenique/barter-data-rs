use std::fmt::Debug;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use chrono::Duration;
use chrono::Utc;
use hmac::Hmac;
use hmac::Mac;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use sha2::Sha256;
use uuid::Uuid;

use crate::exchange::coincall::option::ticker::CoincallOptionTicker;
use crate::exchange::coincall::Coincall;
use crate::exchange::ExchangeServer;
use crate::exchange::StreamSelector;
use crate::subscription::ticker::Tickers;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeId;
use crate::ExchangeWsStream;

pub mod ticker;

pub const JWT_SECRET: &[u8] = b"";

pub const WEBSOCKET_BASE_URL_COINCALL_OPTION: &str = "wss://ws.coincall.com/options";

pub type HmacSha256 = Hmac<Sha256>;

pub type CoincallOption = Coincall<CoincallServerOption>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoincallServerOption;

impl ExchangeServer for CoincallServerOption {
    const ID: ExchangeId = ExchangeId::CoincallOption;

    fn websocket_url() -> &'static str {
        &WEBSOCKET_URL
    }
}

impl<Server> StreamSelector<Tickers> for Coincall<Server>
where
    Server: ExchangeServer + Debug + Send + Sync,
{
    type Stream = ExchangeWsStream<StatelessTransformer<Self, Tickers, CoincallOptionTicker>>;
}

lazy_static! {
    static ref WEBSOCKET_URL: String = {
        let uuid = generate_uuid();
        // TODO: Allow users to specify their own API key and secret
        let api_key = "NLiq8ZnRhFzTozL9TDsW00/P9Wz4hiDI6rDnnup2qXA=";
        let api_secret = "E6DZD13e/Or/oEum6vzGZsTKaJaGUNw2ONEbcyEz1UQ=";
        generate_wss_url(api_key, api_secret, Some(&uuid)).unwrap()
    };
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    sub: String,
    iat: usize,
}

/// Generates a JWT with the given UUID and a secret key. The JWT uses the HS512
/// algorithm and includes expiration and issued-at timestamps.
pub fn generate_jwt(uuid: &str, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    // Get current timestamp and expiration timestamp
    let current_timestamp = Utc::now().timestamp() as usize;
    let exp_timestamp = (Utc::now() + Duration::days(365 * 100)).timestamp() as usize;

    // Define the claims
    let claims = Claims {
        exp: exp_timestamp,
        sub: uuid.to_string(),
        iat: current_timestamp,
    };

    // Define the header
    let header = Header {
        typ: None,
        alg: jsonwebtoken::Algorithm::HS512,
        ..Default::default()
    };

    // Encode the token
    jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(secret))
}

fn generate_wss_url(
    api_key: &str,
    api_secret: &str,
    uuid: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let code = "10";
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .to_string();

    let verb = "GET";
    let uri = "/users/self/verify";
    let auth = format!("{}{}?uuid={}&ts={}", verb, uri, api_key, ts);

    let mut mac =
        HmacSha256::new_from_slice(api_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(auth.as_bytes());
    let sign = hex::encode(mac.finalize().into_bytes()).to_uppercase();

    let url = if let Some(uuid) = uuid {
        format!(
            "{}?code={}&uuid={}&ts={}&sign={}&apiKey={}",
            WEBSOCKET_BASE_URL_COINCALL_OPTION, code, uuid, ts, sign, api_key
        )
    } else {
        format!(
            "{}?code={}&ts={}&sign={}&apiKey={}",
            WEBSOCKET_BASE_URL_COINCALL_OPTION, code, ts, sign, api_key
        )
    };

    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert_eq!(uuid.len(), 32); // UUID without hyphens should be 32
                                    // characters long
    }

    #[test]
    fn test_generate_jwt() {
        let uuid = generate_uuid();
        let secret = b"256-bit-secret";
        let token = generate_jwt(&uuid, secret);
        assert!(token.is_ok());
    }

    #[test]
    fn test_websocket_url() {
        let url = CoincallServerOption::websocket_url();
        assert!(url.contains("wss://ws.coincall.com/options?code=10&uuid="));
        assert!(url.contains("&sign="));
        assert!(url.contains("&apiKey="));
    }
}
