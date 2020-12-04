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
        raw_damage: RawDamage,
        players: &HashMap<Uuid, Rc<Player>>,
        begin: &DateTime<FixedOffset>,
    ) -> ReportResult<Self> {
        let damagee = players.get(&raw_damage.damagee)
            .ok_or(InvalidReportError::MissingPlayerReference { uuid: raw_damage.damagee })?;

        Ok(Self {
            date: raw_damage.date,
            since_beginning: since(&raw_damage.date, begin),
            cause: DamageCause::from_raw(raw_damage.cause, players)?,
            damagee: (*damagee).as_ref().into(),
            damage: raw_damage.damage,
            lethal: raw_damage.lethal,
        })
    }

    pub fn should_merge_with(&self, other: &Damage) -> bool {
        self.cause == other.cause && !self.lethal
    }

    pub fn merge_with (&mut self, other: &Damage) {
        self.damage += other.damage;
        // If the new damage is lethal, so is the previous one grouped with the new.
        self.lethal = other.lethal;
    }

    /// From a vec of raw damages, extract a vec of processed and grouped damages.
    /// Damages are grouped together if they are between the same players, or between a player and
    /// the same entity type, or of the same type; with the same properties (exact same weapon,
    /// etc.); and consecutive.
    ///
    /// The given vec of raw damages is **expected to be sorted chronologically**.
    pub fn from_raw_vec(
        raw_damages: Vec<RawDamage>,
        players: &HashMap<Uuid, Rc<Player>>,
        begin: &DateTime<FixedOffset>,
    ) -> ReportResult<Vec<Self>> {
        let mut previous_damages = Vec::new();
        let mut latest_damage_per_damagee: HashMap<Uuid, Damage> = HashMap::new();

        for damage in raw_damages {
            let damage = Self::from_raw(damage, players, begin)?;

            // If the previously recorded damage is the same (same type, same damager if any, same
            // weapon), we merge them.
            match latest_damage_per_damagee.get_mut(&damage.damagee.uuid) {
                Some(prev_damage) if prev_damage.should_merge_with(&damage) => {
                    prev_damage.merge_with(&damage)
                },
                _ => {
                    let prev_damage = latest_damage_per_damagee.insert(damage.damagee.uuid, damage);
                    if let Some(prev_damage) = prev_damage {
                        previous_damages.push(prev_damage);
                    }
                }
            };
        }

        let damages = previous_damages
            .into_iter()
            .chain(latest_damage_per_damagee.into_iter().map(|(_, d)| d))
            .collect();

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

impl DamageCause {
    pub fn is_creature(&self) -> bool {
        match self {
            DamageCause::Player(_) => true,
            DamageCause::Entity(_) => true,

            _ => false
        }
    }

    pub fn from_raw(raw: RawDamageCause, players: &HashMap<Uuid, Rc<Player>>) -> ReportResult<Self> {
        Ok(match raw {
            RawDamageCause::Player(cause) => DamageCause::Player(PlayerDamageCause {
                player: players.get(&cause.player)
                    .ok_or(InvalidReportError::MissingPlayerReference { uuid: cause.player })?
                    .into(),

                weapon: cause.weapon
            }),
            RawDamageCause::Entity(cause) => DamageCause::Entity(cause),
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
        })
    }
}
