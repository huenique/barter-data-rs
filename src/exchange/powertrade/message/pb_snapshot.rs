use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PriceBookSnapshot {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub symbol: String,
    pub bids: OrderData,
    pub asks: OrderData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrderData {
    pub n_levels: String,
    pub n_orders: String,
    pub levels: Vec<Vec<String>>,
}
