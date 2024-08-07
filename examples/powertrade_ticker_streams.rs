use barter_data::exchange::powertrade::PowerTrade;
use barter_data::streams::Streams;
use barter_data::subscription::ticker::Tickers;
use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionContract;
use barter_integration::model::instrument::kind::OptionExercise;
use barter_integration::model::instrument::kind::OptionKind;
use chrono::TimeZone;
use chrono::Utc;
use tokio_stream::StreamExt;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    init_logging();

    let ticker_streams = Streams::<Tickers>::builder()
        .subscribe([(
            PowerTrade::default(),
            "btc",
            "usd",
            InstrumentKind::Option(OptionContract {
                kind: OptionKind::Put,
                exercise: OptionExercise::European,
                expiry: Utc.timestamp_millis_opt(1724976000000).unwrap(),
                strike: rust_decimal_macros::dec!(65000),
            }),
            Tickers,
        )])
        .init()
        .await
        .unwrap();

    // Join ticker streams
    let mut joined_ticker_stream = ticker_streams.join_map().await;

    // Spawn a task to process Ticker data
    while let Some((exchange, ticker)) = joined_ticker_stream.next().await {
        info!("Exchange: {exchange}, TickerData: {ticker:?}");
    }
}

// Initialise an INFO `Subscriber` for `Tracing` logs and install it as the global default.
fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .with_ansi(cfg!(debug_assertions))
        .pretty()
        .init()
}
