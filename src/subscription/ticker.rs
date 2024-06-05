use crate::SubKind;

use serde::Deserialize;
use serde::Serialize;

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields [`Ticker`] [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Tickers;

impl SubKind for Tickers {
    type Event = Ticker;
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
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
    pub timestamp: u64,
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

/// Greeks data for options.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Greeks {
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub vega: Option<f64>,
    pub rho: Option<f64>,
}
