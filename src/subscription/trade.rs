use barter_integration::model::Side;
use barter_macro::DeSubKind;
use barter_macro::SerSubKind;
use serde::Deserialize;
use serde::Serialize;

use crate::subscription::SubKind;

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields
/// [`PublicTrade`] [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Clone, Copy, DeSubKind, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, SerSubKind)]
pub struct PublicTrades;

impl SubKind for PublicTrades {
    type Event = PublicTrade;
}

/// Normalised Barter [`PublicTrade`] model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct PublicTrade {
    pub id: String,
    pub price: f64,
    pub amount: f64,
    pub side: Side,
}
