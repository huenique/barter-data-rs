use std::time::Duration;

use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use serde_json::json;
use url::Url;

use crate::exchange::aevo::book::l2::AevoBookUpdater;
use crate::exchange::aevo::channel::AevoChannel;
use crate::exchange::aevo::market::AevoMarket;
use crate::exchange::aevo::subscription::AevoSubResponse;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::book::OrderBooksL2;
use crate::transformer::book::MultiBookTransformer;
use crate::ExchangeWsStream;

pub mod channel;

pub mod market;

pub mod subscription;

pub mod message;

pub mod book;

/// [`Aevo`] server base url.
///
/// See docs: <https://docs.aevo.xyz/reference/endpoints>
pub const BASE_URL_AEVO: &str = "wss://ws.aevo.xyz";

/// [`Aevo`] server [`PingInterval`] duration.
///
/// See docs: <https://docs.aevo.xyz/reference/overview-1>
pub const PING_INTERVAL_AEVO: Duration = Duration::from_secs(840);

/// [`Aevo`] exchange.
///
/// See docs: <https://docs.aevo.xyz/reference/overview>
#[derive(
    Clone, Copy, DeExchange, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, SerExchange,
)]
pub struct Aevo;

impl Connector for Aevo {
    const ID: ExchangeId = ExchangeId::Aevo;
    type Channel = AevoChannel;
    type Market = AevoMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = AevoSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_AEVO).map_err(SocketError::UrlParse)
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        let stream_names = exchange_subs
            .into_iter()
            .map(|sub| format!("{}:{}", sub.channel.as_ref(), sub.market.as_ref()))
            .collect::<Vec<String>>();

        vec![WsMessage::Text(
            json!({
                "op": "subscribe",
                "data": stream_names
            })
            .to_string(),
        )]
    }
}

impl StreamSelector<OrderBooksL2> for Aevo {
    type Stream = ExchangeWsStream<MultiBookTransformer<Self, OrderBooksL2, AevoBookUpdater>>;
}
