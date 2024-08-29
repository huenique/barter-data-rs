use serde::Deserialize;
use serde::Serialize;

use crate::exchange::powertrade::utils::de;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub funding_rate: FundingRate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FundingRate {
    #[serde(deserialize_with = "de::de_str_to_i64")]
    pub timestamp: i64,
    pub tradeable_entity_id: String,
    pub applies_from: String,
    pub rate: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub mark_price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub underlying_price: f64,
}
