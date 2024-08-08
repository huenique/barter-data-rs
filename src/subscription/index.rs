use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::SubKind;

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields
/// [`Index`] [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Indices;

impl SubKind for Indices {
    type Event = Index;
}

/// Normalized Barter [`Index`] model.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Index {
    pub index_name: String,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}
