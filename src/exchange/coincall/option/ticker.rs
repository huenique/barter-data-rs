use std::borrow::Cow;
use std::io;
use std::sync::Mutex;

use async_trait::async_trait;
use barter_integration::error::SocketError;
use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::SubscriptionId;
use barter_integration::protocol::websocket::WsMessage;
use chrono::TimeZone;
use chrono::Utc;
use once_cell::sync::Lazy;
use serde::de::Error as _;
use serde::Deserialize;
use tokio::sync::mpsc;
use tracing::debug;

use crate::error::DataError;
use crate::event::MarketIter;
use crate::exchange::coincall::auth::calc_cc_ts;
use crate::exchange::coincall::auth::gen_cc_secret;
use crate::exchange::coincall::auth::gen_cc_sig;
use crate::exchange::coincall::auth::get_cc_auth_parms;
use crate::exchange::coincall::auth::CoincallOptionAuthData;
use crate::exchange::coincall::auth::SigGenParams;
use crate::exchange::coincall::auth::COINCALL_ORDERBOOK_V1;
use crate::exchange::coincall::message::CoincallHeartbeat;
use crate::exchange::coincall::message::CoincallMessage;
use crate::exchange::coincall::message::CoincallObData;
use crate::exchange::coincall::message::CoincallObOrder;
use crate::exchange::coincall::message::CoincallOrderbook;
use crate::exchange::coincall::utils::format_opt_instr;
use crate::exchange::coincall::CoincallChannel;
use crate::exchange::subscription::ExchangeSub;
use crate::exchange::Instrument;
use crate::subscription::ticker::Greeks;
use crate::subscription::ticker::Ticker;
use crate::transformer::ticker::InstrumentTicker;
use crate::transformer::ticker::TickerUpdater;
use crate::ExchangeId;
use crate::Identifier;
use crate::MarketEvent;

// TODO: Add/source the rest of the fields to the Ticker struct
// * rho
// * interest_rate
// * delivery_price
// * current_funding
// * interest_value
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CoincallTicker {
    Heartbeat(CoincallHeartbeat),
    Pricing(CoincallMessage<CoincallOptionPricingData>),
    Orderbook(CoincallMessage<CoincallOptionOrderbookData>),
    OptionChain(CoincallMessage<Vec<CoincallOptionChainData>>),
}

/// Coincall option pricing information.
///
/// See: <https://docs.coincall.com/#options-websocket-pricing-information>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd)]
pub struct CoincallOptionPricingData {
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

/// Coincall option orderbook data.
///
/// See: <https://docs.coincall.com/#options-websocket-orderbook>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CoincallOptionOrderbookData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "asks")]
    pub asks: Vec<CoincallOrderbookData>,
    #[serde(rename = "bids")]
    pub bids: Vec<CoincallOrderbookData>,
    #[serde(rename = "ts")]
    pub timestamp: i64,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq)]
pub struct CoincallOrderbookData {
    #[serde(rename = "pr")]
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    #[serde(rename = "sz")]
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub size: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CoincallOptionChainData {
    #[serde(rename = "mp")]
    pub mark_price: f64,
    #[serde(rename = "lp")]
    pub last_price: f64,
    #[serde(rename = "delta")]
    pub delta: f64,
    #[serde(rename = "theta")]
    pub theta: f64,
    #[serde(rename = "cp")]
    pub change_price: f64,
    #[serde(rename = "biv")]
    pub bid_iv: f64,
    #[serde(rename = "aiv")]
    pub ask_iv: f64,
    #[serde(rename = "cr")]
    pub change_rate: f64,
    #[serde(rename = "bs")]
    pub bid_size: f64,
    #[serde(rename = "as")]
    pub ask_size: f64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "v")]
    pub volume: f64,
    #[serde(rename = "ask")]
    pub ask_price: f64,
    #[serde(rename = "v24")]
    pub volume_24h: f64,
    #[serde(rename = "oi")]
    pub open_interest: f64,
    #[serde(rename = "upv")]
    pub upv: f64,
    #[serde(rename = "up")]
    pub underlying_price: f64,
    #[serde(rename = "bid")]
    pub bid_price: f64,
    #[serde(rename = "gamma")]
    pub gamma: f64,
    #[serde(rename = "vega")]
    pub vega: f64,
    #[serde(rename = "ts")]
    pub timestamp: i64,
}

static SYMBOL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

impl Identifier<Option<SubscriptionId>> for CoincallTicker {
    fn id(&self) -> Option<SubscriptionId> {
        match self {
            CoincallTicker::Pricing(data) => {
                let symbol = check_and_set_symbol(&data.data.symbol);
                Some(ExchangeSub::from((CoincallChannel::TICKER, symbol)).id())
            }
            CoincallTicker::Orderbook(data) => {
                let symbol = check_and_set_symbol(&data.data.symbol);
                Some(ExchangeSub::from((CoincallChannel::TICKER, symbol)).id())
            }
            CoincallTicker::OptionChain(data) => {
                let symbol = data
                    .data
                    .iter()
                    .find(|d| {
                        let current_symbol = SYMBOL.lock().unwrap();
                        d.symbol == *current_symbol
                    })
                    .map(|d| d.symbol.clone())
                    .unwrap_or(String::new());

                Some(ExchangeSub::from((CoincallChannel::TICKER, symbol)).id())
            }
            _ => None,
        }
    }
}

fn check_and_set_symbol(new_symbol: &str) -> String {
    let mut symbol = SYMBOL.lock().unwrap();
    if symbol.is_empty() {
        *symbol = new_symbol.to_string();
    }
    symbol.clone()
}

impl From<(ExchangeId, Instrument, CoincallTicker)> for MarketIter<Ticker> {
    fn from((exchange_id, instrument, ticker): (ExchangeId, Instrument, CoincallTicker)) -> Self {
        let ticker: Ticker = ticker.into();

        Self(vec![Ok(MarketEvent {
            exchange_time: Utc.timestamp_millis_opt(ticker.timestamp).unwrap(),
            received_time: Utc::now(),
            exchange: exchange_id.into(),
            instrument,
            kind: ticker,
        })])
    }
}

impl From<CoincallTicker> for Ticker {
    fn from(data: CoincallTicker) -> Self {
        let mut aggregate = CoincallTickerAggregator::new();
        aggregate.process_message(data);
        aggregate.ticker.clone()
    }
}

#[derive(Clone, Debug)]
pub struct CoincallTickerAggregator {
    ticker: Ticker,
}

impl CoincallTickerAggregator {
    pub fn new() -> Self {
        Self {
            ticker: Ticker::default(),
        }
    }

    pub fn process_message(&mut self, message: CoincallTicker) {
        match message {
            CoincallTicker::Pricing(data) => {
                self.process_pricing_data(&data.data);
            }
            CoincallTicker::OptionChain(data) => {
                let data = data
                    .data
                    .iter()
                    .find(|d| d.symbol == self.ticker.instrument_name)
                    .unwrap_or_else(|| &data.data[0]);

                self.process_option_chain_data(data);
            }
            CoincallTicker::Orderbook(data) => {
                self.process_orderbook_data(&data.data);
            }
            _ => {}
        }
    }

    pub fn process_pricing_data(&mut self, data: &CoincallOptionPricingData) {
        debug!("Processing Coincall pricing data: {:?}", data);
        self.ticker.timestamp = data.timestamp;
        self.ticker.mark_price = data.mark_price;
        self.ticker.last_price = data.last_price;
        self.ticker.open_interest = data.open_interest;
        self.ticker.greeks = Some(Greeks {
            delta: Some(data.delta),
            gamma: Some(data.gamma),
            theta: Some(data.theta),
            vega: Some(data.vega),
            rho: None,
        });
        self.ticker.mark_iv = Some(data.implied_volatility);
        self.ticker.index_price = data.index_price;
    }

    pub fn process_option_chain_data(&mut self, data: &CoincallOptionChainData) {
        debug!("Processing Coincall option chain data: {:?}", data);
        self.ticker.timestamp = data.timestamp;
        self.ticker.mark_price = data.mark_price;
        self.ticker.last_price = data.last_price;
        self.ticker.open_interest = data.open_interest;
        self.ticker.greeks = Some(Greeks {
            delta: Some(data.delta),
            gamma: Some(data.gamma),
            theta: Some(data.theta),
            vega: Some(data.vega),
            rho: None,
        });
        self.ticker.index_price = data.underlying_price;
        self.ticker.bid_iv = Some(data.bid_iv);
        self.ticker.ask_iv = Some(data.ask_iv);
    }

    pub fn process_orderbook_data(&mut self, data: &CoincallOptionOrderbookData) {
        debug!("Processing Coincall orderbook data: {:?}", data);
        let ob_default = &CoincallOrderbookData::default();
        let best_bid = data.bids.first().unwrap_or(ob_default);
        let best_ask = data.asks.first().unwrap_or(ob_default);

        self.ticker.timestamp = data.timestamp;
        self.ticker.best_bid_price = best_bid.price;
        self.ticker.best_ask_price = best_ask.price;
        self.ticker.best_bid_amount = best_bid.size;
        self.ticker.best_ask_amount = best_ask.size;
    }
}

impl Default for CoincallTickerAggregator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoincallTickerUpdater {
    updates_processed: u64,
}

impl CoincallTickerUpdater {
    pub fn new() -> Self {
        Self {
            updates_processed: 0,
        }
    }

    fn construct_ticker_from_update(update: CoincallTicker) -> Ticker {
        let mut aggregator = CoincallTickerAggregator::new();
        aggregator.process_message(update);
        aggregator.ticker.clone()
    }
}

#[async_trait]
impl TickerUpdater for CoincallTickerUpdater {
    type Ticker = Ticker;
    type Update = CoincallTicker;

    async fn init(
        _: mpsc::UnboundedSender<WsMessage>,
        instrument: Instrument,
    ) -> Result<InstrumentTicker<Self>, DataError> {
        let auth = fetch_authentication_parameters().await?;
        let instrument_name = validate_and_format_instrument_name(&instrument)?;
        let (ts, tsdiff) = calc_cc_ts(auth.server_ts);
        let secret = generate_secret(&auth, &instrument_name, ts, tsdiff)?;
        let sign = generate_signature(secret, &auth.key)?;
        let response = send_request(&auth, &instrument_name, &sign, ts, tsdiff).await?;
        let ob_data = extract_orderbook_data(response).await?;
        let ob_default = &CoincallObOrder::default();
        let best_bid = ob_data.bids.first().unwrap_or(ob_default);
        let best_ask = ob_data.asks.first().unwrap_or(ob_default);
        let ticker = InstrumentTicker {
            instrument,
            updater: Self::new(),
            ticker: Ticker {
                instrument_name,
                best_bid_price: best_bid.price.parse().unwrap_or_default(),
                best_ask_price: best_ask.price.parse().unwrap_or_default(),
                best_bid_amount: best_bid.size.parse().unwrap_or_default(),
                best_ask_amount: best_ask.size.parse().unwrap_or_default(),
                ..Default::default()
            },
        };

        debug!("Initialised Coincall ticker: {:?}", ticker);

        Ok(ticker)
    }

    fn update(
        &mut self,
        ticker: &mut Ticker,
        update: Self::Update,
    ) -> Result<Option<Ticker>, DataError> {
        let updated_ticker = Self::construct_ticker_from_update(update);

        ticker.merge(&updated_ticker).map_err(|e| {
            DataError::Socket(SocketError::Deserialise {
                error: serde_json::Error::custom(format!("Failed to merge ticker: {e}")),
                payload: format!("{:?}", updated_ticker),
            })
        })?;

        self.updates_processed += 1;

        Ok(Some(ticker.clone()))
    }
}

async fn fetch_authentication_parameters() -> Result<CoincallOptionAuthData, DataError> {
    get_cc_auth_parms()
        .await
        .map_err(|e| DataError::from(SocketError::Exchange(e.to_string())))
}

fn validate_and_format_instrument_name(instrument: &Instrument) -> Result<String, DataError> {
    match instrument.kind {
        InstrumentKind::Option(option) => Ok(format_opt_instr(
            instrument.base.as_ref(),
            instrument.quote.as_ref(),
            option.expiry,
            option.strike,
            option.kind,
        )),
        _ => Err(DataError::from(SocketError::Exchange(
            "Invalid instrument".to_string(),
        ))),
    }
}

fn generate_secret(
    auth: &CoincallOptionAuthData,
    instrument_name: &str,
    ts: i64,
    tsdiff: i64,
) -> Result<String, DataError> {
    let sig_params = SigGenParams {
        key: auth.key.clone(),
        uuid: auth.uuid.clone(),
        ts,
        tsdiff,
        instrument_name: Cow::Borrowed(instrument_name),
    };
    Ok(gen_cc_secret(
        &sig_params,
        reqwest::Method::GET.as_str(),
        COINCALL_ORDERBOOK_V1,
    ))
}

fn generate_signature(secret: String, key: &str) -> Result<String, DataError> {
    gen_cc_sig(secret, key).map_err(|e| DataError::from(SocketError::Exchange(e.to_string())))
}

async fn send_request(
    auth: &CoincallOptionAuthData,
    instrument_name: &str,
    sign: &str,
    ts: i64,
    tsdiff: i64,
) -> Result<reqwest::Response, DataError> {
    let req = reqwest::Client::new()
        .get(&format!(
            "https://www.coincall.com/api{}/{}",
            COINCALL_ORDERBOOK_V1, instrument_name
        ))
        .header("Authorization", format!("Bearer {}", auth.token))
        .header("Key", &auth.key)
        .header("Sign", sign)
        .header("Ts", ts.to_string())
        .header("Tsdiff", tsdiff.to_string())
        .header("Uuid", &auth.uuid);

    debug!("Sending Coincall request: {:?}", req);

    req.send()
        .await
        .map_err(|e| DataError::from(SocketError::Http(e)))
}

async fn extract_orderbook_data(response: reqwest::Response) -> Result<CoincallObData, DataError> {
    let resp_text = response.text().await.unwrap_or_default();
    let ob: CoincallOrderbook = serde_json::from_str(&resp_text).map_err(|e| {
        DataError::from(SocketError::Deserialise {
            error: serde_json::Error::io(io::Error::new(
                io::ErrorKind::Other,
                format!("Cannot deserialize orderbook response: {}", e),
            )),
            payload: resp_text.clone(),
        })
    })?;
    let msg = ob.msg;

    ob.data.ok_or(DataError::from(SocketError::Exchange(format!(
        "No data in orderbook response: {msg}"
    ))))
}
