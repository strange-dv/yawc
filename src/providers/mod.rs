pub mod openweather;
pub mod provider;
pub mod weatherapi;

pub mod weather;

use crate::errors::{WeatherError, WeatherErrorKind};

/// Returns object of given `provider` if it's supported
pub fn get_provider(provider: Option<String>) -> Result<Box<dyn provider::Provider>, WeatherError> {
    let default_provider = String::from("openweather");

    match provider.unwrap_or(default_provider).as_str() {
        "openweather" => Ok(Box::new(openweather::OpenWeather::new())),
        "weatherapi" => Ok(Box::new(weatherapi::WeatherAPI::new())),
        _ => Err(WeatherError(WeatherErrorKind::ProviderNotFound)),
    }
}
