use std::collections::HashMap;

use crate::exchange::powertrade::channel::PowerTradeChannel;
use crate::exchange::powertrade::message::deliverable::Deliverable;
use crate::exchange::powertrade::message::funding_rate::FundingRate;
use crate::exchange::powertrade::message::products::option::OptionDetails;
use crate::exchange::powertrade::message::rte_last_trade_price::LastTradePrirce;
use crate::exchange::powertrade::message::rte_trade::RteTrade;
use crate::exchange::powertrade::message::top_of_book::TopOfBook;
use crate::exchange::ExchangeSub;
use crate::subscription::ticker::Greeks;
use crate::subscription::ticker::Ticker;
use crate::Identifier;

use super::message::products::option::RiskSnapshot;
use barter_integration::model::SubscriptionId;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProductType {
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "future")]
    Future,
    #[serde(rename = "option")]
    Option(Box<OptionDetails>),
    #[serde(rename = "perpetual")]
    Perpetual,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum PowerTradeTicker {
    #[serde(rename = "deliverable")]
    DeliverableData(Deliverable<ProductType>),
    #[serde(rename = "top_of_book")]
    BestBidAsk(TopOfBook),
    #[serde(rename = "funding_rate")]
    MarkPrice(FundingRate),
    #[serde(rename = "rte_trade")]
    Trade(RteTrade),
    #[serde(rename = "rte_last_trade_price")]
    LastTradePrice(LastTradePrirce),
    #[serde(rename = "risk_snapshot")]
    Greeks(RiskSnapshot),
}

impl Identifier<Option<SubscriptionId>> for PowerTradeTicker {
    fn id(&self) -> Option<SubscriptionId> {
        match self {
            PowerTradeTicker::DeliverableData(data) => {
                Some(ExchangeSub::from((PowerTradeChannel::TICKER, &data.symbol)).id())
            }
            _ => None,
        }
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
            PowerTradeTicker::DeliverableData(data) => self.process_deliverable_data(data),
            PowerTradeTicker::BestBidAsk(data) => self.process_best_bid_ask(data),
            PowerTradeTicker::MarkPrice(data) => self.process_mark_price(data),
            PowerTradeTicker::Trade(data) => self.process_trade(data),
            PowerTradeTicker::LastTradePrice(data) => self.process_last_trade_price(data),
            PowerTradeTicker::Greeks(data) => self.process_greeks(data),
        }
    }

    fn process_deliverable_data(&mut self, data: Deliverable<ProductType>) {
        let timestamp = Utc::now().timestamp() as u64;
        let ticker = self
            .tickers
            .entry(data.symbol.clone())
            .or_insert_with(|| Ticker {
                instrument_name: data.symbol.clone(),
                best_bid_price: 0.0,
                best_ask_price: 0.0,
                best_bid_amount: 0.0,
                best_ask_amount: 0.0,
                mark_price: 0.0,
                last_price: 0.0,
                open_interest: 0.0,
                greeks: None,
                timestamp,
                interest_rate: None,
                mark_iv: None,
                delivery_price: None,
                current_funding: None,
                interest_value: None,
                ask_iv: None,
                bid_iv: None,
                index_price: 0.0,
                state: String::new(),
            });

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
        }

        ticker.timestamp = timestamp;
        ticker.interest_rate = None;
        ticker.mark_iv = None;
        ticker.interest_value = None;
    }

    fn process_best_bid_ask(&mut self, data: TopOfBook) {
        let ticker = self
            .tickers
            .entry(data.tradeable_entity_id.clone())
            .or_insert_with(|| Ticker {
                instrument_name: "".into(),
                best_bid_price: data.buy_price,
                best_ask_price: data.sell_price,
                best_bid_amount: data.buy_quantity,
                best_ask_amount: data.sell_quantity,
                mark_price: 0.0,
                last_price: 0.0,
                open_interest: 0.0,
                greeks: None,
                timestamp: 0,
                interest_rate: None,
                mark_iv: None,
                delivery_price: None,
                current_funding: None,
                interest_value: None,
                ask_iv: None,
                bid_iv: None,
                index_price: 0.0,
                state: String::new(),
            });

        ticker.best_bid_price = data.buy_price;
        ticker.best_ask_price = data.sell_price;
        ticker.best_bid_amount = data.buy_quantity;
        ticker.best_ask_amount = data.sell_quantity;
    }

    fn process_last_trade_price(&mut self, data: LastTradePrirce) {
        let ticker = self
            .tickers
            .entry(data.tradeable_entity_id.clone())
            .or_insert_with(|| Ticker {
                instrument_name: "".into(),
                best_bid_price: 0.0,
                best_ask_price: 0.0,
                best_bid_amount: 0.0,
                best_ask_amount: 0.0,
                mark_price: 0.0,
                last_price: 0.0,
                open_interest: 0.0,
                greeks: None,
                timestamp: 0,
                interest_rate: None,
                mark_iv: None,
                delivery_price: None,
                current_funding: None,
                interest_value: None,
                ask_iv: None,
                bid_iv: None,
                index_price: 0.0,
                state: String::new(),
            });

        ticker.last_price = data.price;
    }

    fn process_trade(&mut self, data: RteTrade) {
        let ticker = self
            .tickers
            .entry(data.tradeable_entity_id.clone())
            .or_insert_with(|| Ticker {
                instrument_name: "".into(),
                best_bid_price: 0.0,
                best_ask_price: 0.0,
                best_bid_amount: 0.0,
                best_ask_amount: 0.0,
                mark_price: 0.0,
                last_price: 0.0,
                open_interest: 0.0,
                greeks: None,
                timestamp: 0,
                interest_rate: None,
                mark_iv: None,
                delivery_price: None,
                current_funding: None,
                interest_value: None,
                ask_iv: None,
                bid_iv: None,
                index_price: 0.0,
                state: String::new(),
            });

        ticker.last_price = data.price;
    }

    fn process_mark_price(&mut self, data: FundingRate) {
        let ticker = self
            .tickers
            .entry(data.tradeable_entity_id.clone())
            .or_insert_with(|| Ticker {
                instrument_name: "".into(),
                best_bid_price: 0.0,
                best_ask_price: 0.0,
                best_bid_amount: 0.0,
                best_ask_amount: 0.0,
                mark_price: data.mark_price,
                last_price: 0.0,
                open_interest: 0.0,
                greeks: None,
                timestamp: 0,
                interest_rate: None,
                mark_iv: None,
                delivery_price: None,
                current_funding: None,
                interest_value: None,
                ask_iv: None,
                bid_iv: None,
                index_price: data.underlying_price,
                state: String::new(),
            });

        ticker.mark_price = data.mark_price;
        ticker.index_price = data.underlying_price;
        ticker.delivery_price = Some(data.underlying_price);
    }

    fn process_greeks(&mut self, data: RiskSnapshot) {
        let greeks = data.theoretical.unwrap_or_default().greeks;
        let ticker = self
            .tickers
            .entry(data.tradeable_entity_id.clone())
            .or_insert_with(|| Ticker {
                instrument_name: data.symbol.clone(),
                best_bid_price: 0.0,
                best_ask_price: 0.0,
                best_bid_amount: 0.0,
                best_ask_amount: 0.0,
                mark_price: 0.0,
                last_price: 0.0,
                open_interest: 0.0,
                greeks: Some(Greeks {
                    delta: Some(greeks.delta),
                    gamma: Some(greeks.gamma),
                    theta: Some(greeks.theta),
                    vega: Some(greeks.vega),
                    rho: Some(greeks.rho),
                }),
                timestamp: 0,
                interest_rate: None,
                mark_iv: None,
                delivery_price: None,
                current_funding: None,
                interest_value: None,
                ask_iv: Some(data.ask.unwrap_or_default().volatility),
                bid_iv: Some(data.bid.unwrap_or_default().volatility),
                index_price: 0.0,
                state: String::new(),
            });

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
