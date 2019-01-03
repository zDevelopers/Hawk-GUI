use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

use crate::report::raw::Player as RawPlayer;
use crate::report::team::TeamColor;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub uuid: Uuid,
    pub name: String,
    pub color: TeamColor,

    pub tag_line: String,
    pub tag_line_secondary: String,
    pub tag_line_details: String,

    pub statistics: Option<PlayerStatistics>
}

impl Player {
    pub fn from_raw(raw_player: &RawPlayer, colors: &HashMap<Uuid, TeamColor>, default_color: &TeamColor) -> Self {
        Self {
            uuid: raw_player.uuid.clone(),
            name: raw_player.name.clone(),
            color: colors.get(&raw_player.uuid).unwrap_or(default_color).clone(),
            tag_line: raw_player.tag_line.clone().unwrap_or("".to_string()).clone(),
            tag_line_secondary: raw_player.tag_line_secondary.clone().unwrap_or("".to_string()).clone(),
            tag_line_details: raw_player.tag_line_details.clone().unwrap_or("".to_string()).clone(),
            statistics: raw_player.statistics.clone()
        }
    }
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
    pub name: String,
    pub color: TeamColor,
}

impl From<Player> for SimplePlayer {
    fn from(player: Player) -> Self {
        SimplePlayer {
            uuid: player.uuid,
            name: player.name,
            color: player.color
        }
    }
}

impl From<&Player> for SimplePlayer {
    fn from(player: &Player) -> Self {
        SimplePlayer {
            uuid: player.uuid.clone(),
            name: player.name.clone(),
            color: player.color.clone()
        }
    }
}

impl From<Rc<Player>> for SimplePlayer {
    fn from(player: Rc<Player>) -> Self {
        SimplePlayer {
            uuid: player.uuid.clone(),
            name: player.name.clone(),
            color: player.color.clone()
        }
    }
}

impl From<&Rc<Player>> for SimplePlayer {
    fn from(player: &Rc<Player>) -> Self {
        SimplePlayer {
            uuid: player.uuid.clone(),
            name: player.name.clone(),
            color: player.color.clone()
        }
    }
}
