use std::str::FromStr;

use barter_integration::model::instrument::Instrument;
use barter_integration::model::Exchange;
use barter_integration::model::Side;
use barter_integration::model::SubscriptionId;
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
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
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PowerTradeOrderBookL3 {
    #[serde(default)]
    pub ob_snapshot: PowerTradeOrderBook,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PowerTradeOrderBook {
    timestamp: String,
    tradeable_entity_id: String,
    market_id: String,
    symbol: String,
    bids: OrderData,
    asks: OrderData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrderData {
    n_levels: String,
    n_orders: String,
    levels: Vec<Vec<Vec<String>>>,
}

impl Identifier<Option<SubscriptionId>> for PowerTradeOrderBookL3 {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((PowerTradeChannel::ORDER_BOOK_L3, &self.ob_snapshot.symbol)).id())
    }
}

impl From<(ExchangeId, Instrument, PowerTradeOrderBookL3)> for MarketIter<OrderBook> {
    fn from(
        (exchange_id, instrument, book): (ExchangeId, Instrument, PowerTradeOrderBookL3),
    ) -> Self {
        let timestamp = DateTime::parse_from_rfc3339(&book.ob_snapshot.timestamp)
            .unwrap_or_else(|_| Utc::now().into())
            .with_timezone(&Utc);

        Self(vec![Ok(MarketEvent {
            exchange_time: timestamp,
            received_time: Utc::now(),
            exchange: Exchange::from(exchange_id),
            instrument,
            kind: OrderBook {
                last_update_time: timestamp,
                bids: parse_order_data(Side::Buy, &book.ob_snapshot.bids),
                asks: parse_order_data(Side::Sell, &book.ob_snapshot.asks),
            },
        })])
    }
}

fn parse_order_data(side: Side, data: &OrderData) -> OrderBookSide {
    OrderBookSide::new(
        side,
        data.levels
            .iter()
            .flat_map(|level_group| {
                level_group.iter().map(|level| {
                    let price =
                        BigDecimal::from_str(&level[0]).unwrap_or_else(|_| BigDecimal::from(0));
                    let amount =
                        BigDecimal::from_str(&level[1]).unwrap_or_else(|_| BigDecimal::from(0));
                    Level {
                        price: bigdecimal_to_f64(price),
                        amount: bigdecimal_to_f64(amount),
                    }
                })
            })
            .collect::<Vec<Level>>(),
    )
}

fn bigdecimal_to_f64(value: BigDecimal) -> f64 {
    value.to_f64().unwrap_or_default()
}
