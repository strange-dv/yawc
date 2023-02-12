pub const CONFIG_FILE_KEY: &str = "CONFIG_FILE_KEY";

/// Returns current config file name
pub fn get_config_file() -> String {
    std::env::var(CONFIG_FILE_KEY).unwrap_or_else(|_| String::from("config.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn returns_needed_config_file() {
        let config_file = String::from("./test_dependencies/config.json");
        std::env::set_var(CONFIG_FILE_KEY, &config_file);

        assert_eq!(config_file, get_config_file());
        std::env::remove_var(CONFIG_FILE_KEY);
    }

    #[test]
    #[serial]
    fn returns_default_config_file() {
        assert_eq!(String::from("config.json"), get_config_file());
    }
}
