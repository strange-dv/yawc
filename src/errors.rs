use std::error::Error;
use std::fmt;

/// Categories of weather errors
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum WeatherErrorKind {
    /// Given provider is not supported
    ProviderNotFound,
}

/// Weather error
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct WeatherError(pub WeatherErrorKind);

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            WeatherErrorKind::ProviderNotFound => write!(f, "Provider not found"),
        }
    }
}

impl Error for WeatherError {}
