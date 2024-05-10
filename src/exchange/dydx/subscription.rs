use crate::exchange::dydx::message::ChannelDataMessage;
use crate::exchange::dydx::message::ErrorMessage;
use crate::exchange::dydx::message::SubscribedMessage;

use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DydxSubResponse {
    Connected(ConnectedMessage),
    Subscribed(SubscribedMessage),
    ChannelData(ChannelDataMessage),
    Error(ErrorMessage),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConnectedMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub connection_id: String,
    pub message_id: u32,
}

impl Validator for DydxSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        match self {
            DydxSubResponse::Connected(_) => Ok(self),
            DydxSubResponse::Subscribed(_) | DydxSubResponse::ChannelData(_) => Ok(self),
            DydxSubResponse::Error(e) => Err(SocketError::Subscribe(format!(
                "Subscription error: {}",
                e.message
            ))),
        }
    }
}
