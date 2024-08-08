use serde::Deserialize;
use serde::Serialize;

use crate::exchange::bitfinex::Bitfinex;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Bitfinex`](super::Bitfinex) market that can be subscribed to.
///
/// See docs: <https://docs.bitfinex.com/docs/ws-public>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BitfinexMarket(pub String);

impl<Kind> Identifier<BitfinexMarket> for Subscription<Bitfinex, Kind> {
    fn id(&self) -> BitfinexMarket {
        BitfinexMarket(format!(
            "t{}{}",
            self.instrument.base.to_string().to_uppercase(),
            self.instrument.quote.to_string().to_uppercase()
        ))
    }
}

impl AsRef<str> for BitfinexMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
