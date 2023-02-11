use super::weather::Weather;
use chrono::NaiveDate;

/// Retrieves information about weather
pub trait Provider {
    fn get_response(&self, address: &str, date: &NaiveDate) -> std::io::Result<serde_json::Value>;
    fn get_weather(&self, address: String, date: NaiveDate) -> std::io::Result<Weather>;
}
