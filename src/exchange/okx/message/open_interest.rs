use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OpenInterestData {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "oi", deserialize_with = "barter_integration::de::de_str")]
    pub oi: f64,
    #[serde(rename = "oiCcy", deserialize_with = "barter_integration::de::de_str")]
    pub oi_ccy: f64,
    #[serde(rename = "ts", deserialize_with = "barter_integration::de::de_str")]
    pub ts: i64,
}
