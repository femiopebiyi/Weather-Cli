use serde::{Deserialize, Serialize};

/// Current weather data
#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentWeather {
    // TODO: Add fields for current weather
    // HINT: You'll need fields like:
    pub location: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub description: String,
    pub humidity: u8,
    pub wind_speed: f64,
    pub pressure: u32,
    pub visibility: u32,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize)]
pub struct OpenWeatherResponse {
    // TODO: Map API response to our CurrentWeather struct
    // API docs: https://openweathermap.org/current
    pub main: MainWeather,
    pub weather: Vec<WeatherDescription>,
    pub wind: Wind,
    pub visibility: Option<u32>,
    pub dt: i64,
    pub name: String,
}

/// Weather forecast for a single day
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastDay {
    // TODO: Add fields for forecast
    // HINT: Similar to CurrentWeather but with:
    pub date: String,
    pub temp_min: f64,
    pub temp_max: f64,
    pub description: String,
    pub precipitation_chance: Option<f64>,
}

/// Complete forecast response
#[derive(Debug, Deserialize, Serialize)]
pub struct Forecast {
    pub location: String,
    pub days: Vec<ForecastDay>,
}

/// Location search result
#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub state: Option<String>,
    pub lat: f64,
    pub lon: f64,
}

// OpenWeatherMap API Response Structures
// These match the actual API response format

#[derive(Debug, Deserialize)]
pub struct MainWeather {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub pressure: u32,
}

#[derive(Debug, Deserialize)]
pub struct WeatherDescription {
    pub description: String,
    pub main: String,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    // TODO: Map API forecast response

    // API docs: https://openweathermap.org/forecast5
    pub list: Vec<ForecastItem>,
    pub city: City,
}

#[derive(Debug, Deserialize)]
pub struct ForecastItem {
    pub dt: i64,
    pub main: MainWeather,
    pub weather: Vec<WeatherDescription>,
    pub pop: Option<f64>, // Probability of precipitation
}

#[derive(Debug, Deserialize)]
pub struct City {
    pub name: String,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct GeocodingResponse {
    pub name: String,
    pub country: String,
    pub state: Option<String>,
    pub lat: f64,
    pub lon: f64,
}

impl From<OpenWeatherResponse> for CurrentWeather {
    fn from(response: OpenWeatherResponse) -> Self {
        CurrentWeather {
            location: response.name,
            temperature: response.main.temp,
            feels_like: response.main.feels_like,
            description: response
                .weather
                .first()
                .map(|w| w.description.clone())
                .unwrap_or_default(),
            humidity: response.main.humidity,
            wind_speed: response.wind.speed,
            pressure: response.main.pressure,
            visibility: response.visibility.unwrap_or(0),
            timestamp: response.dt,
        }
    }
}

impl From<ForecastResponse> for Forecast {
    fn from(response: ForecastResponse) -> Self {
        use std::collections::HashMap;

        let mut daily: HashMap<String, ForecastDay> = HashMap::new();

        for item in &response.list {
            let date = chrono::DateTime::from_timestamp(item.dt, 0)
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();

            let description = item
                .weather
                .first()
                .map(|w| w.description.clone())
                .unwrap_or_default();

            daily
                .entry(date.clone())
                .and_modify(|day| {
                    if item.main.temp < day.temp_min {
                        day.temp_min = item.main.temp;
                    }
                    if item.main.temp > day.temp_max {
                        day.temp_max = item.main.temp;
                    }
                    if let Some(pop) = item.pop {
                        day.precipitation_chance =
                            Some(day.precipitation_chance.unwrap_or(0.0).max(pop));
                    }
                })
                .or_insert(ForecastDay {
                    date,
                    temp_min: item.main.temp,
                    temp_max: item.main.temp,
                    description,
                    precipitation_chance: item.pop,
                });
        }

        let mut days: Vec<ForecastDay> = daily.into_values().collect();
        days.sort_by(|a, b| a.date.cmp(&b.date));

        Forecast {
            location: response.city.name,
            days,
        }
    }
}
