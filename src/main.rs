mod data;
mod utils;

use crate::data::Secret;
use crate::utils::{read_json_file, show_data, write_json_file};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "secret.json";
    let mut secret: Secret = read_json_file(file_path).await?;

    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--location" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: Missing location argument.");
                    return Ok(());
                }
                secret.location = args[i + 1].clone();
                i += 2;
            }
            "--language" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: Missing language argument.");
                    return Ok(());
                }
                secret.language = args[i + 1].clone();
                i += 2;
            }
            "--help" => {
                println!("Usage:");
                println!("  --location <location> : Set the location in the secret.json file.");
                println!("  --language <language> : Set the language in the secret.json file. Only supports Chinese (zh-Hans) and English (en).");
                println!("  --help                : Show this help message.");
                println!();
                return Ok(());
            }
            _ => {
                eprintln!("Error: Unknown command. Use --help for usage information.");
                return Ok(());
            }
        }
    }

    write_json_file(file_path, &secret).await?;
    show_data(&secret).await?;

    Ok(())
}
