use serde::Serialize;

use crate::exchange::dydx::Dydx;
use crate::subscription::book::OrderBooksL2;
use crate::subscription::Subscription;
use crate::Identifier;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DydxChannel(pub &'static str);

impl DydxChannel {
    pub const ORDER_BOOK_L2: Self = Self("v4_orderbook");
}

impl AsRef<str> for DydxChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Identifier<DydxChannel> for Subscription<Dydx, OrderBooksL2> {
    fn id(&self) -> DydxChannel {
        DydxChannel::ORDER_BOOK_L2
    }
}
