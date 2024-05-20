use crate::SubKind;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// Barter [`Subscription`](super::Subscription) [`SubKind`] that yields [`Index`] [`MarketEvent<T>`](crate::event::MarketEvent) events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Indices;

impl SubKind for Indices {
    type Event = Index;
}

/// Normalized Barter [`Index`] model.
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct Index {
    pub index_name: String,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
}
