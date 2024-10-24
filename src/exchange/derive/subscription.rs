use std::collections::HashMap;

use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeriveSubResponse {
    id: Option<String>,
    result: Option<DeriveSubResult>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeriveSubResult {
    status: HashMap<String, String>,
    current_subscriptions: Vec<String>,
}

impl Validator for DeriveSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        // It should be safe to ignore failed subscription responses. Worst case
        // scenario is that we won't receive any data for an invalid
        // subscription
        Ok(self)
        // if let Some(result) = &self.result {
        //     let failed_tickers: Vec<_> = result
        //         .status
        //         .iter()
        //         .filter_map(|(key, value)| {
        //             if value != "ok" {
        //                 Some((key.clone(), value.clone()))
        //             } else {
        //                 None
        //             }
        //         })
        //         .collect();

        //     if failed_tickers.is_empty() {
        //         Ok(self)
        //     } else {
        //         Err(SocketError::Subscribe(format!(
        //             "Received failure subscription response for tickers:
        // {:?}",             failed_tickers
        //         )))
        //     }
        // } else {
        //     Err(SocketError::Subscribe(format!(
        //         "Received failure subscription response: {:?}",
        //         self
        //     )))
        // }
    }
}
