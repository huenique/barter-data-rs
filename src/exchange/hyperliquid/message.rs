use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Deserialize, Serialize)]
pub struct HyperliquidMessage<T> {
    pub channel: String,
    #[serde(default)]
    pub data: T,
}
