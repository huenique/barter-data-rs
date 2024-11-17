use async_trait::async_trait;
use barter_integration::error::SocketError;
use barter_integration::model::instrument::Instrument;
use barter_integration::protocol::websocket::WsMessage;
use chrono::TimeZone;
use chrono::Utc;
use serde::de::Error;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;

use crate::error::DataError;
use crate::event::MarketEvent;
use crate::event::MarketIter;
use crate::exchange::okx::message::funding_rate::FundingRateData;
use crate::exchange::okx::message::index_tickers::IndexTickerData;
use crate::exchange::okx::message::mark_price::MarkPriceData;
use crate::exchange::okx::message::open_interest::OpenInterestData;
use crate::exchange::okx::message::tickers::TickerData;
use crate::exchange::okx::message::OkxMessage;
use crate::exchange::ExchangeId;
use crate::subscription::ticker::Ticker;
use crate::subscription::ticker::TickerState;
use crate::transformer::ticker::InstrumentTicker;
use crate::transformer::ticker::TickerUpdater;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub enum OkxTickerData {
    FundingRate(FundingRateData),
    IndexTickers(IndexTickerData),
    MarkPrice(MarkPriceData),
    OpenInterest(OpenInterestData),
    Tickers(TickerData),
}

pub type OkxTicker = OkxMessage<OkxTickerData>;

impl From<(ExchangeId, Instrument, OkxTicker)> for MarketIter<Ticker> {
    fn from((exchange_id, instrument, ticker): (ExchangeId, Instrument, OkxTicker)) -> Self {
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

impl From<OkxTicker> for Ticker {
    fn from(data: OkxTicker) -> Self {
        let mut aggregate = OkxTickerAggregator::new();
        aggregate.process_message(data);
        aggregate.ticker.clone()
    }
}

#[derive(Debug, Default)]
pub struct OkxTickerAggregator {
    ticker: Ticker,
}

impl OkxTickerAggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process_message(&mut self, data: OkxTicker) {
        let ticker_data = &data.data[0];
        match ticker_data {
            OkxTickerData::FundingRate(message) => {
                self.ticker.instrument_name = message.inst_id.clone();
                self.ticker.funding_rate = Some(message.funding_rate);
                self.ticker.timestamp = message.ts;
            }
            OkxTickerData::IndexTickers(message) => {
                self.ticker.instrument_name = message.inst_id.clone();
                self.ticker.index_price = message.idx_px;
            }
            OkxTickerData::MarkPrice(message) => {
                self.ticker.instrument_name = message.inst_id.clone();
                self.ticker.mark_price = message.mark_px;
                self.ticker.timestamp = message.ts;
            }
            OkxTickerData::OpenInterest(message) => {
                self.ticker.instrument_name = message.inst_id.clone();
                self.ticker.open_interest = message.oi;
                self.ticker.timestamp = message.ts;
            }
            OkxTickerData::Tickers(message) => {
                self.ticker.instrument_name = message.inst_id.clone();
                self.ticker.best_ask_amount = message.ask_sz;
                self.ticker.best_ask_price = message.ask_px;
                self.ticker.best_bid_amount = message.bid_sz;
                self.ticker.best_bid_price = message.bid_px;
                self.ticker.last_price = message.last;
                self.ticker.state = TickerState::Open;
                self.ticker.timestamp = message.ts;
            }
        }
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct OkxTickerUpdater {
    updates_processed: u64,
}

impl OkxTickerUpdater {
    pub fn new() -> Self {
        Self {
            updates_processed: 0,
        }
    }

    fn construct_ticker_from_update(update: OkxTicker) -> Ticker {
        let mut aggregator = OkxTickerAggregator::new();
        aggregator.process_message(update);
        aggregator.ticker.clone()
    }
}

#[async_trait]
impl TickerUpdater for OkxTickerUpdater {
    type Ticker = Ticker;
    type Update = OkxTicker;

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
