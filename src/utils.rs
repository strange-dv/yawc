use chrono::prelude::*;
use chrono::NaiveDate;

/// Parses the `date` argument and returns a date object
pub fn date_parser(arg: &str) -> Result<NaiveDate, clap::Error> {
    Ok(NaiveDate::parse_from_str(arg, "%Y-%m-%d").unwrap_or_else(|_| Utc::now().date_naive()))
}
