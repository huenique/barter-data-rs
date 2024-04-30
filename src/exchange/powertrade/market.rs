use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::Instrument;
use serde::Deserialize;
use serde::Serialize;

use crate::subscription::Subscription;
use crate::Identifier;

use super::PowerTrade;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct PowerTradeMarket(pub String);

impl AsRef<str> for PowerTradeMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<Kind> Identifier<PowerTradeMarket> for Subscription<PowerTrade, Kind> {
    fn id(&self) -> PowerTradeMarket {
        use InstrumentKind::*;
        let Instrument { base, quote, kind } = &self.instrument;

        PowerTradeMarket(match kind {
            Spot => format!(
                "{}-{}",
                base.to_string().to_uppercase(),
                quote.to_string().to_uppercase()
            ),
            Perpetual => format!(
                "{}-{}-PERPETUAL",
                base.to_string().to_uppercase(),
                quote.to_string().to_uppercase(),
            ),
            Future(future) => {
                format!("{:?}", future)
            }
            Option(option) => {
                format!("{:?}", option)
            }
        })
    }
}
