use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MarkPriceData {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "markPx", deserialize_with = "barter_integration::de::de_str")]
    pub mark_px: f64,
    #[serde(rename = "ts", deserialize_with = "barter_integration::de::de_str")]
    pub ts: i64,
}
