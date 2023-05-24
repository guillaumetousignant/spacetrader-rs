use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Sets a custom credentials file [default: credentials.json]
    #[arg(short, long, value_name = "FILE")]
    // Can't figure out how to make a path default value
    credentials: Option<PathBuf>,

    /// Increase verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Command to use
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentResponseData {
    #[serde(rename = "accountId")]
    account_id: String,
    symbol: String,
    headquarters: String,
    credits: i128,
    #[serde(rename = "startingFaction")]
    starting_faction: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentResponse {
    data: AgentResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistrationRequest {
    symbol: String,
    faction: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeliveryResponseData {
    #[serde(rename = "tradeSymbol")]
    trade_symbol: String,
    #[serde(rename = "destinationSymbol")]
    destination_symbol: String,
    #[serde(rename = "unitsRequired")]
    units_required: u128,
    #[serde(rename = "unitsFulfilled")]
    units_fulfilled: u128,
}

#[derive(Debug, Serialize, Deserialize)]
struct PaymentResponseData {
    #[serde(rename = "onAccepted")]
    on_accepted: u128,
    #[serde(rename = "onFulfilled")]
    on_fulfilled: u128,
}

#[derive(Debug, Serialize, Deserialize)]
struct TermsResponseData {
    deadline: String,
    payment: PaymentResponseData,
    deliver: Vec<DeliveryResponseData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContractResponseData {
    id: String,
    #[serde(rename = "factionSymbol")]
    faction_symbol: String,
    #[serde(rename = "type")]
    contract_type: String,
    terms: TermsResponseData,
    accepted: bool,
    fulfilled: bool,
    expiration: String,
    #[serde(rename = "deadlineToAccept")]
    deadline_to_accept: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TraitResponseData {
    symbol: String,
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FactionResponseData {
    symbol: String,
    name: String,
    description: String,
    headquarters: String,
    traits: Vec<TraitResponseData>,
    #[serde(rename = "isRecruiting")]
    is_recruiting: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct LocationResponseData {
    symbol: String,
    #[serde(rename = "type")]
    location_type: String,
    #[serde(rename = "systemSymbol")]
    system_symbol: String,
    x: i128,
    y: i128,
}

#[derive(Debug, Serialize, Deserialize)]
struct RouteResponseData {
    departure: LocationResponseData,
    destination: LocationResponseData,
    arrival: String,
    #[serde(rename = "departureTime")]
    departure_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NavResponseData {
    #[serde(rename = "systemSymbol")]
    system_symbol: String,
    #[serde(rename = "waypointSymbol")]
    waypoint_symbol: String,
    route: RouteResponseData,
    status: String,
    #[serde(rename = "flightMode")]
    flight_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrewResponseData {
    current: u128,
    capacity: u128,
    required: u128,
    rotation: String,
    morale: u128,
    wages: u128,
}

#[derive(Debug, Serialize, Deserialize)]
struct FuelConsumedResponseData {
    amount: u128,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FuelResponseData {
    current: u128,
    capacity: u128,
    consumed: FuelConsumedResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequirementsResponseData {
    power: Option<i128>,
    crew: Option<u128>,
    slots: Option<u128>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FrameResponseData {
    symbol: String,
    name: String,
    description: String,
    #[serde(rename = "moduleSlots")]
    module_slots: u128,
    #[serde(rename = "mountingPoints")]
    mounting_points: u128,
    #[serde(rename = "fuelCapacity")]
    fuel_capacity: u128,
    condition: i128,
    requirements: RequirementsResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReactorResponseData {
    symbol: String,
    name: String,
    description: String,
    condition: i128,
    #[serde(rename = "powerOutput")]
    power_output: i128,
    requirements: RequirementsResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct EngineResponseData {
    symbol: String,
    name: String,
    description: String,
    condition: i128,
    speed: i128,
    requirements: RequirementsResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModuleResponseData {
    symbol: String,
    name: String,
    description: String,
    capacity: Option<i128>,
    requirements: RequirementsResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct MountResponseData {
    symbol: String,
    name: String,
    description: String,
    strength: i128,
    deposits: Option<Vec<String>>,
    requirements: RequirementsResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistrationResponseData {
    name: String,
    #[serde(rename = "factionSymbol")]
    faction_symbol: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InventoryItemResponseData {
    symbol: String,
    name: String,
    description: String,
    units: u128,
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoResponseData {
    capacity: u128,
    units: u128,
    inventory: Vec<InventoryItemResponseData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ShipResponseData {
    symbol: String,
    nav: NavResponseData,
    crew: CrewResponseData,
    fuel: FuelResponseData,
    frame: FrameResponseData,
    reactor: ReactorResponseData,
    engine: EngineResponseData,
    modules: Vec<ModuleResponseData>,
    mounts: Vec<MountResponseData>,
    registration: RegistrationResponseData,
    cargo: CargoResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentRegistrationResponseData {
    token: String,
    agent: AgentResponseData,
    contract: ContractResponseData,
    faction: FactionResponseData,
    ship: ShipResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentRegistrationResponse {
    data: AgentRegistrationResponseData,
}

async fn register(
    path: &Path,
    callsign: &str,
    faction: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let registration_request = RegistrationRequest {
        symbol: callsign.into(),
        faction: faction.into(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.spacetraders.io/v2/register")
        .json(&registration_request)
        .send()
        .await?;
    let agent_response: AgentRegistrationResponse = res.json().await?;

    let credentials = Credentials {
        token: agent_response.data.token.clone(),
    };

    println!("{:#?}", agent_response);

    fs::write(path, serde_json::to_string_pretty(&credentials)?)?;
    Ok(())
}

async fn run(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let credentials_data = fs::read_to_string(path)?;
    let credentials: Credentials = serde_json::from_str(&credentials_data)?;

    let client = reqwest::Client::new();
    let res = client
        .get("https://api.spacetraders.io/v2/my/agent")
        .bearer_auth(credentials.token)
        .send()
        .await?;
    let agent_response: AgentResponse = res.json().await?;
    println!("{:#?}", agent_response);
    Ok(())
}

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
