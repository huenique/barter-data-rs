use serde::Deserialize;
use serde::Serialize;
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TopOfBook {
    #[serde(deserialize_with = "de::de_str_to_i64")]
    pub timestamp: i64,
    pub tradeable_entity_id: String,
    pub market_id: String,
    #[serde(deserialize_with = "de::de_str_to_option_f64")]
    pub buy_price: Option<f64>,
    #[serde(deserialize_with = "de::de_str_to_option_f64")]
    pub buy_quantity: Option<f64>,
    #[serde(deserialize_with = "de::de_str_to_option_f64")]
    pub sell_price: Option<f64>,
    #[serde(deserialize_with = "de::de_str_to_option_f64")]
    pub sell_quantity: Option<f64>,
}

pub mod de {
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::{
        self,
    };

    pub fn de_str_to_option_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "none" {
            Ok(None)
        } else {
            s.parse::<f64>().map(Some).map_err(serde::de::Error::custom)
        }
    }

    pub fn de_str_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<i64>().map_err(serde::de::Error::custom)
    }
}
