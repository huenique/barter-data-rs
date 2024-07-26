use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::Duration;

use crate::exchange::deribit::book::l1::DeribitOrderBookL1;
use crate::exchange::deribit::book::l2::DeribitBookUpdater;
use crate::exchange::deribit::channel::DeribitChannel;
use crate::exchange::deribit::index::DeribitIndex;
use crate::exchange::deribit::market::DeribitMarket;
use crate::exchange::deribit::subscription::DeribitSubResponse;
use crate::exchange::deribit::ticker::DeribitTicker;
use crate::exchange::deribit::trade::DeribitTrades;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeServer;
use crate::exchange::ExchangeSub;
use crate::exchange::PingInterval;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::book::OrderBooksL1;
use crate::subscription::book::OrderBooksL2;
use crate::subscription::index::Indices;
use crate::subscription::ticker::Tickers;
use crate::subscription::trade::PublicTrades;
use crate::transformer::book::MultiBookTransformer;
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

pub mod trade;

pub mod book;

pub mod index;

pub mod ticker;

/// [`Deribit`] server base url.
///
/// See docs: <https://docs.deribit.com/#json-rpc-over-websocket>
// pub const BASE_URL_DERIBIT: &str = "wss://streams.deribit.com/ws/api/v2";
pub const BASE_URL_DERIBIT_MAINNET: &str = "wss://www.deribit.com/ws/api/v2";

pub const BASE_URL_DERIBIT_TESTNET: &str = "wss://test.deribit.com/ws/api/v2";

/// [`Deribit`] server [`PingInterval`] duration.
///
/// See docs: <https://docs.deribit.com/#public-set_heartbeat>
pub const PING_INTERVAL_DERIBIT: Duration = Duration::from_secs(29);

/// Testnet
pub type DeribitTest = Deribit<DeribitTestnetServer>;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DeribitTestnetServer {}

impl ExchangeServer for DeribitTestnetServer {
    const ID: ExchangeId = ExchangeId::DeribitTestnet;

    fn websocket_url() -> &'static str {
        BASE_URL_DERIBIT_TESTNET
    }
}
/// ---

/// Mainnet
pub type DeribitMain = Deribit<DeribitMainnetServer>;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct DeribitMainnetServer {}

impl ExchangeServer for DeribitMainnetServer {
    const ID: ExchangeId = ExchangeId::DeribitMainnet;

    fn websocket_url() -> &'static str {
        BASE_URL_DERIBIT_MAINNET
    }
}
/// ---

/// [`Deribit`] exchange.
///
/// See docs: <https://docs.deribit.com/#json-rpc-over-websocket>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Deribit<Server> {
    server: PhantomData<Server>,
}

impl<Server> Connector for Deribit<Server>
where
    Server: ExchangeServer,
{
    const ID: ExchangeId = Server::ID;
    type Channel = DeribitChannel;
    type Market = DeribitMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = DeribitSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(Server::websocket_url()).map_err(SocketError::UrlParse)
    }

    fn ping_interval() -> Option<super::PingInterval> {
        Some(PingInterval {
            interval: tokio::time::interval(PING_INTERVAL_DERIBIT),
            ping: || WsMessage::Ping("ping".as_bytes().to_vec()),
        })
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        let stream_names = exchange_subs
            .into_iter()
            .map(|sub| sub.channel.as_ref().replace("{}", sub.market.as_ref()))
            .collect::<Vec<String>>();

        vec![WsMessage::Text(
            json!({
                "jsonrpc": "2.0",
                "method": "public/subscribe",
                "params": {
                    "channels": stream_names
                }
            })
            .to_string(),
        )]
    }
}

impl<Server> StreamSelector<PublicTrades> for Deribit<Server>
where
    Server: ExchangeServer + Debug + Send + Sync,
{
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, DeribitTrades>>;
}

impl<Server> StreamSelector<OrderBooksL1> for Deribit<Server>
where
    Server: ExchangeServer + Debug + Send + Sync,
{
    type Stream = ExchangeWsStream<StatelessTransformer<Self, OrderBooksL1, DeribitOrderBookL1>>;
}

impl<Server> StreamSelector<OrderBooksL2> for Deribit<Server>
where
    Server: ExchangeServer + Debug + Send + Sync,
{
    type Stream = ExchangeWsStream<MultiBookTransformer<Self, OrderBooksL2, DeribitBookUpdater>>;
}

impl<Server> StreamSelector<Tickers> for Deribit<Server>
where
    Server: ExchangeServer + Debug + Send + Sync,
{
    type Stream = ExchangeWsStream<StatelessTransformer<Self, Tickers, DeribitTicker>>;
}

impl<Server> StreamSelector<Indices> for Deribit<Server>
where
    Server: ExchangeServer + Debug + Send + Sync,
{
    type Stream = ExchangeWsStream<StatelessTransformer<Self, Indices, DeribitIndex>>;
}

impl<'de, Server> serde::Deserialize<'de> for Deribit<Server>
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

impl<Server> serde::Serialize for Deribit<Server>
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
