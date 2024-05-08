use serde::Serialize;

use crate::subscription::book::OrderBooksL2;
use crate::subscription::Subscription;
use crate::Identifier;

use super::Dydx;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct DydxChannel(pub &'static str);

impl DydxChannel {
    pub const ORDER_BOOK_L2: Self = Self("");
}

impl AsRef<str> for DydxChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl<Server> Identifier<DydxChannel> for Subscription<Dydx<Server>, OrderBooksL2> {
    fn id(&self) -> DydxChannel {
        DydxChannel::ORDER_BOOK_L2
    }
}
