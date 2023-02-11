use crate::errors::WeatherError;
use crate::providers::get_provider;
use chrono::{NaiveDate, Utc};

/// Handles displaying weather for given `address` and `date` using `provider`
pub fn handle(
    provider: Option<String>,
    address: String,
    date: Option<NaiveDate>,
) -> Result<(), WeatherError> {
    let provider = get_provider(provider)?;

    let date = date.unwrap_or_else(|| Utc::now().date_naive());

    println!("{}", provider.get_weather(address, date));

    Ok(())
}
