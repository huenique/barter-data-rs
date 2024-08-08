use crate::exchange::gateio::perpetual::trade::GateioFuturesTrades;
use crate::exchange::gateio::Gateio;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeServer;
use crate::exchange::StreamSelector;
use crate::subscription::trade::PublicTrades;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeWsStream;

/// [`GateioFuturesUsd`] WebSocket server base url.
///
/// See docs: <https://www.gate.io/docs/developers/delivery/ws/en/>
pub const WEBSOCKET_BASE_URL_GATEIO_FUTURES_USD: &str = "wss://fx-ws.gateio.ws/v4/ws/delivery/usdt";

/// [`Gateio`] perpetual usd exchange.
pub type GateioFuturesUsd = Gateio<GateioServerFuturesUsd>;

/// [`Gateio`] perpetual usd [`ExchangeServer`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GateioServerFuturesUsd;

impl ExchangeServer for GateioServerFuturesUsd {
    const ID: ExchangeId = ExchangeId::GateioFuturesUsd;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_GATEIO_FUTURES_USD
    }
}

impl StreamSelector<PublicTrades> for GateioFuturesUsd {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, GateioFuturesTrades>>;
}

/// [`GateioFuturesBtc`] WebSocket server base url.
///
/// See docs: <https://www.gate.io/docs/developers/delivery/ws/en/>
pub const WEBSOCKET_BASE_URL_GATEIO_FUTURES_BTC: &str = "wss://fx-ws.gateio.ws/v4/ws/delivery/btc";

/// [`Gateio`] perpetual btc exchange.
pub type GateioFuturesBtc = Gateio<GateioServerFuturesBtc>;

/// [`Gateio`] perpetual btc [`ExchangeServer`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GateioServerFuturesBtc;

impl ExchangeServer for GateioServerFuturesBtc {
    const ID: ExchangeId = ExchangeId::GateioFuturesBtc;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_GATEIO_FUTURES_BTC
    }
}

impl StreamSelector<PublicTrades> for GateioFuturesBtc {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, GateioFuturesTrades>>;
}
