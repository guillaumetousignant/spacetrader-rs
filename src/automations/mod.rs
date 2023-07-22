mod acquisitions;
mod actions;
mod contracts;
mod mining;
mod queries;
mod ship_automation;
mod utilities;

pub use acquisitions::acquisitions;
pub use contracts::contracts;
pub use mining::mining;
pub use queries::queries;
pub use ship_automation::ShipAutomation;
pub use ship_automation::UnknownShipAutomationError;

const ACQUISITIONS_INTERVAL_SECS: u64 = 10;
const CONTRACTS_INTERVAL_SECS: u64 = 10;
const RATE_LIMIT_MILLI: u64 = 500;
const RATE_LIMIT_BUFFER_MILLI: u64 = 0;
