use std::time::Duration;

use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use serde_json::json;
use tokio::time;
use url::Url;

use crate::exchange::hyperliquid::book::l2::HyperliquidOrderBookUpdater;
use crate::exchange::hyperliquid::channel::HyperliquidChannel;
use crate::exchange::hyperliquid::market::HyperliquidMarket;
use crate::exchange::hyperliquid::subscription::HyperliquidSubResponse;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::exchange::PingInterval;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::book::OrderBooksL2;
use crate::transformer::book::MultiBookTransformer;
use crate::ExchangeWsStream;

pub mod book;

pub mod channel;

pub mod market;

pub mod message;

pub mod subscription;

const HYPERLIQUID_URL: &str = "wss://api.hyperliquid.xyz/ws";
#[derive(
    Clone, Copy, DeExchange, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, SerExchange,
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
