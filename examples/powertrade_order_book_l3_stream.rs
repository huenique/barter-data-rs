use barter_data::exchange::powertrade::PowerTrade;
use barter_data::exchange::ExchangeId;
use barter_data::streams::Streams;
use barter_data::subscription::book::OrderBooksL3;
use barter_integration::model::instrument::kind::InstrumentKind;
use tracing::info;

#[rustfmt::skip]
#[tokio::main]
async fn main() {
    init_logging();

    let mut streams = Streams::<OrderBooksL3>::builder()
        .subscribe([
            (PowerTrade::default(), "btc", "usd", InstrumentKind::Perpetual, OrderBooksL3),
        ])
        .init()
        .await
        .unwrap();

    let mut powertrade_stream = streams
        .select(ExchangeId::PowerTrade)
        .unwrap();

    while let Some(order_book_l3) = powertrade_stream.recv().await {
        info!("MarketEvent<OrderBook>: {order_book_l3:?}");
    }
}

// Initialise an INFO `Subscriber` for `Tracing` Json logs and install it as the global default.
fn init_logging() {
    tracing_subscriber::fmt()
        // Filter messages based on the INFO
        .with_env_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        // Disable colours on release builds
        .with_ansi(cfg!(debug_assertions))
        // Enable Json formatting
        .pretty()
        // Install this Tracing subscriber as global default
        .init();
}
