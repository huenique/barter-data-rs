use serde::Deserialize;
use serde::Serialize;

use crate::exchange::coinbase::Coinbase;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Coinbase`](super::Coinbase) market that can be subscribed to.
///
/// See docs: <https://docs.cloud.coinbase.com/exchange/docs/websocket-overview#subscribe>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CoinbaseMarket(pub String);

impl<Kind> Identifier<CoinbaseMarket> for Subscription<Coinbase, Kind> {
    fn id(&self) -> CoinbaseMarket {
        CoinbaseMarket(format!("{}-{}", self.instrument.base, self.instrument.quote).to_uppercase())
    }
}

impl AsRef<str> for CoinbaseMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
