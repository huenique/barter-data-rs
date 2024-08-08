use serde::Serialize;

use crate::exchange::bitfinex::Bitfinex;
use crate::subscription::trade::PublicTrades;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Bitfinex`](super::Bitfinex) channel to be subscribed to.
///
/// See docs: <https://docs.bitfinex.com/docs/ws-public>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BitfinexChannel(pub &'static str);

impl BitfinexChannel {
    /// [`Bitfinex`] real-time trades channel.
    ///
    /// See docs: <https://docs.bitfinex.com/reference/ws-public-trades>
    pub const TRADES: Self = Self("trades");
}

impl Identifier<BitfinexChannel> for Subscription<Bitfinex, PublicTrades> {
    fn id(&self) -> BitfinexChannel {
        BitfinexChannel::TRADES
    }
}

impl AsRef<str> for BitfinexChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
