use barter_integration::model::instrument::kind::OptionKind;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;

pub fn format_opt_instr(
    base: &str,
    quote: &str,
    expiry: DateTime<Utc>,
    strike: Decimal,
    kind: OptionKind,
) -> String {
    format!(
        "{base}{quote}-{expiry}-{strike}-{kind}",
        base = base,
        quote = quote,
        expiry = format_expiry(expiry),
        strike = strike,
        kind = match kind {
            OptionKind::Call => "C",
            OptionKind::Put => "P",
        },
    )
    .to_uppercase()
}

/// Format the expiry DateTime<Utc> to be Coincall API compatible.
pub fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.date_naive().format("%-d%b%y")
}
