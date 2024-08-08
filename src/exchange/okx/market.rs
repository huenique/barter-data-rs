use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionKind;
use barter_integration::model::instrument::Instrument;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::exchange::okx::Okx;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Okx`](super::Okx) market that can be subscribed to.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OkxMarket(pub String);

impl<Kind> Identifier<OkxMarket> for Subscription<Okx, Kind> {
    fn id(&self) -> OkxMarket {
        use InstrumentKind::*;
        let Instrument { base, quote, kind } = &self.instrument;

        OkxMarket(match kind {
            Spot => format!("{base}-{quote}").to_uppercase(),
            Future(future) => {
                format!("{base}-{quote}-{}", format_expiry(future.expiry)).to_uppercase()
            }
            Perpetual => format!("{base}-{quote}-SWAP").to_uppercase(),
            Option(option) => format!(
                "{base}-{quote}-{}-{}-{}",
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

impl AsRef<str> for OkxMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Format the expiry DateTime<Utc> to be Okx API compatible.
///
/// eg/ "230526" (26th of May 2023)
///
/// See docs: <https://www.okx.com/docs-v5/en/#rest-api-public-data-get-instruments>
fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.date_naive().format("%g%m%d")
}
