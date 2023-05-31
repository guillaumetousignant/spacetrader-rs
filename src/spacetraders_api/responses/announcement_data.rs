use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnouncementData {
    title: String,
    body: String,
}
