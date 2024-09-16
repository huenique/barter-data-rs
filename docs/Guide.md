# Integrating a new exchange

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Directory Structure](#directory-structure)
3. [Creating the Exchange Module](#creating-the-exchange-module)
4. [Implementing the Exchange Traits](#implementing-the-exchange-traits)
5. [Testing the Integration](#testing-the-integration)
6. [Contributing Guidelines](#contributing-guidelines)

## Prerequisites

Before you start, ensure you have the following:

- A basic understanding of Rust programming.
- Access to the exchange's WebSocket API documentation.

## Directory Structure

When integrating a new exchange, the structure should follow the established patterns. Below are examples for different exchanges:

| **hamburger**                                                                                                                                      | **foo**                                                                                                                                                                                                                                 | **bar**                                                                                                                                                                                                                                                                                               |
| -------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <pre>src/exchange/hamburger<br>├── *channel.rs<br>├── *market.rs<br>├── *message.rs<br>├── *mod.rs<br>├── \*subscription.rs<br>└── ticker.rs</pre> | <pre>src/exchange/foo<br>├── book/<br>│ ├── l1.rs<br>│ ├── l2.rs<br>│ └── mod.rs<br>├── *channel.rs<br>├── index.rs<br>├── *market.rs<br>├── *message.rs<br>├── *mod.rs<br>├── \*subscription.rs<br>└── ticker.rs<br>└── trade.rs</pre> | <pre>src/exchange/bar<br>├── book/<br>│ ├── l1.rs<br>│ ├── l2.rs<br>│ └── mod.rs<br>├── *channel.rs<br>├── futures/<br>│ ├── l2.rs<br>│ ├── liquidation.rs<br>│ └── mod.rs<br>├── *market.rs<br>├── *mod.rs<br>├── spot/<br>│ ├── l2.rs<br>│ └── mod.rs<br>├── *subscription.rs<br>└── trade.rs</pre> |

Each exchange has a structure suited to its specific needs, such as handling futures, spot markets, or different levels of order book data. The core modules typically include:

- **`channel.rs`** – Defines and manages the exchange's data channels (e.g., ticker, order book) for real-time market data.
- **`market.rs`** – Translates subscription requests into exchange-specific market formats and handles instrument identification.
- **`message.rs`** – Formats incoming WebSocket messages and structures them for the library’s use.
- **`subscription.rs`** – Manages subscription requests and responses for data streams like tickers or order books.
- **`mod.rs`** – Serves as the main entry point, integrating all components and defining the overall structure for the exchange module.

## Creating the Exchange Module

The exchange module is a key component for connecting and interacting with exchanges. It facilitates smooth communication between the library and various exchanges by managing data streams and handling subscription requests.

- It connects to the exchange’s WebSocket or API.
- Manages subscriptions to real-time data streams like price updates and order books.
- Implements the `Connector` trait, which standardizes communication between the library and the exchange.
- Defines how subscription requests are sent, processed, and validated.
- Supports consistent and scalable integration of multiple exchanges.

To create an exchange module, you need to implement the `Connector` trait, which defines how to connect, subscribe, and interact with a specific exchange. Here’s what you need to implement for a new exchange:

1. **Connector Identifier** (`const ID: ExchangeId`) – Define a unique ID for the exchange, such as `BinanceSpot` or `DeribitMainnet`.
2. **Channel and Market Types** (`type Channel` and `type Market`) – These types map subscriptions to specific exchange channels (e.g., market tickers) and markets (e.g., BTC/USDT).
3. **Subscriber Type** (`type Subscriber`) – Manages WebSocket connections and handles data subscriptions.
4. **Subscription Validator** (`type SubValidator`) – Confirms that subscription requests sent to the exchange are successfully processed.
5. **Subscription Response** (`type SubResponse`) – The deserializable type for the exchange's response to subscription requests, implementing `Validator` to verify success.
6. **WebSocket URL** (`fn url() -> Result<Url, SocketError>`) – Specifies the WebSocket URL for connecting to the exchange.
7. **Ping Interval** (`fn ping_interval() -> Option<PingInterval>`) – Optionally defines custom WebSocket pings if required by the exchange.
8. **Subscription Requests** (`fn requests(...) -> Vec<WsMessage>`) – Converts a list of subscriptions into WebSocket messages that can be sent to the exchange.
9. **Expected Responses** (`fn expected_responses(...) -> usize`) – Specifies the number of responses expected for the subscription requests.
10. **Subscription Timeout** (`fn subscription_timeout() -> Duration`) – Specifies how long to wait for subscription confirmations.

### Example: Connector Implementation

Here’s an example `Connector` implementation for a hypothetical `ExampleExchange`:

```rust
// omitting imports for brevity

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ExampleExchange;

impl Connector for ExampleExchange {
    const ID: ExchangeId = ExchangeId::ExampleExchange;

    type Channel = ExampleExchangeChannel;
    type Market = ExampleExchangeMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = ExampleExchangeSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse("wss://example.com/websocket").map_err(|e| SocketError::Url(e.to_string()))
    }

    fn ping_interval() -> Option<PingInterval> {
        Some(PingInterval {
            interval: tokio::time::interval(Duration::from_secs(30)),
            ping: || WsMessage::Text("{\"type\": \"ping\"}".into()),
        })
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        exchange_subs
            .into_iter()
            .map(|sub| {
                let message = format!(
                    "{{\"action\": \"subscribe\", \"channel\": \"{}\", \"market\": \"{}\"}}",
                    sub.channel.as_ref(),
                    sub.market.as_ref(),
                );
                WsMessage::Text(message)
            })
            .collect()
    }

    fn expected_responses(map: &Map<Instrument>) -> usize {
        map.0.len()
    }

    fn subscription_timeout() -> Duration {
        Duration::from_secs(10)
    }
}
```

## Implementing the Exchange Traits

To integrate `ExampleExchange`, you’ll need to implement various traits that define how the exchange. These traits will manage subscription formats, transform exchange-specific data into barter-standard messages, and validate responses from the exchange.

### Example: `ExampleExchange` Integration

1. **Market Definition**  
   `src/exchange/example/market.rs`

   The `ExampleMarket` struct defines how a market is translated from a Barter `Subscription` into a string format for `ExampleExchange`. This implementation supports different instrument types such as options and futures.

   ```rust
   #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
   pub struct ExampleMarket(pub String);

   impl<Kind> Identifier<ExampleMarket> for Subscription<ExampleExchange, Kind> {
       fn id(&self) -> ExampleMarket {
           let Instrument { base, kind, .. } = &self.instrument;

           ExampleMarket(match kind {
               Option(option) => format!(
                   "{base}-{}-{}-{}",
                   format_expiry(option.expiry),
                   option.strike,
                   match option.kind {
                       OptionKind::Call => "C",
                       OptionKind::Put => "P",
                   }
               ).to_uppercase(),
               Spot => format!("{base}/SPOT"),
               Future(future) => format!("{base}/FUTURE-{}", future.expiry),
               _ => todo!(),
           })
       }
   }

   fn format_expiry(expiry: DateTime<Utc>) -> String {
       expiry.format("%Y%m%d").to_string()
   }
   ```

2. **Channel Definition**  
   `src/exchange/example/channel.rs`

   `ExampleChannel` defines the available channels (e.g., ticker or order book updates) for the exchange and converts the `Subscription` into the appropriate channel format.

   ```rust
   #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
   pub struct ExampleChannel(pub &'static str);

   impl ExampleChannel {
       pub const TICKER: Self = Self("ticker.{}.100ms");
   }

   impl Identifier<ExampleChannel> for Subscription<ExampleExchange, Tickers> {
       fn id(&self) -> ExampleChannel {
           ExampleChannel::TICKER
       }
   }

   impl AsRef<str> for ExampleChannel {
       fn as_ref(&self) -> &str {
           self.0
       }
   }
   ```

3. **Subscription Validation**  
   `src/exchange/example/subscription.rs`

   `ExampleSubResponse` validates the responses received from `ExampleExchange` after sending subscription requests.

   ```rust
   #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
   pub struct ExampleSubResponse {
       id: Option<String>,
       result: Option<ExampleSubResult>,
   }

   impl Validator for ExampleSubResponse {
       fn validate(self) -> Result<Self, SocketError> {
           // Basic validation: checking if the response contains a result
           if self.result.is_some() {
               Ok(self)
           } else {
               Err(SocketError::Subscribe("Invalid subscription response".into()))
           }
       }
   }
   ```

## Testing the Integration

When it comes to testing your exchange integration, there's no one-size-fits-all approach—it’s subjective and depends on what you think might break. The general idea is to focus on areas that are more likely to fail. For unit tests, you should look at things like:

- Parsing WebSocket messages and formatting them correctly.
- Handling subscription requests and ensuring they’re processed as expected.
- Converting exchange-specific data into the library’s standard format, like market or ticker data.
- Validating subscription responses, especially when things go wrong.

As for integration testing, it can be done through example modules or by actually using the library in real-world scenarios to see how everything works together. The key is to test the things that are likely to go wrong, while also making sure the overall integration functions smoothly in practice.

## Contributing Guidelines

See the [Contributing Guidelines](CONTRIBUTING.md) for detailed instructions on how to contribute to the library. Make sure to follow the guidelines to maintain consistency and quality across the codebase.
