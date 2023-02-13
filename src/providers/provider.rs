use super::weather::Weather;
use chrono::NaiveDate;

/// Retrieves information about weather
pub trait Provider {
    fn get_response(&self, address: &str, date: NaiveDate) -> std::io::Result<serde_json::Value> {
        self.form_request(address, date)?
            .call()
            .map_err(|e| match e {
                ureq::Error::Status(code, response) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "Provider returned error:\nStatus code: {}\nError: {:?}",
                        code,
                        response.into_string().unwrap()
                    ),
                ),
                ureq::Error::Transport(transport) => std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Error calling provider: {:?}", transport.message()),
                ),
            })?
            .into_json()
    }
    fn get_weather(&self, address: String, date: NaiveDate) -> std::io::Result<Weather> {
        let response = self.get_response(&address, date)?;

        Ok(Weather::new(
            self.form_weather_report(response)?,
            address,
            date,
        ))
    }

    fn form_request(&self, address: &str, date: NaiveDate) -> std::io::Result<ureq::Request>;
    fn form_weather_report(&self, response: serde_json::Value) -> std::io::Result<String>;
}
