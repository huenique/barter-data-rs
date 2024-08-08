use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use barter_data::exchange::deribit::DeribitMain;
use barter_data::exchange::ExchangeId;
use barter_data::streams::Streams;
use barter_data::subscription::ticker::Tickers;
use barter_integration::model::instrument::kind::InstrumentKind;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialise INFO Tracing log subscriber
    init_logging();

    // Initialise Ticker streams for Deribit
    let mut streams = Streams::<Tickers>::builder()
        .subscribe([(
            DeribitMain::default(),
            "btc",
            "usd",
            InstrumentKind::Perpetual,
            Tickers,
        )])
        .init()
        .await
        .unwrap();

    let close_stream = Arc::new(AtomicBool::new(false));
    let close_stream_clone = Arc::clone(&close_stream);

    tokio::spawn(async move {
        info!("Spawned task to close Deribit stream after 10 seconds");
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        close_stream_clone.store(true, Ordering::SeqCst);

        info!("Deribit stream closed");
    });

    let mut ticker_streams = streams
        .select(ExchangeId::DeribitMainnet)
        .expect("Invalid exchange ID");

    while let Some(ticker) = ticker_streams.recv().await {
        info!("TickerData: {ticker:?}");

        if close_stream.load(Ordering::SeqCst) {
            ticker_streams.close();
            break;
        }
    }
}

// Initialise an INFO `Subscriber` for `Tracing` logs and install it as the
// global default.
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
