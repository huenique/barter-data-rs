use serde::Deserialize;
use serde::Serialize;

use crate::subscription::book::Level;

pub mod l2;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct DeriveLevel(pub String, pub String);

impl From<DeriveLevel> for Level {
    fn from(level: DeriveLevel) -> Self {
        Self {
            price: level.0.parse().unwrap(),
            amount: level.1.parse().unwrap(),
        }
    }
}
