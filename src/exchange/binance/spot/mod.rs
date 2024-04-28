use self::l2::BinanceSpotBookUpdater;
use super::Binance;
use super::ExchangeServer;
use crate::exchange::ExchangeId;
use crate::exchange::StreamSelector;
use crate::subscription::book::OrderBooksL2;
use crate::transformer::book::MultiBookTransformer;
use crate::ExchangeWsStream;

/// Level 2 OrderBook types (top of book) and spot
/// [`OrderBookUpdater`](crate::transformer::book::OrderBookUpdater) implementation.
pub mod l2;

/// [`BinanceSpot`] WebSocket server base url.
///
/// See docs: <https://binance-docs.github.io/apidocs/spot/en/#websocket-market-streams>
pub const WEBSOCKET_BASE_URL_BINANCE_SPOT: &str = "wss://stream.binance.com:9443/ws";

/// [`Binance`](super::Binance) spot exchange.
pub type BinanceSpot = Binance<BinanceServerSpot>;

/// [`Binance`](super::Binance) spot [`ExchangeServer`](super::super::ExchangeServer).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct BinanceServerSpot;

impl ExchangeServer for BinanceServerSpot {
    const ID: ExchangeId = ExchangeId::BinanceSpot;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_BINANCE_SPOT
    }
}

impl StreamSelector<OrderBooksL2> for BinanceSpot {
    type Stream =
        ExchangeWsStream<MultiBookTransformer<Self, OrderBooksL2, BinanceSpotBookUpdater>>;
}
