use serde::Serialize;

use crate::exchange::okx::Okx;
use crate::subscription::trade::PublicTrades;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Okx`](super::Okx) channel to be subscribed to.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OkxChannel(pub &'static str);

impl OkxChannel {
    /// [`Okx`] real-time trades channel.
    ///
    /// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel-trades-channel>
    pub const TRADES: Self = Self("trades");
}

impl Identifier<OkxChannel> for Subscription<Okx, PublicTrades> {
    fn id(&self) -> OkxChannel {
        OkxChannel::TRADES
    }
}

impl AsRef<str> for OkxChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
