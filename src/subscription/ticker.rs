use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

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
#[derive(Clone, Copy, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
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
        (Value::Object(ref mut map1), Value::Object(map2)) => {
            for (k, v) in map2 {
                map1.entry(k).or_insert(v.clone());
            }
            Ok(Value::Object(map1.clone()))
        }
        (_, v) => Ok(v),
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

impl fmt::Display for Ticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ticker {{
    instrument_name: {},
    best_bid_price: {:.2},
    best_ask_price: {:.2},
    best_bid_amount: {:.2},
    best_ask_amount: {:.2},
    mark_price: {:.2},
    last_price: {:.2},
    open_interest: {:.2},
    state: {},
    timestamp: {},
    greeks: {},
    interest_rate: {:?},
    mark_iv: {:?},
    delivery_price: {:?},
    current_funding: {:?},
    interest_value: {:?},
    ask_iv: {:?},
    bid_iv: {:?},
    index_price: {:.2}
}}",
            self.instrument_name,
            self.best_bid_price,
            self.best_ask_price,
            self.best_bid_amount,
            self.best_ask_amount,
            self.mark_price,
            self.last_price,
            self.open_interest,
            self.state,
            self.timestamp,
            match &self.greeks {
                Some(greeks) => format!("{:?}", greeks),
                None => "None".to_string(),
            },
            self.interest_rate,
            self.mark_iv,
            self.delivery_price,
            self.current_funding,
            self.interest_value,
            self.ask_iv,
            self.bid_iv,
            self.index_price
        )
    }
}

impl fmt::Debug for Greeks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Greeks {{
    delta: {:?},
    gamma: {:?},
    theta: {:?},
    vega: {:?},
    rho: {:?}
}}",
            self.delta, self.gamma, self.theta, self.vega, self.rho
        )
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
