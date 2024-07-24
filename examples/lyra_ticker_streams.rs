use barter_data::exchange::lyra::Lyra;
use barter_data::streams::Streams;
use barter_data::subscription::ticker::Tickers;
use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionContract;
use barter_integration::model::instrument::kind::OptionExercise;
use barter_integration::model::instrument::kind::OptionKind;
use chrono::TimeZone;
use chrono::Utc;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    init_logging();

    let ticker_streams = Streams::<Tickers>::builder()
        .subscribe([(
            Lyra,
            "btc",
            "usd",
            InstrumentKind::Option(OptionContract {
                kind: OptionKind::Call,
                exercise: OptionExercise::American,
                expiry: Utc.timestamp_millis_opt(1721952000000).unwrap(),
                strike: rust_decimal_macros::dec!(65000),
            }),
            Tickers,
        )])
        .init()
        .await
        .unwrap();

    let mut joined_ticker_stream = ticker_streams.join().await;

    while let Some(event) = joined_ticker_stream.recv().await {
        info!("MarketEvent<Ticker>: {event:?}");
    }
}

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
