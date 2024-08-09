use serde::Serialize;

use crate::exchange::coincall::Coincall;
use crate::subscription::ticker::Tickers;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Coincall`](super::Coincall) channel to be subscribed to.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CoincallChannel(pub &'static str);

impl CoincallChannel {
    /// [`Coincall`](super::Coincall) Pricing information channel.
    ///
    /// See docs: <https://docs.coincall.com/#options-websocket-pricing-information>
    pub const TICKER: Self = Self("3");
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
