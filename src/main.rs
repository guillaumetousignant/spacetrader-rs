use chrono::Local;
use clap::Parser;
use env_logger::Builder;
use log::warn;
use log::LevelFilter;
use spacetraders_rs::commands::info;
use spacetraders_rs::commands::register;
use spacetraders_rs::commands::run;
use spacetraders_rs::commands::status;
use spacetraders_rs::helpers::Cli;
use spacetraders_rs::helpers::Commands;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    let log_level = match cli.quiet {
        true => LevelFilter::Off,
        false => match cli.verbose {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            4 => LevelFilter::Trace,
            _ => LevelFilter::Trace,
        },
    };

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log_level)
        .init();

    if cli.verbose > 4 {
        warn!("Verbosity level is above maximum of 4");
    }

    match &cli.command {
        Commands::Info { credentials, ships } => {
            return info(credentials, ships).await;
        }
        Commands::Register {
            credentials,
            ships,
            callsign,
            faction,
        } => {
            return register(credentials, ships, callsign, faction).await;
        }
        Commands::Run { credentials, ships } => {
            return run(credentials, ships).await;
        }
        Commands::Status {} => {
            return status().await;
        }
    }
}
