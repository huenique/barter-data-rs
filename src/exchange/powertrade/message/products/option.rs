use serde::Deserialize;
use serde::Serialize;

use crate::exchange::powertrade::utils::de;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OptionDetails {
    pub option: MoreOptionDetails,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expiry {
    pub datetime: DateTime,
    pub timezone: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Date {
    pub year: String,
    pub month: String,
    pub day: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Time {
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
    pub nanoseconds: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub risk_snapshot: RiskSnapshot,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RiskSnapshot {
    pub symbol: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    #[serde(deserialize_with = "de::de_str_to_i64")]
    pub timestamp: i64,
    pub time_to_expire: f64,
    pub theoretical: Option<PriceDetail>,
    pub mid: Option<PriceDetail>,
    pub bid: Option<PriceDetail>,
    pub ask: Option<PriceDetail>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct PriceDetail {
    pub price: f64,
    pub volatility: f64,
    pub greeks: Greeks,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Greeks {
    pub delta: f64,
    pub vega: f64,
    pub theta: f64,
    pub rho: f64,
    pub gamma: f64,
}
