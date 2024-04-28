use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct HyperliquidMessage<T> {
    pub channel: String,
    pub data: T,
}
