use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionKind;
use barter_integration::model::instrument::Instrument;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::Utc;
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
            Spot => format!("{}-SPOT", base.to_string().to_uppercase(),),
            Perpetual => format!(
                "{}-{}-PERPETUAL",
                base.to_string().to_uppercase(),
                quote.to_string().to_uppercase(),
            ),
            Future(future) => {
                format!(
                    "{base}-{expiry}-{kind}",
                    base = base.to_string().to_uppercase(),
                    expiry = format_expiry(future.expiry),
                    kind = "F",
                )
            }
            Option(option) => format!(
                "{base}-{expiry}-{strike}{kind}",
                base = base.to_string().to_uppercase(),
                expiry = format_expiry(option.expiry),
                strike = option.strike,
                kind = match option.kind {
                    OptionKind::Call => "C",
                    OptionKind::Put => "P",
                },
            ),
        })
    }
}

fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.date_naive().format("%Y%m%d")
}
