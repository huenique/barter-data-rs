pub mod book;
pub mod channel;
pub mod market;
pub mod message;
pub mod subscription;

use std::time::Duration;

use barter_integration::error::SocketError;
use barter_integration::model::instrument::Instrument;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use url::Url;

use crate::exchange::Connector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::book::OrderBooksL3;
use crate::subscription::Map;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeWsStream;

use self::book::l3::PowerTradeOrderBookL3;
use self::channel::PowerTradeChannel;
use self::market::PowerTradeMarket;
use self::subscription::PowerTradePlatformEvent;

use super::subscription::ExchangeSub;
use super::ExchangeId;
use super::PingInterval;
use super::StreamSelector;

/// <https://power-trade.github.io/api-docs-source/ws_feeds.html#Market_Feeds_Connection_Parameters>
pub const BASE_URL_POWERTRADE: &str = "wss://api.wss.prod.power.trade/v1/feeds/market_data?mbp_period=1&mbo_period=0&snapshot_depth=100";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DeExchange, SerExchange)]
pub struct PowerTrade {
    pub connection_params: Vec<(&'static str, &'static str)>,
}

impl Connector for PowerTrade {
    const ID: ExchangeId = ExchangeId::PowerTrade;
    type Channel = PowerTradeChannel;
    type Market = PowerTradeMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = PowerTradePlatformEvent;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_POWERTRADE).map_err(SocketError::UrlParse)
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        exchange_subs
            .into_iter()
            .map(|sub| {
                let subscribe = serde_json::json!({
                    "subscribe": {
                        "symbol": sub.market.as_ref(),
                    },
                });
                WsMessage::Text(subscribe.to_string())
            })
            .collect()
    }

    fn ping_interval() -> Option<PingInterval> {
        None
    }

    fn expected_responses(map: &Map<Instrument>) -> usize {
        map.0.len()
    }

    fn subscription_timeout() -> Duration {
        super::DEFAULT_SUBSCRIPTION_TIMEOUT
    }
}

impl StreamSelector<OrderBooksL3> for PowerTrade {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, OrderBooksL3, PowerTradeOrderBookL3>>;
}
