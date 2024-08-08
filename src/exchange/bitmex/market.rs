use serde::Deserialize;
use serde::Serialize;

use crate::exchange::bitmex::Bitmex;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Bitmex`] market that can be subscribed to.
///
/// See docs: <https://www.bitmex.com/app/wsAPI>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BitmexMarket(pub String);

impl<Kind> Identifier<BitmexMarket> for Subscription<Bitmex, Kind> {
    fn id(&self) -> BitmexMarket {
        // Notes:
        // - Must be uppercase since Bitmex sends message with uppercase MARKET (eg/
        //   XBTUSD).
        BitmexMarket(format!("{}{}", self.instrument.base, self.instrument.quote).to_uppercase())
    }
}

impl AsRef<str> for BitmexMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
