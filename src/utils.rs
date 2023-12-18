use crate::data::{CurrentWeather, DailyData, DailyWeatherResult, NowWeatherResult, Secret};

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

const BASE_URL: &str = "https://api.seniverse.com/v3/weather";

async fn reset_json_file() -> Result<(), Box<dyn std::error::Error>> {
    let default_api_key: String = "SkyWErkPwye-1C6wv".to_string();
    let default_location: String = "GuangZhou".to_string();
    let default_language: String = "en".to_string();

    let secret_data: Secret = Secret {
        api_key: default_api_key.clone(),
        location: default_location.clone(),
        language: default_language.clone(),
    };

    let json_string: String = serde_json::to_string_pretty(&secret_data)?;

    let mut file: File = File::create("secret.json")?;
    file.write_all(json_string.as_bytes())?;
    println!("Data successfully written to the secret.json file.");

    Ok(())
}

pub async fn read_json_file(file_path: &str) -> Result<Secret, Box<dyn std::error::Error>> {
    if !Path::new(file_path).exists() {
        reset_json_file().await?;
    }

    let mut file: File = File::open(file_path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    let secret: Secret = match serde_json::from_str(&contents) {
        Ok(s) => s,
        Err(_) => {
            reset_json_file().await?;
            Secret::default()
        }
    };

    Ok(secret)
}

pub async fn write_json_file(
    file_path: &str,
    secret: &Secret,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: File = File::create(file_path)?;
    let serialized_data: String = serde_json::to_string_pretty(secret)?;
    let bytes_written: usize = file.write(serialized_data.as_bytes())?;

    if bytes_written > 0 {
        Ok(())
    } else {
        println!("Failed to write data to the file.\n");
        Ok(())
    }
}

async fn get_current_weather(
    secret: &Secret,
) -> Result<CurrentWeather, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}/now.json?key={}&location={}&language={}&unit=c",
        BASE_URL, secret.api_key, secret.location, secret.language
    );

    let res: reqwest::Response = reqwest::get(&url).await?;

    if res.status().is_success() {
        let body: String = res.text().await?;

        let weather_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(result) = weather_data["results"].get(0) {
            let result: NowWeatherResult = serde_json::from_value(result.clone())?;

            let current_weather: CurrentWeather = CurrentWeather {
                text: result.now.text,
                temperature: result.now.temperature,
            };

            return Ok(current_weather);
        } else {
            println!("No results found!");
        }
    } else {
        println!("Request failed with status: {}", res.status());
    }

    Err("Error occurred during weather retrieval".into())
}

async fn get_future_weather(secret: &Secret) -> Result<Vec<DailyData>, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}/daily.json?key={}&location={}&language={}&unit=c&start=0&days=3",
        BASE_URL, secret.api_key, secret.location, secret.language
    );

    let res: reqwest::Response = reqwest::get(&url).await?;

    if res.status().is_success() {
        let body: String = res.text().await?;

        let weather_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(result) = weather_data["results"].get(0) {
            let result: DailyWeatherResult = serde_json::from_value(result.clone())?;

            let daily_weather: Vec<DailyData> = result.daily[1..].to_vec();

            return Ok(daily_weather);
        } else {
            println!("No results found!");
        }
    } else {
        println!("Request failed with status: {}", res.status());
    }

    Err("Error occurred during weather retrieval".into())
}

pub async fn show_data(secret: &Secret) -> Result<(), Box<dyn std::error::Error>> {
    let current_weather: CurrentWeather = get_current_weather(secret).await?;
    let daily_weather: Vec<DailyData> = get_future_weather(secret).await?;

    let loaction_en_to_zh: String = match secret.location.as_str() {
        "GuangZhou" => "广州".to_string(),
        "ShenZhen" => "深圳".to_string(),
        "ShangHai" => "上海".to_string(),
        "BeiJing" => "北京".to_string(),
        _ => secret.location.clone(),
    };

    if secret.language == "zh-Hans" {
        print!("{} (￣︶￣)↗ | ", loaction_en_to_zh);
        print!("{} | ", current_weather.text);
        println!("{}°C", current_weather.temperature);

        for daily_data in &daily_weather {
            println!("·");
            print!("{} | ", daily_data.date);
            print!("白天：{} | ", daily_data.text_day);
            print!("夜晚：{} | ", daily_data.text_night);
            print!("{}°C ~ ", daily_data.low);
            println!("{}°C", daily_data.high);
        }
    } else if secret.language == "en" {
        print!("{} (￣︶￣)↗ | ", secret.location);
        print!("{} | ", current_weather.text);
        println!("{}°C", current_weather.temperature);

        for daily_data in &daily_weather {
            println!("·");
            print!("{} | ", daily_data.date);
            print!("Day: {} | ", daily_data.text_day);
            print!("Night: {} | ", daily_data.text_night);
            print!("{}°C ~ ", daily_data.low);
            println!("{}°C", daily_data.high);
        }
    }

    Ok(())
}
