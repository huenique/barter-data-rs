use barter_data::exchange::hyperliquid::Hyperliquid;
use barter_data::exchange::ExchangeId;
use barter_data::streams::Streams;
use barter_data::subscription::book::OrderBooksL2;
use barter_integration::model::instrument::kind::InstrumentKind;
use tracing::info;

#[rustfmt::skip]
#[tokio::main]
async fn main() {
    init_logging();

    // Initialise OrderBooksL2 Streams for Hyperliquid only
    // '--> each call to StreamBuilder::subscribe() creates a separate WebSocket connection
    let mut streams = Streams::<OrderBooksL2>::builder()
        .subscribe([
            (Hyperliquid::default(), "BTC", "", InstrumentKind::Spot, OrderBooksL2),
        ])

        .init()
        .await
        .unwrap();

    // Select the ExchangeId::Hyperliquid stream
    // Notes:
    //  - Use `streams.select(ExchangeId)` to interact with the individual exchange streams!
    //  - Use `streams.join()` to join all exchange streams into a single mpsc::UnboundedReceiver!
    let mut hyperliquid_stream = streams
        .select(ExchangeId::Hyperliquid)
        .unwrap();

    while let Some(order_book_l2) = hyperliquid_stream.recv().await {
        info!("MarketEvent<OrderBook>: {order_book_l2:?}");
    }
}

// Initialise an INFO `Subscriber` for `Tracing` Json logs and install it as the global default.
fn init_logging() {
    tracing_subscriber::fmt()
        // Filter messages based on the INFO
        .with_env_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // Disable colours on release builds
        .with_ansi(cfg!(debug_assertions))
        // Enable Json formatting
        .json()
        // Install this Tracing subscriber as global default
        .init();
}
