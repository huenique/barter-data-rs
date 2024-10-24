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
