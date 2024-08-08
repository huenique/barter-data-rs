use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::event::MarketIter;
use crate::exchange::coincall::message::CoincallHeartbeat;
use crate::exchange::coincall::message::CoincallMessage;
use crate::exchange::coincall::CoincallChannel;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::Instrument;
use crate::subscription::ticker::Ticker;
use crate::ExchangeId;
use crate::Identifier;
use crate::MarketEvent;

// pub type CoincallOptionTicker = CoincallMessage<CoincallOptionTickerData>;
// enum CoincallHeartbeat and CoincallMessage<CoincallOptionTickerData> for
// CoincallOptionTicker
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CoincallOptionTicker {
    Heartbeat(CoincallHeartbeat),
    Data(CoincallMessage<CoincallOptionTickerData>),
}

/// Coincall option ticker data.
///
/// See: <https://docs.coincall.com/#options-websocket-pricing-information>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct CoincallOptionTickerData {
    #[serde(rename = "uv")]
    pub trade_value: f64,
    #[serde(rename = "rt")]
    pub remain_timestamp: i64,
    #[serde(rename = "mp")]
    pub mark_price: f64,
    #[serde(rename = "lp")]
    pub last_price: f64,
    #[serde(rename = "ip")]
    pub index_price: f64,
    #[serde(rename = "delta")]
    pub delta: f64,
    #[serde(rename = "h")]
    pub price_24h_high: f64,
    #[serde(rename = "l")]
    pub price_24h_low: f64,
    #[serde(rename = "iv")]
    pub implied_volatility: f64,
    #[serde(rename = "theta")]
    pub theta: f64,
    #[serde(rename = "cp")]
    pub change_price: f64,
    #[serde(rename = "pr0")]
    pub price_24h_open: f64,
    #[serde(rename = "cr")]
    pub change_rate: f64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "uv24")]
    pub volume_usd_24h: f64,
    #[serde(rename = "v")]
    pub volume: f64,
    #[serde(rename = "v24")]
    pub volume_24h: f64,
    #[serde(rename = "oi")]
    pub open_interest: f64,
    #[serde(rename = "up")]
    pub underlying_price: f64,
    #[serde(rename = "gamma")]
    pub gamma: f64,
    #[serde(rename = "vega")]
    pub vega: f64,
    #[serde(rename = "ts")]
    pub timestamp: i64,
}

impl Identifier<Option<SubscriptionId>> for CoincallOptionTicker {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((CoincallChannel::TICKER, "")).id())
    }
}

impl From<(ExchangeId, Instrument, CoincallOptionTicker)> for MarketIter<Ticker> {
    fn from(
        (exchange_id, instrument, _ticker): (ExchangeId, Instrument, CoincallOptionTicker),
    ) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: DateTime::<Utc>::default().with_timezone(&Utc),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind: Ticker::default(),
        })])
    }
}
