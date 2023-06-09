use clap::Subcommand;
use std::path::PathBuf;

const DEFAULT_CREDENTIALS_PATH: &str = "credentials.json";
const DEFAULT_SHIPS_PATH: &str = "ships.json";

#[derive(Subcommand)]
pub enum Commands {
    /// Shows info about an agent
    Info {
        /// Path to credentials file
        #[arg(short='c', long, value_name = "FILE", default_value = PathBuf::from(DEFAULT_CREDENTIALS_PATH).into_os_string())]
        credentials: PathBuf,

        /// Path to ships file
        #[arg(short='s', long, value_name = "FILE", default_value = PathBuf::from(DEFAULT_SHIPS_PATH).into_os_string())]
        ships: PathBuf,
    },

    /// Registers a new agent
    Register {
        /// Path to credentials file
        #[arg(short='c', long, value_name = "FILE", default_value = PathBuf::from(DEFAULT_CREDENTIALS_PATH).into_os_string())]
        credentials: PathBuf,

        /// Path to ships file
        #[arg(short='s', long, value_name = "FILE", default_value = PathBuf::from(DEFAULT_SHIPS_PATH).into_os_string())]
        ships: PathBuf,

        /// Callsign to register
        #[arg(short = 'i', long)]
        callsign: String,

        /// Starting faction
        #[arg(short, long, default_value_t = String::from("COSMIC"))]
        faction: String,
    },

    /// Runs the program with an existing agent
    Run {
        /// Path to credentials file
        #[arg(short='c', long, value_name = "FILE", default_value = PathBuf::from(DEFAULT_CREDENTIALS_PATH).into_os_string())]
        credentials: PathBuf,

        /// Path to ships file
        #[arg(short='s', long, value_name = "FILE", default_value = PathBuf::from(DEFAULT_SHIPS_PATH).into_os_string())]
        ships: PathBuf,
    },

    /// Gets the status of the server
    Status {},
}
