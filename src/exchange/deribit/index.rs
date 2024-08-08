use barter_integration::model::Exchange;
use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::event::MarketIter;
use crate::exchange::deribit::message::DeribitSingleDataMessage;
use crate::exchange::deribit::DeribitChannel;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::Instrument;
use crate::subscription::index::Index;
use crate::ExchangeId;
use crate::Identifier;
use crate::MarketEvent;

pub type DeribitIndex = DeribitSingleDataMessage<DeribitIndexData>;

impl Identifier<Option<SubscriptionId>> for DeribitIndex {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((DeribitChannel::INDEX, &self.params.data.index_name)).id())
    }
}

impl From<(ExchangeId, Instrument, DeribitIndex)> for MarketIter<Index> {
    fn from((exchange_id, instrument, index): (ExchangeId, Instrument, DeribitIndex)) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: DateTime::parse_from_rfc3339(&index.params.data.timestamp.to_string())
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            received_time: Utc::now(),
            exchange: Exchange::from(exchange_id),
            instrument,
            kind: Index::from(index.params.data),
        })])
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DeribitIndexData {
    pub index_name: String,
    pub price: f64,
    pub timestamp: u64,
}

impl From<DeribitIndexData> for Index {
    fn from(data: DeribitIndexData) -> Self {
        Index {
            index_name: data.index_name,
            price: data.price,
            timestamp: DateTime::<Utc>::from_naive_utc_and_offset(
                DateTime::from_timestamp(data.timestamp as i64, 0)
                    .unwrap()
                    .naive_utc(),
                Utc,
            ),
        }
    }
}
