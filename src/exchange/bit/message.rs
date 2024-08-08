use serde::Deserialize;
use serde::Serialize;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BitWsMessage<BitWsChannel, BitWsType> {
    Type(BitWsType),
    Channel(BitWsChannel),
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitPong {
    #[serde(rename = "type")]
    pub message_type: String,
    pub result: BitPongResult,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitPongResult {
    pub code: i64,
    pub message: String,
    pub data: BitPongData,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct BitPongData {
    pub id: i64,
    pub timestamp: i64,
}
