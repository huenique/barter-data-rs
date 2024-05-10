use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub enum PowerTradeSubResponse {
    #[serde(rename = "subscribed")]
    Subscribed(Subscribed),
    #[serde(rename = "subscribe_error")]
    Error(SubscribeError),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscribed {
    pub tradeable_entity_id: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeError {
    pub message: String,
}

impl Validator for PowerTradeSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        match self {
            PowerTradeSubResponse::Subscribed(_) => Ok(self),
            PowerTradeSubResponse::Error(failure) => Err(SocketError::Subscribe(format!(
                "received failure subscription response with message: {}",
                failure.message,
            ))),
            PowerTradeSubResponse::Unknown => Ok(self),
        }
    }
}
