use std::str::FromStr;

use barter_data::exchange::lyra::Lyra;
use barter_data::streams::Streams;
use barter_data::subscription::ticker::Tickers;
use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionContract;
use barter_integration::model::instrument::kind::OptionExercise;
use barter_integration::model::instrument::kind::OptionKind;
use chrono::TimeZone;
use chrono::Utc;
use rust_decimal::Decimal;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    init_logging();

    let options = vec![
        (OptionKind::Call, 30000),
        (OptionKind::Call, 40000),
        (OptionKind::Call, 45000),
        (OptionKind::Call, 50000),
        (OptionKind::Call, 52000),
        (OptionKind::Call, 54000),
        (OptionKind::Call, 55000),
        (OptionKind::Call, 56000),
        (OptionKind::Call, 58000),
        (OptionKind::Call, 60000),
        (OptionKind::Call, 62000),
        (OptionKind::Call, 64000),
        (OptionKind::Call, 65000),
        (OptionKind::Call, 66000),
        (OptionKind::Call, 68000),
        (OptionKind::Call, 70000),
        (OptionKind::Call, 72000),
        (OptionKind::Call, 74000),
        (OptionKind::Call, 75000),
        (OptionKind::Call, 76000),
        (OptionKind::Call, 80000),
        (OptionKind::Call, 85000),
        (OptionKind::Call, 90000),
        (OptionKind::Call, 100000),
        (OptionKind::Call, 110000),
        (OptionKind::Call, 120000),
        (OptionKind::Call, 130000),
        (OptionKind::Call, 140000),
        (OptionKind::Call, 160000),
        (OptionKind::Call, 180000),
        (OptionKind::Put, 30000),
        (OptionKind::Put, 40000),
        (OptionKind::Put, 45000),
        (OptionKind::Put, 50000),
        (OptionKind::Put, 52000),
        (OptionKind::Put, 54000),
        (OptionKind::Put, 55000),
        (OptionKind::Put, 56000),
        (OptionKind::Put, 58000),
        (OptionKind::Put, 60000),
        (OptionKind::Put, 62000),
        (OptionKind::Put, 64000),
        (OptionKind::Put, 65000),
        (OptionKind::Put, 66000),
        (OptionKind::Put, 68000),
        (OptionKind::Put, 70000),
        (OptionKind::Put, 72000),
        (OptionKind::Put, 74000),
        (OptionKind::Put, 75000),
        (OptionKind::Put, 76000),
        (OptionKind::Put, 80000),
        (OptionKind::Put, 85000),
        (OptionKind::Put, 90000),
        (OptionKind::Put, 100000),
        (OptionKind::Put, 110000),
        (OptionKind::Put, 120000),
        (OptionKind::Put, 130000),
        (OptionKind::Put, 140000),
        (OptionKind::Put, 160000),
        (OptionKind::Put, 180000),
    ];

    let subscriptions = options
        .into_iter()
        .map(|(kind, strike)| {
            (
                Lyra,
                "btc",
                "usd",
                InstrumentKind::Option(OptionContract {
                    kind,
                    exercise: OptionExercise::European,
                    expiry: Utc.timestamp_millis_opt(1724976000000).unwrap(),
                    strike: Decimal::from_str(&strike.to_string()).unwrap(),
                }),
                Tickers,
            )
        })
        .collect::<Vec<_>>();

    let ticker_streams = Streams::<Tickers>::builder()
        .subscribe(subscriptions)
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
                .with_default_directive(tracing_subscriber::filter::LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .with_ansi(cfg!(debug_assertions))
        .pretty()
        .init()
}
