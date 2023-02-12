pub mod provider;
pub mod visualcrossing;
pub mod weatherapi;

pub mod weather;

use crate::errors::{WeatherError, WeatherErrorKind};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;

lazy_static! {
    pub static ref PROVIDERS: HashMap<&'static str, Box<dyn provider::Provider + Sync>> = {
        let mut m: HashMap<&'static str, Box<dyn provider::Provider + Sync>> = HashMap::new();

        // visual crossing
        m.insert(
            visualcrossing::PROVIDER_NAME,
            Box::new(visualcrossing::VisualCrossing {}),
        );

        // weatherapi
        m.insert(
            weatherapi::PROVIDER_NAME,
            Box::new(weatherapi::WeatherAPI {})
        );

        m
    };
}

/// Returns object of given `provider` if it's supported
pub fn get_provider(
    provider: Option<String>,
) -> Result<&'static (dyn provider::Provider + Sync), Box<dyn Error>> {
    let provider = match provider {
        Some(provider_name) => provider_name,
        // default provider
        None => String::from(visualcrossing::PROVIDER_NAME),
    };

    if let Some(provider_object) = PROVIDERS.get(provider.as_str()) {
        return Ok(&**provider_object);
    }

    Err(Box::new(WeatherError(WeatherErrorKind::ProviderNotFound)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_existing_provider() {
        let _provider = get_provider(Some(String::from(visualcrossing::PROVIDER_NAME))).unwrap();
    }

    #[test]
    #[should_panic]
    fn cannot_get_non_existing_provider() {
        let _provider = get_provider(Some(String::from("provider1"))).unwrap();
    }
}
