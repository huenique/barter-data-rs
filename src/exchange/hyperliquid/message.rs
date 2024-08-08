use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct HyperliquidMessage<T> {
    pub channel: String,
    #[serde(default)]
    pub data: T,
}
