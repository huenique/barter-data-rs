use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DydxResponse {}

impl Validator for DydxResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        todo!()
    }
}
