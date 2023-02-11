use chrono::NaiveDate;
use std::fmt;

/// Weather representation
pub struct Weather {
    weather_string: String,
    address: String,
    date: NaiveDate,
}

impl Weather {
    pub fn new(weather_string: String, address: String, date: NaiveDate) -> Weather {
        Weather {
            weather_string,
            address,
            date,
        }
    }
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Weather for {} on {}: {}",
            self.address, self.date, self.weather_string
        )
    }
}
