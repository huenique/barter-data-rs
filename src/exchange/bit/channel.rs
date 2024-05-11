use crate::exchange::bit::Bit;
use crate::subscription::book::OrderBooksL2;
use crate::Identifier;
use crate::Subscription;

use serde::Serialize;

/// Type that defines how to translate a Barter [`Subscription`] into a [`Bit`](super::Bit)
/// channel to be subscribed to.
///
/// See docs: <https://www.bit.com/docs/en-us/#channel-summary>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct BitChannel(pub &'static str);

impl BitChannel {
    /// [`BIT`](super::Bit) real-time OrderBook Level2 channel name.
    ///
    /// See docs: <https://www.bit.com/docs/en-us/#order-book-channel>
    pub const ORDER_BOOK_L2: Self = Self("order_book.1.100");
}

impl AsRef<str> for BitChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Identifier<BitChannel> for Subscription<Bit, OrderBooksL2> {
    fn id(&self) -> BitChannel {
        BitChannel::ORDER_BOOK_L2
    }
}
