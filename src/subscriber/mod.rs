use async_trait::async_trait;
use barter_integration::error::SocketError;
use barter_integration::model::instrument::Instrument;
use barter_integration::protocol::websocket::connect;
use barter_integration::protocol::websocket::WebSocket;
use futures::SinkExt;
use serde::Deserialize;
use serde::Serialize;
use tracing::debug;
use tracing::info;

use crate::exchange::Connector;
use crate::subscriber::mapper::SubscriptionMapper;
use crate::subscriber::mapper::WebSocketSubMapper;
use crate::subscriber::validator::SubscriptionValidator;
use crate::subscription::Map;
use crate::subscription::SubKind;
use crate::subscription::Subscription;
use crate::subscription::SubscriptionMeta;
use crate::Identifier;

/// [`SubscriptionMapper`](mapper::SubscriptionMapper) implementations defining
/// how to map a collection of Barter [`Subscription`]s into exchange specific
/// [`SubscriptionMeta`].
pub mod mapper;

/// [`SubscriptionValidator`](validator::SubscriptionValidator) implementations
/// defining how to validate actioned [`Subscription`]s were successful.
pub mod validator;

/// Defines how to connect to a socket and subscribe to market data streams.
#[async_trait]
pub trait Subscriber {
    type SubMapper: SubscriptionMapper;

    async fn subscribe<Exchange, Kind>(
        subscriptions: &[Subscription<Exchange, Kind>],
    ) -> Result<(WebSocket, Map<Instrument>), SocketError>
    where
        Exchange: Connector + Send + Sync,
        Kind: SubKind + Send + Sync,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>;
}

/// Standard [`Subscriber`] for [`WebSocket`]s suitable for most exchanges.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebSocketSubscriber;

#[async_trait]
impl Subscriber for WebSocketSubscriber {
    type SubMapper = WebSocketSubMapper;

    async fn subscribe<Exchange, Kind>(
        subscriptions: &[Subscription<Exchange, Kind>],
    ) -> Result<(WebSocket, Map<Instrument>), SocketError>
    where
        Exchange: Connector + Send + Sync,
        Kind: SubKind + Send + Sync,
        Subscription<Exchange, Kind>: Identifier<Exchange::Channel> + Identifier<Exchange::Market>,
    {
        // Define variables for logging ergonomics
        let exchange = Exchange::ID;
        let url = Exchange::url()?;
        debug!(%exchange, %url, ?subscriptions, "subscribing to WebSocket");

        // Connect to exchange
        let mut websocket = connect(url).await?;
        debug!(%exchange, ?subscriptions, "connected to WebSocket");

        // Map &[Subscription<Exchange, Kind>] to SubscriptionMeta
        let SubscriptionMeta {
            instrument_map,
            subscriptions,
        } = Self::SubMapper::map::<Exchange, Kind>(subscriptions);

        // Send Subscriptions over WebSocket
        for subscription in subscriptions {
            debug!(%exchange, payload = ?subscription, "sending exchange subscription");
            websocket.send(subscription).await?;
        }

        // Validate Subscription responses
        let map =
            Exchange::SubValidator::validate::<Exchange, Kind>(instrument_map, &mut websocket)
                .await?;

        info!(%exchange, "subscribed to WebSocket");
        Ok((websocket, map))
    }
}
