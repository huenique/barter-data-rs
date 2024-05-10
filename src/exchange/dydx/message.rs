use crate::exchange::dydx::book::DydxLevel;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DydxMessage {
    Subscribed(SubscribedMessage),
    ChannelData(ChannelDataMessage),
    ErrorMessage(ErrorMessage),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribedMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub connection_id: String,
    pub message_id: u64,
    pub id: String,
    pub channel: String,
    pub contents: OrderBookSnapshotContents,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBookSnapshotContents {
    pub bids: Vec<DydxLevel>,
    pub asks: Vec<DydxLevel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelDataMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub connection_id: String,
    pub message_id: u64,
    pub id: String,
    pub channel: String,
    pub version: String,
    pub contents: ChannelDataMessageContents,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub connection_id: String,
    pub message_id: u64,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelDataMessageContents {
    #[serde(default, deserialize_with = "optional_vec_vec_string")]
    pub bids: Option<Vec<DydxLevel>>,
    #[serde(default, deserialize_with = "optional_vec_vec_string")]
    pub asks: Option<Vec<DydxLevel>>,
}

fn optional_vec_vec_string<'de, D>(deserializer: D) -> Result<Option<Vec<DydxLevel>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Vec::<DydxLevel>::deserialize(deserializer)?;
    Ok(Some(value))
}
