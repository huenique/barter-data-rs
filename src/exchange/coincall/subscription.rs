use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CoincallSubResponse {}

impl Validator for CoincallSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        Ok(self)
    }
}
