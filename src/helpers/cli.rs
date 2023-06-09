use super::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Command to use
    #[command(subcommand)]
    pub command: Commands,

    /// Increase verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Silence all output
    #[arg(short, long)]
    pub quiet: bool,
}
