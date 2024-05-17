use barter_data::exchange::hyperliquid::Hyperliquid;
use barter_data::exchange::ExchangeId;
use barter_data::streams::Streams;
use barter_data::subscription::book::OrderBooksL2;
use barter_integration::model::instrument::kind::InstrumentKind;
use tabled::Table;
use tabled::Tabled;

#[derive(Tabled)]
struct TabledOrderBook {
    #[tabled(rename = "Price")]
    price: f64,
    #[tabled(rename = "Size (BTC)")]
    size: f64,
}

#[tokio::main]
async fn main() {
    init_logging();

    // Initialise OrderBooksL2 Streams for Hyperliquid only
    // '--> each call to StreamBuilder::subscribe() creates a separate WebSocket connection
    let mut streams = Streams::<OrderBooksL2>::builder()
        .subscribe([(
            Hyperliquid::default(),
            "btc",
            "usd",
            InstrumentKind::Spot,
            OrderBooksL2,
        )])
        .init()
        .await
        .unwrap();

    let mut stream = streams.select(ExchangeId::Hyperliquid).unwrap();

    loop {
        // Clear the console
        print!("{esc}c", esc = 27 as char);

        // Read the next order book data from the stream
        let order_book_l2 = stream.recv().await.unwrap();

        // Extract bids and asks from the order book data
        let mut bids = order_book_l2
            .kind
            .bids
            .levels
            .iter()
            .map(|b| TabledOrderBook {
                price: b.price,
                size: b.amount,
            })
            .collect::<Vec<_>>();
        let mut asks = order_book_l2
            .kind
            .asks
            .levels
            .iter()
            .map(|a| TabledOrderBook {
                price: a.price,
                size: a.amount,
            })
            .collect::<Vec<_>>();

        bids.truncate(11);
        asks.truncate(11);

        // sort asks. the lowest price is at the bottom, like in typical orderbooks
        asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        asks.reverse();

        // Create tables for bids and asks
        let bids_table = Table::new(bids);
        let asks_table = Table::new(asks);

        // Print the tables on the same line
        println!("Asks:");
        println!("{}", asks_table);
        println!("Bids:");
        println!("{}", bids_table);
    }
}

// Initialise an INFO `Subscriber` for `Tracing` Json logs and install it as the global default.
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
        .init();
}
