use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use barter_data::event::MarketEvent;
use barter_data::exchange::deribit::DeribitMain;
use barter_data::streams::Streams;
use barter_data::subscription::ticker::Ticker;
use barter_data::subscription::ticker::Tickers;
use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionContract;
use barter_integration::model::instrument::kind::OptionExercise;
use barter_integration::model::instrument::kind::OptionKind;
use chrono::TimeZone;
use chrono::Utc;
use futures::task::Context;
use futures::task::Poll;
use tokio::sync::watch;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::Duration as TokioDuration;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    init_logging();

    // Set expiry timestamp for option contract
    let expiry_timestamp = 1727966280000;

    // Configure and subscribe to Deribit options stream
    let ticker_stream = setup_ticker_stream(expiry_timestamp).await;

    // Create watch channel for the close signal
    let (close_tx, close_rx) = watch::channel(false);
    let close_signal = Arc::new(AtomicBool::new(false));

    // Start tasks to handle the expiry check and close signal
    let ticker_stream: Arc<Mutex<_>> = Arc::new(Mutex::new(ticker_stream));

    spawn_expiry_checker(expiry_timestamp, close_tx);
    spawn_close_signal_handler(
        Arc::clone(&ticker_stream),
        close_rx.clone(),
        Arc::clone(&close_signal),
    );

    // Process ticker data
    process_ticker_data(ticker_stream, close_signal, close_rx).await;

    info!("Market data streaming task exited.");
}

/// Initializes the ticker stream and sets up the subscription to Deribit
/// options
async fn setup_ticker_stream(
    expiry_timestamp: i64,
) -> tokio::sync::mpsc::UnboundedReceiver<MarketEvent<Ticker>> {
    let stream_builder = Streams::<Tickers>::builder();
    let builder = stream_builder.subscribe([(
        DeribitMain::default(),
        "btc",
        "usd",
        InstrumentKind::Option(OptionContract {
            kind: OptionKind::Call,
            exercise: OptionExercise::American,
            expiry: Utc.timestamp_millis_opt(expiry_timestamp).unwrap(),
            strike: rust_decimal_macros::dec!(65000),
        }),
        Tickers,
    )]);

    builder.init().await.unwrap().join().await
}

/// Spawns a task to check the expiry and send a close signal if expired
fn spawn_expiry_checker(expiry_timestamp: i64, close_tx: watch::Sender<bool>) {
    tokio::spawn(async move {
        let current_timestamp = Utc::now().timestamp_millis();
        if current_timestamp >= expiry_timestamp {
            info!(
                "Option contract already expired. Timestamp: {}",
                expiry_timestamp
            );
            close_tx.send(true).unwrap();
            return;
        }

        loop {
            let current_timestamp = Utc::now().timestamp_millis();
            if current_timestamp >= expiry_timestamp {
                info!(
                    "Option contract has expired. Timestamp: {}",
                    expiry_timestamp
                );
                close_tx.send(true).unwrap();
                break;
            }

            sleep(TokioDuration::from_secs(1)).await;
        }
    });
}

/// Spawns a task to handle the close signal and close the ticker stream
/// gracefully
fn spawn_close_signal_handler(
    ticker_stream: Arc<Mutex<tokio::sync::mpsc::UnboundedReceiver<MarketEvent<Ticker>>>>,
    mut close_rx: watch::Receiver<bool>,
    close_signal: Arc<AtomicBool>,
) {
    tokio::spawn(async move {
        while close_rx.changed().await.is_ok() {
            if *close_rx.borrow() {
                info!("Received close signal. Closing the ticker stream.");
                close_signal.store(true, Ordering::SeqCst);

                let mut ticker_stream = ticker_stream.lock().await;
                info!("Invoking close() on the ticker stream");
                ticker_stream.close();
                info!("Market event stream closed.");
                break;
            }
        }
    });
}

/// Processes ticker data and exits when the stream is closed
async fn process_ticker_data(
    ticker_stream: Arc<Mutex<tokio::sync::mpsc::UnboundedReceiver<MarketEvent<Ticker>>>>,
    close_signal: Arc<AtomicBool>,
    close_rx: watch::Receiver<bool>,
) {
    while !close_signal.load(Ordering::SeqCst) {
        let mut ticker_stream = ticker_stream.lock().await;

        // Check for new close signal changes in the loop
        if close_rx.has_changed().unwrap_or(false) && *close_rx.borrow() {
            info!("Close signal received, breaking out of the data processing loop.");
            break;
        }

        match ticker_stream.poll_recv(&mut Context::from_waker(futures::task::noop_waker_ref())) {
            Poll::Ready(Some(data)) => {
                info!("TickerData: {data:?}");
            }
            Poll::Ready(None) => {
                info!("Ticker stream has closed, exiting loop.");
                break;
            }
            Poll::Pending => {
                drop(ticker_stream);
            }
        }
    }
}

/// Initializes logging with the `tracing` crate
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
