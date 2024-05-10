use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionStatus {
    timestamp: String,
    n_subscribed_tradeable_entities: String,
    n_total_tradeable_entities: String,
    n_total_multi_leg_tradeable_entities: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FundingRateUpdate {
    timestamp: String,
    utc_timestamp: Option<String>,
    tradeable_entity_id: String,
    applies_from: String,
    rate: String,
    mark_price: String,
    underlying_price: String,
}

// For OB L1. Currently unused.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TopOfBook {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub buy_price: String,
    pub buy_quantity: String,
    pub sell_price: String,
    pub sell_quantity: String,
}
