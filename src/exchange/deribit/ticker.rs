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
use crate::subscription::ticker::Greeks;
use crate::subscription::ticker::Ticker;
use crate::ExchangeId;
use crate::Identifier;
use crate::MarketEvent;

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
            last_price: data.last_price.unwrap_or(0.0),
            open_interest: data.open_interest.unwrap_or(0.0),
            state: data.state,
            timestamp: data.timestamp,
            greeks: data.greeks.map(|g| Greeks {
                delta: Some(g.delta),
                gamma: Some(g.gamma),
                theta: Some(g.theta),
                vega: Some(g.vega),
                rho: Some(g.rho),
            }),
            interest_rate: data.interest_rate,
            mark_iv: data.mark_iv,
            delivery_price: data.estimated_delivery_price,
            current_funding: data.current_funding,
            interest_value: data.interest_value,
            ask_iv: data.ask_iv,
            bid_iv: data.bid_iv,
            index_price: data.index_price,
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
    pub open_interest: Option<f64>,
    pub greeks: Option<DeribitGreeks>,
    pub timestamp: i64,
    pub interest_rate: Option<f64>,
    pub mark_iv: Option<f64>,
    pub estimated_delivery_price: Option<f64>,
    pub ask_iv: Option<f64>,
    pub bid_iv: Option<f64>,
    pub index_price: f64,
    pub state: String,
    pub funding_8h: Option<f64>,
    pub current_funding: Option<f64>,
    pub interest_value: Option<f64>,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct DeribitGreeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
}
