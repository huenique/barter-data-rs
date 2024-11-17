use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct TickerData {
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "last", deserialize_with = "barter_integration::de::de_str")]
    pub last: f64,
    #[serde(rename = "lastSz", deserialize_with = "barter_integration::de::de_str")]
    pub last_sz: f64,
    #[serde(rename = "askPx", deserialize_with = "barter_integration::de::de_str")]
    pub ask_px: f64,
    #[serde(rename = "askSz", deserialize_with = "barter_integration::de::de_str")]
    pub ask_sz: f64,
    #[serde(rename = "bidPx", deserialize_with = "barter_integration::de::de_str")]
    pub bid_px: f64,
    #[serde(rename = "bidSz", deserialize_with = "barter_integration::de::de_str")]
    pub bid_sz: f64,
    #[serde(
        rename = "open24h",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub open24h: f64,
    #[serde(
        rename = "high24h",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub high24h: f64,
    #[serde(rename = "low24h", deserialize_with = "barter_integration::de::de_str")]
    pub low24h: f64,
    #[serde(
        rename = "sodUtc0",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub sod_utc0: f64,
    #[serde(
        rename = "sodUtc8",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub sod_utc8: f64,
    #[serde(
        rename = "volCcy24h",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub vol_ccy24h: f64,
    #[serde(rename = "vol24h", deserialize_with = "barter_integration::de::de_str")]
    pub vol24h: f64,
    #[serde(rename = "ts", deserialize_with = "barter_integration::de::de_str")]
    pub ts: i64,
}
