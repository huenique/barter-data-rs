use barter_integration::model::instrument::Instrument;
use barter_integration::model::Exchange;
use barter_integration::model::Side;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::event::MarketEvent;
use crate::event::MarketIter;
use crate::exchange::okx::message::OkxMessage;
use crate::exchange::ExchangeId;
use crate::subscription::trade::PublicTrade;

/// Terse type alias for an [`Okx`](super::Okx) real-time trades WebSocket
/// message.
pub type OkxTrades = OkxMessage<OkxTrade>;

/// [`Okx`](super::Okx) real-time trade WebSocket message.
///
/// See [`OkxMessage`] for full raw payload examples.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel-trades-channel>
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct OkxTrade {
    #[serde(rename = "tradeId")]
    pub id: String,
    #[serde(rename = "px", deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    #[serde(rename = "sz", deserialize_with = "barter_integration::de::de_str")]
    pub amount: f64,
    pub side: Side,
    #[serde(
        rename = "ts",
        deserialize_with = "barter_integration::de::de_str_u64_epoch_ms_as_datetime_utc"
    )]
    pub time: DateTime<Utc>,
}

impl From<(ExchangeId, Instrument, OkxTrades)> for MarketIter<PublicTrade> {
    fn from((exchange_id, instrument, trades): (ExchangeId, Instrument, OkxTrades)) -> Self {
        trades
            .data
            .into_iter()
            .map(|trade| {
                Ok(MarketEvent {
                    exchange_time: trade.time,
                    received_time: Utc::now(),
                    exchange: Exchange::from(exchange_id),
                    instrument: instrument.clone(),
                    kind: PublicTrade {
                        id: trade.id,
                        price: trade.price,
                        amount: trade.amount,
                        side: trade.side,
                    },
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod de {
        use std::time::Duration;

        use barter_integration::de::datetime_utc_from_epoch_duration;
        use barter_integration::error::SocketError;
        use barter_integration::model::SubscriptionId;

        use super::*;

        #[test]
        fn test_okx_message_trades() {
            let input = r#"
            {
                "arg": {
                    "channel": "trades",
                    "instId": "BTC-USDT"
                },
                "data": [
                    {
                        "instId": "BTC-USDT",
                        "tradeId": "130639474",
                        "px": "42219.9",
                        "sz": "0.12060306",
                        "side": "buy",
                        "ts": "1630048897897"
                    }
                ]
            }
            "#;

            let actual = serde_json::from_str::<OkxTrades>(input);
            let expected: Result<OkxTrades, SocketError> = Ok(OkxTrades {
                subscription_id: SubscriptionId::from("trades|BTC-USDT"),
                data: vec![OkxTrade {
                    id: "130639474".to_string(),
                    price: 42219.9,
                    amount: 0.12060306,
                    side: Side::Buy,
                    time: datetime_utc_from_epoch_duration(Duration::from_millis(1630048897897)),
                }],
            });

            match (actual, expected) {
                (Ok(actual), Ok(expected)) => {
                    assert_eq!(actual, expected, "TC failed")
                }
                (Err(_), Err(_)) => {
                    // Test passed
                }
                (actual, expected) => {
                    // Test failed
                    panic!("TC failed because actual != expected. \nActual: {actual:?}\nExpected: {expected:?}\n");
                }
            }
        }
    }
}
