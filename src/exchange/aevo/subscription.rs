use barter_integration::error::SocketError;
use barter_integration::Validator;
use serde::Deserialize;
use serde::Serialize;

/// [`Aevo`](super::Aevo) WebSocket subscription response.
///
/// ### Raw Payload Examples
/// #### Subscription Orderbook Ok Response
/// ```json
/// {
///    "data" : ["orderbook:BTC-30JUN23-30000-C"]
///  }
///  
/// ```
///
/// #### Subscription Orderbook Error Response
/// ```json
/// {
///    "data" : []
///  }
/// ```
///
/// See docs: <https://docs.deribit.com/#public-subscribe>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AevoSubResponse {
    pub data: Vec<String>,
    pub error: Option<String>,
}

impl Validator for AevoSubResponse {
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
