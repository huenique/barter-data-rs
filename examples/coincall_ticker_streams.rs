use barter_data::exchange::coincall::option::server_noauth::CoincallServerOptionNoAuth;
use barter_data::streams::Streams;
use barter_data::subscription::ticker::Tickers;
use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionContract;
use barter_integration::model::instrument::kind::OptionExercise;
use barter_integration::model::instrument::kind::OptionKind;
use chrono::TimeZone;
use chrono::Utc;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use tokio_stream::StreamExt;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    init_logging();

    let instrument_data = [
        ("BTCUSD", 1727424000000, 42000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 42000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 69000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 69000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 54000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 54000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 160000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 160000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 36000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 36000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 130000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 130000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 24000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 24000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 45000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 45000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 62000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 62000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 72000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 72000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 41000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 41000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 15000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 15000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 75000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 75000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 70000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 70000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 19000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 19000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 100000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 100000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 18000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 18000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 57000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 57000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 110000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 110000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 115000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 115000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 38000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 38000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 25000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 25000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 59000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 59000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 200000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 200000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 80000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 80000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 33000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 33000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 34000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 34000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 120000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 120000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 22000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 22000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 56000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 56000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 68000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 68000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 35000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 35000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 26000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 26000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 65000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 65000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 30000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 30000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 27000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 27000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 64000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 64000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 50000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 50000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 105000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 105000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 90000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 90000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 40000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 40000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 21000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 21000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 37000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 37000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 240000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 240000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 63000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 63000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 23000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 23000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 10000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 10000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 44000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 44000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 95000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 95000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 29000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 29000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 67000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 67000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 32000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 32000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 53000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 53000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 48000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 48000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 60000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 60000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 49000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 49000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 46000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 46000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 31000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 31000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 28000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 28000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 20000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 20000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 125000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 125000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 58000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 58000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 55000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 55000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 66000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 66000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 43000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 43000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 39000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 39000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 180000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 180000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 140000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 140000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 52000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 52000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 51000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 51000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 47000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 47000, OptionKind::Put),
        ("BTCUSD", 1727424000000, 85000, OptionKind::Call),
        ("BTCUSD", 1727424000000, 85000, OptionKind::Put),
    ];

    let subscriptions = instrument_data
        .iter()
        .map(|(_symbol, expiry, strike, kind)| {
            (
                CoincallServerOptionNoAuth::default(),
                "btc",
                "usd",
                InstrumentKind::Option(OptionContract {
                    kind: kind.clone(),
                    exercise: OptionExercise::European,
                    expiry: Utc.timestamp_millis_opt(*expiry).unwrap(),
                    strike: Decimal::from_i64(*strike).unwrap(),
                }),
                Tickers,
            )
        });

    let ticker_streams = Streams::<Tickers>::builder()
        .subscribe(subscriptions)
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

// Initialise an INFO `Subscriber` for `Tracing` logs and install it as the
// global default.
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
