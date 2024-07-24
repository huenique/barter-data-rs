use serde::Serialize;

use crate::subscription::ticker::Tickers;
use crate::subscription::Subscription;
use crate::Identifier;

use super::Lyra;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct LyraChannel(pub &'static str);

impl LyraChannel {
    /// [`Lyra`](super::Lyra) real-time Ticker channel.
    ///
    /// See docs: <https://docs.lyra.finance/reference/ticker-instrument_name-interval>
    pub const TICKER: Self = Self("ticker.{}.100");
}

impl Identifier<LyraChannel> for Subscription<Lyra, Tickers> {
    fn id(&self) -> LyraChannel {
        LyraChannel::TICKER
    }
}

impl AsRef<str> for LyraChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}
