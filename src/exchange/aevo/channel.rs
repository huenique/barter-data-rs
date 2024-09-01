use serde::Serialize;

use crate::exchange::aevo::Aevo;
use crate::subscription::book::OrderBooksL2;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Aevo`](super::Aevo) channel to be subscribed to.
///
/// See docs: <https://docs.aevo.xyz/reference/publish-channel>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AevoChannel(pub &'static str);

impl AevoChannel {
    /// [`Aevo`](super::Aevo) OrderBook Level2 channel name (100ms).
    ///
    /// See docs: <https://api-docs.aevo.xyz/reference/subscribe-orderbook-throttled>
    pub const ORDER_BOOK_L2: Self = Self("orderbook-100ms");
}

impl Identifier<AevoChannel> for Subscription<Aevo, OrderBooksL2> {
    fn id(&self) -> AevoChannel {
        AevoChannel::ORDER_BOOK_L2
    }
}

impl AsRef<str> for AevoChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
