mod data;
use data::Secret;

mod utils;
use utils::read_json_file;
use utils::show_data;
use utils::write_json_file;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "secret.json";
    let mut secret: Secret = read_json_file(file_path).await?;

    let args: Vec<String> = env::args().collect();
    let operation: &str = if args.len() > 1 { &args[1] } else { "." };

    match operation {
        "." => {
            show_data(&secret.api_key, &secret.location).await?;
        }
        "--location" => {
            if args.len() < 3 {
                eprintln!("Error: Missing location argument.");
                return Ok(());
            }
            secret.location = args[2].clone();
            write_json_file(file_path, &secret).await?;
        }
        "--help" => {
            println!("Usage:");
            println!("  --location <location> : Set the location in the secret.json file.");
            println!("  --help                : Show this help message.");
            println!();
            return Ok(());
        }
        _ => {
            eprintln!("Error: Unknown command. Use --help for usage information.");
            return Ok(());
        }
    }

    Ok(())
}
