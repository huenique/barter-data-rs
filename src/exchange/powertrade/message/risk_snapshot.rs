use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    risk_snapshot: RiskSnapshot,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
struct PriceDetail {
    price: String,
    volatility: String,
    greeks: Greeks,
}

#[derive(Debug, Deserialize, Serialize)]
struct Greeks {
    delta: String,
    vega: String,
    theta: String,
    rho: String,
    gamma: String,
}
