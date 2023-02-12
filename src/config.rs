pub const CONFIG_FILE_KEY: &str = "CONFIG_FILE_KEY";

/// Returns current config file name
pub fn get_config_file() -> String {
    std::env::var(CONFIG_FILE_KEY).unwrap_or_else(|_| String::from("config.json"))
}
