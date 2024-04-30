use super::message::SubscriptionResult;
use barter_integration::error::SocketError;
use barter_integration::Validator;

pub type PowerTradeSubResponse = SubscriptionResult;

impl Validator for PowerTradeSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        match self {
            SubscriptionResult::Subscribed(_) => Ok(self),
            SubscriptionResult::Error(failure) => Err(SocketError::Subscribe(format!(
                "received failure subscription response with message: {}",
                failure.message,
            ))),
        }
    }
}
