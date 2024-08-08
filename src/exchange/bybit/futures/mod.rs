use crate::exchange::bybit::Bybit;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeServer;

/// [`BybitPerpetualsUsd`] WebSocket server base url.
///
/// See docs: <https://bybit-exchange.github.io/docs/v5/ws/connect>
pub const WEBSOCKET_BASE_URL_BYBIT_PERPETUALS_USD: &str = "wss://stream.bybit.com/v5/public/linear";

/// [`Bybit`] perpetual exchange.
pub type BybitPerpetualsUsd = Bybit<BybitServerPerpetualsUsd>;

/// [`Bybit`] perpetual [`ExchangeServer`].
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BybitServerPerpetualsUsd;

impl ExchangeServer for BybitServerPerpetualsUsd {
    const ID: ExchangeId = ExchangeId::BybitPerpetualsUsd;

    fn websocket_url() -> &'static str {
        WEBSOCKET_BASE_URL_BYBIT_PERPETUALS_USD
    }
}
