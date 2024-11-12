pub mod message;

use barter_integration::model::SubscriptionId;
use chrono::DateTime;
use chrono::Utc;

use crate::event::MarketIter;
use crate::exchange::derive::channel::DeriveChannel;
use crate::exchange::derive::ticker::message::DeriveInstrumentTicker;
use crate::exchange::derive::ticker::message::TickerMessage;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::Instrument;
use crate::subscription::ticker::Greeks;
use crate::subscription::ticker::Ticker;
use crate::ExchangeId;
use crate::Identifier;
use crate::MarketEvent;

pub type DeriveTicker = TickerMessage;

impl Identifier<Option<SubscriptionId>> for DeriveTicker {
    fn id(&self) -> Option<SubscriptionId> {
        Some(
            ExchangeSub::from((
                DeriveChannel::TICKER,
                &self.params.data.instrument_ticker.instrument_name,
            ))
            .id(),
        )
    }
}

impl From<(ExchangeId, Instrument, DeriveTicker)> for MarketIter<Ticker> {
    fn from((exchange_id, instrument, ticker): (ExchangeId, Instrument, DeriveTicker)) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: DateTime::parse_from_rfc3339(&ticker.params.data.timestamp.to_string())
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind: ticker.params.data.instrument_ticker.into(),
        })])
    }
}

impl From<DeriveInstrumentTicker> for Ticker {
    fn from(data: DeriveInstrumentTicker) -> Self {
        Ticker {
            instrument_name: data.instrument_name,
            best_bid_price: data.best_bid_price,
            best_ask_price: data.best_ask_price,
            best_bid_amount: data.best_bid_amount,
            best_ask_amount: data.best_ask_amount,
            mark_price: data.mark_price,
            last_price: 0.0,
            open_interest: data.stats.open_interest,
            state: if data.is_active {
                "open".into()
            } else {
                "closed".into()
            },
            timestamp: data.timestamp,
            greeks: Some(Greeks {
                delta: Some(data.option_pricing.delta),
                gamma: Some(data.option_pricing.gamma),
                theta: Some(data.option_pricing.theta),
                vega: Some(data.option_pricing.vega),
                rho: Some(data.option_pricing.rho),
            }),
            interest_rate: None,
            mark_iv: Some(data.option_pricing.iv),
            delivery_price: Some(0_f64),
            current_funding: Some(0_f64),
            interest_value: Some(0_f64),
            ask_iv: Some(data.option_pricing.ask_iv),
            bid_iv: Some(data.option_pricing.bid_iv),
            index_price: data.index_price,
        }
    }
}
