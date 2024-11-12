use barter_integration::model::Side;
use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::TimeZone as _;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::event::MarketIter;
use crate::exchange::derive::book::DeriveLevel;
use crate::exchange::derive::channel::DeriveChannel;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::Instrument;
use crate::subscription::book::OrderBook;
use crate::subscription::book::OrderBookSide;
use crate::ExchangeId;
use crate::Identifier;
use crate::MarketEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookMessage {
    pub method: String,
    pub params: Params,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    pub channel: String,
    pub data: DeriveInstrumentOrderBook,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeriveInstrumentOrderBook {
    pub timestamp: u64,
    pub instrument_name: String,
    pub publish_id: u64,
    pub bids: Vec<DeriveLevel>,
    pub asks: Vec<DeriveLevel>,
}

pub type DeriveOrderBookL2 = OrderBookMessage;

impl Identifier<Option<SubscriptionId>> for DeriveOrderBookL2 {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((DeriveChannel::ORDER_BOOK, &self.params.data.instrument_name)).id())
    }
}

impl From<(ExchangeId, Instrument, DeriveOrderBookL2)> for MarketIter<OrderBook> {
    fn from(
        (exchange_id, instrument, order_book): (ExchangeId, Instrument, DeriveOrderBookL2),
    ) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: DateTime::parse_from_rfc3339(
                &order_book.params.data.timestamp.to_string(),
            )
            .unwrap_or_else(|_| Utc::now().into())
            .with_timezone(&Utc),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind: order_book.params.data.into(),
        })])
    }
}

impl From<DeriveInstrumentOrderBook> for OrderBook {
    fn from(data: DeriveInstrumentOrderBook) -> Self {
        OrderBook {
            instrument_name: data.instrument_name,
            last_update_time: Utc.from_utc_datetime(
                &DateTime::<Utc>::from_timestamp(data.timestamp as i64, 0)
                    .expect("Invalid timestamp")
                    .naive_utc(),
            ),
            bids: OrderBookSide::new(Side::Buy, data.bids),
            asks: OrderBookSide::new(Side::Sell, data.asks),
        }
    }
}
