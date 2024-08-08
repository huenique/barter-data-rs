use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use serde_json::json;
use url::Url;

use crate::exchange::coinbase::channel::CoinbaseChannel;
use crate::exchange::coinbase::market::CoinbaseMarket;
use crate::exchange::coinbase::subscription::CoinbaseSubResponse;
use crate::exchange::coinbase::trade::CoinbaseTrade;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::trade::PublicTrades;
use crate::transformer::stateless::StatelessTransformer;
use crate::ExchangeWsStream;

/// Defines the type that translates a Barter
/// [`Subscription`](crate::subscription::Subscription) into an exchange
/// [`Connector`] specific channel used for generating [`Connector::requests`].
pub mod channel;

/// Defines the type that translates a Barter
/// [`Subscription`](crate::subscription::Subscription) into an exchange
/// [`Connector`] specific market used for generating [`Connector::requests`].
pub mod market;

/// [`Subscription`](crate::subscription::Subscription) response type and
/// response [`Validator`](barter_integration::Validator) for [`Coinbase`].
pub mod subscription;

/// Public trade types for [`Coinbase`].
pub mod trade;

/// [`Coinbase`] server base url.
///
/// See docs: <https://docs.cloud.coinbase.com/exchange/docs/websocket-overview>
pub const BASE_URL_COINBASE: &str = "wss://ws-feed.exchange.coinbase.com";

/// [`Coinbase`] exchange.
///
/// See docs: <https://docs.cloud.coinbase.com/exchange/docs/websocket-overview>
#[derive(
    Clone, Copy, DeExchange, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, SerExchange,
)]
pub struct Coinbase;

impl Connector for Coinbase {
    const ID: ExchangeId = ExchangeId::Coinbase;
    type Channel = CoinbaseChannel;
    type Market = CoinbaseMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = CoinbaseSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_COINBASE).map_err(SocketError::UrlParse)
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        exchange_subs
            .into_iter()
            .map(|ExchangeSub { channel, market }| {
                WsMessage::Text(
                    json!({
                        "type": "subscribe",
                        "product_ids": [market.as_ref()],
                        "channels": [channel.as_ref()],
                    })
                    .to_string(),
                )
            })
            .collect()
    }
}

impl StreamSelector<PublicTrades> for Coinbase {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, CoinbaseTrade>>;
}
