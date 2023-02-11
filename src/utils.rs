/// Parses API key and API base url for given provider
pub fn parse_config_for(provider: &serde_json::Value) -> std::io::Result<(String, String)> {
    let api_key = provider["api_key"].as_str().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "{} not configure. Use `configure` command to configure provider credentials",
                provider
            ),
        )
    })?;
    let api_base_url = provider["api_base_url"].as_str().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "{} not configure. Use `configure` command to configure provider credentials",
                provider
            ),
        )
    })?;

    Ok((String::from(api_key), String::from(api_base_url)))
}
