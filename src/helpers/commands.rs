use clap::Subcommand;
use std::path::PathBuf;

const DEFAULT_PATH: &str = "credentials.json";

#[derive(Subcommand)]
pub enum Commands {
    /// Registers a new agent
    Register {
        /// Path to credentials file
        #[arg(short='p', long, value_name = "FILE", default_value = PathBuf::from(DEFAULT_PATH).into_os_string())]
        credentials: PathBuf,

        /// Callsign to register
        #[arg(short, long)]
        callsign: String,

        /// Starting faction
        #[arg(short, long, default_value_t = String::from("COSMIC"))]
        faction: String,
    },

    /// Runs the program with an existing agent
    Run {
        /// Path to credentials file
        #[arg(short='p', long, value_name = "FILE", default_value = PathBuf::from(DEFAULT_PATH).into_os_string())]
        credentials: PathBuf,
    },

    /// Gets the status of the server
    Status {},
}
