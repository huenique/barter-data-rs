use crate::error::DataError;
use crate::exchange::hyperliquid::message::HyperliquidMessage;
use crate::exchange::subscription::ExchangeSub;
use crate::subscription::book::OrderBook;
use crate::subscription::book::OrderBookSide;
use crate::transformer::book::InstrumentOrderBook;
use crate::transformer::book::OrderBookUpdater;
use crate::Identifier;

use async_trait::async_trait;
use barter_integration::model::instrument::Instrument;
use barter_integration::model::Side;
use barter_integration::model::SubscriptionId;
use barter_integration::protocol::websocket::WsMessage;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;

pub type HyperliquidOrderBookL2Snapshot = HyperliquidMessage<WsBook>;

impl Identifier<Option<SubscriptionId>> for HyperliquidOrderBookL2Snapshot {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((&self.channel, &self.data.coin)).id())
    }
}

// Order book snapshot for L2 as per the WsBook interface
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WsBook {
    pub coin: String,
    pub levels: (Vec<WsLevel>, Vec<WsLevel>), // tuple of bids and asks
    pub time: u64,
}

impl WsBook {
    /// Creates an empty WsBook for the given coin
    pub fn new(coin: &str) -> Self {
        Self {
            coin: coin.to_string(),
            levels: (vec![], vec![]),
            time: 0,
        }
    }
}

// Level detail for order books as per the WsLevel interface
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct WsLevel {
    pub px: String, // price
    pub sz: String, // size
    pub n: u32,     // number of orders
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct HyperliquidOrderBookUpdater {}

#[async_trait]
impl OrderBookUpdater for HyperliquidOrderBookUpdater {
    type OrderBook = OrderBook;
    type Update = HyperliquidOrderBookL2Snapshot;

    async fn init<Exchange, Kind>(
        _: mpsc::UnboundedSender<WsMessage>,
        instrument: Instrument,
    ) -> Result<InstrumentOrderBook<Self>, DataError>
    where
        Exchange: Send,
        Kind: Send,
    {
        Ok(InstrumentOrderBook {
            instrument: instrument.clone(),
            updater: Self {},
            book: OrderBook::from(HyperliquidOrderBookL2Snapshot {
                channel: String::new(),
                data: WsBook::new(instrument.base.as_ref()),
            }),
        })
    }

    fn update(
        &mut self,
        book: &mut OrderBook,
        update: Self::Update,
    ) -> Result<Option<OrderBook>, DataError> {
        book.last_update_time = Utc::now();
        book.bids.upsert(update.data.levels.0);
        book.asks.upsert(update.data.levels.1);

        Ok(Some(book.clone()))
    }
}

impl From<HyperliquidOrderBookL2Snapshot> for OrderBook {
    fn from(snapshot: HyperliquidOrderBookL2Snapshot) -> Self {
        Self {
            bids: OrderBookSide::new(Side::Buy, snapshot.data.levels.0),
            asks: OrderBookSide::new(Side::Sell, snapshot.data.levels.1),
            last_update_time: Utc::now(),
        }
    }
}
