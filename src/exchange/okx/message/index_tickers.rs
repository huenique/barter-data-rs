use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct IndexTickerData {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "idxPx", deserialize_with = "barter_integration::de::de_str")]
    pub idx_px: f64,
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
    #[serde(rename = "ts", deserialize_with = "barter_integration::de::de_str")]
    pub ts: i64,
}
