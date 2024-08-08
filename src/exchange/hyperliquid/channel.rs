use std::fmt::Debug;

use serde::Serialize;

use crate::exchange::hyperliquid::Hyperliquid;
use crate::subscription::book::OrderBooksL2;
use crate::subscription::Subscription;
use crate::Identifier;

/// Represents different channels available for subscription in the Hyperliquid
/// WebSocket API.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HyperliquidChannel(pub &'static str);

impl HyperliquidChannel {
    pub const ORDER_BOOK_L2: Self = Self("l2Book");
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
