use crate::exchange::gateio::perpetual::trade::GateioFuturesTrades;
use crate::exchange::gateio::Gateio;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeServer;
use crate::exchange::StreamSelector;
use crate::subscription::trade::PublicTrades;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeWsStream;

/// [`GateioOptions`] WebSocket server base url.
///
/// See docs: <https://www.gate.io/docs/developers/futures/ws/en/>
pub const WEBSOCKET_BASE_URL_GATEIO_OPTIONS_USD: &str = "wss://op-ws.gateio.live/v4/ws";

/// [`Gateio`] options exchange.
pub type GateioOptions = Gateio<GateioServerOptions>;

/// [`Gateio`] options [`ExchangeServer`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GateioServerOptions;

impl ExchangeServer for GateioServerOptions {
    const ID: ExchangeId = ExchangeId::GateioOptions;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_GATEIO_OPTIONS_USD
    }
}

impl StreamSelector<PublicTrades> for GateioOptions {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, GateioFuturesTrades>>;
}
