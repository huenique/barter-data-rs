use serde::Deserialize;
use serde::Serialize;

use crate::exchange::hyperliquid::Hyperliquid;
use crate::subscription::Subscription;
use crate::Identifier;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HyperliquidMarket(pub String);

impl<Kind> Identifier<HyperliquidMarket> for Subscription<Hyperliquid, Kind> {
    fn id(&self) -> HyperliquidMarket {
        HyperliquidMarket(format!("{}", self.instrument.base).to_uppercase())
    }
}

impl AsRef<str> for HyperliquidMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
