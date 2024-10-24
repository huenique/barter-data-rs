use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use serde_json::json;
use url::Url;

use crate::exchange::derive::channel::DeriveChannel;
use crate::exchange::derive::market::DeriveMarket;
use crate::exchange::derive::subscription::DeriveSubResponse;
use crate::exchange::derive::ticker::DeriveTicker;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::ticker::Tickers;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeWsStream;

pub mod channel;

pub mod market;

pub mod subscription;

pub mod message;

pub mod ticker;

/// [`Derive`] server base url.
///
/// See docs: <https://docs.derive.finance/reference/subscribe>
pub const BASE_URL_DERIVE: &str = "wss://api.derive.finance/ws";

/// [`Derive`] exchange.
///
/// See docs: <https://docs.derive.finance/reference/json-rpc#websocket>
#[derive(
    Clone, Copy, DeExchange, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, SerExchange,
)]
pub struct Derive;

impl Connector for Derive {
    const ID: ExchangeId = ExchangeId::Derive;
    type Channel = DeriveChannel;
    type Market = DeriveMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = DeriveSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_DERIVE).map_err(SocketError::UrlParse)
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        let stream_names = exchange_subs
            .into_iter()
            .map(|sub| sub.channel.as_ref().replace("{}", sub.market.as_ref()))
            .collect::<Vec<String>>();

        vec![WsMessage::Text(
            json!({"id":"ws-subscribe","method":"subscribe","params":{"channels":stream_names}})
                .to_string(),
        )]
    }
}

impl StreamSelector<Tickers> for Derive {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, Tickers, DeriveTicker>>;
}
