use barter_integration::model::instrument::kind::InstrumentKind;
use barter_integration::model::instrument::kind::OptionKind;
use barter_integration::model::instrument::Instrument;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::exchange::deribit::Deribit;
use crate::subscription::Subscription;
use crate::Identifier;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Deribit`](super::Deribit) market that can be subscribed to.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel>
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DeribitMarket(pub String);

impl<Server, Kind> Identifier<DeribitMarket> for Subscription<Deribit<Server>, Kind> {
    fn id(&self) -> DeribitMarket {
        use InstrumentKind::*;

        let Instrument { base, quote, kind } = &self.instrument;

        DeribitMarket(match kind {
            Spot => format!("{base}_{quote}"),
            Future(future) => format!("{base}-{}", format_expiry(future.expiry)).to_uppercase(),
            Perpetual => format!("{base}-PERPETUAL").to_uppercase(),
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

impl AsRef<str> for DeribitMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.date_naive().format("%-d%b%y")
}
