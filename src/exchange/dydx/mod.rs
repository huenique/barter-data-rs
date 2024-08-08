use std::fmt::Debug;

use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use url::Url;

use crate::exchange::dydx::book::l2::DydxOrderBookUpdater;
use crate::exchange::dydx::channel::DydxChannel;
use crate::exchange::dydx::market::DydxMarket;
use crate::exchange::dydx::subscription::DydxSubResponse;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
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

const BASE_URL_DYDX: &str = "wss://indexer.dydx.trade/v4/ws";
#[derive(
    Clone, Copy, DeExchange, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, SerExchange,
)]
pub struct Dydx;

impl Connector for Dydx {
    const ID: ExchangeId = ExchangeId::Dydx;
    type Channel = DydxChannel;
    type Market = DydxMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = DydxSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_DYDX).map_err(SocketError::UrlParse)
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        exchange_subs
            .into_iter()
            .map(|sub| {
                WsMessage::Text(
                    serde_json::json!({
                        "type": "subscribe",
                        "channel": sub.channel.as_ref(),
                        "id": sub.market.as_ref(),
                    })
                    .to_string(),
                )
            })
            .collect()
    }
}

impl StreamSelector<OrderBooksL2> for Dydx {
    type Stream = ExchangeWsStream<MultiBookTransformer<Self, OrderBooksL2, DydxOrderBookUpdater>>;
}
