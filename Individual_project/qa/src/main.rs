use std::io::{self};
use reqwest::Url;

const API_KEY: &str = "73219f57b5b246895d0caa8be76ec1de";
const API_ENDPOINT: &str = "https://api.openweathermap.org/data/2.5/weather";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the city name from the user.
    println!("Enter the name of the city:");
    let mut city = String::new();
    io::stdin().read_line(&mut city)?;

    // Build the API URL.
    let url = Url::parse_with_params(API_ENDPOINT, &[("q", city.trim()), ("appid", API_KEY)])?;

    // Send the API request and parse the response.
    let response = reqwest::blocking::get(url)?;
    let json = response.json::<serde_json::Value>()?;

    // Extract the temperature and description from the JSON response.
    let temperature = json["main"]["temp"].as_f64().unwrap() - 273.15; // Convert from Kelvin to Celsius
    let description = json["weather"][0]["description"].as_str().unwrap();

    // Print the weather information to the console.
    println!("Temperature: {:.1} °C", temperature);
    println!("Description: {}", description);

    Ok(())
}