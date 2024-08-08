use std::fmt::Debug;

use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct HyperliquidSubResponse {
    pub channel: String,
    pub data: SubscriptionData,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionData {
    pub method: String,
    pub subscription: SubscriptionDetails,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriptionDetails {
    #[serde(rename = "type")]
    pub subscription_type: String,
    pub coin: Option<String>,
}

impl Validator for HyperliquidSubResponse {
    fn validate(self) -> Result<Self, SocketError> {
        if self.channel == "subscriptionResponse" {
            Ok(self)
        } else {
            Err(SocketError::Subscribe(format!(
                "Subscription failed with response: {:?}",
                self
            )))
        }
    }
}
