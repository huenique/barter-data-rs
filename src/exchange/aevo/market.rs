use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionKind;
use barter_integration::model::instrument::Instrument;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::exchange::aevo::Aevo;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Aevo`](super::Aevo) market that can be subscribed to.
///
/// See docs: <https://docs.aevo.xyz/reference/subscribe>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AevoMarket(pub String);

impl<Kind> Identifier<AevoMarket> for Subscription<Aevo, Kind> {
    fn id(&self) -> AevoMarket {
        use InstrumentKind::*;

        let Instrument { base, quote, kind } = &self.instrument;

        AevoMarket(match kind {
            Spot => format!("{base}_{quote}").to_uppercase(),
            Future(future) => format!("{base}-{}", format_expiry(future.expiry)).to_uppercase(),
            Perpetual => format!("{base}-PERP").to_uppercase(),
            Option(option) => format!(
                "{base}-{}-{}-{}",
                format_expiry(option.expiry),
                option.strike,
                match option.kind {
                    OptionKind::Call => "C",
                    OptionKind::Put => "P",
                },
            )
            .to_uppercase(),
        })
    }
}

impl AsRef<str> for AevoMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Format the expiry DateTime<Utc> to be Aevo API compatible.
///
/// eg/ "21JUN23" (21th of June 2023)
///
/// See docs: <https://docs.aevo.xyz/reference/getinstrumentinstrumentname>
fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.date_naive().format("%-d%b%y")
}
