use crate::providers::provider::Provider;
use crate::providers::weather::Weather;
use chrono::NaiveDate;

/// Retrieves information about weather using OpenWeather API
pub struct OpenWeather {}

impl OpenWeather {
    pub fn new() -> OpenWeather {
        OpenWeather {}
    }
}

impl Provider for OpenWeather {
    fn get_weather(&self, address: String, date: NaiveDate) -> Weather {
        Weather::new(String::from("<weather>"), address, date)
    }
}
