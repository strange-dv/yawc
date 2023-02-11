pub mod provider;
pub mod visualcrossing;
pub mod weatherapi;

pub mod weather;

use crate::errors::{WeatherError, WeatherErrorKind};

/// Returns object of given `provider` if it's supported
pub fn get_provider(provider: Option<String>) -> Result<Box<dyn provider::Provider>, WeatherError> {
    let default_provider = String::from("visualcrossing");

    match provider.unwrap_or(default_provider).as_str() {
        "visualcrossing" => {
            let api_key = String::from("");
            let api_base_url = String::from("https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline");
            Ok(Box::new(visualcrossing::VisualCrossing::new(
                api_key,
                api_base_url,
            )))
        }
        "weatherapi" => {
            let api_key = String::from("");
            let api_base_url = String::from("https://api.weatherapi.com/v1/history.json");
            Ok(Box::new(weatherapi::WeatherAPI::new(api_key, api_base_url)))
        }
        _ => Err(WeatherError(WeatherErrorKind::ProviderNotFound)),
    }
}
