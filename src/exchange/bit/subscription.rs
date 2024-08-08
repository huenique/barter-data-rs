use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitSubResponse {
    pub channel: String,
    pub timestamp: i64,
    pub data: BitSubResponseData,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubResponse {
    pub channel: String,
    pub timestamp: i64,
    pub data: BitSubResponseData,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitSubResponseData {
    pub code: i64,
    pub message: Option<String>,
}

impl Validator for BitSubResponse {
    fn validate(self) -> Result<Self, barter_integration::error::SocketError>
    where
        Self: Sized,
    {
        match self.data.code {
            0 => Ok(self),
            _ => Err(SocketError::Subscribe(format!(
                "received failure subscription response code: {} with message: {}",
                self.data.code,
                self.data.message.unwrap_or_default()
            ))),
        }
    }
}
