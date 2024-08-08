use serde::Serialize;

use crate::exchange::deribit::Deribit;
use crate::subscription::book::OrderBooksL1;
use crate::subscription::book::OrderBooksL2;
use crate::subscription::index::Indices;
use crate::subscription::ticker::Tickers;
use crate::subscription::trade::PublicTrades;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Deribit`](super::Deribit) channel to be subscribed to.
///
/// See docs: <https://docs.deribit.com/#subscriptions>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeribitChannel(pub &'static str);

impl DeribitChannel {
    /// [`Deribit`] real-time trades channel.
    ///
    /// See docs: <https://docs.deribit.com/#trades-instrument_name-interval>
    pub const TRADES_RAW: Self = Self("trades.{}.100ms");

    /// [`Deribit`](super::Deribit) real-time OrderBook Level1 (top of book)
    /// channel name.
    ///
    /// See docs:<https://docs.deribit.com/#quote-instrument_name>
    pub const ORDER_BOOK_L1: Self = Self("quote.{}");

    /// [`Deribit`](super::Deribit) OrderBook Level2 channel name.
    ///
    /// See docs: <https://docs.deribit.com/#book-instrument_name-interval>
    pub const ORDER_BOOK_L2: Self = Self("book.{}.100ms");

    /// [`Deribit`](super::Deribit) real-time Ticker channel.
    ///
    /// See docs: <https://docs.deribit.com/#ticker-instrument_name-interval>
    pub const TICKER: Self = Self("ticker.{}.100ms");

    /// [`Deribit`](super::Deribit) real-time Index channel.
    ///
    /// See docs: <https://docs.deribit.com/#deribit_price_index-currency>
    pub const INDEX: Self = Self("deribit_price_index.{}");
}

impl<Server> Identifier<DeribitChannel> for Subscription<Deribit<Server>, PublicTrades> {
    fn id(&self) -> DeribitChannel {
        DeribitChannel::TRADES_RAW
    }
}

impl<Server> Identifier<DeribitChannel> for Subscription<Deribit<Server>, OrderBooksL1> {
    fn id(&self) -> DeribitChannel {
        DeribitChannel::ORDER_BOOK_L1
    }
}

impl<Server> Identifier<DeribitChannel> for Subscription<Deribit<Server>, OrderBooksL2> {
    fn id(&self) -> DeribitChannel {
        DeribitChannel::ORDER_BOOK_L2
    }
}

impl<Server> Identifier<DeribitChannel> for Subscription<Deribit<Server>, Tickers> {
    fn id(&self) -> DeribitChannel {
        DeribitChannel::TICKER
    }
}

impl<Server> Identifier<DeribitChannel> for Subscription<Deribit<Server>, Indices> {
    fn id(&self) -> DeribitChannel {
        DeribitChannel::INDEX
    }
}

impl AsRef<str> for DeribitChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
