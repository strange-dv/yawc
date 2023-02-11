use crate::providers::provider::Provider;
use crate::providers::weather::Weather;
use chrono::NaiveDate;

/// Retrieves information about weather using WeatherAPI API
pub struct WeatherAPI {}

impl WeatherAPI {
    pub fn new() -> WeatherAPI {
        WeatherAPI {}
    }
}

impl Provider for WeatherAPI {
    fn get_weather(&self, address: String, date: NaiveDate) -> Weather {
        Weather::new(String::from("<weather>"), address, date)
    }
}
