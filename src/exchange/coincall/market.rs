use crate::exchange::coincall::Coincall;
use crate::subscription::Subscription;
use crate::Identifier;

use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionKind;
use barter_integration::model::instrument::Instrument;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Coincall`](super::Coincall) market that can be subscribed to.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct CoincallMarket(pub String);

impl<Server, Kind> Identifier<CoincallMarket> for Subscription<Coincall<Server>, Kind> {
    fn id(&self) -> CoincallMarket {
        use InstrumentKind::*;

        let Instrument {
            base,
            quote: _,
            kind,
        } = &self.instrument;

        CoincallMarket(match kind {
            Spot => todo!(),
            Future(_future) => todo!(),
            Perpetual => todo!(),
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

impl AsRef<str> for CoincallMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Format the expiry DateTime<Utc> to be Coincall API compatible.
///
/// eg/ "21JUN23" (21th of June 2023)
fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.date_naive().format("%-d%b%y")
}
