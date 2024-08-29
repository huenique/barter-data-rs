use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct CoincallMessage<Data> {
    #[serde(rename = "dt")]
    pub data_type: u8,
    #[serde(rename = "c")]
    pub code: u8,
    #[serde(rename = "d")]
    pub data: Data,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct CoincallHeartbeat {
    #[serde(rename = "c")]
    pub code: u8,
    #[serde(rename = "rc")]
    pub response_code: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoincallOrderbook {
    pub code: i32,
    pub msg: String,
    pub i18n_args: Option<String>,
    pub data: Option<CoincallObData>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CoincallObData {
    pub symbol: String,
    pub display_name: Option<String>,
    pub strike: f64,
    pub bids: Vec<CoincallObOrder>,
    pub asks: Vec<CoincallObOrder>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CoincallObOrder {
    pub size: String,
    pub price: String,
}
