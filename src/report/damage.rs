use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use chrono::{DateTime, FixedOffset};
use strum_macros::Display;
use uuid::Uuid;

use crate::report::errors::{InvalidReportError, ReportResult};
use crate::report::item::Item;
use crate::report::player::{Player, SimplePlayer};
use crate::report::raw::Damage as RawDamage;
use crate::report::raw::DamageCause as RawDamageCause;
use crate::report::report::since;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Damage {
    pub date: DateTime<FixedOffset>,
    pub since_beginning: Duration,
    pub cause: DamageCause,
    pub damagee: SimplePlayer,
    pub damage: u16,
    pub lethal: bool,
}

impl Damage {
    pub fn from_raw(
        raw_damage: &RawDamage,
        players: &HashMap<Uuid, Rc<Player>>,
        begin: &DateTime<FixedOffset>,
    ) -> ReportResult<Self> {
        match players.get(&raw_damage.damagee) {
            Some(damagee) => Ok(Self {
                date: raw_damage.date.clone(),
                since_beginning: since(&raw_damage.date, begin),
                cause: match &raw_damage.cause {
                    RawDamageCause::Player(cause) => DamageCause::Player(PlayerDamageCause {
                        player: match players.get(&cause.player) {
                            Some(player) => (*player).as_ref().into(),
                            None => Err(InvalidReportError::MissingPlayerReference {
                                uuid: cause.player,
                            })?
                        },

                        weapon: cause.weapon.clone()
                    }),
                    RawDamageCause::Entity(cause) => DamageCause::Entity(cause.clone()),
                    RawDamageCause::BlockExplosion => DamageCause::BlockExplosion,
                    RawDamageCause::Contact => DamageCause::Contact,
                    RawDamageCause::Cramming => DamageCause::Cramming,
                    RawDamageCause::DragonBreath => DamageCause::DragonBreath,
                    RawDamageCause::Drowning => DamageCause::Drowning,
                    RawDamageCause::Dryout => DamageCause::Dryout,
                    RawDamageCause::Fall => DamageCause::Fall,
                    RawDamageCause::FallingBlock => DamageCause::FallingBlock,
                    RawDamageCause::Fire | RawDamageCause::FireTick => DamageCause::Fire,
                    RawDamageCause::FlyIntoWall => DamageCause::FlyIntoWall,
                    RawDamageCause::HotFloor => DamageCause::HotFloor,
                    RawDamageCause::Lava => DamageCause::Lava,
                    RawDamageCause::Lightning => DamageCause::Lightning,
                    RawDamageCause::Magic => DamageCause::Magic,
                    RawDamageCause::Melting => DamageCause::Melting,
                    RawDamageCause::Poison => DamageCause::Poison,
                    RawDamageCause::Projectile => DamageCause::Projectile,
                    RawDamageCause::Starvation => DamageCause::Starvation,
                    RawDamageCause::Suffocation => DamageCause::Suffocation,
                    RawDamageCause::Suicide => DamageCause::Suicide,
                    RawDamageCause::Thorns => DamageCause::Thorns,
                    RawDamageCause::Void => DamageCause::Void,
                    RawDamageCause::Wither => DamageCause::Wither,
                    RawDamageCause::Command => DamageCause::Command,
                    RawDamageCause::Unknown => DamageCause::Unknown
                },
                damagee: (*damagee).as_ref().into(),
                damage: raw_damage.damage,
                lethal: raw_damage.lethal,
            }),
            None => Err(InvalidReportError::MissingPlayerReference {
                uuid: raw_damage.damagee,
            }),
        }
    }

    pub fn from_raw_vec(
        raw_damages: &Vec<RawDamage>,
        players: &HashMap<Uuid, Rc<Player>>,
        begin: &DateTime<FixedOffset>,
    ) -> ReportResult<Vec<Self>> {
        let mut damages = Vec::new();

        for damage in raw_damages {
            damages.push(Self::from_raw(damage, players, begin)?);
        }

        damages.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(damages)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Display)]
#[serde(rename_all = "snake_case", tag = "type")]
#[strum(serialize_all = "snake_case")]
pub enum DamageCause {
    Player(PlayerDamageCause),
    Entity(EntityDamageCause),

    BlockExplosion,
    Contact,
    Cramming,
    DragonBreath,
    Drowning,
    Dryout,
    Fall,
    FallingBlock,
    Fire,
    FireTick, // Is merged with fire while processing
    FlyIntoWall,
    HotFloor,
    Lava,
    Lightning,
    Magic,
    Melting,
    Poison,
    Projectile,
    Starvation,
    Suffocation,
    Suicide,
    Thorns,
    Void,
    Wither,

    Command,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct EntityDamageCause {
    pub entity: String,
    pub weapon: Option<Item>
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct PlayerDamageCause {
    pub player: SimplePlayer,
    pub weapon: Option<Item>
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
    Thorns,

    Unknown,
}

impl DamageCause {
    pub fn is_creature(&self) -> bool {
        match self {
            DamageCause::Player(_) => true,
            DamageCause::Entity(_) => true,

            _ => false
        }
    }
}
