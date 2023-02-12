use super::weather::Weather;
use crate::{config, utils};
use chrono::NaiveDate;

/// Retrieves information about weather
pub trait Provider {
    fn load_configs(&self, provider: String) -> std::io::Result<(String, String)> {
        let config_file = config::get_config_file();
        let configs: serde_json::Value = serde_json::from_str(
            std::fs::read_to_string(&config_file)
                .map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Error while loading the config file {}", &config_file),
                    )
                })?
                .as_str(),
        )?;

        let provider_configs = &configs[&provider];

        utils::parse_config_for(provider_configs)
    }
    fn get_response(&self, address: &str, date: NaiveDate) -> std::io::Result<serde_json::Value>;
    fn get_weather(&self, address: String, date: NaiveDate) -> std::io::Result<Weather>;
}
