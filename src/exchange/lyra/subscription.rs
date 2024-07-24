use std::collections::HashMap;

use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct LyraSubResponse {
    id: String,
    result: Option<LyraSubResult>,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct LyraSubResult {
    status: HashMap<String, String>,
    current_subscriptions: Vec<String>,
}

impl Validator for LyraSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {
        if let Some(result) = &self.result {
            let failed_tickers: Vec<_> = result
                .status
                .iter()
                .filter_map(|(key, value)| {
                    if value != "ok" {
                        Some((key.clone(), value.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            if failed_tickers.is_empty() {
                Ok(self)
            } else {
                Err(SocketError::Subscribe(format!(
                    "Received failure subscription response for tickers: {:?}",
                    failed_tickers
                )))
            }
        } else {
            Err(SocketError::Subscribe(format!(
                "Received failure subscription response: {:?}",
                self
            )))
        }
    }
}
