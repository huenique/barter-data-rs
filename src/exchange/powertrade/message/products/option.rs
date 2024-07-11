use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OptionDetails {
    pub option: MoreOptionDetails,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MoreOptionDetails {
    pub expiry: Expiry,
    pub strike_price: String,
    pub option_type: String,
    pub exercise_style: String,
    pub valuation_approach: String,
    pub delivery_style: String,
    pub underlying_deliverable_id: String,
    pub contract_size_deliverable_id: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub contract_size: f64,
    pub settlement_deliverable_id: String,
    pub utc_creation_time: String,
    pub creation_source_id: String,
    pub margin_spec_id: String,
    pub strikes_spec_id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Expiry {
    pub datetime: DateTime,
    pub timezone: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Date {
    pub year: String,
    pub month: String,
    pub day: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Time {
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
    pub nanoseconds: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Response {
    pub risk_snapshot: RiskSnapshot,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct RiskSnapshot {
    pub symbol: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub timestamp: String,
    pub time_to_expire: String,
    pub theoretical: Option<PriceDetail>,
    pub mid: Option<PriceDetail>,
    pub bid: Option<PriceDetail>,
    pub ask: Option<PriceDetail>,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct PriceDetail {
    pub price: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub volatility: f64,
    pub greeks: Greeks,
}

#[derive(Copy, Clone, Default, Serialize, Deserialize, Debug)]
pub struct Greeks {
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub delta: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub vega: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub theta: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub rho: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub gamma: f64,
}
