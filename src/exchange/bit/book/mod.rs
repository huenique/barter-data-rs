pub mod l2;

use crate::subscription::book::Level;

use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BitLevel {
    pub price: BigDecimal,
    pub size: BigDecimal,
}

impl From<BitLevel> for Level {
    fn from(level: BitLevel) -> Self {
        Self {
            price: level
                .price
                .to_f64()
                .expect("Failed to convert BigDecimal to f64"),
            amount: level
                .size
                .to_f64()
                .expect("Failed to convert BigDecimal to f64"),
        }
    }
}
