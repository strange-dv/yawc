use crate::errors::WeatherError;

/// Handles configuring given `provider`
pub fn handle(provider: String) -> Result<(), WeatherError> {
    println!("Configured {}", provider);
    Ok(())
}
