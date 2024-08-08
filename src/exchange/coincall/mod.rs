use std::fmt::Debug;
use std::marker::PhantomData;

use crate::exchange::coincall::channel::CoincallChannel;
use crate::exchange::coincall::market::CoincallMarket;
use crate::exchange::coincall::subscription::CoincallSubResponse;
use crate::exchange::coincall::ticker::CoincallTicker;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeServer;
use crate::exchange::ExchangeSub;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::ticker::Tickers;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeWsStream;

use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use serde_json::json;
use url::Url;

pub mod channel;

pub mod market;

pub mod subscription;

pub mod message;

pub mod ticker;

/// [`Coincall`] server base url.
pub const BASE_URL_COINCALL: &str = "";

/// [`Coincall`] exchange.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Coincall<Server> {
    server: PhantomData<Server>,
}

impl<Server> Connector for Coincall<Server>
where
    Server: ExchangeServer,
{
    const ID: ExchangeId = Server::ID;
    type Channel = CoincallChannel;
    type Market = CoincallMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = CoincallSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_COINCALL).map_err(SocketError::UrlParse)
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

impl<Server> StreamSelector<Tickers> for Coincall<Server>
where
    Server: ExchangeServer + Debug + Send + Sync,
{
    type Stream = ExchangeWsStream<StatelessTransformer<Self, Tickers, CoincallTicker>>;
}

impl<'de, Server> serde::Deserialize<'de> for Coincall<Server>
where
    Server: ExchangeServer,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let input = <String as serde::Deserialize>::deserialize(deserializer)?;
        let expected = Self::ID.as_str();

        if input.as_str() == Self::ID.as_str() {
            Ok(Self::default())
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(input.as_str()),
                &expected,
            ))
        }
    }
}

impl<Server> serde::Serialize for Coincall<Server>
where
    Server: ExchangeServer,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let exchange_id = Self::ID.as_str();
        serializer.serialize_str(exchange_id)
    }
}
