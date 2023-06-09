use super::Announcement;
use super::Leaderboard;
use super::Link;
use super::ServerResets;
use super::Stats;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    status: String,
    version: String,
    #[serde(rename = "resetDate")]
    reset_date: String,
    description: String,
    stats: Stats,
    leaderboards: Leaderboard,
    #[serde(rename = "serverResets")]
    server_resets: ServerResets,
    announcements: Vec<Announcement>,
    links: Vec<Link>,
}
