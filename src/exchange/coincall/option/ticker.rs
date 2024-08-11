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
use crate::subscription::ticker::Greeks;
use crate::subscription::ticker::Ticker;
use crate::ExchangeId;
use crate::Identifier;
use crate::MarketEvent;

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
        match self {
            CoincallOptionTicker::Data(data) => {
                Some(ExchangeSub::from((CoincallChannel::TICKER, data.data.symbol.clone())).id())
            }
            _ => None,
        }
    }
}

impl From<(ExchangeId, Instrument, CoincallOptionTicker)> for MarketIter<Ticker> {
    fn from(
        (exchange_id, instrument, ticker): (ExchangeId, Instrument, CoincallOptionTicker),
    ) -> Self {
        let ticker = match ticker {
            CoincallOptionTicker::Data(data) => Ticker::from(data.data),
            _ => return Self(vec![]),
        };

        Self(vec![Ok(MarketEvent {
            exchange_time: DateTime::<Utc>::default().with_timezone(&Utc),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind: ticker,
        })])
    }
}

impl From<CoincallOptionTickerData> for Ticker {
    fn from(data: CoincallOptionTickerData) -> Self {
        Ticker {
            instrument_name: data.symbol,
            best_bid_price: 0.0,
            best_ask_price: 0.0,
            best_bid_amount: 0.0,
            best_ask_amount: 0.0,
            mark_price: data.mark_price,
            last_price: data.last_price,
            open_interest: data.open_interest,
            state: "".to_string(),
            timestamp: data.timestamp,
            greeks: Some(Greeks {
                delta: Some(data.delta),
                gamma: Some(data.gamma),
                theta: Some(data.theta),
                vega: Some(data.vega),
                rho: None,
            }),
            interest_rate: Some(0f64),
            mark_iv: Some(data.implied_volatility),
            delivery_price: Some(0f64),
            current_funding: Some(0f64),
            interest_value: Some(0f64),
            ask_iv: Some(0f64),
            bid_iv: Some(0f64),
            index_price: data.index_price,
        }
    }
}
