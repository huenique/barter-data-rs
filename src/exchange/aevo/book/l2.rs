use super::AevoLevel;
use crate::error::DataError;
use crate::exchange::aevo::channel::AevoChannel;
use crate::exchange::aevo::message::AevoMessage;
use crate::exchange::subscription::ExchangeSub;
use crate::subscription::book::Level;
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
use crc32fast::Hasher;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;

/// Terse type alias for an [`Aevo`](super::Aevo) real-time trades WebSocket message.
pub type AevoOrderBookL2 = AevoMessage<AevoOrderBookL2Delta>;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct AevoOrderBookL2Delta {
    pub r#type: String,
    pub instrument_id: String,
    pub instrument_name: String,
    pub instrument_type: String,
    pub bids: Vec<AevoLevel>,
    pub asks: Vec<AevoLevel>,
    // pub last_updated: u64, // nanoseconds
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub checksum: u32,
}

impl Identifier<Option<SubscriptionId>> for AevoOrderBookL2 {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((AevoChannel::ORDER_BOOK_L2, &self.data.instrument_name)).id())
    }
}

impl From<AevoOrderBookL2Delta> for OrderBook {
    fn from(snapshot: AevoOrderBookL2Delta) -> Self {
        Self {
            last_update_time: Utc::now(),
            bids: OrderBookSide::new(Side::Buy, snapshot.bids),
            asks: OrderBookSide::new(Side::Sell, snapshot.asks),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct AevoBookUpdater {
    pub updates_processed: u64,
    pub last_checksum: u32,
}

impl AevoBookUpdater {
    pub fn new(checksum: u32) -> Self {
        Self {
            updates_processed: 0,
            last_checksum: checksum,
        }
    }

    pub fn checksum(&self, bids: &[Level], asks: &[Level]) -> u32 {
        let mut preimage = String::new();
        let iterations = bids.len().max(asks.len());

        for index in 0..iterations.min(100) {
            if let Some(bid) = bids.get(index) {
                preimage.push_str(&format!("{}:{}:", bid.price, bid.amount));
            }

            if let Some(ask) = asks.get(index) {
                preimage.push_str(&format!("{}:{}:", ask.price, ask.amount));
            }
        }

        preimage.pop(); // strip last colon
        let mut hasher = Hasher::new();
        hasher.update(preimage.as_bytes());
        hasher.finalize()
    }

    pub fn checksum2(&self, bids: &[AevoLevel], asks: &[AevoLevel]) -> u32 {
        let mut preimage = String::new();
        let iterations = bids.len().max(asks.len());

        for index in 0..iterations.min(100) {
            if let Some(bid) = bids.get(index) {
                preimage.push_str(&format!("{}:{}:", bid.price, bid.amount));
            }

            if let Some(ask) = asks.get(index) {
                preimage.push_str(&format!("{}:{}:", ask.price, ask.amount));
            }
        }

        preimage.pop(); // strip last colon
        let mut hasher = Hasher::new();
        hasher.update(preimage.as_bytes());
        hasher.finalize()
    }
}

#[async_trait]
impl OrderBookUpdater for AevoBookUpdater {
    type OrderBook = OrderBook;
    type Update = AevoOrderBookL2;

    async fn init<Exchange, Kind>(
        _: mpsc::UnboundedSender<WsMessage>,
        instrument: Instrument,
    ) -> Result<InstrumentOrderBook<Self>, DataError>
    where
        Exchange: Send,
        Kind: Send,
    {
        // No need to fetch snapshot separately, as the first notification will contain the whole book
        // Initialize empty OrderBook
        Ok(InstrumentOrderBook {
            instrument,
            updater: Self::new(0),
            book: OrderBook::from(AevoOrderBookL2Delta {
                r#type: String::new(),
                instrument_id: String::new(),
                instrument_name: String::new(),
                instrument_type: String::new(),
                bids: Vec::new(),
                asks: Vec::new(),
                checksum: 0,
            }),
        })
    }

    fn update(
        &mut self,
        book: &mut Self::OrderBook,
        update: Self::Update,
    ) -> Result<Option<Self::OrderBook>, DataError> {
        // If the checksum matches the previous message, ignore it. Aevo currently sends a snapshot
        // message with the full order book, but will switch to incremental updates in the future.
        if self.last_checksum == update.data.checksum && update.data.r#type == "snapshot" {
            return Ok(None);
        }

        // When updating OrderBook metadata and levels, it's important to note that each event's
        // data shows the exact quantity for a price level. If the quantity is 0, then the price
        // level should be removed.
        book.last_update_time = Utc::now();
        book.bids.upsert(update.data.bids);
        book.asks.upsert(update.data.asks);

        if update.data.r#type == "snapshot" {
            let checksum = self.checksum(&book.bids.levels, &book.asks.levels);
            if checksum != update.data.checksum {
                return Err(DataError::InvalidSequence {
                    prev_last_update_id: self.last_checksum as u64,
                    first_update_id: update.data.checksum as u64,
                });
            }

            self.last_checksum = checksum;
        }

        // Update OrderBookUpdater metadata
        self.updates_processed += 1;
        Ok(Some(book.snapshot()))
    }
}
