use std::collections::HashMap;
use std::io::Read;

use failure::{Error, ResultExt};
use serde_json;
use uuid::Uuid;

use crate::serde_instant::Instant;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum TeamColor {
    AQUA,
    BLACK,
    BLUE,
    DARK_AQUA,
    DARK_BLUE,
    DARK_GRAY,
    DARK_GREEN,
    DARK_PURPLE,
    GOLD,
    GRAY,
    GREEN,
    LIGHT_PURPLE,
    RED,
    WHITE,
    YELLOW
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub uuid: Uuid,
    pub name: String,
    pub statistics: Option<PlayerStatistics>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStatistics {
    pub play_time: Option<u16>,
    pub sneak_time: Option<u16>,
    pub movements: Option<PlayerStatisticsMovements>,
    pub actions: Option<PlayerStatisticsActions>,
    pub mining: Option<HashMap<String, u16>>,
    pub pickup: Option<HashMap<String, u16>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStatisticsMovements {
    pub walk_one_cm: Option<u32>,
    pub crouch_one_cm: Option<u32>,
    pub sprint_one_cm: Option<u32>,
    pub swim_one_cm: Option<u32>,
    pub fall_one_cm: Option<u32>,
    pub climb_one_cm: Option<u32>,
    pub fly_one_cm: Option<u32>,
    pub walk_under_water_one_cm: Option<u32>,
    pub minecart_one_cm: Option<u32>,
    pub boat_one_cm: Option<u32>,
    pub pig_one_cm: Option<u32>,
    pub horse_one_cm: Option<u32>,
    pub aviate_one_cm: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStatisticsActions {
    pub jump: Option<u32>,
    pub eat: Option<u32>,
    pub mob_kills: Option<u32>,
    pub player_kills: Option<u32>,
    pub drops: Option<u32>,
    pub enchant_item: Option<u32>,
    pub animals_bread: Option<u32>,
    pub fish_caught: Option<u32>,
    pub talked_to_villager: Option<u32>,
    pub traded_with_villager: Option<u32>,
    pub interact_with_crafting_table: Option<u32>,
    pub interact_with_furnace: Option<u32>,
    pub interact_with_brewingstand: Option<u32>,
    pub open_chest: Option<u32>,
    pub trigger_trapped_chest: Option<u32>,
    pub pot_flower: Option<u32>,
    pub play_record: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub color: TeamColor,
    pub players: Vec<Player>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    pub damage_cause: DamageCause,
    pub weapon: Option<Weapon>,
    pub damager: Uuid,
    pub damagee: Uuid,
    pub damage: u16,
    pub lethal: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum DamageCause {
    PLAYER,       // TODO: “moved“ cause
    PLAYER_SWEEP, // TODO: added cause
    ZOMBIE,
    SKELETON,
    PIGMAN,
    SPIDER,
    CAVE_SPIDER,  // TODO: different name in DamagesLogger
    CREEPER,
    ENDERMAN,
    SLIME,
    GHAST,
    MAGMA_CUBE,
    BLAZE,
    WOLF,
    ANGRY_WOLF,   // TODO: idem
    SILVERFISH,
    IRON_GOLEM,
    ZOMBIE_VILLAGER,
    THUNDERBOLT,
    ENDER_DRAGON,
    WITHER,
    WITHER_SKELETON,

    FIRE,
    LAVA,
    CACTUS,
    TNT,
    FALL,
    SUFFOCATION,
    DROWNING,
    STARVATION,

    COMMAND,
    UNKNOWN
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Weapon {
    FISTS,

    SWORD_WOOD,
    SWORD_STONE,
    SWORD_IRON,
    SWORD_GOLD,
    SWORD_DIAMOND,

    AXE_WOOD,
    AXE_STONE,
    AXE_IRON,
    AXE_GOLD,
    AXE_DIAMOND,

    BOW,

    MAGIC,
    THORNS
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heal {
    pub heal_cause: HealCause,
    pub healed: Uuid,
    pub heal: u16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum HealCause {
    NATURAL,
    GOLDEN_APPLE,
    NOTCH_APPLE,
    HEALING_POTION,
    COMMAND,
    UNKNOWN
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    pub match_uuid: Uuid,
    pub title: String,
    pub date: Instant,
    pub teams: Vec<Team>,
    pub damages: Vec<Damage>,
    pub heals: Vec<Heal>
}


///
/// Reads a raw JSON report to the raw Rust structure.
///
pub fn read_report<R: Read>(reader: R) -> Result<Report, Error> {
    Ok(serde_json::from_reader(reader).context("Unable to parse raw report JSON")?)
}
