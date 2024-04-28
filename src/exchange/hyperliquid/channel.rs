use std::fmt::Debug;

use crate::exchange::hyperliquid::Hyperliquid;
use crate::subscription::book::OrderBooksL2;
use crate::subscription::Subscription;
use crate::Identifier;
use serde::Serialize;

/// Represents different channels available for subscription in the Hyperliquid WebSocket API.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct HyperliquidChannel(pub &'static str);

impl HyperliquidChannel {
    /// Channel for receiving trade data updates for a specific coin.
    pub const TRADES: Self = Self("trades");

    /// Channel for receiving level 2 order book updates for a specific coin.
    pub const ORDER_BOOK_L2: Self = Self("l2Book");

    /// Channel for receiving candle data for a specific coin and interval.
    pub const CANDLE: Self = Self("candle");
}

impl AsRef<str> for HyperliquidChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Identifier<HyperliquidChannel> for Subscription<Hyperliquid, OrderBooksL2> {
    fn id(&self) -> HyperliquidChannel {
        HyperliquidChannel::ORDER_BOOK_L2
    }
}
