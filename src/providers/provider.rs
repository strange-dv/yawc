use super::weather::Weather;
use chrono::NaiveDate;

/// Retrieves information about weather
pub trait Provider {
    fn get_response(&self, address: &str, date: NaiveDate) -> std::io::Result<serde_json::Value>;
    fn form_weather_report(&self, response: serde_json::Value) -> std::io::Result<String>;
    fn get_weather(&self, address: String, date: NaiveDate) -> std::io::Result<Weather> {
        let response = self.get_response(&address, date)?;

        Ok(Weather::new(
            self.form_weather_report(response)?,
            address,
            date,
        ))
    }
}
