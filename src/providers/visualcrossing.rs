use crate::providers::provider::Provider;
use crate::providers::weather::Weather;
use chrono::{DateTime, NaiveDate};

/// Retrieves information about weather using OpenWeather API
pub struct AerisWeather {
    api_key: String,
    api_base_url: String,
}

impl AerisWeather {
    pub fn new(api_key: String, api_base_url: String) -> AerisWeather {
        AerisWeather {
            api_key,
            api_base_url,
        }
    }
}

impl Provider for AerisWeather {
    fn get_weather(&self, address: String, date: NaiveDate) -> std::io::Result<Weather> {
        Ok(Weather::new(String::from("<weather>"), address, date))
    }
}
