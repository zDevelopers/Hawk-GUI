use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::report::*;

#[inline(always)]
fn default_false() -> bool {
    false
}
#[inline(always)]
pub fn default_team_color() -> team::TeamColor {
    team::TeamColor::None
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub uuid: Uuid,
    pub name: String,

    pub tag_line: Option<String>,
    pub tag_line_secondary: Option<String>,
    pub tag_line_details: Option<String>,

    pub statistics: Option<player::PlayerStatistics>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,

    #[serde(default = "default_team_color")]
    pub color: team::TeamColor,

    pub players: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    pub date: DateTime<FixedOffset>,
    pub cause: damage::DamageCause,
    pub weapon: Option<damage::Weapon>,
    pub weapon_name: Option<String>,
    pub weapon_enchantments: Option<HashMap<String, u32>>,
    pub damager: Option<Uuid>,
    pub damagee: Uuid,
    pub damage: u16,

    #[serde(default = "default_false")]
    pub lethal: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heal {
    pub date: DateTime<FixedOffset>,
    pub cause: heal::HealCause,
    pub healed: Uuid,
    pub heal: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub date: DateTime<FixedOffset>,

    #[serde(default = "event::default_event_type", rename = "type")]
    pub event_type: event::EventType,

    pub title: String,
    pub description: Option<String>,

    pub icon: event::EventIcon,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    pub match_uuid: Uuid,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub minecraft: Option<String>,

    #[serde(default = "settings::default_settings")]
    pub settings: settings::Settings,

    pub players: Vec<Player>,
    pub teams: Vec<Team>,
    pub winners: Option<Vec<Uuid>>,
    pub damages: Vec<Damage>,
    pub heals: Vec<Heal>,
    pub events: Vec<Event>,
}
