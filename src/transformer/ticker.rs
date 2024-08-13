use std::cmp::Ordering;
use std::marker::PhantomData;

use async_trait::async_trait;
use barter_integration::model::instrument::Instrument;
use barter_integration::model::SubscriptionId;
use barter_integration::protocol::websocket::WsMessage;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;

use crate::error::DataError;
use crate::event::MarketIter;
use crate::subscription::ticker::Ticker;
use crate::subscription::Map;
use crate::transformer::Transformer;
use crate::Connector;
use crate::ExchangeTransformer;
use crate::Identifier;
use crate::MarketEvent;
use crate::SubKind;

#[async_trait]
pub trait TickerUpdater
where
    Self: Sized,
{
    type Ticker;
    type Update;

    async fn init(
        ws_sink_tx: mpsc::UnboundedSender<WsMessage>,
        instrument: Instrument,
    ) -> Result<InstrumentTicker<Self>, DataError>;

    fn update(
        &mut self,
        ticker: &mut Self::Ticker,
        update: Self::Update,
    ) -> Result<Option<Self::Ticker>, DataError>;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InstrumentTicker<Updater> {
    pub instrument: Instrument,
    pub updater: Updater,
    pub ticker: Ticker,
}

impl<Updater: PartialEq> Eq for InstrumentTicker<Updater> {}

impl<Updater: PartialEq + PartialOrd> Ord for InstrumentTicker<Updater> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ticker.cmp(&other.ticker)
    }
}

impl<Updater: PartialEq + PartialOrd> PartialOrd for InstrumentTicker<Updater> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
