use super::SubKind;
use barter_integration::model::Side;
use barter_macro::DeSubKind;
use barter_macro::SerSubKind;
use serde::Deserialize;
use serde::Serialize;

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields [`PublicTrade`]
/// [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, DeSubKind, SerSubKind)]
pub struct PublicTrades;

impl SubKind for PublicTrades {
    type Event = PublicTrade;
}

/// Normalised Barter [`PublicTrade`] model.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct PublicTrade {
    pub id: String,
    pub price: f64,
    pub amount: f64,
    pub side: Side,
}
