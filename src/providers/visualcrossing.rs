use crate::providers::provider::Provider;
use crate::providers::weather::Weather;
use chrono::NaiveDate;

/// Retrieves information about weather using weather.visualcrossing API
pub struct VisualCrossing {
    api_key: String,
    api_base_url: String,
}

impl VisualCrossing {
    pub fn new(api_key: String, api_base_url: String) -> VisualCrossing {
        VisualCrossing {
            api_key,
            api_base_url,
        }
    }
}

impl Provider for VisualCrossing {
    /// Returns weather using VisualCrossing.
    /// Docs can be found at https://www.visualcrossing.com/resources/documentation/weather-api/timeline-weather-api/
    fn get_response(
        &self,
        address: &str,
        date: &NaiveDate,
    ) -> std::io::Result<serde_json::Value> {
        ureq::get(format!("{}/{}/{}", self.api_base_url, address, date).as_str())
            .query("key", self.api_key.as_str())
            .query("unitGroup", "metric")
            .call()
            .map_err(|e| match e {
                ureq::Error::Status(code, response) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "VisualCrossing returned error:\nStatus code: {}\nError: {:?}",
                        code,
                        response.into_string().unwrap()
                    ),
                ),
                ureq::Error::Transport(transport) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Error calling VisualCrossing: {:?}", transport.message()),
                ),
            })?
            .into_json()
    }

    fn get_weather(&self, address: String, date: NaiveDate) -> std::io::Result<Weather> {
        let response = self.get_response(&address, &date)?;

        let day = &response["days"].get(0).ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "No forecast for that day available",
        ))?;

        Ok(Weather::new(
            format!(
                "{}, temperature was {}C°",
                day["description"].as_str().ok_or_else(|| std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "No forecast for that day available",
                ))?,
                day["temp"]
            ),
            address,
            date,
        ))
    }
}
