use serde::Serialize;

use crate::exchange::bitmex::Bitmex;
use crate::subscription::trade::PublicTrades;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Bitmex`](super::Bitmex) channel to be subscribed to.
///
/// See docs: <https://www.bitmex.com/app/wsAPI>
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BitmexChannel(pub &'static str);

impl BitmexChannel {
    /// [`Bitmex`] real-time trades channel name.
    ///
    /// See docs: <https://www.bitmex.com/app/wsAPI>
    pub const TRADES: Self = Self("trade");
}

impl Identifier<BitmexChannel> for Subscription<Bitmex, PublicTrades> {
    fn id(&self) -> BitmexChannel {
        BitmexChannel::TRADES
    }
}

impl AsRef<str> for BitmexChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
