use serde::Deserialize;
use serde::Serialize;
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CoincallMessage<T> {
    pub data: T,
    pub channel: Option<String>,
    pub id: Option<String>,
    pub error: Option<String>,
}
