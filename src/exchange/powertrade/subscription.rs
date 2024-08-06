use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PowerTradeSubResponse {
    Subscribed {
        #[serde(rename = "subscribed")]
        subscribed: Subscribed,
    },
    Error {
        #[serde(rename = "subscribe_error")]
        error: SubscribeError,
    },
    Unknown(Value),
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
            PowerTradeSubResponse::Subscribed { .. } => Ok(self),
            PowerTradeSubResponse::Error { error } => Err(SocketError::Subscribe(format!(
                "received failure subscription response with message: {}",
                error.message,
            ))),
            PowerTradeSubResponse::Unknown(_) => Ok(self),
        }
    }
}
