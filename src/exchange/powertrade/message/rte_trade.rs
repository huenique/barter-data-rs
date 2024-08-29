use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub rte_trade: RteTrade,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RteTrade {
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub timestamp: i64,
    pub symbol: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub trade_id: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    pub price_type: String,
    pub quantity: String,
    pub quantity_in_underlying: String,
    pub buy_display_order_id: String,
    pub sell_display_order_id: String,
}
