use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    risk_snapshot: RiskSnapshot,
}

#[derive(Serialize, Deserialize, Debug)]
struct RiskSnapshot {
    symbol: String,
    tradeable_entity_id: String,
    market_id: String,
    timestamp: String,
    time_to_expire: String,
    theoretical: Option<String>,
    mid: PriceDetail,
    bid: PriceDetail,
    ask: PriceDetail,
}

#[derive(Serialize, Deserialize, Debug)]
struct PriceDetail {
    price: String,
    volatility: String,
    greeks: Greeks,
}

#[derive(Serialize, Deserialize, Debug)]
struct Greeks {
    delta: String,
    vega: String,
    theta: String,
    rho: String,
    gamma: String,
}
