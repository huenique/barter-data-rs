use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeResponse {
    pub id: String,
    pub result: SubscribeResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeResult {
    pub status: std::collections::HashMap<String, String>,
    pub current_subscriptions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionNotification {
    pub method: String,
    pub params: SubscriptionParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionParams {
    pub channel: String,
    pub data: SubscriptionData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionData {
    pub timestamp: i64,
    pub instrument_ticker: InstrumentTicker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentTicker {
    pub instrument_type: String,
    pub instrument_name: String,
    pub scheduled_activation: i64,
    pub scheduled_deactivation: i64,
    pub is_active: bool,
    pub tick_size: String,
    pub minimum_amount: String,
    pub maximum_amount: String,
    pub amount_step: String,
    pub mark_price_fee_rate_cap: String,
    pub maker_fee_rate: String,
    pub taker_fee_rate: String,
    pub base_fee: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub option_details: Option<OptionDetails>,
    pub perp_details: Option<PerpDetails>,
    pub erc20_details: Option<Erc20Details>,
    pub base_asset_address: String,
    pub base_asset_sub_id: String,
    pub pro_rata_fraction: String,
    pub fifo_min_allocation: String,
    pub pro_rata_amount_step: String,
    pub best_ask_amount: String,
    pub best_ask_price: String,
    pub best_bid_amount: String,
    pub best_bid_price: String,
    pub five_percent_bid_depth: String,
    pub five_percent_ask_depth: String,
    pub option_pricing: Option<OptionPricing>,
    pub index_price: String,
    pub mark_price: String,
    pub stats: Stats,
    pub timestamp: i64,
    pub min_price: String,
    pub max_price: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OptionDetails {}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerpDetails {
    pub index: String,
    pub max_rate_per_hour: String,
    pub min_rate_per_hour: String,
    pub static_interest_rate: String,
    pub aggregate_funding: String,
    pub funding_rate: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Erc20Details {}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OptionPricing {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub contract_volume: String,
    pub num_trades: String,
    pub open_interest: String,
    pub high: String,
    pub low: String,
    pub percent_change: String,
    pub usd_change: String,
}
