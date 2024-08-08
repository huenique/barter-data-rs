use barter_data::exchange::aevo::Aevo;
use barter_data::exchange::ExchangeId;
use barter_data::streams::Streams;
use barter_data::subscription::book::OrderBooksL2;
use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionContract;
use barter_integration::model::instrument::kind::OptionExercise;
use barter_integration::model::instrument::kind::OptionKind;
use chrono::TimeZone;
use chrono::Utc;
use tracing::info;

#[rustfmt::skip]
#[tokio::main]
async fn main() {
    // Initialise INFO Tracing log subscriber
    init_logging();

    // Initialise OrderBooksL2 Streams for Deribit only
    // '--> each call to StreamBuilder::subscribe() creates a separate WebSocket connection
    let mut streams = Streams::<OrderBooksL2>::builder()
        .subscribe([
            (Aevo::default(), "eth", "usdc", InstrumentKind::Option(call_contract()), OrderBooksL2),
        ])

        .init()
        .await
        .unwrap();

    let mut aevo_stream = streams
        .select(ExchangeId::Aevo)
        .unwrap();

    while let Some(order_book_l2) = aevo_stream.recv().await {
        info!("MarketEvent<OrderBook>: {order_book_l2:?}");
    }
}

// Initialise an INFO `Subscriber` for `Tracing` Json logs and install it as the
// global default.
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
        .pretty()
        // Install this Tracing subscriber as global default
        .init()
}

fn call_contract() -> OptionContract {
    OptionContract {
        kind: OptionKind::Call,
        exercise: OptionExercise::American,
        expiry: Utc.timestamp_millis_opt(1721347200000).unwrap(),
        strike: rust_decimal_macros::dec!(3200),
    }
}
