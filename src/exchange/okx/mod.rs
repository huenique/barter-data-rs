use std::time::Duration;

use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use serde_json::json;
use ticker::OkxTickerUpdater;
use tracing::info;
use url::Url;

use crate::exchange::okx::channel::OkxChannel;
use crate::exchange::okx::market::OkxMarket;
use crate::exchange::okx::subscription::OkxSubResponse;
use crate::exchange::okx::trade::OkxTrades;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::exchange::PingInterval;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::ticker::Tickers;
use crate::subscription::trade::PublicTrades;
use crate::transformer::stateless::StatelessTransformer;
use crate::transformer::ticker::MultiTickerTransformer;
use crate::ExchangeWsStream;

/// Defines the type that translates a Barter
/// [`Subscription`](crate::subscription::Subscription) into an exchange
/// [`Connector`] specific channel used for generating [`Connector::requests`].
pub mod channel;

/// Defines the type that translates a Barter
/// [`Subscription`](crate::subscription::Subscription) into an exchange
/// [`Connector`] specific market used for generating [`Connector::requests`].
pub mod market;

/// Message types for [`Okx`].
pub mod message;

/// [`Subscription`](crate::subscription::Subscription) response type and
/// response [`Validator`](barter_integration::Validator) for [`Okx`].
pub mod subscription;

/// Public trade types for [`Okx`].
pub mod trade;

/// Public ticker types for [`Okx`].
pub mod ticker;

/// [`Okx`] server base url.
///
/// See docs: <https://www.okx.com/docs-v5/en/#overview-api-resources-and-support>
pub const BASE_URL_OKX: &str = "wss://wsaws.okx.com:8443/ws/v5/public";

/// [`Okx`] server [`PingInterval`] duration.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-connect>
pub const PING_INTERVAL_OKX: Duration = Duration::from_secs(29);

/// [`Okx`] exchange.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api>
#[derive(
    Clone, Copy, DeExchange, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, SerExchange,
)]
pub struct Okx;

impl Connector for Okx {
    const ID: ExchangeId = ExchangeId::Okx;
    type Channel = OkxChannel;
    type Market = OkxMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = OkxSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_OKX).map_err(SocketError::UrlParse)
    }

    fn ping_interval() -> Option<PingInterval> {
        Some(PingInterval {
            interval: tokio::time::interval(PING_INTERVAL_OKX),
            ping: || WsMessage::text("ping"),
        })
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        // Check if all channels are delimited by a dot ('.')
        let all_channels_have_dot = exchange_subs
            .iter()
            .all(|sub| sub.channel.as_ref().contains('.'));

        if all_channels_have_dot {
            info!("All channels have dots, processing each channel separately");

            // If channels have dots, process each channel separately
            exchange_subs
                .into_iter()
                .map(|sub| {
                    // Split the channel by '.' and create a JSON object for each part
                    let args = sub
                        .channel
                        .as_ref()
                        .split('.')
                        .map(|channel_part| {
                            json!({
                                "channel": channel_part,
                                "instId": sub.market.as_ref()
                            })
                        })
                        .collect::<Vec<_>>();

                    // Create a single WsMessage::Text for each ExchangeSub entry
                    WsMessage::Text(
                        json!({
                            "op": "subscribe",
                            "args": args
                        })
                        .to_string(),
                    )
                })
                .collect::<Vec<_>>()
        } else {
            info!("Channels do not contain dots, processing all channels together");

            // If channels do not contain dots, return a single WsMessage::Text with all exchange_subs
            vec![WsMessage::Text(
                json!({
                    "op": "subscribe",
                    "args": exchange_subs
                        .into_iter()
                        .map(|sub| {
                            json!({
                                "channel": sub.channel.as_ref(),
                                "instId": sub.market.as_ref()
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .to_string(),
            )]
        }
    }
}

impl StreamSelector<PublicTrades> for Okx {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, OkxTrades>>;
}

impl StreamSelector<Tickers> for Okx {
    type Stream = ExchangeWsStream<MultiTickerTransformer<Self, Tickers, OkxTickerUpdater>>;
}
