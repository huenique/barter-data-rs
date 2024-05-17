use barter_data::event::MarketEvent;
use barter_data::exchange::deribit::Deribit;
use barter_data::exchange::ExchangeId;
use barter_data::streams::Streams;
use barter_data::subscription::book::OrderBookL1;
use barter_data::subscription::book::OrderBooksL1;
use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionContract;
use barter_integration::model::instrument::kind::OptionExercise;
use barter_integration::model::instrument::kind::OptionKind;
use chrono::prelude::*;
use chrono::Utc;
use clap::Arg;
use clap::Command;
use headless_chrome::Browser;
use headless_chrome::LaunchOptionsBuilder;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;
use tabled::settings::Panel;
use tabled::settings::Style;
use tabled::Table;
use tabled::Tabled;

#[derive(Deserialize, Clone, Debug)]
struct IndexPriceResponse {
    result: IndexPrice,
}

#[derive(Deserialize, Clone, Debug)]
struct IndexPrice {
    index_price: Decimal,
}

#[derive(Deserialize, Clone, Debug)]
struct OptionInstrument {
    instrument_name: String,
    expiration_timestamp: i64,
    strike: Decimal,
    option_type: String,
}

#[derive(Deserialize, Clone, Debug)]
struct OptionInstrumentsResponse {
    result: Vec<OptionInstrument>,
}

#[derive(Clone, Debug)]
struct OptionData {
    strike_price: Decimal,
    best_bid: f64,
    best_ask: f64,
    best_bid_amount: f64,
    best_ask_amount: f64,
}

#[derive(Clone, Debug)]
struct OrderBookL1Data {
    strike_price: Decimal,
    best_bid: f64,
    best_ask: f64,
    best_bid_amount: f64,
    best_ask_amount: f64,
}

#[derive(Deserialize, Debug)]
struct OrderBookResponse {
    result: OrderBookResult,
}

#[derive(Deserialize, Debug)]
struct OrderBookResult {
    best_bid_price: f64,
    best_ask_price: f64,
    best_bid_amount: f64,
    best_ask_amount: f64,
}

#[derive(Tabled)]
struct ValidationResult {
    #[tabled(rename = "Field")]
    field: String,
    #[tabled(rename = "Fetched")]
    fetched: String,
    #[tabled(rename = "Normalized")]
    normalized: String,
    #[tabled(rename = "Result")]
    result: String,
}

#[tokio::main]
async fn main() {
    init_logging();

    let matches = Command::new("Deribit Options Order Book L1 Stream")
        .version("1.0")
        .author("Hju Kneyck Flores <hju@kodecraft.dev>")
        .about("Streams L1 Order Book data for Deribit options")
        .arg(
            Arg::new("expiry")
                .short('e')
                .long("expiry")
                .value_name("EXPIRY_DATE")
                .help("Expiry date in YYYY-MM-DD format")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("currency")
                .short('c')
                .long("currency")
                .value_name("CURRENCY")
                .help("Underlying currency (e.g., BTC, ETH)")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    let expiry_date_str = matches.get_one::<String>("expiry").unwrap();
    let expiry_date = NaiveDate::parse_from_str(expiry_date_str, "%Y-%m-%d")
        .expect("Invalid date format")
        .and_hms_opt(8, 0, 0) // Set to 8 AM
        .unwrap();

    let expiry_timestamp = expiry_date
        .and_local_timezone(Utc)
        .unwrap()
        .timestamp_millis();

    let currency = matches
        .get_one::<String>("currency")
        .unwrap()
        .to_lowercase();

    let client = Client::new();
    let index_price = fetch_index_price(&client, &currency)
        .await
        .expect("Failed to fetch index price");

    let option_instruments = fetch_option_instruments(&client, &currency, expiry_timestamp)
        .await
        .expect("Failed to fetch option instruments");

    let (itm_contracts, otm_contracts) = find_nearest_contracts(index_price, option_instruments);

    // Step 1: Run Barter Data Subscriptions
    let (mut streams, strike_map) =
        create_streams_and_strike_map(&itm_contracts, &otm_contracts, &currency, expiry_timestamp)
            .await;

    let mut deribit_stream = streams
        .select(ExchangeId::Deribit)
        .expect("No Deribit stream available");

    // Collect data from the stream
    let mut normalized_data = Vec::new();
    let duration = Duration::from_secs(10); // collect data for 10 seconds
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < duration {
        if let Some(order_book_l1) = deribit_stream.recv().await {
            if let InstrumentKind::Option(option_contract) = &order_book_l1.instrument.kind {
                let expiry = option_contract.expiry;
                let strike = option_contract.strike;

                // Format the instrument string for the strike map
                let instrument_string = format!(
                    "({}_usdc, option_{}_american_{}-UTC_{})",
                    order_book_l1.instrument.base,
                    match option_contract.kind {
                        OptionKind::Call => "call",
                        OptionKind::Put => "put",
                    },
                    expiry.format("%Y-%m-%d"),
                    strike
                );

                if let Some(strike_price) = strike_map.get(&instrument_string) {
                    normalized_data.push(OrderBookL1Data {
                        strike_price: *strike_price,
                        best_bid: order_book_l1.kind.best_bid.price,
                        best_ask: order_book_l1.kind.best_ask.price,
                        best_bid_amount: order_book_l1.kind.best_bid.amount,
                        best_ask_amount: order_book_l1.kind.best_ask.amount,
                    });
                }
            }
        }
    }

    // Step 2: Fetch the option order books from the Deribit API
    let fetched_data = fetch_option_order_books(&client, &itm_contracts, &otm_contracts)
        .await
        .expect("Failed to fetch option order books");

    // Step 3: Validate Data
    let validation_results = validate_data(&fetched_data, &normalized_data);

    // Print validation results in a table grouped by strike price
    print_validation_results_table(validation_results);
}

async fn create_streams_and_strike_map<'a>(
    itm_contracts: &'a [OptionInstrument],
    otm_contracts: &'a [OptionInstrument],
    currency: &'a str,
    expiry_timestamp: i64,
) -> (Streams<MarketEvent<OrderBookL1>>, HashMap<String, Decimal>) {
    let mut streams_builder = Streams::<MarketEvent<OrderBookL1>>::builder();
    let mut strike_map = HashMap::new();

    // Combine ITM and OTM contracts and iterate over them
    for contract in itm_contracts.iter().chain(otm_contracts.iter()) {
        let instrument_name = format!(
            "({}_usdc, option_{}_american_{}-UTC_{})",
            currency,
            contract.option_type.to_lowercase(),
            DateTime::<Utc>::from_timestamp_millis(expiry_timestamp)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string(),
            contract.strike
        );
        strike_map.insert(instrument_name.clone(), contract.strike);

        // Add subscription to the stream builder
        streams_builder = streams_builder.subscribe([(
            Deribit::default(),
            currency,
            "usdc",
            InstrumentKind::Option(OptionContract {
                kind: match contract.option_type.as_str() {
                    "call" => OptionKind::Call,
                    "put" => OptionKind::Put,
                    _ => panic!("Invalid option type"),
                },
                exercise: OptionExercise::American,
                expiry: Utc.timestamp_millis_opt(expiry_timestamp).unwrap(),
                strike: contract.strike,
            }),
            OrderBooksL1,
        )]);
    }

    let streams = streams_builder.init().await.unwrap();
    (streams, strike_map)
}

async fn fetch_index_price(client: &Client, currency: &str) -> Result<Decimal, reqwest::Error> {
    let url = format!(
        "https://www.deribit.com/api/v2/public/get_index_price?index_name={}_usd",
        currency
    );
    let res = client
        .get(&url)
        .send()
        .await?
        .json::<IndexPriceResponse>()
        .await?;
    Ok(res.result.index_price)
}

async fn fetch_option_instruments(
    client: &Client,
    currency: &str,
    expiry_timestamp: i64,
) -> Result<Vec<OptionInstrument>, reqwest::Error> {
    let url = format!(
        "https://www.deribit.com/api/v2/public/get_instruments?currency={}&kind=option&expired=false",
        currency
    );
    let res = client
        .get(&url)
        .send()
        .await?
        .json::<OptionInstrumentsResponse>()
        .await?;
    Ok(res
        .result
        .into_iter()
        .filter(|instrument| instrument.expiration_timestamp == expiry_timestamp)
        .collect())
}

fn find_nearest_contracts(
    index_price: Decimal,
    instruments: Vec<OptionInstrument>,
) -> (Vec<OptionInstrument>, Vec<OptionInstrument>) {
    let mut instruments = instruments;
    instruments.sort_by_key(|instrument| instrument.strike);

    let itm_contracts = instruments
        .iter()
        .filter(|&instrument| instrument.strike < index_price)
        .rev()
        .take(3)
        .cloned()
        .collect::<Vec<OptionInstrument>>();

    let otm_contracts = instruments
        .iter()
        .filter(|&instrument| instrument.strike > index_price)
        .take(3)
        .cloned()
        .collect::<Vec<OptionInstrument>>();

    (itm_contracts, otm_contracts)
}

async fn fetch_option_order_books(
    client: &Client,
    itm_contracts: &[OptionInstrument],
    otm_contracts: &[OptionInstrument],
) -> Result<Vec<OptionData>, reqwest::Error> {
    let mut option_data = Vec::new();

    for contract in itm_contracts.iter().chain(otm_contracts.iter()) {
        let url = format!(
            "https://www.deribit.com/api/v2/public/get_order_book?depth=1&instrument_name={}",
            contract.instrument_name
        );
        let res = client
            .get(&url)
            .send()
            .await?
            .json::<OrderBookResponse>()
            .await?;

        option_data.push(OptionData {
            strike_price: contract.strike,
            best_bid: res.result.best_bid_price,
            best_ask: res.result.best_ask_price,
            best_bid_amount: res.result.best_bid_amount,
            best_ask_amount: res.result.best_ask_amount,
        });
    }

    Ok(option_data)
}

fn print_validation_results_table(validation_results: Vec<(Decimal, Vec<ValidationResult>)>) {
    for (strike_price, results) in validation_results {
        let mut table = Table::new(results);
        let table_ = table
            .with(Style::extended())
            .with(Panel::header(format!("Strike Price: {}", strike_price)));

        println!("{table_}\n");
    }
}

#[allow(dead_code)]
async fn scrape_option_data(url: &str) -> Result<Vec<OptionData>, Box<dyn std::error::Error>> {
    let browser = Browser::new(LaunchOptionsBuilder::default().build().unwrap())?;
    let tab = browser.new_tab()?;
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    // Allow time for client-side rendering
    std::thread::sleep(std::time::Duration::from_secs(5));

    let options_data: Vec<OptionData> = tab
        .wait_for_elements("#OptionsTableContainer-default div[role='row']")?
        .iter()
        .map(|row| {
            let cells = row.find_elements("div[role='cell']").unwrap();
            OptionData {
                strike_price: cells[0].get_inner_text().unwrap().trim().parse().unwrap(),
                best_bid: cells[1].get_inner_text().unwrap().trim().parse().unwrap(),
                best_ask: cells[2].get_inner_text().unwrap().trim().parse().unwrap(),
                best_bid_amount: cells[3].get_inner_text().unwrap().trim().parse().unwrap(),
                best_ask_amount: cells[4].get_inner_text().unwrap().trim().parse().unwrap(),
            }
        })
        .collect();

    Ok(options_data)
}

fn validate_data(
    fetched_data: &[OptionData],
    normalized_data: &[OrderBookL1Data],
) -> Vec<(Decimal, Vec<ValidationResult>)> {
    let mut validation_results = Vec::new();

    for option in fetched_data {
        let normalized_entry = normalized_data
            .iter()
            .find(|entry| entry.strike_price == option.strike_price);

        let mut results = Vec::new();

        if let Some(normalized_entry) = normalized_entry {
            results.push(ValidationResult {
                field: "Best Bid".to_string(),
                fetched: option.best_bid.to_string(),
                normalized: normalized_entry.best_bid.to_string(),
                result: if (option.best_bid - normalized_entry.best_bid).abs() <= f64::EPSILON {
                    "Match".to_string()
                } else {
                    "Mismatch".to_string()
                },
            });

            results.push(ValidationResult {
                field: "Best Ask".to_string(),
                fetched: option.best_ask.to_string(),
                normalized: normalized_entry.best_ask.to_string(),
                result: if (option.best_ask - normalized_entry.best_ask).abs() <= f64::EPSILON {
                    "Match".to_string()
                } else {
                    "Mismatch".to_string()
                },
            });

            results.push(ValidationResult {
                field: "Best Bid Amount".to_string(),
                fetched: option.best_bid_amount.to_string(),
                normalized: normalized_entry.best_bid_amount.to_string(),
                result: if (option.best_bid_amount - normalized_entry.best_bid_amount).abs()
                    <= f64::EPSILON
                {
                    "Match".to_string()
                } else {
                    "Mismatch".to_string()
                },
            });

            results.push(ValidationResult {
                field: "Best Ask Amount".to_string(),
                fetched: option.best_ask_amount.to_string(),
                normalized: normalized_entry.best_ask_amount.to_string(),
                result: if (option.best_ask_amount - normalized_entry.best_ask_amount).abs()
                    <= f64::EPSILON
                {
                    "Match".to_string()
                } else {
                    "Mismatch".to_string()
                },
            });
        } else {
            results.push(ValidationResult {
                field: "Normalized Data".to_string(),
                fetched: "Not found".to_string(),
                normalized: "Not found".to_string(),
                result: "No Data".to_string(),
            });
        }

        validation_results.push((option.strike_price, results));
    }

    validation_results
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
