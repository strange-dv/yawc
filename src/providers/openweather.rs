use crate::providers::provider::Provider;
use crate::providers::weather::Weather;
use chrono::NaiveDate;

/// Retrieves information about weather using OpenWeather API
pub struct OpenWeather {
    api_key: String,
    api_base_url: String,
}

impl OpenWeather {
    pub fn new(api_key: String, api_base_url: String) -> OpenWeather {
        OpenWeather { api_key, api_base_url }
    }
}

impl Provider for OpenWeather {
    fn get_weather(&self, address: String, date: NaiveDate) -> std::io::Result<Weather> {
        Ok(Weather::new(String::from("<weather>"), address, date))
    }
}
