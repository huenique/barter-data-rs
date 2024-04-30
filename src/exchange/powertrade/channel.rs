use crate::exchange::powertrade::PowerTrade;
use crate::subscription::book::OrderBooksL3;
use crate::Identifier;
use crate::Subscription;
use serde::Serialize;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct PowerTradeChannel(pub &'static str);

impl PowerTradeChannel {
    pub const ORDER_BOOK_L3: Self = Self("{}");
}

impl AsRef<str> for PowerTradeChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Identifier<PowerTradeChannel> for Subscription<PowerTrade, OrderBooksL3> {
    fn id(&self) -> PowerTradeChannel {
        PowerTradeChannel::ORDER_BOOK_L3
    }
}
