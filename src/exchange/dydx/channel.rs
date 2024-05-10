use serde::Serialize;

use crate::subscription::book::OrderBooksL3;
use crate::subscription::Subscription;
use crate::Identifier;

use super::Dydx;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct DydxChannel(pub &'static str);

impl DydxChannel {
    pub const ORDER_BOOK_L3: Self = Self("v4_orderbook");
}

impl AsRef<str> for DydxChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Identifier<DydxChannel> for Subscription<Dydx, OrderBooksL3> {
    fn id(&self) -> DydxChannel {
        DydxChannel::ORDER_BOOK_L3
    }
}
