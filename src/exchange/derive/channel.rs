use serde::Serialize;

use crate::exchange::derive::Derive;
use crate::subscription::book::OrderBook;
use crate::subscription::ticker::Tickers;
use crate::subscription::Subscription;
use crate::Identifier;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeriveChannel(pub &'static str);

impl DeriveChannel {
    /// [`Derive`](super::Derive) real-time Ticker channel.
    ///
    /// See docs: <https://docs.derive.finance/reference/ticker-instrument_name-interval>
    pub const TICKER: Self = Self("ticker.{}.100");

    /// [`Derive`](super::Derive) real-time Order Book channel.
    ///
    /// See docs: <https://docs.derive.xyz/reference/orderbook-instrument_name-group-depth>
    pub const ORDER_BOOK: Self = Self("orderbook.{}.1.100");
}

impl Identifier<DeriveChannel> for Subscription<Derive, Tickers> {
    fn id(&self) -> DeriveChannel {
        DeriveChannel::TICKER
    }
}

impl Identifier<DeriveChannel> for Subscription<Derive, OrderBook> {
    fn id(&self) -> DeriveChannel {
        DeriveChannel::ORDER_BOOK
    }
}

impl AsRef<str> for DeriveChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
