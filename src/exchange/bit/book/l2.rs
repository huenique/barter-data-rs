use barter_integration::model::instrument::Instrument;
use barter_integration::model::Exchange;
use barter_integration::model::Side;
use barter_integration::model::SubscriptionId;
use chrono::TimeZone;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::event::MarketEvent;
use crate::event::MarketIter;
use crate::exchange::bit::book::BitLevel;
use crate::exchange::bit::message::BitPong;
use crate::exchange::bit::message::BitWsMessage;
use crate::exchange::bit::BitChannel;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::ExchangeId;
use crate::subscription::book::OrderBook;
use crate::subscription::book::OrderBookSide;
use crate::Identifier;

pub type BitOrderBookL2 = BitWsMessage<BitOrderBookL2Snapshot, BitPong>;

impl Identifier<Option<SubscriptionId>> for BitOrderBookL2 {
    fn id(&self) -> Option<SubscriptionId> {
        match self {
            BitWsMessage::Channel(sub) => {
                Some(ExchangeSub::from((BitChannel::ORDER_BOOK_L2, &sub.data.instrument_id)).id())
            }
            _ => None,
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitOrderBookL2Snapshot {
    pub channel: String,
    pub timestamp: i64,
    pub module: String,
    pub data: OrderBookData,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderBookData {
    pub asks: Vec<BitLevel>,
    pub bids: Vec<BitLevel>,
    pub display_name: String,
    pub instrument_id: String,
    pub sequence: i64,
    pub timestamp: i64,
}

impl From<(ExchangeId, Instrument, BitOrderBookL2)> for MarketIter<OrderBook> {
    fn from((exchange_id, instrument, book): (ExchangeId, Instrument, BitOrderBookL2)) -> Self {
        Self(match book {
            BitWsMessage::Channel(sub) => {
                vec![Ok(MarketEvent {
                    exchange_time: Utc.timestamp_millis_opt(sub.timestamp).unwrap(),
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument,
                    kind: OrderBook {
                        last_update_time: Utc.timestamp_millis_opt(sub.timestamp).unwrap(),
                        bids: OrderBookSide::new(Side::Sell, sub.data.bids),
                        asks: OrderBookSide::new(Side::Buy, sub.data.asks),
                    },
                })]
            }
            _ => vec![],
        })
    }
}
