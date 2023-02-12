use crate::config;

/// Parses API key and API base url for given provider
pub fn parse_config_for(provider: String) -> std::io::Result<(String, String)> {
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

    let api_key = provider_configs["api_key"].as_str().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "{} not configured. Use `configure` command to configure provider credentials",
                provider
            ),
        )
    })?;
    let api_base_url = provider_configs["api_base_url"].as_str().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "{} not configured. Use `configure` command to configure provider credentials",
                provider
            ),
        )
    })?;

    Ok((String::from(api_key), String::from(api_base_url)))
}
