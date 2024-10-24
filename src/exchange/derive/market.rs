use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionKind;
use barter_integration::model::instrument::Instrument;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::exchange::derive::Derive;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Derive`](super::Derive) market that can be subscribed to.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeriveMarket(pub String);

impl<Kind> Identifier<DeriveMarket> for Subscription<Derive, Kind> {
    fn id(&self) -> DeriveMarket {
        use InstrumentKind::*;

        let Instrument {
            base,
            quote: _,
            kind,
        } = &self.instrument;

        DeriveMarket(match kind {
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
            Spot => todo!(),
            Future(_) => todo!(),
            Perpetual => todo!(),
        })
    }
}

impl AsRef<str> for DeriveMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.format("%Y%m%d")
}
