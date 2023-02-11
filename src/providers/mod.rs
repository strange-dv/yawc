pub mod provider;
pub mod visualcrossing;
pub mod weatherapi;

pub mod weather;

use crate::errors::{WeatherError, WeatherErrorKind};
use std::error::Error;
use crate::utils;

/// Returns object of given `provider` if it's supported
pub fn get_provider(
    provider: Option<String>,
) -> Result<Box<dyn provider::Provider>, Box<dyn Error>> {
    let provider = match provider {
        Some(provider_name) => provider_name,
        None => String::from(visualcrossing::PROVIDER_NAME)
    };

    let configs: serde_json::Value =
        serde_json::from_str(std::fs::read_to_string("config.json")?.as_str())?;

    let provider_configs = &configs[&provider];

    match provider.as_str() {
        visualcrossing::PROVIDER_NAME => {
           let (api_key, api_base_url) = utils::parse_config_for(provider_configs)?;
            Ok(Box::new(visualcrossing::VisualCrossing::new(
                api_key,
                api_base_url,
            )))
        }
        weatherapi::PROVIDER_NAME => {
            let (api_key, api_base_url) = utils::parse_config_for(provider_configs)?;
            Ok(Box::new(weatherapi::WeatherAPI::new(
                api_key,
                api_base_url,
            )))
        }
        _ => Err(Box::new(WeatherError(WeatherErrorKind::ProviderNotFound))),
    }
}
