use serde::Deserialize;
use serde::Serialize;

use crate::subscription::book::Level;

pub mod l1;
pub mod l2;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Delta {
    New,
    Change,
    Delete,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct DeribitLevel(pub Delta, pub f64, pub f64);

impl From<DeribitLevel> for Level {
    fn from(level: DeribitLevel) -> Self {
        Self {
            price: level.1,
            amount: level.2,
        }
    }
}
