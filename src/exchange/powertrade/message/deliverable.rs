use crate::exchange::powertrade::message::products::option::OptionDetails;

use serde::Deserialize;
use serde::Serialize;

/// https://power-trade.github.io/api-docs-source/ws_feeds.html#deliverable
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<Details> {
    pub deliverable: Deliverable<Details>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deliverable<Details> {
    pub deliverable_id: String,
    pub symbol: String,
    pub tags: Vec<String>,
    pub decimal_places: String,
    pub listing_status: String,
    pub details: Details,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ProductType {
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "future")]
    Future,
    #[serde(rename = "option")]
    Option(OptionDetails),
    #[serde(rename = "perpetual")]
    Perpetual,
    #[serde(other)]
    Unknown,
}
