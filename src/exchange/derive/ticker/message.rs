use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct OptionPricing {
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub delta: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub theta: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub gamma: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub vega: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub iv: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub rho: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub mark_price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub forward_price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub bid_iv: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub ask_iv: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptionDetails {
    pub index: String,
    pub expiry: i64,
    pub strike: String,
    pub option_type: String,
    pub settlement_price: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    pub contract_volume: String,
    pub num_trades: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub open_interest: f64,
    pub high: String,
    pub low: String,
    pub percent_change: String,
    pub usd_change: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeriveInstrumentTicker {
    pub instrument_type: String,
    pub instrument_name: String,
    pub scheduled_activation: i64,
    pub scheduled_deactivation: i64,
    pub is_active: bool,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub tick_size: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub minimum_amount: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub maximum_amount: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub amount_step: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub mark_price_fee_rate_cap: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub maker_fee_rate: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub taker_fee_rate: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub base_fee: f64,
    pub base_currency: String,
    pub quote_currency: String,
    pub option_details: OptionDetails,
    pub perp_details: Option<String>,
    pub erc20_details: Option<String>,
    pub base_asset_address: String,
    pub base_asset_sub_id: String,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub pro_rata_fraction: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub fifo_min_allocation: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub pro_rata_amount_step: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub best_ask_amount: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub best_ask_price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub best_bid_amount: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub best_bid_price: f64,
    pub option_pricing: OptionPricing,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub index_price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub mark_price: f64,
    pub stats: Stats,
    pub timestamp: i64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub min_price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub max_price: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub timestamp: i64,
    pub instrument_ticker: DeriveInstrumentTicker,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
    pub channel: String,
    pub data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub method: String,
    pub params: Params,
}
