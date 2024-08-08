use barter_integration::model::Side;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::subscription::SubKind;

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields
/// [`Liquidation`] [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Liquidations;

impl SubKind for Liquidations {
    type Event = Liquidation;
}

/// Normalised Barter [`Liquidation`] model.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Liquidation {
    pub side: Side,
    pub price: f64,
    pub quantity: f64,
    pub time: DateTime<Utc>,
}
