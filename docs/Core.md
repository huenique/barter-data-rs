# Technical Documentation for Core Functionalities

## 1. **Call Hierarchy and Flow**

### **High-Level Overview**

The library is primarily designed to manage WebSocket connections for subscribing to market data from various exchanges. The system’s architecture involves components like `StreamBuilder`, `Subscriber`, and `Validator`, which together establish and manage WebSocket connections, sending subscription requests, and handling the received data.

### **Detailed Call Hierarchy**

The core of the system revolves around three main steps:

1. **Initialization** (`StreamBuilder::init`)
2. **Subscription** (`Subscriber::subscribe`)
3. **Validation and Data Processing** (`SubscriptionValidator::validate` and `MarketStream`)

---

### **1.1 Initialization** (`StreamBuilder::init`)

- This method is part of the `Streams` builder and serves as the entry point for establishing WebSocket connections.
- **Flow**:
  1. The `Streams::builder()` method is invoked to set up a new instance for subscribing to exchange streams.
  2. `StreamBuilder::subscribe()` is called with specific subscription details (e.g., exchange name, instrument, type of data). Each call opens a separate WebSocket connection for the exchange.
  3. Finally, `StreamBuilder::init()` starts the process by calling `Subscriber::subscribe()` to establish WebSocket connections.

---

### **1.2 Subscription** (`Subscriber::subscribe`)

- The **`WebSocketSubscriber`** is a concrete implementation of the `Subscriber` trait, responsible for establishing WebSocket connections and sending subscription requests to exchanges.

  **Flow of Operations**:

- **Connection to WebSocket**:

  - The `connect(url)` function is used to establish the WebSocket connection with the exchange. This is initiated by:

    ```rust
    let mut websocket = connect(url).await?;
    ```

  - It connects to the exchange endpoint, setting up the stream for receiving and sending messages.

- **Mapping Subscriptions**:

  - Subscriptions are mapped from Barter's internal representation to exchange-specific payloads using the `WebSocketSubMapper::map()` method. This prepares the data payload that is sent over the WebSocket connection.

    ```rust
    let SubscriptionMeta { instrument_map, subscriptions } = Self::SubMapper::map::<Exchange, Kind>(subscriptions);
    ```

- **Sending Subscription Requests**:

  - The actual subscription requests are sent to the exchange via the WebSocket using `websocket.send(subscription)`. Each subscription request corresponds to a specific instrument or market.

    ```rust
    for subscription in subscriptions {
        websocket.send(subscription).await?;
    }
    ```

- **Returning the WebSocket and Instrument Map**:

  - After the subscriptions are sent, the WebSocket and instrument map (a mapping of `SubscriptionId` to `Instrument`) are returned for further processing.

    ```rust
    Ok((websocket, map))
    ```

---

### **1.3 Validation and Data Processing** (`SubscriptionValidator::validate`)

- Once the subscription requests are sent, the WebSocket waits for responses from the exchange to validate whether the subscriptions were successful. The `validate` function handles the subscription acknowledgments.

  **Key Steps**:

  1. **Timeout Handling**: If no response is received within a predefined `timeout`, the subscription is considered failed:

     ```rust
     _ = tokio::time::sleep(timeout) => {
         break Err(SocketError::Subscribe("subscription validation timeout reached"));
     }
     ```

  2. **Message Parsing and Validation**: Incoming WebSocket messages are parsed using the `WebSocketParser`, and their subscription success is determined. If the message indicates success, it increments the `success_responses` counter:

     ```rust
     let response = match message {
         Some(response) => response,
         None => break Err(SocketError::Subscribe("WebSocket stream terminated unexpectedly".to_string())),
     };

     match Self::Parser::parse::<Exchange::SubResponse>(response) {
         Some(Ok(response)) => match response.validate() {
             Ok(response) => { success_responses += 1; },
             Err(err) => break Err(err)
         },
     }
     ```

  3. **Completion**: The validator keeps processing messages until the expected number of successful subscription responses (`expected_responses`) is received. Once this is done, the WebSocket connection is marked as successfully established.

---

## 2. **WebSocket Connections Initialization**

The WebSocket initialization is tightly coupled with the `Subscriber` trait, which is implemented by `WebSocketSubscriber`. Here’s a technical walkthrough of how WebSocket connections are initialized:

### **2.1 WebSocket Setup Process**

1. **URL Generation**:

   - Each exchange provides a specific WebSocket URL for market data. The exchange-specific `Connector` trait is used to retrieve the correct URL.

   ```rust
   let url = Exchange::url()?;
   ```

2. **Connection Initialization**:

   - The `connect(url)` function is used to establish a WebSocket connection. It is asynchronous and ensures that the connection is successfully opened before proceeding.

   ```rust
   let mut websocket = connect(url).await?;
   ```

3. **Subscription Payload Construction**:

   - The `SubscriptionMapper::map()` method generates the payload for each subscription, which includes translating internal subscription representations (e.g., subscribing to `BTC/USD trades`) into a format that is compatible with the exchange’s API.

   ```rust
   let SubscriptionMeta { instrument_map, subscriptions } = Self::SubMapper::map::<Exchange, Kind>(subscriptions);
   ```

4. **Sending Subscriptions**:

   - The subscription payloads are sent via the WebSocket connection using `websocket.send()`. Each subscription is transmitted in sequence:

   ```rust
   websocket.send(subscription).await?;
   ```

---

## 3. **Handling Connections and Disconnections**

The system handles WebSocket connections and disconnections through various mechanisms. It maintains robust error handling and reconnection logic to ensure reliable data streaming.

### **3.1 Connection Handling**

- After initializing the WebSocket, the system enters a loop, listening for incoming messages (market data) from the exchange.
- Each incoming message is passed to the appropriate parser and handler, ensuring it conforms to the expected format (e.g., trade data, order book updates).

  Example:

  ```rust
  while let Some(message) = websocket.next().await {
      let parsed_message = WebSocketParser::parse(message);
      // Process parsed message
  }
  ```

### **3.2 Disconnection Handling**

1. **Unexpected Disconnections**:

   - If the WebSocket stream closes unexpectedly, it returns an error. The system is designed to catch these errors and terminate the stream gracefully:

   ```rust
   if let None = websocket.next() {
       return Err(SocketError::Subscribe("WebSocket stream terminated unexpectedly".to_string()));
   }
   ```

2. **Timeout Handling**:

   - A timeout mechanism ensures that if no messages are received from the WebSocket within a certain time frame, the system assumes the connection is broken or the subscription failed:

   ```rust
   _ = tokio::time::sleep(timeout) => {
       break Err(SocketError::Subscribe("subscription validation timeout reached"));
   }
   ```

3. **Retries and Reconnection**:
   - Although the code doesn’t explicitly show reconnection logic, it can be added by catching the disconnection errors and reinitializing the WebSocket connection by calling `Subscriber::subscribe()` again.
