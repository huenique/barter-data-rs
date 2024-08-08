use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use serde_json::json;
use url::Url;

use crate::exchange::lyra::channel::LyraChannel;
use crate::exchange::lyra::market::LyraMarket;
use crate::exchange::lyra::subscription::LyraSubResponse;
use crate::exchange::lyra::ticker::LyraTicker;
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

/// [`Lyra`] server base url.
///
/// See docs: <https://docs.lyra.finance/reference/subscribe>
pub const BASE_URL_LYRA: &str = "wss://api.lyra.finance/ws";

/// [`Lyra`] exchange.
///
/// See docs: <https://docs.lyra.finance/reference/json-rpc#websocket>
#[derive(
    Clone, Copy, DeExchange, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, SerExchange,
)]
pub struct Lyra;

impl Connector for Lyra {
    const ID: ExchangeId = ExchangeId::Lyra;
    type Channel = LyraChannel;
    type Market = LyraMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = LyraSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_LYRA).map_err(SocketError::UrlParse)
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

impl StreamSelector<Tickers> for Lyra {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, Tickers, LyraTicker>>;
}
