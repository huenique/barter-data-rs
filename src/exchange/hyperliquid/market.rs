use super::Hyperliquid;
use crate::subscription::Subscription;
use crate::Identifier;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
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
