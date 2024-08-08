use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use crate::subscription::book::Level;

pub mod l2;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct AevoLevel {
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub amount: f64,
    #[serde(default, deserialize_with = "deserialize_optional_f64")]
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

fn deserialize_optional_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    if let Some(val) = opt {
        val.parse::<f64>()
            .map(Some)
            .map_err(serde::de::Error::custom)
    } else {
        Ok(None)
    }
}
