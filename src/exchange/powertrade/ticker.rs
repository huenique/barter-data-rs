use std::collections::HashMap;
use std::str::FromStr;

use crate::event::MarketIter;
use crate::exchange::powertrade::channel::PowerTradeChannel;
use crate::exchange::powertrade::message::deliverable::Deliverable;
use crate::exchange::powertrade::message::deliverable::ProductType;
use crate::exchange::powertrade::message::funding_rate::FundingRate;
use crate::exchange::powertrade::message::products::option::RiskSnapshot;
use crate::exchange::powertrade::message::rte_last_trade_price::LastTradePrice;
use crate::exchange::powertrade::message::rte_trade::RteTrade;
use crate::exchange::powertrade::message::top_of_book::TopOfBook;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::exchange::Instrument;
use crate::subscription::ticker::Greeks;
use crate::subscription::ticker::Ticker;
use crate::Identifier;
use crate::MarketEvent;

use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)] // Use untagged to handle different structures without requiring a specific tag
pub enum PowerTradeTicker {
    DeliverableData {
        #[serde(rename = "deliverable")]
        deliverable: Deliverable<ProductType>,
    },
    BestBidAsk {
        #[serde(rename = "top_of_book")]
        best_bid_ask: TopOfBook,
    },
    MarkPrice {
        #[serde(rename = "funding_rate")]
        mark_price: FundingRate,
    },
    Trade {
        #[serde(rename = "rte_trade")]
        trade: RteTrade,
    },
    LastTradePrice {
        #[serde(rename = "rte_last_trade_price")]
        last_trade_price: LastTradePrice,
    },
    Greeks {
        #[serde(rename = "risk_snapshot")]
        greeks: RiskSnapshot,
    },
    Unknown(serde_json::Value),
}

impl Identifier<Option<SubscriptionId>> for PowerTradeTicker {
    fn id(&self) -> Option<SubscriptionId> {
        match self {
            PowerTradeTicker::DeliverableData { deliverable } => {
                Some(ExchangeSub::from((PowerTradeChannel::TICKER, &deliverable.symbol)).id())
            }
            _ => None,
        }
    }
}

impl From<(ExchangeId, Instrument, PowerTradeTicker)> for MarketIter<Ticker> {
    fn from((exchange_id, instrument, ticker): (ExchangeId, Instrument, PowerTradeTicker)) -> Self {
        let kind: Ticker = ticker.into();

        Self(vec![Ok(MarketEvent {
            exchange_time: DateTime::from_timestamp_nanos(kind.timestamp),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind,
        })])
    }
}

impl From<PowerTradeTicker> for Ticker {
    fn from(data: PowerTradeTicker) -> Self {
        let mut aggregate = Aggregator::new();
        aggregate.process_message(data);
        let ticker = aggregate.tickers.values().next().unwrap();
        ticker.clone()
    }
}

#[derive(Debug, Default)]
pub struct Aggregator {
    tickers: HashMap<String, Ticker>,
}

impl Aggregator {
    pub fn new() -> Self {
        Self {
            tickers: HashMap::new(),
        }
    }

    pub fn process_message(&mut self, message: PowerTradeTicker) {
        match message {
            PowerTradeTicker::DeliverableData { deliverable } => {
                self.process_deliverable_data(deliverable);
            }
            PowerTradeTicker::BestBidAsk { best_bid_ask } => {
                self.process_best_bid_ask(best_bid_ask);
            }
            PowerTradeTicker::MarkPrice { mark_price } => {
                self.process_mark_price(mark_price);
            }
            PowerTradeTicker::Trade { trade } => {
                self.process_trade(trade);
            }
            PowerTradeTicker::LastTradePrice { last_trade_price } => {
                self.process_last_trade_price(last_trade_price);
            }
            PowerTradeTicker::Greeks { greeks } => {
                self.process_greeks(greeks);
            }
            PowerTradeTicker::Unknown(_) => {}
        }
    }

    fn initialize_ticker(&mut self, symbol: String) -> &mut Ticker {
        self.tickers
            .entry(symbol.clone())
            .or_insert_with(|| Ticker {
                instrument_name: symbol.clone(),
                best_bid_price: 0.0,
                best_ask_price: 0.0,
                best_bid_amount: 0.0,
                best_ask_amount: 0.0,
                mark_price: 0.0,
                last_price: 0.0,
                open_interest: 0.0,
                greeks: None,
                timestamp: Utc::now().timestamp_nanos_opt().unwrap_or_default(),
                interest_rate: None,
                mark_iv: None,
                delivery_price: None,
                current_funding: None,
                interest_value: None,
                ask_iv: None,
                bid_iv: None,
                index_price: 0.0,
                state: String::new(),
            })
    }

    fn process_deliverable_data(&mut self, data: Deliverable<ProductType>) {
        let ticker = self.initialize_ticker(data.symbol.clone());

        match data.details {
            ProductType::Spot => {
                // ticker.instrument_name = data.symbol;
            }
            ProductType::Future => {
                // ticker.instrument_name = data.symbol;
            }
            ProductType::Option(option) => {
                let option = option.option;
                ticker.instrument_name = data.symbol;
                ticker.open_interest = option.contract_size;
                ticker.state = data.listing_status;
            }
            ProductType::Perpetual => {
                // ticker.instrument_name = data.symbol;
            }
            _ => {}
        }

        ticker.interest_rate = None;
        ticker.mark_iv = None;
        ticker.interest_value = None;
    }

    fn process_best_bid_ask(&mut self, data: TopOfBook) {
        let ticker = self.initialize_ticker(data.tradeable_entity_id.clone());

        ticker.timestamp =
            i64::from_str(&data.timestamp).expect("Failed to parse timestamp string");
        ticker.best_bid_price = data.buy_price;
        ticker.best_ask_price = data.sell_price;
        ticker.best_bid_amount = data.buy_quantity;
        ticker.best_ask_amount = data.sell_quantity;
    }

    fn process_last_trade_price(&mut self, data: LastTradePrice) {
        let ticker = self.initialize_ticker(data.tradeable_entity_id.clone());
        ticker.last_price = data.price;
    }

    fn process_trade(&mut self, data: RteTrade) {
        let ticker = self.initialize_ticker(data.tradeable_entity_id.clone());
        ticker.last_price = data.price;
    }

    fn process_mark_price(&mut self, data: FundingRate) {
        let ticker = self.initialize_ticker(data.tradeable_entity_id.clone());

        ticker.mark_price = data.mark_price;
        ticker.index_price = data.underlying_price;
        ticker.delivery_price = Some(data.underlying_price);
    }

    fn process_greeks(&mut self, data: RiskSnapshot) {
        let greeks = data.theoretical.unwrap_or_default().greeks;
        let ticker = self.initialize_ticker(data.tradeable_entity_id.clone());

        ticker.greeks = Some(Greeks {
            delta: Some(greeks.delta),
            gamma: Some(greeks.gamma),
            theta: Some(greeks.theta),
            vega: Some(greeks.vega),
            rho: Some(greeks.rho),
        });
    }

    pub fn get_ticker(&self, instrument_name: &str) -> Option<&Ticker> {
        self.tickers.get(instrument_name)
    }
}
