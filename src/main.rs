mod errors;
mod handlers;
mod providers;

use chrono::prelude::*;
use clap::{Parser, Subcommand};

/// Weather CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "weather")]
#[command(about = "Weather CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Configure credentials for `provider`,
    /// where `provider` is some short name for a concrete weather API
    #[command(arg_required_else_help = true)]
    Configure {
        /// Provider's name
        provider: String,
    },
    /// Show weather for the provided `address`
    /// The date parameter is optional, default is now
    Get {
        /// Specific address to get weather for
        #[arg(required = true)]
        address: String,
        /// Specific date to get weather for
        // we don't need to specify parser since NaiveDate implements the FromStr trait
        date: Option<NaiveDate>,
        #[arg(long, require_equals = true)]
        provider: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    let result = match args.command {
        Commands::Configure { provider } => handlers::configure::handle(provider),
        Commands::Get {
            address,
            date,
            provider,
        } => handlers::get::handle(provider, address, date),
    };

    match result {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
