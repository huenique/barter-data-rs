use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::event::MarketIter;
use crate::exchange::coincall::CoincallChannel;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::Instrument;
use crate::subscription::ticker::Ticker;
use crate::ExchangeId;
use crate::Identifier;
use crate::MarketEvent;
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct CoincallTicker {}

impl Identifier<Option<SubscriptionId>> for CoincallTicker {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((CoincallChannel::TICKER, "")).id())
    }
}

impl From<(ExchangeId, Instrument, CoincallTicker)> for MarketIter<Ticker> {
    fn from((exchange_id, instrument, _ticker): (ExchangeId, Instrument, CoincallTicker)) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: DateTime::<Utc>::default().with_timezone(&Utc),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind: Ticker::default(),
        })])
    }
}
