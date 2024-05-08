use crate::exchange::dydx::Dydx;
use crate::subscription::Subscription;
use crate::Identifier;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DydxMarket {}

impl<Server, Kind> Identifier<DydxMarket> for Subscription<Dydx<Server>, Kind> {
    fn id(&self) -> DydxMarket {
        DydxMarket {}
    }
}

impl AsRef<str> for DydxMarket {
    fn as_ref(&self) -> &str {
        ""
    }
}
