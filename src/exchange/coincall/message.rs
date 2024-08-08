use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct CoincallMessage<Data> {
    #[serde(rename = "dt")]
    pub data_type: u8,
    #[serde(rename = "c")]
    pub channel: u8,
    #[serde(rename = "d")]
    pub data: Data,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct CoincallHeartbeat {
    #[serde(rename = "c")]
    pub channel: u8,
    #[serde(rename = "rc")]
    pub response_code: u8,
}
