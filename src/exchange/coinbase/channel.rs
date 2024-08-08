use serde::Serialize;

use crate::exchange::coinbase::Coinbase;
use crate::subscription::trade::PublicTrades;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Coinbase`](super::Coinbase) channel to be subscribed to.
///
/// See docs: <https://docs.cloud.coinbase.com/exchange/docs/websocket-overview#subscribe>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CoinbaseChannel(pub &'static str);

impl CoinbaseChannel {
    /// [`Coinbase`] real-time trades channel.
    ///
    /// See docs: <https://docs.cloud.coinbase.com/exchange/docs/websocket-channels#match>
    pub const TRADES: Self = Self("matches");
}

impl Identifier<CoinbaseChannel> for Subscription<Coinbase, PublicTrades> {
    fn id(&self) -> CoinbaseChannel {
        CoinbaseChannel::TRADES
    }
}

impl AsRef<str> for CoinbaseChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
