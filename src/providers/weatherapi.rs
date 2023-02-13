use crate::providers::provider::Provider;
use crate::utils;
use chrono::NaiveDate;
use serde_json;
use serde_json::Value;
use ureq;
use ureq::Request;

/// `WeatherAPI` key name
pub const PROVIDER_NAME: &str = "weatherapi";

/// Retrieves information about weather using <https://www.weatherapi.com> API
pub struct WeatherAPI {}

impl Provider for WeatherAPI {
    /// Forms a request to `WeatherAPI`.
    /// Docs can be found at <https://www.weatherapi.com/api-explorer.aspx#history>
    fn form_request(&self, address: &str, date: NaiveDate) -> std::io::Result<Request> {
        let (api_key, api_base_url) = utils::parse_config_for(String::from(PROVIDER_NAME))?;

        Ok(ureq::get(api_base_url.as_str())
            .query("key", api_key.as_str())
            .query("q", address)
            .query("dt", &date.to_string()))
    }

    fn form_weather_report(&self, response: Value) -> std::io::Result<String> {
        let day = &response["forecast"]["forecastday"].get(0).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No forecast for that day available",
            )
        })?["day"];

        Ok(format!(
            "{}, temperature was {}C°",
            day["condition"]["text"]
                .as_str()
                .ok_or_else(|| std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "No forecast for that day available",
                ))?,
            day["avgtemp_c"]
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
        let weatherapi = WeatherAPI {};

        let correct_response: Value = serde_json::from_str(
            std::fs::read_to_string("test_dependencies/weatherapi_kyiv_2023-01-01.json")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        assert_eq!(
            weatherapi.form_weather_report(correct_response).unwrap(),
            String::from("Overcast, temperature was 9.3C°")
        );
    }

    #[test]
    #[serial]
    fn forms_right_api_request() {
        std::env::set_var(config::CONFIG_FILE_KEY, "./test_dependencies/config.json");

        let weatherapi = WeatherAPI {};

        let date = NaiveDate::from_ymd_opt(2023, 2, 10).unwrap();
        let address = String::from("address");

        let correct_request = ureq::get("https://api.weatherapi.com/v1/history.json")
            .query("key", "api_key")
            .query("q", &address)
            .query("dt", &date.to_string());

        assert_eq!(
            weatherapi.form_request(&address, date).unwrap().url(),
            correct_request.url()
        );

        std::env::remove_var(config::CONFIG_FILE_KEY);
    }
}
