use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct CoincallSubResponse {
    pub data: Vec<String>,
    pub error: Option<String>,
}

impl Validator for CoincallSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        match (self.error.as_ref(), self.data.len()) {
            (Some(e), _) => Err(SocketError::Subscribe(format!(
                "Received failure subscription response with message: {e}"
            ))),
            (_, 0) => Err(SocketError::Subscribe(
                "Received empty subscription response".to_string(),
            )),
            _ => Ok(self),
        }
    }
}
