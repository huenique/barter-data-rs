use crate::error::DataError;
use crate::exchange::dydx::book::DydxLevel;
use crate::exchange::dydx::channel::DydxChannel;
use crate::exchange::dydx::message::ChannelDataMessage;
use crate::exchange::dydx::message::ChannelDataMessageContents;
use crate::exchange::dydx::message::DydxMessage;
use crate::exchange::dydx::message::OrderBookSnapshotContents;
use crate::exchange::dydx::message::SubscribedMessage;
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
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;

pub type DydxOrderBookL3 = DydxMessage;

impl Identifier<Option<SubscriptionId>> for DydxOrderBookL3 {
    fn id(&self) -> Option<SubscriptionId> {
        match self {
            DydxMessage::Subscribed(SubscribedMessage { id, .. })
            | DydxMessage::ChannelData(ChannelDataMessage { id, .. }) => {
                let instrument_name = id.clone();
                Some(ExchangeSub::from((DydxChannel::ORDER_BOOK_L3, &instrument_name)).id())
            }
            _ => None,
        }
    }
}

impl From<DydxOrderBookL3> for OrderBook {
    fn from(snapshot: DydxOrderBookL3) -> Self {
        match snapshot {
            DydxMessage::ChannelData(ChannelDataMessage {
                contents: ChannelDataMessageContents { bids, asks },
                ..
            }) => Self {
                last_update_time: Utc::now(),
                bids: parse_order_data(Side::Buy, &bids.unwrap_or_default()),
                asks: parse_order_data(Side::Sell, &asks.unwrap_or_default()),
            },
            DydxMessage::Subscribed(SubscribedMessage {
                contents: OrderBookSnapshotContents { bids, asks },
                ..
            }) => Self {
                last_update_time: Utc::now(),
                bids: OrderBookSide::new(Side::Buy, bids),
                asks: OrderBookSide::new(Side::Sell, asks),
            },
            _ => Self {
                last_update_time: Utc::now(),
                bids: OrderBookSide::new(Side::Buy, Vec::<Level>::new()),
                asks: OrderBookSide::new(Side::Sell, Vec::<Level>::new()),
            },
        }
    }
}

fn parse_order_data(side: Side, data: &Vec<DydxLevel>) -> OrderBookSide {
    OrderBookSide::new(
        side,
        data.iter()
            .map(|level_group| {
                let price = level_group.price;
                let amount = level_group.size;
                Level { price, amount }
            })
            .collect::<Vec<Level>>(),
    )
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DydxOrderBookUpdater {
    pub updates_processed: u64,
    pub message_id: u64,
}

impl DydxOrderBookUpdater {
    pub fn new(message_id: u64) -> Self {
        Self {
            updates_processed: 0,
            message_id,
        }
    }
}

#[async_trait]
impl OrderBookUpdater for DydxOrderBookUpdater {
    type OrderBook = OrderBook;
    type Update = DydxOrderBookL3;

    async fn init<Exchange, Kind>(
        _: UnboundedSender<WsMessage>,
        instrument: Instrument,
    ) -> Result<InstrumentOrderBook<Self>, DataError>
    where
        Exchange: Send,
        Kind: Send,
    {
        // No need for a separate snapshot fetch; the first notification will have the whole book.
        Ok(InstrumentOrderBook {
            instrument,
            updater: Self::new(0),
            book: OrderBook::from(DydxOrderBookL3::Subscribed(SubscribedMessage {
                message_type: String::new(),
                connection_id: String::new(),
                message_id: 0,
                id: String::new(),
                channel: String::new(),
                contents: OrderBookSnapshotContents {
                    bids: Vec::new(),
                    asks: Vec::new(),
                },
            })),
        })
    }

    fn update(
        &mut self,
        book: &mut Self::OrderBook,
        update: Self::Update,
    ) -> Result<Option<Self::OrderBook>, DataError> {
        match update {
            DydxMessage::ChannelData(ChannelDataMessage {
                contents: ChannelDataMessageContents { bids, asks },
                message_id,
                ..
            }) => {
                check_message_id(message_id, self)?;
                book.last_update_time = Utc::now();
                book.bids.upsert(bids.iter().map(|level_group| {
                    let price = level_group[0].price;
                    let amount = level_group[0].size;
                    Level { price, amount }
                }));
                book.asks.upsert(asks.iter().map(|level_group| {
                    let price = level_group[0].price;
                    let amount = level_group[0].size;
                    Level { price, amount }
                }));
            }
            DydxMessage::Subscribed(SubscribedMessage {
                contents,
                message_id,
                ..
            }) => {
                check_message_id(message_id, self)?;
                book.last_update_time = Utc::now();
                // We can safely overwrite the existing book since this is a snapshot.
                book.bids = OrderBookSide::new(Side::Buy, contents.bids);
                book.asks = OrderBookSide::new(Side::Sell, contents.asks);
            }
            _ => {}
        }

        Ok(Some(book.snapshot()))
    }
}

fn check_message_id(message_id: u64, updater: &DydxOrderBookUpdater) -> Result<(), DataError> {
    if message_id <= updater.message_id {
        return Err(DataError::InvalidSequence {
            prev_last_update_id: updater.message_id,
            first_update_id: message_id,
        });
    }

    Ok(())
}
