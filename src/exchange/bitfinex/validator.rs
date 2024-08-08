use async_trait::async_trait;
use barter_integration::error::SocketError;
use barter_integration::model::instrument::Instrument;
use barter_integration::model::SubscriptionId;
use barter_integration::protocol::websocket::WebSocket;
use barter_integration::protocol::websocket::WebSocketParser;
use barter_integration::protocol::StreamParser;
use barter_integration::Validator;
use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use tracing::debug;

use crate::exchange::bitfinex::subscription::BitfinexPlatformEvent;
use crate::exchange::bitfinex::subscription::BitfinexSubResponse;
use crate::exchange::Connector;
use crate::exchange::ExchangeSub;
use crate::subscriber::validator::SubscriptionValidator;
use crate::subscription::Map;
use crate::subscription::SubKind;
use crate::Identifier;

/// [`Bitfinex`](super::Bitfinex) specific [`SubscriptionValidator`].
///
/// ### Notes
/// - Required because Bitfinex has a non-self-describing data format after
///   subscriptions have been validated.
/// - The [`BitfinexChannelId`](super::subscription::BitfinexChannelId) is used
///   to identify the [`Subscription`](crate::subscription::Subscription)
///   associated with incoming events, rather than a `String` channel-market
///   identifier.
/// - Therefore the [`SubscriptionId`] format must change during
///   [`BitfinexWebSocketSubValidator::validate`] to use the
///   [`BitfinexChannelId`](super::subscription::BitfinexChannelId) (see module
///   level "SubscriptionId" documentation notes for more details).
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BitfinexWebSocketSubValidator;

#[async_trait]
impl SubscriptionValidator for BitfinexWebSocketSubValidator {
    type Parser = WebSocketParser;

    async fn validate<Exchange, Kind>(
        mut map: Map<Instrument>,
        websocket: &mut WebSocket,
    ) -> Result<Map<Instrument>, SocketError>
    where
        Exchange: Connector + Send,
        Kind: SubKind + Send,
    {
        // Establish exchange specific subscription validation parameters
        let timeout = Exchange::subscription_timeout();
        let expected_responses = Exchange::expected_responses(&map);

        // Parameter to keep track of successful Subscription outcomes
        // '--> Bitfinex sends snapshots as the first message, so count them also
        let mut success_responses = 0usize;
        let mut init_snapshots_received = 0usize;

        loop {
            // Break if all Subscriptions were a success
            if success_responses == expected_responses
                && init_snapshots_received == expected_responses
            {
                debug!(exchange = %Exchange::ID, "validated exchange WebSocket subscriptions");
                break Ok(map);
            }

            tokio::select! {
                // If timeout reached, return SubscribeError
                _ = tokio::time::sleep(timeout) => {
                    break Err(SocketError::Subscribe(
                        format!("subscription validation timeout reached: {:?}", timeout)
                    ))
                },
                // Parse incoming messages and determine subscription outcomes
                message = websocket.next() => {
                    let response = match message {
                        Some(response) => response,
                        None => break Err(SocketError::Subscribe("WebSocket stream terminated unexpectedly".to_string()))
                    };

                    match Self::Parser::parse::<BitfinexPlatformEvent>(response) {
                        Some(Ok(response)) => match response.validate() {
                            // Bitfinex server is online
                            Ok(BitfinexPlatformEvent::PlatformStatus(status)) => {
                                debug!(
                                    exchange = %Exchange::ID,
                                    %success_responses,
                                    %expected_responses,
                                    payload = ?status,
                                    "received Bitfinex platform status",
                                );
                            }

                            // Subscription success
                            Ok(BitfinexPlatformEvent::Subscribed(response)) => {
                                // Determine SubscriptionId associated with the success response
                                let BitfinexSubResponse { channel, market, channel_id } = &response;
                                let subscription_id = ExchangeSub::from((channel, market)).id();

                                // Replace SubscriptionId with SubscriptionId(channel_id)
                                if let Some(subscription) = map.0.remove(&subscription_id) {
                                    success_responses += 1;
                                    map.0.insert(SubscriptionId(channel_id.0.to_string()), subscription);

                                    debug!(
                                        exchange = %Exchange::ID,
                                        %success_responses,
                                        %expected_responses,
                                        payload = ?response,
                                        "received valid Ok subscription response",
                                    );
                                }
                            }

                            // Subscription failure
                            Err(err) => break Err(err),

                            // Not reachable after BitfinexPlatformEvent validate()
                            Ok(BitfinexPlatformEvent::Error(error)) => panic!("{error:?}"),
                        }
                        Some(Err(SocketError::Deserialise { error, payload })) if success_responses >= 1 => {
                            // Already active Bitfinex subscriptions will send initial snapshots
                            init_snapshots_received += 1;
                            debug!(
                                exchange = %Exchange::ID,
                                ?error,
                                %success_responses,
                                %expected_responses,
                                %payload,
                                "failed to deserialise non SubResponse payload"
                            );
                            continue
                        }
                        Some(Err(SocketError::Terminated(close_frame))) => {
                            break Err(SocketError::Subscribe(
                                format!("received WebSocket CloseFrame: {close_frame}")
                            ))
                        }
                        _ => {
                            // Pings, Pongs, Frames, etc.
                            continue
                        }
                    }
                }
            }
        }
    }
}
