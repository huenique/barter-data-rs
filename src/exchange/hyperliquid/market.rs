use super::Hyperliquid;
use crate::subscription::book::OrderBooksL2;
use crate::subscription::Subscription;
use crate::Identifier;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct HyperliquidMarket(pub String);

impl Identifier<HyperliquidMarket> for Subscription<Hyperliquid, OrderBooksL2> {
    fn id(&self) -> HyperliquidMarket {
        HyperliquidMarket(
            format!("{}{}", self.instrument.base, self.instrument.quote).to_uppercase(),
        )
    }
}

impl AsRef<str> for HyperliquidMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
