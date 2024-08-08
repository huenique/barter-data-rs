use serde::Serialize;

use crate::exchange::coincall::Coincall;
use crate::subscription::ticker::Tickers;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Coincall`](super::Coincall) channel to be subscribed to.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct CoincallChannel(pub &'static str);

impl CoincallChannel {
    /// [`Coincall`](super::Coincall) OrderBook Level2 channel name (raw updates).
    pub const TICKER: Self = Self("ticker");
}

impl<Server> Identifier<CoincallChannel> for Subscription<Coincall<Server>, Tickers> {
    fn id(&self) -> CoincallChannel {
        CoincallChannel::TICKER
    }
}

impl AsRef<str> for CoincallChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
