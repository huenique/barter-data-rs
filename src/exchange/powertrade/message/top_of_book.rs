use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TopOfBook {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub buy_price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub buy_quantity: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub sell_price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub sell_quantity: f64,
}
