pub mod l2;

use crate::subscription::book::Level;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct DydxLevel {
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub size: f64,
}

impl From<DydxLevel> for Level {
    fn from(level: DydxLevel) -> Self {
        Self {
            price: level.price,
            amount: level.size,
        }
    }
}
