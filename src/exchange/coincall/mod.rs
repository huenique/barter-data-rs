use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::Duration;

use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use option::ticker::CoincallTickerUpdater;
use serde_json::json;
use url::Url;

use crate::exchange::coincall::channel::CoincallChannel;
use crate::exchange::coincall::market::CoincallMarket;
use crate::exchange::coincall::subscription::CoincallSubResponse;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeServer;
use crate::exchange::ExchangeSub;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::ticker::Tickers;
use crate::transformer::ticker::MultiTickerTransformer;
use crate::ExchangeWsStream;
use crate::PingInterval;

pub mod auth;

pub mod channel;

pub mod market;

pub mod subscription;

pub mod message;

pub mod utils;

/// [`ExchangeServer`] and [`StreamSelector`] implementations for
/// [`Coincall`](option::CoincallOption).
pub mod option;

/// [`Coincall`] server [`PingInterval`] duration.
///
/// See docs: <https://docs.coincall.com/#options-websocket>
pub const PING_INTERVAL_COINCALL: Duration = Duration::from_secs(29);

/// [`Coincall`] exchange.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
        Url::parse(Server::websocket_url()).map_err(SocketError::UrlParse)
    }

    fn ping_interval() -> Option<super::PingInterval> {
        Some(PingInterval {
            interval: tokio::time::interval(PING_INTERVAL_COINCALL),
            ping: || WsMessage::Text(json!({"c": 11}).to_string()),
        })
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        exchange_subs
            .into_iter()
            .flat_map(|ExchangeSub { channel, market }| {
                let chs = channel.as_ref().split('_').collect::<Vec<&str>>();
                chs.into_iter()
                    .map(|ch| {
                        WsMessage::Text(
                            json!({
                                "c": 20,
                                "dt": ch.parse::<i32>().unwrap(),
                                "d": {
                                    "s": market.as_ref()
                                }
                            })
                            .to_string(),
                        )
                    })
                    .collect::<Vec<WsMessage>>()
            })
            .collect()
    }
}

impl<Server> StreamSelector<Tickers> for Coincall<Server>
where
    Server: ExchangeServer + Debug + Send + Sync,
{
    type Stream = ExchangeWsStream<MultiTickerTransformer<Self, Tickers, CoincallTickerUpdater>>;
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
