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
    /// To get the complete ticker information, we need to subscribe to the following channels:
    /// - <https://docs.coincall.com/#options-websocket-pricing-information>
    /// - <https://docs.coincall.com/#options-websocket-orderbook>
    pub const TICKER: Self = Self("3_5");

    /// [`Coincall`](super::Coincall) Order book channel.
    ///
    /// See docs: <https://docs.coincall.com/#options-websocket-orderbook>
    pub const ORDER_BOOK_L2: Self = Self("5");
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
