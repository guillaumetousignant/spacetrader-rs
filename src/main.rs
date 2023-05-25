use clap::Parser;
use spacetraders_rs::commands::register;
use spacetraders_rs::commands::run;
use spacetraders_rs::utilities::Cli;
use spacetraders_rs::utilities::Commands;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let path: &Path = match cli.credentials.as_deref() {
        Some(credentials_path) => credentials_path,
        None => Path::new("credentials.json"),
    };

    match &cli.command {
        Commands::Register { callsign, faction } => {
            return register(path, callsign, faction).await;
        }
        Commands::Run {} => {
            return run(path).await;
        }
    }
}
