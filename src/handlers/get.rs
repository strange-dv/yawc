use chrono::{NaiveDate, Utc};

pub fn handle(address: String, date: Option<NaiveDate>) {
    println!(
        "Weather for {} at {}",
        address,
        date.unwrap_or_else(|| Utc::now().date_naive())
    );
}
