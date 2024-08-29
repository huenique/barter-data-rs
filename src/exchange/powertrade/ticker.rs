use std::error::Error;

use async_trait::async_trait;
use barter_integration::error::SocketError;
use barter_integration::model::instrument::Instrument;
use barter_integration::model::SubscriptionId;
use barter_integration::protocol::websocket::WsMessage;
use cached::proc_macro::cached;
use chrono::TimeZone;
use chrono::Utc;
use serde::de::Error as _;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;
use tracing::debug;

use crate::error::DataError;
use crate::event::MarketEvent;
use crate::event::MarketIter;
use crate::exchange::powertrade::channel::PowerTradeChannel;
use crate::exchange::powertrade::message::deliverable::Deliverable;
use crate::exchange::powertrade::message::deliverable::ProductType;
use crate::exchange::powertrade::message::funding_rate::FundingRate;
use crate::exchange::powertrade::message::last_trade_price::LastTradePrice;
use crate::exchange::powertrade::message::products::option::RiskSnapshot;
use crate::exchange::powertrade::message::rte_last_trade_price::RteLastTradePrice;
use crate::exchange::powertrade::message::rte_trade::RteTrade;
use crate::exchange::powertrade::message::top_of_book::TopOfBook;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::subscription::ticker::Greeks;
use crate::subscription::ticker::Ticker;
use crate::subscription::ticker::TickerState;
use crate::transformer::ticker::InstrumentTicker;
use crate::transformer::ticker::TickerUpdater;
use crate::Identifier;

const POWERTRADE_TRADEABLE_ENTITY_API: &str =
    "https://api.rest.prod.power.trade/v1/market_data/tradeable_entity/";

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerTradeInstrumentSummary {
    pub symbol: String,
    pub base_volume: String,
    pub volume: String,
    pub price_change: String,
    pub low_24: String,
    pub high_24: String,
    pub last_price: String,
    pub open_interest: String,
    pub best_bid: String,
    pub best_ask: String,
    pub index_price: String,
    pub product_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PowerTradeTicker {
    Deliverable {
        #[serde(rename = "deliverable")]
        deliverable: Deliverable<ProductType>,
    },
    TopOfBook {
        #[serde(rename = "top_of_book")]
        top_of_book: TopOfBook,
    },
    FundingRate {
        #[serde(rename = "funding_rate")]
        funding_rate: FundingRate,
    },
    Trade {
        #[serde(rename = "rte_trade")]
        rte_trade: RteTrade,
    },
    LastTradePrice {
        #[serde(rename = "last_trade_price")]
        last_trade_price: LastTradePrice,
    },
    RteLastTradePrice {
        #[serde(rename = "rte_last_trade_price")]
        rte_last_trade_price: RteLastTradePrice,
    },
    RiskSnapshot {
        #[serde(rename = "risk_snapshot")]
        risk_snapshot: RiskSnapshot,
    },
    Unknown(serde_json::Value),
}

const UNSUPPORTED: &str = "Unsupported PowerTrade ProductType variant";
const FAILED_FETCH: &str = "Failed to fetch symbol";

impl Identifier<Option<SubscriptionId>> for PowerTradeTicker {
    fn id(&self) -> Option<SubscriptionId> {
        match self {
            PowerTradeTicker::Deliverable { deliverable } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    match &deliverable.details {
                        ProductType::Option(_) => deliverable.symbol.clone(),
                        _ => UNSUPPORTED.into(),
                    },
                ))
                .id(),
            ),
            PowerTradeTicker::TopOfBook { top_of_book } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    fetch_symbol(&top_of_book.tradeable_entity_id).unwrap_or(format!(
                        "BestBidAsk {}: {}",
                        FAILED_FETCH, top_of_book.tradeable_entity_id
                    )),
                ))
                .id(),
            ),
            PowerTradeTicker::FundingRate { funding_rate } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    fetch_symbol(&funding_rate.tradeable_entity_id).unwrap_or(format!(
                        "MarkPrice {}: {}",
                        FAILED_FETCH, funding_rate.tradeable_entity_id
                    )),
                ))
                .id(),
            ),
            PowerTradeTicker::Trade { rte_trade } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    fetch_symbol(&rte_trade.tradeable_entity_id).unwrap_or(format!(
                        "Trade {}: {}",
                        FAILED_FETCH, rte_trade.tradeable_entity_id
                    )),
                ))
                .id(),
            ),
            PowerTradeTicker::LastTradePrice { last_trade_price } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    fetch_symbol(&last_trade_price.tradeable_entity_id).unwrap_or(format!(
                        "LastTradePrice {}: {}",
                        FAILED_FETCH, last_trade_price.tradeable_entity_id
                    )),
                ))
                .id(),
            ),
            PowerTradeTicker::RteLastTradePrice {
                rte_last_trade_price,
            } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    fetch_symbol(&rte_last_trade_price.tradeable_entity_id).unwrap_or(format!(
                        "RteLastTradePrice {}: {}",
                        FAILED_FETCH, rte_last_trade_price.tradeable_entity_id
                    )),
                ))
                .id(),
            ),
            PowerTradeTicker::RiskSnapshot {
                risk_snapshot: greeks,
            } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    fetch_symbol(&greeks.tradeable_entity_id).unwrap_or(format!(
                        "Greeks {}: {}",
                        FAILED_FETCH, greeks.tradeable_entity_id
                    )),
                ))
                .id(),
            ),
            PowerTradeTicker::Unknown(_) => None,
        }
    }
}

impl From<(ExchangeId, Instrument, PowerTradeTicker)> for MarketIter<Ticker> {
    fn from((exchange_id, instrument, ticker): (ExchangeId, Instrument, PowerTradeTicker)) -> Self {
        let kind: Ticker = ticker.into();

        Self(vec![Ok(MarketEvent {
            exchange_time: Utc.timestamp_nanos(kind.timestamp),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind,
        })])
    }
}

impl From<PowerTradeTicker> for Ticker {
    fn from(data: PowerTradeTicker) -> Self {
        let mut aggregate = PowerTradeTickerAggregator::new();
        aggregate.process_message(data);
        aggregate.ticker.clone()
    }
}

enum LTPrice {
    LastTradePrice(LastTradePrice),
    RteLastTradePrice(RteLastTradePrice),
}

#[derive(Debug, Default)]
pub struct PowerTradeTickerAggregator {
    ticker: Ticker,
}

impl PowerTradeTickerAggregator {
    pub fn new() -> Self {
        Self {
            ticker: Ticker::default(),
        }
    }

    // TODO: Ensure timestamp is updated on every message
    pub fn process_message(&mut self, message: PowerTradeTicker) {
        match message {
            PowerTradeTicker::Deliverable { deliverable } => {
                debug!("Processing deliverable data: {:?}", deliverable);
                self.process_deliverable_data(deliverable);
            }
            PowerTradeTicker::TopOfBook { top_of_book } => {
                debug!("Processing best bid ask: {:?}", top_of_book);
                self.process_best_bid_ask(top_of_book);
            }
            PowerTradeTicker::FundingRate { funding_rate } => {
                debug!("Processing mark price: {:?}", funding_rate);
                self.process_mark_price(funding_rate);
            }
            PowerTradeTicker::Trade { rte_trade } => {
                debug!("Processing trade: {:?}", rte_trade);
                self.process_trade(rte_trade);
            }
            PowerTradeTicker::LastTradePrice { last_trade_price } => {
                debug!("Processing last trade price: {:?}", last_trade_price);
                self.process_last_trade_price(LTPrice::LastTradePrice(last_trade_price));
            }
            PowerTradeTicker::RteLastTradePrice {
                rte_last_trade_price,
            } => {
                debug!(
                    "Processing rte last trade price: {:?}",
                    rte_last_trade_price
                );
                self.process_last_trade_price(LTPrice::RteLastTradePrice(rte_last_trade_price));
            }
            PowerTradeTicker::RiskSnapshot {
                risk_snapshot: greeks,
            } => {
                debug!("Processing greeks: {:?}", greeks);
                self.process_greeks(greeks);
            }
            PowerTradeTicker::Unknown(_) => {}
        }
    }

    fn process_deliverable_data(&mut self, data: Deliverable<ProductType>) {
        self.ticker.timestamp = Utc::now().timestamp_nanos_opt().unwrap_or_default();

        match data.details {
            ProductType::Spot => {}
            ProductType::Future => {}
            ProductType::Option(option) => {
                let option = option.option;
                self.ticker.instrument_name = data.symbol;
                self.ticker.open_interest = option.contract_size;
                self.ticker.state = TickerState::from(data.listing_status.as_str());
            }
            ProductType::Perpetual => {}
            _ => {}
        }

        self.ticker.interest_rate = None;
        self.ticker.mark_iv = None;
        self.ticker.interest_value = None;
    }

    fn process_best_bid_ask(&mut self, data: TopOfBook) {
        self.ticker.timestamp = data.timestamp;
        self.ticker.best_bid_price = data.buy_price.unwrap_or_default();
        self.ticker.best_ask_price = data.sell_price.unwrap_or_default();
        self.ticker.best_bid_amount = data.buy_quantity.unwrap_or_default();
        self.ticker.best_ask_amount = data.sell_quantity.unwrap_or_default();
    }

    fn process_last_trade_price(&mut self, data: LTPrice) {
        match data {
            LTPrice::LastTradePrice(data) => {
                self.ticker.timestamp = data.timestamp;
                self.ticker.last_price = data.price;
            }
            LTPrice::RteLastTradePrice(data) => {
                self.ticker.timestamp = data.timestamp;
                self.ticker.last_price = data.price;
            }
        }
    }

    fn process_trade(&mut self, data: RteTrade) {
        self.ticker.timestamp = data.timestamp;
        self.ticker.last_price = data.price;
    }

    fn process_mark_price(&mut self, data: FundingRate) {
        self.ticker.timestamp = data.timestamp;
        self.ticker.mark_price = data.mark_price;
        self.ticker.index_price = data.underlying_price;
        self.ticker.delivery_price = Some(data.underlying_price);
        self.ticker.current_funding = Some(0f64);
    }

    fn process_greeks(&mut self, data: RiskSnapshot) {
        self.ticker.timestamp = data.timestamp;

        let greeks = match data.mid {
            Some(mid) => mid.greeks,
            None => {
                return;
            }
        };

        self.ticker.greeks = Some(Greeks {
            delta: Some(greeks.delta),
            gamma: Some(greeks.gamma),
            theta: Some(greeks.theta),
            vega: Some(greeks.vega),
            rho: Some(greeks.rho),
        });
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct PowerTradeTickerUpdater {
    updates_processed: u64,
}

impl PowerTradeTickerUpdater {
    pub fn new() -> Self {
        Self {
            updates_processed: 0,
        }
    }

    fn construct_ticker_from_update(update: PowerTradeTicker) -> Ticker {
        let mut aggregator = PowerTradeTickerAggregator::new();
        aggregator.process_message(update);
        aggregator.ticker.clone()
    }
}

#[async_trait]
impl TickerUpdater for PowerTradeTickerUpdater {
    type Ticker = Ticker;
    type Update = PowerTradeTicker;

    async fn init(
        _: mpsc::UnboundedSender<WsMessage>,
        instrument: Instrument,
    ) -> Result<InstrumentTicker<Self>, DataError> {
        Ok(InstrumentTicker {
            instrument,
            updater: Self::new(),
            ticker: Ticker::default(),
        })
    }

    fn update(
        &mut self,
        ticker: &mut Self::Ticker,
        update: Self::Update,
    ) -> Result<Option<Self::Ticker>, DataError> {
        let updated_ticker = Self::construct_ticker_from_update(update);

        ticker.merge(&updated_ticker).map_err(|e| {
            DataError::Socket(SocketError::Deserialise {
                error: serde_json::Error::custom(format!("Failed to merge ticker: {e}")),
                payload: format!("{:?}", updated_ticker),
            })
        })?;

        self.updates_processed += 1;

        Ok(Some(ticker.clone()))
    }
}

#[cached(
    ty = "cached::UnboundCache<String, String>",
    create = "{ cached::UnboundCache::new() }",
    convert = "{ tradeable_entity_id.to_string() }",
    result = true
)]
fn fetch_symbol(tradeable_entity_id: &str) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "{}{}/summary",
        POWERTRADE_TRADEABLE_ENTITY_API, tradeable_entity_id
    );
    let summary: PowerTradeInstrumentSummary = tokio::task::block_in_place(|| {
        let rt = tokio::runtime::Handle::current();
        let response = rt.block_on(reqwest::get(&url))?;
        rt.block_on(response.json::<PowerTradeInstrumentSummary>())
    })?;
    Ok(summary.symbol)
}
