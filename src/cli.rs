use clap::{Parser, Subcommand};

/// A beautiful weather CLI application
#[derive(Parser, Debug)]
#[command(name = "weather")]
#[command(about = "Get weather information from the command line", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get current weather for a location
    Current {
        /// Location (city name, zip code, or coordinates)
        #[arg(value_name = "LOCATION")]
        location: String,
    },

    /// Get weather forecast
    Forecast {
        /// Location (city name, zip code, or coordinates)
        #[arg(value_name = "LOCATION")]
        location: String,

        /// Number of days (1-5)
        #[arg(short, long, default_value_t = 3)]
        days: u8,
    },

    /// Search for locations
    Search {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,
    },
}
