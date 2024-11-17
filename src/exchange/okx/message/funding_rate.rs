use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct FundingRateData {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(
        rename = "fundingRate",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub funding_rate: f64,
    #[serde(
        rename = "fundingTime",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub funding_time: i64,
    #[serde(
        rename = "maxFundingRate",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub max_funding_rate: f64,
    #[serde(
        rename = "minFundingRate",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub min_funding_rate: f64,
    #[serde(
        rename = "premium",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub premium: f64,
    #[serde(
        rename = "settFundingRate",
        deserialize_with = "barter_integration::de::de_str"
    )]
    pub sett_funding_rate: f64,
    #[serde(rename = "settState")]
    pub sett_state: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(
        rename = "nextFundingRate",
        deserialize_with = "crate::deserializer::deserialize_optional_f64"
    )]
    pub next_funding_rate: Option<f64>,
    #[serde(
        rename = "nextFundingTime",
        deserialize_with = "crate::deserializer::deserialize_optional_i64"
    )]
    pub next_funding_time: Option<i64>,
    #[serde(rename = "ts", deserialize_with = "barter_integration::de::de_str")]
    pub ts: i64,
}
