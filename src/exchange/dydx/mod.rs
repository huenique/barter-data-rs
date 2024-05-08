pub mod book;
pub mod channel;
pub mod market;
pub mod subscription;
use self::market::DydxMarket;
use self::subscription::DydxResponse;
use super::ExchangeServer;
use crate::exchange::dydx::channel::DydxChannel;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use std::fmt::Debug;
use std::marker::PhantomData;
use url::Url;

const DYDX_URL: &str = "wss://indexer.dydx.trade/v4/ws";

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Dydx<Server> {
    server: PhantomData<Server>,
}

impl<Server> Connector for Dydx<Server>
where
    Server: ExchangeServer,
{
    const ID: ExchangeId = Server::ID;
    type Channel = DydxChannel;
    type Market = DydxMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = DydxResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(Server::websocket_url()).map_err(|e| SocketError::UrlParse)
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        todo!()
    }
}

impl<'de, Server> serde::Deserialize<'de> for Dydx<Server>
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

impl<Server> serde::Serialize for Dydx<Server>
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
