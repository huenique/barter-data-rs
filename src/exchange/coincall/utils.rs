use barter_integration::model::instrument::kind::OptionKind;
use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::ParseError;
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

pub fn ddmmmyy_to_unix_timestamp(date_str: &str, hour: Option<u32>) -> Result<i64, ParseError> {
    let format = "%d%b%y";
    let naive_date = NaiveDate::parse_from_str(date_str, format)?;

    let hour = hour.unwrap_or(0);
    let naive_datetime = NaiveDateTime::new(
        naive_date,
        chrono::NaiveTime::from_hms_opt(hour, 0, 0).expect("Invalid time"),
    );
    let unix_timestamp = naive_datetime.and_utc().timestamp_millis();

    Ok(unix_timestamp)
}
