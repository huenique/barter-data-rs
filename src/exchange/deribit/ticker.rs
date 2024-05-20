use crate::event::MarketIter;
use crate::exchange::deribit::message::DeribitSingleDataMessage;
use crate::exchange::deribit::DeribitChannel;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::Instrument;
use crate::subscription::ticker::Greeks;
use crate::subscription::ticker::Ticker;
use crate::ExchangeId;
use crate::Identifier;
use crate::MarketEvent;

use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

pub type DeribitTicker = DeribitSingleDataMessage<DeribitTickerData>;

impl Identifier<Option<SubscriptionId>> for DeribitTicker {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((DeribitChannel::TICKER, &self.params.data.instrument_name)).id())
    }
}

impl From<(ExchangeId, Instrument, DeribitTicker)> for MarketIter<Ticker> {
    fn from((exchange_id, instrument, ticker): (ExchangeId, Instrument, DeribitTicker)) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: DateTime::parse_from_rfc3339(&ticker.params.data.timestamp.to_string())
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind: ticker.params.data.into(),
        })])
    }
}

impl From<DeribitTickerData> for Ticker {
    fn from(data: DeribitTickerData) -> Self {
        Ticker {
            instrument_name: data.instrument_name,
            best_bid_price: data.best_bid_price,
            best_ask_price: data.best_ask_price,
            best_bid_amount: data.best_bid_amount,
            best_ask_amount: data.best_ask_amount,
            mark_price: data.mark_price,
            last_price: data.last_price,
            volume_24h: data.volume,
            high_24h: data.high,
            low_24h: data.low,
            open_interest: data.open_interest,
            greeks: data.greeks.map(|g| Greeks {
                delta: g.delta,
                gamma: g.gamma,
                theta: g.theta,
                vega: g.vega,
                rho: g.rho,
            }),
            timestamp: DateTime::<Utc>::from_naive_utc_and_offset(
                DateTime::from_timestamp(data.timestamp as i64, 0)
                    .unwrap()
                    .naive_utc(),
                Utc,
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeribitTickerData {
    pub instrument_name: String,
    pub best_bid_price: f64,
    pub best_ask_price: f64,
    pub best_bid_amount: f64,
    pub best_ask_amount: f64,
    pub mark_price: f64,
    pub last_price: Option<f64>,
    pub volume_usd: Option<f64>,
    pub volume: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open_interest: Option<f64>,
    pub greeks: Option<DeribitGreeks>,
    pub timestamp: u64,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct DeribitGreeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}
