use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct CoincallMessage<T> {
    pub data: T,
    pub channel: Option<String>,
    pub id: Option<String>,
    pub error: Option<String>,
}
