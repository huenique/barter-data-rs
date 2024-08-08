use barter_macro::DeExchange;
use barter_macro::SerExchange;

use crate::exchange::gateio::spot::trade::GateioSpotTrade;
use crate::exchange::gateio::Gateio;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeServer;
use crate::exchange::StreamSelector;
use crate::subscription::trade::PublicTrades;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeWsStream;

/// Public trades types.
pub mod trade;

/// [`GateioSpot`] WebSocket server base url.
///
/// See docs: <https://www.gate.io/docs/developers/apiv4/ws/en/>
pub const WEBSOCKET_BASE_URL_GATEIO_SPOT: &str = "wss://api.gateio.ws/ws/v4/";

/// [`Gateio`](super::Gateio) spot exchange.
pub type GateioSpot = Gateio<GateioServerSpot>;

/// [`Gateio`](super::Gateio) spot [`ExchangeServer`].
#[derive(
    Clone, Copy, DeExchange, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, SerExchange,
)]
pub struct GateioServerSpot;

impl ExchangeServer for GateioServerSpot {
    const ID: ExchangeId = ExchangeId::GateioSpot;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_GATEIO_SPOT
    }
}

impl StreamSelector<PublicTrades> for GateioSpot {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, GateioSpotTrade>>;
}
