use std::env::{self};

mod api;
mod cli;
mod display;
mod error;
mod weather;

use clap::Parser;
use cli::{Cli, Commands};

use crate::{
    api::{fetch_current_weather, fetch_forecast, search_locations},
    display::{show_forecast, show_locations},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Load environment variables

    dotenv::dotenv().ok();
    let api_key = env::var("OPENWEATHER_API_KEY")?;
    println!("API Key loaded: {}...", &api_key[..10]);

    // HINT: dotenv::dotenv().ok();

    let args = Cli::parse();

    match args.command {
        Commands::Current { location } => {
            // TODO: Fetch current weather
            // HINT: api::fetch_current_weather(&location).await

            let weather = fetch_current_weather(&location).await?;

            // TODO: Handle the result
            // HINT: match result { Ok(weather) => display::show_current(&weather), Err(e) => ... }

            display::show_current(&weather);
        }

        Commands::Forecast { location, days } => {
            // TODO: Fetch forecast
            // HINT: api::fetch_forecast(&location, days).await
            let forecast = fetch_forecast(&location, days).await?;

            println!("Fetching {}-day forecast for: {}", days, location);
            // TODO: Display forecast
            // HINT: display::show_forecast(&forecast)
            show_forecast(&forecast);
        }

        Commands::Search { query } => {
            // TODO: Search for locations
            // HINT: api::search_locations(&query).await
            let locations = search_locations(&query).await?;

            // TODO: Display matching locations
            // HINT: display::show_locations(&locations)
            println!("Searching for locations matching: {}", query);
            show_locations(&locations);
        }
    }

    Ok(())
}
