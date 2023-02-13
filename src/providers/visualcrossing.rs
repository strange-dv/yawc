use crate::providers::provider::Provider;
use crate::utils;
use chrono::NaiveDate;
use serde_json::Value;

/// `VisualCrossing` key name
pub const PROVIDER_NAME: &str = "visualcrossing";

/// Retrieves information about weather using <https://weather.visualcrossing.com> API
pub struct VisualCrossing {}

impl Provider for VisualCrossing {
    /// Forms a request to `VisualCrossing`
    /// Docs can be found at <https://www.visualcrossing.com/resources/documentation/weather-api/timeline-weather-api/>
    fn form_request(&self, address: &str, date: NaiveDate) -> std::io::Result<ureq::Request> {
        let (api_key, api_base_url) = utils::parse_config_for(String::from(PROVIDER_NAME))?;

        Ok(
            ureq::get(format!("{api_base_url}/{address}/{date}").as_str())
                .query("key", api_key.as_str())
                .query("unitGroup", "metric"),
        )
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
    use crate::config;
    use serial_test::serial;

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

    #[test]
    #[serial]
    fn forms_right_api_request() {
        std::env::set_var(config::CONFIG_FILE_KEY, "./test_dependencies/config.json");

        let visualcrossing = VisualCrossing {};

        let date = NaiveDate::from_ymd_opt(2023, 2, 10).unwrap();
        let address = String::from("address");

        let correct_request = ureq::get(format!("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline/{address}/{date}").as_str())
            .query("key", "api_key")
            .query("unitGroup", "metric");

        assert_eq!(
            visualcrossing.form_request(&address, date).unwrap().url(),
            correct_request.url()
        );

        std::env::remove_var(config::CONFIG_FILE_KEY);
    }
}
