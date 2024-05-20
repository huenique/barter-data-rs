use crate::SubKind;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields [`Ticker`] [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Tickers;

impl SubKind for Tickers {
    type Event = Ticker;
}

/// Normalized Barter [`Ticker`] model.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Ticker {
    pub instrument_name: String,
    pub best_bid_price: f64,
    pub best_ask_price: f64,
    pub best_bid_amount: f64,
    pub best_ask_amount: f64,
    pub mark_price: f64,
    pub last_price: Option<f64>,
    pub volume_24h: Option<f64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub open_interest: Option<f64>,
    pub greeks: Option<Greeks>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}
