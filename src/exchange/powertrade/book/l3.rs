use barter_integration::model::instrument::Instrument;
use barter_integration::model::Exchange;
use barter_integration::model::Side;
use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::event::MarketEvent;
use crate::event::MarketIter;
use crate::exchange::powertrade::channel::PowerTradeChannel;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::ExchangeId;
use crate::subscription::book::Level;
use crate::subscription::book::OrderBook;
use crate::subscription::book::OrderBookSide;
use crate::Identifier;

/// See: <https://power-trade.github.io/api-docs-source/ws_feeds.html#ob_snapshot>
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct PowerTradeOrderBookL3 {
    pub timestamp: DateTime<Utc>,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub symbol: String,
    pub bids: PowerTradeOrderBookL3Bids,
    pub asks: PowerTradeOrderBookL3Asks,
}

type Price = f64;
type Size = f64;
type DisplayOrderId = String;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct PowerTradeOrderBookL3Bids {
    pub n_levels: String,
    pub n_orders: String,
    pub levels: Vec<Vec<(Price, Size, DisplayOrderId)>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct PowerTradeOrderBookL3Asks {
    pub n_levels: String,
    pub n_orders: String,
    pub levels: Vec<Vec<(Price, Size, DisplayOrderId)>>,
}

impl Identifier<Option<SubscriptionId>> for PowerTradeOrderBookL3 {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((PowerTradeChannel::ORDER_BOOK_L3, &self.symbol)).id())
    }
}

impl From<(ExchangeId, Instrument, PowerTradeOrderBookL3)> for MarketIter<OrderBook> {
    fn from(
        (exchange_id, instrument, book): (ExchangeId, Instrument, PowerTradeOrderBookL3),
    ) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: book.timestamp,
            received_time: Utc::now(),
            exchange: Exchange::from(exchange_id),
            instrument,
            kind: OrderBook {
                last_update_time: book.timestamp,
                bids: OrderBookSide::new(
                    Side::Buy,
                    book.bids
                        .levels
                        .iter()
                        .map(|level| Level::new(level[0].0, level[0].1))
                        .collect::<Vec<Level>>(),
                ),
                asks: OrderBookSide::new(
                    Side::Sell,
                    book.asks
                        .levels
                        .iter()
                        .map(|level| Level::new(level[0].0, level[0].1))
                        .collect::<Vec<Level>>(),
                ),
            },
        })])
    }
}
