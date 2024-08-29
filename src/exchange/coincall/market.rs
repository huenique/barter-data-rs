use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::Instrument;
use serde::Deserialize;
use serde::Serialize;

use crate::exchange::coincall::utils::format_opt_instr;
use crate::exchange::coincall::Coincall;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Coincall`](super::Coincall) market that can be subscribed to.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CoincallMarket(pub String);

impl<Server, Kind> Identifier<CoincallMarket> for Subscription<Coincall<Server>, Kind> {
    fn id(&self) -> CoincallMarket {
        use InstrumentKind::*;

        let Instrument { base, quote, kind } = &self.instrument;

        CoincallMarket(match kind {
            Spot => todo!(),
            Future(_future) => todo!(),
            Perpetual => todo!(),
            Option(option) => format_opt_instr(
                base.as_ref(),
                quote.as_ref(),
                option.expiry,
                option.strike,
                option.kind,
            ),
        })
    }
}

impl AsRef<str> for CoincallMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
