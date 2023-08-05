use crate::data::CurrentWeather;
use crate::data::DailyData;
use crate::data::DailyWeatherResult;
use crate::data::NowWeatherResult;
use crate::Secret;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

const BASE_URL: &str = "https://api.seniverse.com/v3/weather";

async fn reset_json_file() -> Result<(), Box<dyn std::error::Error>> {
    let default_api_key: String = "SkyWErkPwye-1C6wv".to_string();
    let default_location: String = "ShanTou".to_string();
    let default_language: String = "en".to_string();

    let secret_data: Secret = Secret {
        api_key: default_api_key.clone(),
        location: default_location.clone(),
        language: default_language.clone(),
    };

    let json_string: String =
        serde_json::to_string(&secret_data).expect("JSON serialization failed.");

    let mut file: File = File::create("secret.json").unwrap_or_else(|err: std::io::Error| {
        eprintln!("Error creating file: {}", err);
        File::open("dummy.json").expect("Error opening dummy file")
    });

    if let Err(err) = file.write_all(json_string.as_bytes()) {
        eprintln!("Error writing to file: {}", err);
    } else {
        println!("Data successfully written to the secret.json file.");
    }

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

    if secret.language == "zh-Hans" {
        println!("在{} （￣︶￣）↗", secret.location);
        println!("现在是{}天", current_weather.text);
        println!("气温：{}°C", current_weather.temperature);

        for daily_data in &daily_weather {
            println!("-");
            println!("{}", daily_data.date);
            println!("----------");
            println!("白天：{}", daily_data.text_day);
            println!("夜晚：{}", daily_data.text_night);
            println!("最高气温：{}°C", daily_data.high);
            println!("最低气温：{}°C", daily_data.low);
        }
    } else if secret.language == "en" {
        println!("In {} (￣︶￣)↗", secret.location);
        println!("Now is {}", current_weather.text);
        println!("Temperature: {}°C", current_weather.temperature);

        for daily_data in &daily_weather {
            println!("-");
            println!("{}", daily_data.date);
            println!("----------");
            println!("Day: {}", daily_data.text_day);
            println!("Night: {}", daily_data.text_night);
            println!("High: {}°C", daily_data.high);
            println!("Low: {}°C", daily_data.low);
        }
    }

    Ok(())
}
