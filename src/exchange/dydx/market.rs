use crate::exchange::dydx::Dydx;
use crate::subscription::Subscription;
use crate::Identifier;

use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::Instrument;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DydxMarket(pub String);

impl AsRef<str> for DydxMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<Kind> Identifier<DydxMarket> for Subscription<Dydx, Kind> {
    fn id(&self) -> DydxMarket {
        use InstrumentKind::*;
        let Instrument { base, quote, kind } = &self.instrument;

        DydxMarket(match kind {
            Perpetual => format!(
                "{}-{}",
                base.to_string().to_uppercase(),
                quote.to_string().to_uppercase()
            ),
            _ => String::new(),
        })
    }
}
