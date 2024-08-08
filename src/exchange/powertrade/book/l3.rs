use barter_integration::model::instrument::Instrument;
use barter_integration::model::Exchange;
use barter_integration::model::Side;
use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde_json::Value;

use crate::event::MarketEvent;
use crate::event::MarketIter;
use crate::exchange::powertrade::channel::PowerTradeChannel;
use crate::exchange::powertrade::message::pb_snapshot::OrderData;
use crate::exchange::powertrade::message::pb_snapshot::PriceBookSnapshot;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::ExchangeId;
use crate::subscription::book::Level;
use crate::subscription::book::OrderBook;
use crate::subscription::book::OrderBookSide;
use crate::Identifier;

/// The channel sends many messages, but we only care about "pb_snapshot". See:
/// - <https://power-trade.github.io/api-docs-source/ws_feeds.html#Feeds_Introduction>
/// - <https://power-trade.github.io/api-docs-source/ws_feeds.html#pb_snapshot>
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum PowerTradeOrderBookL3 {
    OrderBookL3(Box<PriceBookSnapshot>),
    Ignored,
}

impl<'de> Deserialize<'de> for PowerTradeOrderBookL3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        if let Some(snapshot) = value.get("pb_snapshot") {
            let parsed: Result<PriceBookSnapshot, _> = serde_json::from_value(snapshot.clone());
            if let Ok(snapshot) = parsed {
                return Ok(PowerTradeOrderBookL3::OrderBookL3(Box::new(snapshot)));
            }
        }
        Ok(PowerTradeOrderBookL3::Ignored)
    }
}

impl Identifier<Option<SubscriptionId>> for PowerTradeOrderBookL3 {
    fn id(&self) -> Option<SubscriptionId> {
        match self {
            PowerTradeOrderBookL3::OrderBookL3(book) => {
                Some(ExchangeSub::from((PowerTradeChannel::ORDER_BOOK_L3, &book.symbol)).id())
            }
            _ => None,
        }
    }
}

impl From<(ExchangeId, Instrument, PowerTradeOrderBookL3)> for MarketIter<OrderBook> {
    fn from(
        (exchange_id, instrument, book): (ExchangeId, Instrument, PowerTradeOrderBookL3),
    ) -> Self {
        match book {
            PowerTradeOrderBookL3::OrderBookL3(book) => {
                let timestamp = DateTime::parse_from_rfc3339(&book.timestamp)
                    .unwrap_or_else(|_| Utc::now().into())
                    .with_timezone(&Utc);

                Self(vec![Ok(MarketEvent {
                    exchange_time: timestamp,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument,
                    kind: OrderBook {
                        last_update_time: timestamp,
                        bids: parse_order_data(Side::Buy, &book.bids),
                        asks: parse_order_data(Side::Sell, &book.asks),
                    },
                })])
            }
            _ => Self(vec![]),
        }
    }
}

fn parse_order_data(side: Side, data: &OrderData) -> OrderBookSide {
    OrderBookSide::new(
        side,
        data.levels
            .iter()
            .map(|level_group| {
                let price = &level_group[0];
                let amount = &level_group[1];
                Level {
                    price: price.parse::<f64>().unwrap(),
                    amount: amount.parse::<f64>().unwrap(),
                }
            })
            .collect::<Vec<Level>>(),
    )
}
