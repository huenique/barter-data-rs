use serde::Deserialize;
use serde::Serialize;

use crate::exchange::powertrade::utils::de;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub last_trade_prirce: RteLastTradePrice,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RteLastTradePrice {
    #[serde(deserialize_with = "de::de_str_to_i64")]
    pub timestamp: i64,
    pub tradeable_entity_id: String,
    pub market_id: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    pub price_type: String,
}
