use serde::Deserialize;
use serde::Serialize;
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub last_trade_prirce: RteLastTradePrice,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RteLastTradePrice {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    pub price_type: String,
}
