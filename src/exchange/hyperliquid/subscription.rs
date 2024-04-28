use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct HyperliquidSubResponse {
    pub channel: String,
    pub data: SubscriptionData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionData {
    pub method: String,
    pub subscription: SubscriptionDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionDetails {
    #[serde(rename = "type")]
    pub subscription_type: String,
    pub coin: Option<String>,
}

impl Validator for HyperliquidSubResponse {
    fn validate(self) -> Result<Self, SocketError> {
        if self.channel == "subscriptionResponse" {
            // TODO: Add additional checks as necessary
            Ok(self)
        } else {
            Err(SocketError::Subscribe(format!(
                "Subscription failed with response: {:?}",
                self
            )))
        }
    }
}
