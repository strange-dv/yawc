use crate::providers::provider::Provider;
use chrono::NaiveDate;
use serde_json::Value;
use crate::utils;

/// `VisualCrossing` key name
pub const PROVIDER_NAME: &str = "visualcrossing";

/// Retrieves information about weather using <https://weather.visualcrossing.com> API
pub struct VisualCrossing {}

impl Provider for VisualCrossing {
    /// Returns weather using `VisualCrossing`.
    /// Docs can be found at <https://www.visualcrossing.com/resources/documentation/weather-api/timeline-weather-api/>
    fn get_response(&self, address: &str, date: NaiveDate) -> std::io::Result<Value> {
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

    fn form_weather_report(&self, response: Value) -> std::io::Result<String> {
        let day = &response["days"].get(0).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No forecast for that day available",
            )
        })?;

        Ok(format!(
            "{}, temperature was {}C°",
            day["description"]
                .as_str()
                .ok_or_else(|| std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "No forecast for that day available",
                ))?,
            day["temp"]
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_right_weather_report() {
        let weatherapi = VisualCrossing {};

        let correct_response: Value = serde_json::from_str(
            std::fs::read_to_string("test_dependencies/visualcrossing_kyiv_2023-01-01.json")
                .unwrap()
                .as_str(),
        )
            .unwrap();

        assert_eq!(
            weatherapi.form_weather_report(correct_response).unwrap(),
            String::from("Partly cloudy throughout the day., temperature was 10.1C°")
        );
    }
}