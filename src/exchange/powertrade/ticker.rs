use std::collections::HashMap;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::error::DataError;
use crate::event::MarketEvent;
use crate::event::MarketIter;
use crate::exchange::powertrade::channel::PowerTradeChannel;
use crate::exchange::powertrade::message::deliverable::Deliverable;
use crate::exchange::powertrade::message::deliverable::ProductType;
use crate::exchange::powertrade::message::funding_rate::FundingRate;
use crate::exchange::powertrade::message::products::option::RiskSnapshot;
use crate::exchange::powertrade::message::rte_last_trade_price::LastTradePrice;
use crate::exchange::powertrade::message::rte_trade::RteTrade;
use crate::exchange::powertrade::message::top_of_book::TopOfBook;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::exchange::SocketError;
use crate::subscription::ticker::Greeks;
use crate::subscription::ticker::Ticker;
use crate::subscription::Map;
use crate::subscription::SubKind;
use crate::transformer::ticker::InstrumentTicker;
use crate::transformer::ticker::TickerUpdater;
use crate::transformer::ExchangeTransformer;
use crate::Identifier;

use async_trait::async_trait;
use barter_integration::model::instrument::Instrument;
use barter_integration::model::SubscriptionId;
use barter_integration::protocol::websocket::WsMessage;
use barter_integration::Transformer;
use chrono::TimeZone;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
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
                instrument_name: instrument.to_string(),
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
        // Use the Aggregator to process the update
        let mut aggregator = Aggregator::new();
        aggregator.process_message(update);

        let updated_ticker = aggregator
            .tickers
            .get(&ticker.instrument_name)
            .cloned()
            .ok_or(DataError::Socket(SocketError::Unidentifiable(
                self.id().unwrap(),
            )))?;
        *ticker = updated_ticker;
        Ok(Some(ticker.clone()))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct MultiTickerTransformer<Exchange, Kind, Updater> {
    pub ticker_map: Map<InstrumentTicker<Updater>>,
    phantom: PhantomData<(Exchange, Kind)>,
}

#[async_trait]
impl<Exchange, Kind, Updater> ExchangeTransformer<Exchange, Kind>
    for MultiTickerTransformer<Exchange, Kind, Updater>
where
    Exchange: Connector + Send,
    Kind: SubKind<Event = Ticker> + Send,
    Updater: TickerUpdater<Ticker = Kind::Event> + Send,
    Updater::Update: Identifier<Option<SubscriptionId>> + for<'de> Deserialize<'de>,
{
    async fn new(
        ws_sink_tx: mpsc::UnboundedSender<WsMessage>,
        map: Map<Instrument>,
    ) -> Result<Self, DataError> {
        let (sub_ids, init_ticker_requests): (Vec<_>, Vec<_>) = map
            .0
            .into_iter()
            .map(|(sub_id, instrument)| (sub_id, Updater::init(ws_sink_tx.clone(), instrument)))
            .unzip();

        let init_tickers = futures::future::join_all(init_ticker_requests)
            .await
            .into_iter()
            .collect::<Result<Vec<InstrumentTicker<Updater>>, DataError>>()?;

        let ticker_map = sub_ids
            .into_iter()
            .zip(init_tickers.into_iter())
            .collect::<Map<InstrumentTicker<Updater>>>();

        Ok(Self {
            ticker_map,
            phantom: PhantomData,
        })
    }
}

impl<Exchange, Kind, Updater> Transformer for MultiTickerTransformer<Exchange, Kind, Updater>
where
    Exchange: Connector,
    Kind: SubKind<Event = Ticker>,
    Updater: TickerUpdater<Ticker = Kind::Event>,
    Updater::Update: Identifier<Option<SubscriptionId>> + for<'de> Deserialize<'de>,
{
    type Error = DataError;
    type Input = Updater::Update;
    type Output = MarketEvent<Kind::Event>;
    type OutputIter = Vec<Result<Self::Output, Self::Error>>;

    fn transform(&mut self, update: Self::Input) -> Self::OutputIter {
        let subscription_id = match update.id() {
            Some(subscription_id) => subscription_id,
            None => return vec![],
        };

        let ticker = match self.ticker_map.find_mut(&subscription_id) {
            Ok(ticker) => ticker,
            Err(unidentifiable) => return vec![Err(DataError::Socket(unidentifiable))],
        };

        let InstrumentTicker {
            instrument,
            ticker,
            updater,
        } = ticker;

        match updater.update(ticker, update) {
            Ok(Some(ticker)) => {
                MarketIter::<Ticker>::from((Exchange::ID, instrument.clone(), ticker)).0
            }
            Ok(None) => vec![],
            Err(error) => vec![Err(error)],
        }
    }
}
