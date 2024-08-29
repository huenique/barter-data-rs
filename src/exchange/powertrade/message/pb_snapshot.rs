use serde::Deserialize;
use serde::Serialize;

use crate::exchange::powertrade::utils::de;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PriceBookSnapshot {
    #[serde(deserialize_with = "de::de_str_to_i64")]
    pub timestamp: i64,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub symbol: String,
    pub bids: OrderData,
    pub asks: OrderData,
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OrderData {
    pub n_levels: String,
    pub n_orders: String,
    pub levels: Vec<Vec<String>>,
}
