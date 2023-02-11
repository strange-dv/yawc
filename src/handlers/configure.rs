/// Handles configuring given `provider`
pub fn handle(provider: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Configured {provider}");
    Ok(())
}
