use crate::weather::*;
use chrono::{Local, TimeZone};
use colored::Colorize;

/// Display current weather
pub fn show_current(weather: &CurrentWeather) {
    println!(
        "\n{}",
        "╔═══════════════════════════════════╗".bright_cyan()
    );
    println!("{}", "║     CURRENT WEATHER".bright_cyan().bold());
    println!("{}", "╚═══════════════════════════════════╝".bright_cyan());

    let emoji = get_weather_emoji(&weather.description);

    println!("  📍 {}", weather.location.bright_yellow().bold());
    println!(
        "  {} {} (feels like {})",
        emoji,
        format_temp(weather.temperature).bold(),
        format_temp(weather.feels_like)
    );
    println!("  {}", weather.description.bright_white());
    println!();
    println!(
        "  💧 Humidity: {:>4}%",
        weather.humidity.to_string().bright_blue()
    );
    println!(
        "  💨 Wind:     {:>4} m/s",
        format!("{:.1}", weather.wind_speed).bright_blue()
    );
    println!(
        "  🔽 Pressure: {:>4} hPa",
        weather.pressure.to_string().bright_blue()
    );
    println!(
        "  👁️  Visibility: {} m",
        weather.visibility.to_string().bright_blue()
    );
    println!();
    println!(
        "  🕐 {}",
        format_timestamp(weather.timestamp).bright_black()
    );
    println!();
}

/// Display weather forecast
pub fn show_forecast(forecast: &Forecast) {
    println!(
        "\n{}",
        format!(
            "📅 {}-Day Forecast for {}",
            forecast.days.len() - 1,
            forecast.location
        )
        .bright_cyan()
        .bold()
    );

    println!("{}", "─".repeat(50).bright_black());

    for day in forecast.days.iter().take(forecast.days.len() - 1) {
        let emoji = get_weather_emoji(&day.description);
        let precip = day
            .precipitation_chance
            .map(|p| format!(" 💧 {:.0}%", p * 100.0))
            .unwrap_or_default();

        println!(
            "  {} {:<12}  {} / {}  {} {}",
            emoji,
            day.date,
            format_temp(day.temp_min),
            format_temp(day.temp_max),
            day.description.bright_white(),
            precip.bright_blue()
        );
    }

    println!("{}\n", "─".repeat(50).bright_black());
}

/// Display location search results
pub fn show_locations(locations: &[Location]) {
    if locations.is_empty() {
        println!("{}", "No locations found".yellow());
        return;
    }

    println!("\n{}", "🔍 Search Results:".bright_cyan().bold());
    println!("{}", "─".repeat(50).bright_black());

    for (i, loc) in locations.iter().enumerate() {
        let state_str = loc
            .state
            .as_deref()
            .map(|s| format!(", {}", s))
            .unwrap_or_default();

        println!(
            "  {} {}{}, {}",
            format!("[{}]", i + 1).bright_cyan(),
            loc.name.bright_white().bold(),
            state_str,
            loc.country.bright_yellow()
        );
        println!("     📍 {:.4}, {:.4}", loc.lat, loc.lon);
    }

    println!("{}\n", "─".repeat(50).bright_black());
}

/// Get weather emoji based on description
fn get_weather_emoji(description: &str) -> &str {
    let desc = description.to_lowercase();
    if desc.contains("thunderstorm") {
        "⛈️"
    } else if desc.contains("drizzle") {
        "🌦️"
    } else if desc.contains("rain") {
        "🌧️"
    } else if desc.contains("snow") {
        "❄️"
    } else if desc.contains("mist") || desc.contains("fog") || desc.contains("haze") {
        "🌫️"
    } else if desc.contains("clear") {
        "☀️"
    } else if desc.contains("cloud") {
        "☁️"
    } else {
        "🌡️"
    }
}

/// Get temperature color based on value
fn get_temp_color(temp: f64) -> colored::Color {
    match temp {
        t if t < 0.0 => colored::Color::BrightBlue,
        t if t < 10.0 => colored::Color::Cyan,
        t if t < 20.0 => colored::Color::Yellow,
        t if t < 30.0 => colored::Color::BrightYellow,
        _ => colored::Color::Red,
    }
}

/// Format temperature with color
fn format_temp(temp: f64) -> String {
    format!("{:.1}°C", temp)
        .color(get_temp_color(temp))
        .to_string()
}

/// Convert Unix timestamp to readable time
fn format_timestamp(timestamp: i64) -> String {
    match Local.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%A, %b %d, %Y at %H:%M").to_string(),
        _ => "Unknown time".to_string(),
    }
}
