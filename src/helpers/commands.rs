use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Registers a new agent
    Register {
        /// Callsign to register
        #[arg(short, long)]
        callsign: String,

        /// Starting faction
        #[arg(short, long, default_value_t = String::from("COSMIC"))]
        faction: String,
    },
    /// Runs the program with an existing agent
    Run {},
}
