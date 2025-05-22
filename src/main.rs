use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::io;
use colored::*;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    name: String,
    main: Main,
    weather: Vec<Weather>,
    wind: Wind, 
    id: u32
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    pressure: f64,
    humidity: f64,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64
}

fn display_data(weather_data: WeatherResponse) {
    println!("\nWeather in {} (City ID: {})", 
        format!("{}", weather_data.name).yellow().bold(),
        format!("{}", weather_data.id).yellow().bold(),
    );
    println!(
        "> {} {}",
        "Temperature:".green().bold(),
        format!("{}°C", kelvin_to_celsius(weather_data.main.temp)).bold()
    );

    println!(
        "> {} {} hPa",
        "Pressure:".red(),
        format!("{}", weather_data.main.pressure).bold()
    );

    println!(
        "> {} {}%",
        "Humidity:".cyan(),
        format!("{}", weather_data.main.humidity).bold()
    );
    println!(
        "> {} {}",
        "Description:".magenta(),
        format!("{}", weather_data.weather[0].description).bold()
    );

    println!(
        "> {} {}m/s",
        "Wind:".blue(),
        format!("{}", weather_data.wind.speed).bold()
    );
}

fn kelvin_to_celsius(k: f64) -> f64 {
    (k - 273.15).round()
}

async fn fetch_weather(city: &str, api_key: &str) -> Result<WeatherResponse, Box<dyn std::error::Error>> {

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city,
        api_key
    );

    let response = reqwest::get(&url).await?;
    // let weather_response: Value = response.json().await?; // Retrieve all data from the API
    let weather_response: WeatherResponse = response.json().await?;

    // println!("{:#?}", weather_response);
    // display_data(weather_response);
    Ok(weather_response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    const API_KEY: &str = "92aea18badbffad8095f83189c0ed170";

    loop {
        let mut city = String::new();
        println!("{}", "Enter the city:".blue().bold());
        io::stdin().read_line(&mut city).unwrap();
        let city = city.trim();

        let weather_data: WeatherResponse = fetch_weather(city, API_KEY).await?;

        display_data(weather_data);


        let mut input = String::new();
        println!("{}", "\nDo you want to enter another city? (yes/no)".green().bold());
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
    
        if input != "yes" {
            println!("{}", "\nThank you for using this tool!😎".red().bold());
            break;
        }
    
    }





    Ok(())

}