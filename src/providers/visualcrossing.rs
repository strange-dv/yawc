use crate::providers::provider::Provider;
use crate::providers::weather::Weather;
use chrono::NaiveDate;
use crate::utils;

/// `VisualCrossing` key name
pub const PROVIDER_NAME: &str = "visualcrossing";

/// Retrieves information about weather using <https://weather.visualcrossing.com> API
pub struct VisualCrossing {}

impl Provider for VisualCrossing {
    /// Returns weather using `VisualCrossing`.
    /// Docs can be found at <https://www.visualcrossing.com/resources/documentation/weather-api/timeline-weather-api/>
    fn get_response(&self, address: &str, date: NaiveDate) -> std::io::Result<serde_json::Value> {
        let (api_key, api_base_url) = utils::parse_config_for(String::from(PROVIDER_NAME))?;

        ureq::get(format!("{api_base_url}/{address}/{date}").as_str())
            .query("key", api_key.as_str())
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
        let response = self.get_response(&address, date)?;

        let day = &response["days"].get(0).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No forecast for that day available",
            )
        })?;

        Ok(Weather::new(
            format!(
                "{}, temperature was {}C°",
                day["description"]
                    .as_str()
                    .ok_or_else(|| std::io::Error::new(
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
