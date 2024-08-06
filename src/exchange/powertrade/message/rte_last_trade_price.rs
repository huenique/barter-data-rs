use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub last_trade_prirce: LastTradePrice,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastTradePrice {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    pub price_type: String,
}
