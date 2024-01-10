mod data;
mod utils;

use crate::data::Secret;
use crate::utils::{read_json_file, show_data, write_json_file};
use std::env;
use std::error::Error;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut secret: Secret = load_secret().await?;
    let args: Vec<String> = env::args().collect();

    parse_arguments(&args, &mut secret)?;

    let file_path_str = get_secret_file_path().to_string_lossy().to_string();
    write_json_file(&file_path_str, &secret).await?;
    show_data(&secret).await?;

    Ok(())
}

async fn load_secret() -> Result<Secret, Box<dyn Error>> {
    let file_path: PathBuf = get_secret_file_path();
    let file_path_str: std::borrow::Cow<'_, str> = file_path.to_string_lossy();
    read_json_file(&file_path_str).await
}

fn get_secret_file_path() -> PathBuf {
    let mut home_dir: PathBuf = dirs::home_dir().expect("Failed to get home directory");
    home_dir.push("secret.json");
    home_dir
}

fn parse_arguments(args: &[String], secret: &mut Secret) -> Result<(), Box<dyn Error>> {
    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--location" => {
                parse_location_arg(args, &mut i, secret)?;
            }
            "--language" => {
                parse_language_arg(args, &mut i, secret)?;
            }
            "--help" => {
                print_help();
                return Ok(());
            }
            _ => {
                eprintln!("Error: Unknown command. Use --help for usage information.");
                return Ok(());
            }
        }
    }
    Ok(())
}

fn parse_location_arg(
    args: &[String],
    i: &mut usize,
    secret: &mut Secret,
) -> Result<(), Box<dyn Error>> {
    if *i + 1 >= args.len() {
        eprintln!("Error: Missing location argument.");
        return Ok(());
    }
    secret.location = args[*i + 1].clone();
    *i += 2;
    Ok(())
}

fn parse_language_arg(
    args: &[String],
    i: &mut usize,
    secret: &mut Secret,
) -> Result<(), Box<dyn Error>> {
    if *i + 1 >= args.len() {
        eprintln!("Error: Missing language argument.");
        return Ok(());
    }
    secret.language = args[*i + 1].clone();
    *i += 2;
    Ok(())
}

fn print_help() {
    println!("Usage:");
    println!("  --location <location> : Set the location in the secret.json file.");
    println!("  --language <language> : Set the language in the secret.json file. Only supports Chinese (zh-Hans) and English (en).");
    println!("  --help                : Show this help message.");
    println!();
}
