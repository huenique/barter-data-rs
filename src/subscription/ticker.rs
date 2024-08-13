use std::cmp::Ordering;
use std::error::Error;

use chrono::TimeZone;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::event::MarketIter;
use crate::subscription::Instrument;
use crate::ExchangeId;
use crate::MarketEvent;
use crate::SubKind;

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields
/// [`Ticker`] [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tickers;

impl SubKind for Tickers {
    type Event = Ticker;
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ticker {
    pub instrument_name: String,
    pub best_bid_price: f64,
    pub best_ask_price: f64,
    pub best_bid_amount: f64,
    pub best_ask_amount: f64,
    pub mark_price: f64,
    pub last_price: f64,
    pub open_interest: f64,
    pub state: String,
    pub timestamp: i64,
    pub greeks: Option<Greeks>,
    pub interest_rate: Option<f64>,   // Option specific
    pub mark_iv: Option<f64>,         // Option specific
    pub delivery_price: Option<f64>,  // Settlement price when state is closed
    pub current_funding: Option<f64>, // Perpetual specific
    pub interest_value: Option<f64>,  // Perpetual specific
    pub ask_iv: Option<f64>,
    pub bid_iv: Option<f64>,
    pub index_price: f64,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Greeks {
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub vega: Option<f64>,
    pub rho: Option<f64>,
}

impl Ticker {
    pub fn merge(&mut self, other: &Self) -> Result<(), Box<dyn Error>> {
        let self_obj: Value = serde_json::to_value(&*self)
            .map_err(|e| format!("Failed to serialize self ticker: {}", e))?;
        let update_obj: Value = serde_json::to_value(other)
            .map_err(|e| format!("Failed to serialize ticker update: {}", e))?;
        let merged_obj = merge_object(self_obj, update_obj)
            .map_err(|e| format!("Failed to merge objects: {}", e))?;
        let merged_ticker: Ticker = serde_json::from_value(merged_obj)
            .map_err(|e| format!("Failed to deserialize merged ticker: {}", e))?;

        *self = merged_ticker;
        Ok(())
    }
}

fn merge_object(v1: Value, v2: Value) -> Result<Value, Box<dyn Error>> {
    match (v1, v2) {
        (Value::Object(mut map1), Value::Object(map2)) => {
            for (k, v) in map2 {
                if !is_default_value(&v) {
                    map1.insert(k, v);
                }
            }
            Ok(Value::Object(map1))
        }
        (v, _) => Ok(v), // If v2 is not an object, return v1 as the merged result
    }
}

fn is_default_value(v: &Value) -> bool {
    match v {
        Value::String(s) => s.is_empty(),
        Value::Number(n) => n.as_f64().unwrap_or_default() == 0.0,
        Value::Bool(b) => !*b,
        Value::Array(arr) => arr.is_empty(),
        Value::Object(map) => map.is_empty(),
        Value::Null => true,
    }
}

impl Eq for Ticker {}

impl Ord for Ticker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.instrument_name
            .cmp(&other.instrument_name)
            .then_with(|| self.timestamp.cmp(&other.timestamp))
    }
}

impl PartialOrd for Ticker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(ExchangeId, Instrument, Ticker)> for MarketIter<Ticker> {
    fn from((exchange_id, instrument, ticker): (ExchangeId, Instrument, Ticker)) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: Utc.timestamp_nanos(ticker.timestamp),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind: ticker,
        })])
    }
}
