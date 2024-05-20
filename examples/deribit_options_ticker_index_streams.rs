use barter_data::exchange::deribit::Deribit;
use barter_data::streams::Streams;
use barter_data::subscription::index::Indices;
use barter_data::subscription::ticker::Tickers;
use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionContract;
use barter_integration::model::instrument::kind::OptionExercise;
use barter_integration::model::instrument::kind::OptionKind;
use chrono::TimeZone;
use chrono::Utc;
use futures::StreamExt;
use tokio::spawn;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialise INFO Tracing log subscriber
    init_logging();

    // Initialise Ticker streams for Deribit
    let ticker_streams = Streams::<Tickers>::builder()
        .subscribe([(
            Deribit,
            "btc",
            "usd",
            InstrumentKind::Option(OptionContract {
                kind: OptionKind::Call,
                exercise: OptionExercise::American,
                expiry: Utc.timestamp_millis_opt(1719561600000).unwrap(),
                strike: rust_decimal_macros::dec!(65000),
            }),
            Tickers,
        )])
        .init()
        .await
        .unwrap();

    // Initialise Index streams for Deribit
    let index_streams = Streams::<Indices>::builder()
        .subscribe([(
            Deribit::default(),
            "btc",
            "usdc",
            InstrumentKind::Spot,
            Indices,
        )])
        .init()
        .await
        .unwrap();

    // Join ticker streams
    let mut joined_ticker_stream = ticker_streams.join_map().await;

    // Join index streams
    let mut joined_index_stream = index_streams.join_map().await;

    // Spawn a task to process Ticker data
    spawn(async move {
        while let Some((exchange, ticker)) = joined_ticker_stream.next().await {
            info!("Exchange: {exchange}, TickerData: {ticker:?}");
        }
    });

    // Process Index data
    while let Some((exchange, index)) = joined_index_stream.next().await {
        info!("Exchange: {exchange}, IndexData: {index:?}");
    }
}

// Initialise an INFO `Subscriber` for `Tracing` logs and install it as the global default.
fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_ansi(cfg!(debug_assertions))
        .pretty()
        .init()
}
