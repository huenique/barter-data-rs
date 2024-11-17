use serde::Deserialize;
use serde::Serialize;

use crate::subscription::book::Level;

pub mod l2;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct AevoLevel {
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub amount: f64,
    #[serde(
        default,
        deserialize_with = "crate::deserializer::deserialize_optional_f64"
    )]
    pub iv: Option<f64>,
}

impl From<AevoLevel> for Level {
    fn from(level: AevoLevel) -> Self {
        Self {
            price: level.price,
            amount: level.amount,
        }
    }
}
