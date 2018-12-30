use std::collections::HashMap;

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub uuid: Uuid,
    pub name: String,

    pub tag_line: Option<String>,
    pub tag_line_secondary: Option<String>,
    pub tag_line_details: Option<String>,

    pub statistics: Option<PlayerStatistics>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStatistics {
    pub generic: Option<HashMap<String, u32>>,
    pub used: Option<HashMap<String, u32>>,
    pub mined: Option<HashMap<String, u32>>,
    pub picked_up: Option<HashMap<String, u32>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimplePlayer {
    pub uuid: Uuid,
    pub name: String
}
