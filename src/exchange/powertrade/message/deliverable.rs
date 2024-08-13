use serde::Deserialize;
use serde::Serialize;

use crate::exchange::powertrade::message::products::option::OptionDetails;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<Details> {
    pub deliverable: Deliverable<Details>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Deliverable<Details> {
    pub deliverable_id: String,
    pub symbol: String,
    pub tags: Vec<String>,
    pub decimal_places: String,
    pub listing_status: String,
    pub details: Details,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ProductType {
    #[default]
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
