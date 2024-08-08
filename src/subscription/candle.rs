use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::subscription::SubKind;

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields
/// [`Candle`] [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Candles;

impl SubKind for Candles {
    type Event = Candle;
}

/// Normalised Barter OHLCV [`Candle`] model.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Candle {
    pub close_time: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub trade_count: u64,
}
