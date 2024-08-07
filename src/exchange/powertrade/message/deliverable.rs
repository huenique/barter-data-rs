use crate::exchange::powertrade::message::products::option::OptionDetails;

use serde::Deserialize;
use serde::Serialize;

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
#[serde(untagged)]
pub enum ProductType {
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "future")]
    Future,
    #[serde(rename = "option")]
    Option(Box<OptionDetails>),
    #[serde(rename = "perpetual")]
    Perpetual,
    Unknown(serde_json::Value),
}
