use crate::event::MarketEvent;
use crate::event::MarketIter;
use crate::exchange::powertrade::channel::PowerTradeChannel;
use crate::exchange::powertrade::message::FundingRateUpdate;
use crate::exchange::powertrade::message::SubscriptionStatus;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::ExchangeId;
use crate::subscription::book::Level;
use crate::subscription::book::OrderBook;
use crate::subscription::book::OrderBookSide;
use crate::Identifier;

use barter_integration::model::instrument::Instrument;
use barter_integration::model::Exchange;
use barter_integration::model::Side;
use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// The channel sends many messages, but we only care about "pb_snapshot," so we don't have to build
/// and maintain a local order book. See:
/// - <https://power-trade.github.io/api-docs-source/ws_feeds.html#Feeds_Introduction>
/// - <https://power-trade.github.io/api-docs-source/ws_feeds.html#pb_snapshot>
#[derive(Debug, Serialize, Deserialize)]
pub enum PowerTradeOrderBookL3 {
    #[serde(rename = "pb_snapshot")]
    OrderBookL3(PriceBookSnapshot),
    #[serde(rename = "funding_rate")]
    FundingRate(FundingRateUpdate),
    #[serde(rename = "subscriptions_status")]
    SubscriptionStatus(SubscriptionStatus),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PriceBookSnapshot {
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
    levels: Vec<Vec<String>>,
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
