use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionKind;
use barter_integration::model::instrument::Instrument;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::exchange::bit::Bit;
use crate::subscription::Subscription;
use crate::Identifier;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BitMarket(pub String);

impl AsRef<str> for BitMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<Kind> Identifier<BitMarket> for Subscription<Bit, Kind> {
    fn id(&self) -> BitMarket {
        use InstrumentKind::*;
        let Instrument { base, quote, kind } = &self.instrument;

        BitMarket(match kind {
            Perpetual => format!(
                "{}-{}-PERPETUAL",
                base.to_string().to_uppercase(),
                quote.to_string().to_uppercase()
            ),
            Option(option) => format!(
                "{base}-{quote}-{expiry}-{strike}-{kind}",
                base = base.to_string().to_uppercase(),
                quote = quote.to_string().to_uppercase(),
                expiry = format_expiry(option.expiry).to_string().to_uppercase(),
                strike = option.strike,
                kind = match option.kind {
                    OptionKind::Call => "C",
                    OptionKind::Put => "P",
                },
            ),
            _ => String::new(),
        })
    }
}

fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.date_naive().format("%-d%b%y")
}
