pub mod l2;

use crate::subscription::book::Level;

use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct BitLevel {
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub size: f64,
}

impl From<BitLevel> for Level {
    fn from(level: BitLevel) -> Self {
        Self {
            price: level.price,
            amount: level.size,
        }
    }
}
