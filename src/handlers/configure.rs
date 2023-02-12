use crate::errors::{WeatherError, WeatherErrorKind};
use crate::{config, providers};
use dialoguer::{theme::ColorfulTheme, Input};
use serde_json::json;
use serde_json::map::Map;
use validator::validate_url;

/// Handles configuring given `provider`
pub fn handle(provider: String) -> Result<(), Box<dyn std::error::Error>> {
    if !providers::PROVIDERS.contains_key(provider.as_str()) {
        return Err(Box::new(WeatherError(WeatherErrorKind::ProviderNotFound)));
    }

    let api_key: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("API key:")
        .interact_text()
        .unwrap();

    let api_base_url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("API key:")
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if !validate_url(input) {
                    return Err("This is not a URL");
                }
                Ok(())
            }
        })
        .interact_text()
        .unwrap();

    let mut configs: Map<String, serde_json::Value> =
        serde_json::from_str(std::fs::read_to_string(config::get_config_file())?.as_str())?;

    configs.insert(
        provider,
        json!({
            "api_key": api_key,
            "api_base_url": api_base_url
        }),
    );

    std::fs::write(
        config::get_config_file(),
        serde_json::to_string_pretty(&configs)?,
    )?;

    println!("Provider configured!");

    Ok(())
}
