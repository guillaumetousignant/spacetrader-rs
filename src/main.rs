use clap::Parser;
use spacetraders_rs::commands::info;
use spacetraders_rs::commands::register;
use spacetraders_rs::commands::run;
use spacetraders_rs::commands::status;
use spacetraders_rs::helpers::Cli;
use spacetraders_rs::helpers::Commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Info { credentials } => {
            return info(credentials).await;
        }
        Commands::Register {
            credentials,
            callsign,
            faction,
        } => {
            return register(credentials, callsign, faction).await;
        }
        Commands::Run { credentials } => {
            return run(credentials).await;
        }
        Commands::Status {} => {
            return status().await;
        }
    }
}
