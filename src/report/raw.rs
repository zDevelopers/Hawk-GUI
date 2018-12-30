use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::report::*;

fn default_false() -> bool { false }
fn default_team_color() -> team::TeamColor { team::TeamColor::Black }


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,

    #[serde(default = "default_team_color")]
    pub color: team::TeamColor,

    pub players: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    pub damage_cause: damage::DamageCause,
    pub weapon: Option<damage::Weapon>,
    pub damager: Uuid,
    pub damagee: Uuid,
    pub damage: u16,

    #[serde(default = "default_false")]
    pub lethal: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heal {
    pub heal_cause: heal::HealCause,
    pub healed: Uuid,
    pub heal: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    pub match_uuid: Uuid,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub settings: Option<settings::Settings>,
    pub players: Vec<player::Player>,
    pub teams: Vec<Team>,
    pub damages: Vec<Damage>,
    pub heals: Vec<Heal>,
    pub events: Vec<event::Event>,
}
