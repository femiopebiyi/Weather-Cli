use crate::error::{Result, WeatherError};
use crate::weather::*;

use std::env;

const BASE_URL: &str = "https://api.openweathermap.org/data/2.5";
const GEO_URL: &str = "https://api.openweathermap.org/geo/1.0";

/// Get API key from environment
fn get_api_key() -> Result<String> {
    // TODO: Get API key from environment variable
    dotenv::dotenv().ok();
    match env::var("OPENWEATHER_API_KEY") {
        Ok(key) => Ok(key),
        Err(error) => Err(WeatherError::EnvError(error.to_string())),
    }
}

/// Fetch current weather for a location
pub async fn fetch_current_weather(location: &str) -> Result<CurrentWeather> {
    // TODO: Build the API URL
    // HINT: format!("{}/weather?q={}&appid={}&units=metric", BASE_URL, location, api_key)
    let api_key = get_api_key()?;
    let url = format!(
        "{}/weather?q={}&appid={}&units=metric",
        BASE_URL, location, api_key
    );
    println!("Request URL: {}", url);

    // TODO: Make HTTP request
    // HINT: reqwest::get(url).await?

    let response = reqwest::get(&url)
        .await
        .map_err(WeatherError::NetworkError)?;
    println!("Response status: {}", response.status());

    // Check for HTTP errors
    if !response.status().is_success() {
        return Err(handle_api_error(response.status(), location));
    }

    // TODO: Check response status
    // HINT: response.error_for_status()?

    // TODO: Parse JSON response
    // HINT: response.json::<OpenWeatherResponse>().await?
    let api_response = response
        .json::<OpenWeatherResponse>()
        .await
        .map_err(WeatherError::NetworkError)?;
    println!("API response parsed successfully");

    // TODO: Convert to CurrentWeather
    // HINT: Ok(api_response.into())
    Ok(api_response.into())
}

/// Fetch weather forecast for a location
pub async fn fetch_forecast(location: &str, days: u8) -> Result<Forecast> {
    // TODO: Validate days (1-5)
    // HINT: if days > 5 || days < 1 { return Err(...) }
    if days > 5 || days < 1 {
        return Err(WeatherError::RequestFailed(
            "Days must be between 1 and 5".to_string(),
        ));
    }

    // TODO: Build forecast URL
    // HINT: format!("{}/forecast?q={}&appid={}&units=metric&cnt={}", BASE_URL, location, api_key, days * 8)
    // NOTE: API returns data in 3-hour intervals, so 8 intervals per day
    let api_key = get_api_key()?;
    let url = format!(
        "{}/forecast?q={}&appid={}&units=metric&cnt={}",
        BASE_URL,
        location,
        api_key,
        days * 8
    );
    println!("Request URL: {}", url);

    // TODO: Make request and parse
    // HINT: Similar to fetch_current_weather
    let response = reqwest::get(&url)
        .await
        .map_err(WeatherError::NetworkError)?;
    println!("Response status: {}", response.status());

    // TODO: Group forecast items by day
    // HINT: You'll need to process the list of ForecastItems

    let api_response = response
        .json::<ForecastResponse>()
        .await
        .map_err(WeatherError::NetworkError)?;

    // TODO: Convert to our Forecast struct
    Ok(api_response.into())
}

/// Search for locations by name
pub async fn search_locations(query: &str) -> Result<Vec<Location>> {
    // TODO: Build geocoding URL
    // HINT: format!("{}/direct?q={}&limit=5&appid={}", GEO_URL, query, api_key)
    let url = format!(
        "{}/direct?q={}&limit=5&appid={}",
        GEO_URL,
        query,
        get_api_key()?
    );

    // TODO: Make request
    // HINT: Similar to fetch_current_weather
    let response = reqwest::get(&url)
        .await
        .map_err(WeatherError::NetworkError)?;

    // TODO: Parse as Vec<GeocodingResponse>
    let geocoding_response = response
        .json::<Vec<GeocodingResponse>>()
        .await
        .map_err(WeatherError::NetworkError)?;

    // TODO: Convert to Vec<Location>
    // HINT: map GeocodingResponse to Location

    Ok(geocoding_response
        .into_iter()
        .map(|location| Location {
            country: location.country,
            lat: location.lat,
            lon: location.lon,
            name: location.name,
            state: location.state,
        })
        .collect::<Vec<Location>>())
}

/// Helper function to handle API errors
fn handle_api_error(status: reqwest::StatusCode, location: &str) -> WeatherError {
    match status.as_u16() {
        401 => WeatherError::InvalidApiKey,
        404 => WeatherError::LocationNotFound(location.to_string()),
        _ => WeatherError::RequestFailed(format!("Status: {}", status)),
    }
}
