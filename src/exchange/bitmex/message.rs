use barter_integration::model::SubscriptionId;
use serde::Deserialize;
use serde::Serialize;

use crate::exchange::bitmex::trade::BitmexTrade;
use crate::Identifier;

/// ### Raw Payload Examples
/// See docs: <https://www.bitmex.com/app/wsAPI#Response-Format>
/// #### Trade payload
/// ```json
/// {
///     "table": "trade",
///     "action": "insert",
///     "data": [
///         {
///             "timestamp": "2023-02-18T09:27:59.701Z",
///             "symbol": "XBTUSD",
///             "side": "Sell",
///             "size": 200,
///             "price": 24564.5,
///             "tickDirection": "MinusTick",
///             "trdMatchID": "31e50cb7-e005-a44e-f354-86e88dff52eb",
///             "grossValue": 814184,
///             "homeNotional": 0.00814184,
///             "foreignNotional": 200,
///             "trdType": "Regular"
///         }
///     ]
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BitmexMessage<T> {
    pub table: String,
    pub data: Vec<T>,
}

impl Identifier<Option<SubscriptionId>> for BitmexTrade {
    fn id(&self) -> Option<SubscriptionId> {
        self.data
            .first()
            .map(|trade| SubscriptionId(format!("{}|{}", self.table, trade.symbol)))
            .or(None)
    }
}
