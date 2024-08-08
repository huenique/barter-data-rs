use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub funding_rate: FundingRate,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FundingRate {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub applies_from: String,
    pub rate: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub mark_price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub underlying_price: f64,
}
