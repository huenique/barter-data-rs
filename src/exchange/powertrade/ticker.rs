use async_trait::async_trait;
use barter_integration::model::instrument::Instrument;
use barter_integration::model::SubscriptionId;
use barter_integration::protocol::websocket::WsMessage;
use chrono::TimeZone;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;
use tracing::info;

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
use crate::transformer::ticker::InstrumentTicker;
use crate::transformer::ticker::TickerUpdater;
use crate::Identifier;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PowerTradeTicker {
    DeliverableData {
        #[serde(rename = "deliverable")]
        deliverable: Deliverable<ProductType>,
    },
    BestBidAsk {
        #[serde(rename = "top_of_book")]
        top_of_book: TopOfBook,
    },
    MarkPrice {
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
    Greeks {
        #[serde(rename = "risk_snapshot")]
        risk_snapshot: RiskSnapshot,
    },
    Unknown(serde_json::Value),
}

impl Identifier<Option<SubscriptionId>> for PowerTradeTicker {
    fn id(&self) -> Option<SubscriptionId> {
        // TODO:
        // * Since we can't use composite keys rn to use different identifiers for the
        //   same
        // * instrument, we can try fetching the symbol using the tradeable_entity_id or
        //   vice
        // * versa. We'd have to use a local data structure in this module to map the
        //   identifier to
        // * the symbol.
        match self {
            PowerTradeTicker::DeliverableData { deliverable } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    match &deliverable.details {
                        ProductType::Option(_) => deliverable.symbol.clone(),
                        _ => String::new(),
                    },
                ))
                .id(),
            ),
            PowerTradeTicker::BestBidAsk { top_of_book } => Some(
                ExchangeSub::from((PowerTradeChannel::TICKER, &top_of_book.tradeable_entity_id))
                    .id(),
            ),
            PowerTradeTicker::MarkPrice { funding_rate } => Some(
                ExchangeSub::from((PowerTradeChannel::TICKER, &funding_rate.tradeable_entity_id))
                    .id(),
            ),
            PowerTradeTicker::Trade { rte_trade } => Some(
                ExchangeSub::from((PowerTradeChannel::TICKER, &rte_trade.tradeable_entity_id)).id(),
            ),
            PowerTradeTicker::LastTradePrice { last_trade_price } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    &last_trade_price.tradeable_entity_id,
                ))
                .id(),
            ),
            PowerTradeTicker::RteLastTradePrice {
                rte_last_trade_price,
            } => Some(
                ExchangeSub::from((
                    PowerTradeChannel::TICKER,
                    &rte_last_trade_price.tradeable_entity_id,
                ))
                .id(),
            ),
            PowerTradeTicker::Greeks {
                risk_snapshot: greeks,
            } => Some(
                ExchangeSub::from((PowerTradeChannel::TICKER, &greeks.tradeable_entity_id)).id(),
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

    pub fn process_message(&mut self, message: PowerTradeTicker) {
        match message {
            PowerTradeTicker::DeliverableData { deliverable } => {
                self.process_deliverable_data(deliverable);
            }
            PowerTradeTicker::BestBidAsk {
                top_of_book: best_bid_ask,
            } => {
                self.process_best_bid_ask(best_bid_ask);
            }
            PowerTradeTicker::MarkPrice {
                funding_rate: mark_price,
            } => {
                self.process_mark_price(mark_price);
            }
            PowerTradeTicker::Trade { rte_trade: trade } => {
                self.process_trade(trade);
            }
            PowerTradeTicker::LastTradePrice { last_trade_price } => {
                self.process_last_trade_price(LTPrice::LastTradePrice(last_trade_price));
            }
            PowerTradeTicker::RteLastTradePrice {
                rte_last_trade_price,
            } => {
                self.process_last_trade_price(LTPrice::RteLastTradePrice(rte_last_trade_price));
            }
            PowerTradeTicker::Greeks {
                risk_snapshot: greeks,
            } => {
                self.process_greeks(greeks);
            }
            PowerTradeTicker::Unknown(_) => {}
        }
    }

    fn process_deliverable_data(&mut self, data: Deliverable<ProductType>) {
        match data.details {
            ProductType::Spot => {}
            ProductType::Future => {}
            ProductType::Option(option) => {
                let option = option.option;
                self.ticker.instrument_name = data.symbol;
                self.ticker.open_interest = option.contract_size;
                self.ticker.state = data.listing_status;
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
                self.ticker.last_price = data.price;
            }
            LTPrice::RteLastTradePrice(data) => {
                self.ticker.last_price = data.price;
            }
        }
    }

    fn process_trade(&mut self, data: RteTrade) {
        self.ticker.last_price = data.price;
    }

    fn process_mark_price(&mut self, data: FundingRate) {
        self.ticker.mark_price = data.mark_price;
        self.ticker.index_price = data.underlying_price;
        self.ticker.delivery_price = Some(data.underlying_price);
    }

    fn process_greeks(&mut self, data: RiskSnapshot) {
        let greeks = data.theoretical.unwrap_or_default().greeks;

        self.ticker.greeks = Some(Greeks {
            delta: Some(greeks.delta),
            gamma: Some(greeks.gamma),
            theta: Some(greeks.theta),
            vega: Some(greeks.vega),
            rho: Some(greeks.rho),
        });
    }
}

#[async_trait]
impl TickerUpdater for PowerTradeTicker {
    type Ticker = Ticker;
    type Update = PowerTradeTicker;

    async fn init(
        _: mpsc::UnboundedSender<WsMessage>,
        instrument: Instrument,
    ) -> Result<InstrumentTicker<Self>, DataError> {
        // Initialize the ticker state with default values
        Ok(InstrumentTicker {
            instrument: instrument.clone(),
            updater: Self::DeliverableData {
                deliverable: Deliverable {
                    deliverable_id: String::new(),
                    symbol: instrument.to_string(),
                    tags: vec![],
                    decimal_places: String::new(),
                    listing_status: String::new(),
                    details: ProductType::Spot,
                },
            },
            ticker: Ticker {
                instrument_name: String::new(),
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
            },
        })
    }

    fn update(
        &mut self,
        ticker: &mut Self::Ticker,
        update: Self::Update,
    ) -> Result<Option<Self::Ticker>, DataError> {
        // let mut aggregator = PowerTradeTickerAggregator::new();
        // aggregator.process_message(update);

        info!("Updating ticker with {:?}", update);

        // let updated_ticker = aggregator.ticker;

        // let mut ticker = ticker.clone();
        // ticker.merge(&updated_ticker).map_err(|e| {
        //     DataError::Socket(SocketError::Deserialise {
        //         error: serde_json::Error::custom(format!("Failed to merge ticker:
        // {e}")),         payload: updated_ticker.to_string(),
        //     })
        // })?;

        Ok(Some(ticker.clone()))
    }
}
