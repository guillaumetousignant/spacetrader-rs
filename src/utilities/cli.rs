use super::Commands;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Sets a custom credentials file [default: credentials.json]
    #[arg(short, long, value_name = "FILE")]
    // Can't figure out how to make a path default value
    pub credentials: Option<PathBuf>,

    /// Increase verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Command to use
    #[command(subcommand)]
    pub command: Commands,
}
