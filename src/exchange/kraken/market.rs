use serde::Deserialize;
use serde::Serialize;

use crate::exchange::kraken::Kraken;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Kraken`](super::Kraken) market that can be subscribed to.
///
/// See docs: <https://docs.kraken.com/websockets/#message-subscribe>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct KrakenMarket(pub String);

impl<Kind> Identifier<KrakenMarket> for Subscription<Kraken, Kind> {
    fn id(&self) -> KrakenMarket {
        KrakenMarket(format!("{}/{}", self.instrument.base, self.instrument.quote).to_uppercase())
    }
}

impl AsRef<str> for KrakenMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
