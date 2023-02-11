use crate::providers::provider::Provider;
use crate::providers::weather::Weather;
use chrono::NaiveDate;
use serde_json;
use ureq;

/// Retrieves information about weather using WeatherAPI API
pub struct WeatherAPI {
    api_key: String,
    api_base_url: String,
}

impl WeatherAPI {
    pub fn new(api_key: String, api_base_url: String) -> WeatherAPI {
        WeatherAPI {
            api_key,
            api_base_url,
        }
    }

    /// Returns weather using WeatherAPI.
    /// Docs can be found at https://www.weatherapi.com/api-explorer.aspx#history
    fn get_response(
        &self,
        address: &String,
        date: &NaiveDate,
    ) -> std::io::Result<serde_json::Value> {
        ureq::get(self.api_base_url.as_str())
            .query("key", self.api_key.as_str())
            .query("q", address.as_str())
            .query("dt", &date.to_string())
            .call()
            .map_err(|e| match e {
                ureq::Error::Status(code, response) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "WeatherAPI returned error:\nStatus code: {}\nError: {:?}",
                        code,
                        response.into_string().unwrap()
                    ),
                ),
                ureq::Error::Transport(transport) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Error calling WeatherAPI: {:?}", transport.message()),
                ),
            })?
            .into_json()
    }
}

impl Provider for WeatherAPI {
    fn get_weather(&self, address: String, date: NaiveDate) -> std::io::Result<Weather> {
        let response = self.get_response(&address, &date)?;

        let day = &response["forecast"]["forecastday"]
            .get(0)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("No forecast for that day available"),
            ))?["day"];

        Ok(Weather::new(
            format!(
                "{}, temperature was {}C°",
                day["condition"]["text"]
                    .as_str()
                    .ok_or(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("No forecast for that day available"),
                    ))?,
                day["avgtemp_c"]
            ),
            address,
            date,
        ))
    }
}
