use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum WeatherError {
    // TODO: Define error types
    #[error("API request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid API key. Please check your .env file")]
    InvalidApiKey,

    #[error("Location not found: {0}")]
    LocationNotFound(String),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Environment variable error: {0}")]
    EnvError(String),
}

pub type Result<T> = std::result::Result<T, WeatherError>;
