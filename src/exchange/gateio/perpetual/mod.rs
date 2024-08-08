use crate::exchange::gateio::perpetual::trade::GateioFuturesTrades;
use crate::exchange::gateio::Gateio;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeServer;
use crate::exchange::StreamSelector;
use crate::subscription::trade::PublicTrades;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeWsStream;

/// Public trades types.
pub mod trade;

/// [`GateioPerpetualsUsd`] WebSocket server base url.
///
/// See docs: <https://www.gate.io/docs/developers/futures/ws/en/>
pub const WEBSOCKET_BASE_URL_GATEIO_PERPETUALS_USD: &str = "wss://fx-ws.gateio.ws/v4/ws/usdt";

/// [`Gateio`] perpetual usd exchange.
pub type GateioPerpetualsUsd = Gateio<GateioServerPerpetualsUsd>;

/// [`Gateio`] perpetual usd [`ExchangeServer`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GateioServerPerpetualsUsd;

impl ExchangeServer for GateioServerPerpetualsUsd {
    const ID: ExchangeId = ExchangeId::GateioPerpetualsUsd;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_GATEIO_PERPETUALS_USD
    }
}

impl StreamSelector<PublicTrades> for GateioPerpetualsUsd {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, GateioFuturesTrades>>;
}

/// [`GateioPerpetualsBtc`] WebSocket server base url.
///
/// See docs: <https://www.gate.io/docs/developers/futures/ws/en/>
pub const WEBSOCKET_BASE_URL_GATEIO_PERPETUALS_BTC: &str = "wss://fx-ws.gateio.ws/v4/ws/btc";

/// [`Gateio`] perpetual btc exchange.
pub type GateioPerpetualsBtc = Gateio<GateioServerPerpetualsBtc>;

/// [`Gateio`] perpetual btc [`ExchangeServer`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GateioServerPerpetualsBtc;

impl ExchangeServer for GateioServerPerpetualsBtc {
    const ID: ExchangeId = ExchangeId::GateioPerpetualsBtc;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_GATEIO_PERPETUALS_BTC
    }
}

impl StreamSelector<PublicTrades> for GateioPerpetualsBtc {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, GateioFuturesTrades>>;
}
