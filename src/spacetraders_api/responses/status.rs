use super::AnnouncementData;
use super::LeaderboardData;
use super::LinkData;
use super::ServerResetsData;
use super::StatsData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    status: String,
    version: String,
    #[serde(rename = "resetDate")]
    reset_date: String,
    description: String,
    stats: StatsData,
    leaderboards: LeaderboardData,
    #[serde(rename = "serverResets")]
    server_resets: ServerResetsData,
    announcements: Vec<AnnouncementData>,
    links: Vec<LinkData>,
}
