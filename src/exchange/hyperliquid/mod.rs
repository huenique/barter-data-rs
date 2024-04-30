pub mod book;
pub mod channel;
pub mod market;
pub mod message;
pub mod subscription;
pub mod validator;

use self::book::l2::HyperliquidOrderBookUpdater;
use self::channel::HyperliquidChannel;
use self::market::HyperliquidMarket;
use self::subscription::HyperliquidSubResponse;
use super::PingInterval;
use super::StreamSelector;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::book::OrderBooksL2;
use crate::transformer::book::MultiBookTransformer;
use crate::ExchangeWsStream;
use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use serde_json::json;
use std::time::Duration;
use tokio::time;
use url::Url;

const HYPERLIQUID_URL: &str = "wss://api.hyperliquid.xyz/ws";

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DeExchange, SerExchange,
)]
pub struct Hyperliquid;

impl Connector for Hyperliquid {
    const ID: ExchangeId = ExchangeId::Hyperliquid;
    type Channel = HyperliquidChannel;
    type Market = HyperliquidMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = HyperliquidSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(HYPERLIQUID_URL).map_err(SocketError::UrlParse)
    }

    fn ping_interval() -> Option<PingInterval> {
        Some(PingInterval {
            interval: time::interval(Duration::from_millis(30_000)),
            ping: || WsMessage::Text(serde_json::json!({ "method": "ping" }).to_string()),
        })
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        exchange_subs
            .into_iter()
            .map(|sub| {
                WsMessage::Text(
                    json!({
                        "method": "subscribe",
                        "subscription": {
                            "type": sub.channel.as_ref(),
                            "coin": sub.market.as_ref(),
                        }
                    })
                    .to_string(),
                )
            })
            .collect()
    }
}

impl StreamSelector<OrderBooksL2> for Hyperliquid {
    type Stream =
        ExchangeWsStream<MultiBookTransformer<Self, OrderBooksL2, HyperliquidOrderBookUpdater>>;
}
