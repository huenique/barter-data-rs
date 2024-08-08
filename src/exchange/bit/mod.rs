use std::time::Duration;

use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use tokio::time;
use url::Url;

use crate::exchange::bit::book::l2::BitOrderBookL2;
use crate::exchange::bit::channel::BitChannel;
use crate::exchange::bit::market::BitMarket;
use crate::exchange::bit::subscription::BitSubResponse;
use crate::exchange::Connector;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::book::OrderBooksL2;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeId;
use crate::ExchangeWsStream;
use crate::PingInterval;

pub mod book;

pub mod channel;

pub mod market;

pub mod message;

pub mod subscription;

const BASE_URL_BIT: &str = "wss://ws.bit.com";
#[derive(
    Clone, Copy, DeExchange, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, SerExchange,
)]
pub struct Bit;

impl Connector for Bit {
    const ID: ExchangeId = ExchangeId::Bit;
    type Channel = BitChannel;
    type Market = BitMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = BitSubResponse;

    fn url() -> Result<url::Url, barter_integration::error::SocketError> {
        Url::parse(BASE_URL_BIT).map_err(SocketError::UrlParse)
    }

    fn ping_interval() -> Option<PingInterval> {
        Some(PingInterval {
            interval: time::interval(Duration::from_millis(30_000)),
            ping: || WsMessage::Text(serde_json::json!({ "type": "ping" }).to_string()),
        })
    }

    fn requests(
        exchange_subs: Vec<super::subscription::ExchangeSub<Self::Channel, Self::Market>>,
    ) -> Vec<barter_integration::protocol::websocket::WsMessage> {
        exchange_subs
            .into_iter()
            .map(|sub| {
                barter_integration::protocol::websocket::WsMessage::Text(
                    serde_json::json!({
                        "type": "subscribe",
                        "instruments": [sub.market.as_ref()],
                        "channels": [sub.channel.as_ref()],
                        "interval": "raw",
                    })
                    .to_string(),
                )
            })
            .collect()
    }
}

impl StreamSelector<OrderBooksL2> for Bit {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, OrderBooksL2, BitOrderBookL2>>;
}
