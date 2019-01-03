use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::report::errors::{InvalidReportError, ReportResult};
use crate::report::player::{Player, SimplePlayer};
use crate::report::raw::Damage as RawDamage;
use crate::report::report::since;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    pub date: DateTime<FixedOffset>,
    pub since_beginning: Duration,
    pub cause: DamageCause,
    pub weapon: Weapon,
    pub damager: Option<SimplePlayer>,
    pub damagee: SimplePlayer,
    pub damage: u16,
    pub lethal: bool
}

impl Damage {
    pub fn from_raw(raw_damage: &RawDamage, players: &HashMap<Uuid, Rc<Player>>, begin: &DateTime<FixedOffset>) -> ReportResult<Self> {
        match players.get(&raw_damage.damagee) {
            Some(damagee) => Ok(Self {
                date: raw_damage.date.clone(),
                since_beginning: since(&raw_damage.date, begin),
                cause: raw_damage.cause.clone(),
                weapon: raw_damage.weapon.clone().unwrap_or(Weapon::Fists).clone(),
                damager: match raw_damage.damager {
                    Some(damager) => match players.get(&damager) {
                        Some(damager) => Some((*damager).as_ref().into()),
                        None => Err(InvalidReportError::MissingPlayerReference { uuid: raw_damage.damagee })?
                    },
                    None => None
                },
                damagee: (*damagee).as_ref().into(),
                damage: raw_damage.damage,
                lethal: raw_damage.lethal
            }),
            None => Err(InvalidReportError::MissingPlayerReference { uuid: raw_damage.damagee })
        }
    }

    pub fn from_raw_vec(raw_damages: &Vec<RawDamage>, players: &HashMap<Uuid, Rc<Player>>, begin: &DateTime<FixedOffset>) -> ReportResult<Vec<Self>> {
        let mut damages = Vec::new();

        for damage in raw_damages {
            damages.push(Self::from_raw(damage, players, begin)?);
        }

        damages.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(damages)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DamageCause {
    Player,       // TODO: “moved“ cause
    PlayerSweep,  // TODO: added cause
    Zombie,
    Skeleton,
    Pigman,
    Spider,
    CaveSpider,   // TODO: different name in DamagesLogger
    Creeper,
    Enderman,
    Slime,
    Ghast,
    MagmaCube,
    Blaze,
    Wolf,
    AngryWolf,    // TODO: idem
    Silverfish,
    IronGolem,
    ZombieVillager,
    EnderDragon,
    Wither,
    WitherSkeleton,

    Fire,
    Lava,
    Thunderbolt,
    Cactus,
    TNT,
    Fall,
    Suffocation,
    Drowning,
    Starvation,

    Command,
    Unknown
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Weapon {
    Fists,

    SwordWood,
    SwordStone,
    SwordIron,
    SwordGold,
    SwordDiamond,

    AxeWood,
    AxeStone,
    AxeIron,
    AxeGold,
    AxeDiamond,

    Bow,

    Magic,
    Thorns
}
