use crate::exchange::bybit::Bybit;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeServer;

/// [`BybitSpot`] WebSocket server base url.
///
/// See docs: <https://bybit-exchange.github.io/docs/v5/ws/connect>
pub const WEBSOCKET_BASE_URL_BYBIT_SPOT: &str = "wss://stream.bybit.com/v5/public/spot";

/// [`Bybit`] spot exchange.
pub type BybitSpot = Bybit<BybitServerSpot>;

/// [`Bybit`] spot [`ExchangeServer`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BybitServerSpot;

impl ExchangeServer for BybitServerSpot {
    const ID: ExchangeId = ExchangeId::BybitSpot;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_BYBIT_SPOT
    }
}
