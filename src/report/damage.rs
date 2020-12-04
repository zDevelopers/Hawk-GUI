use std::cell::RefCell;
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

    /// From a vec of raw damages, extract a vec of processed and grouped damages.
    /// Damages are grouped together if they are between the same players, or between a player and
    /// the same entity type, or of the same type, with the same properties (exact same weapon,
    /// etc.) and consecutive.
    ///
    /// The given vec of raw damages is **expected to be sorted chronologically**.
    pub fn from_raw_vec(
        raw_damages: &Vec<RawDamage>,
        players: &HashMap<Uuid, Rc<Player>>,
        begin: &DateTime<FixedOffset>,
    ) -> ReportResult<Vec<Self>> {
        let mut damages = Vec::new();
        let mut latest_damage_per_damagee: HashMap<Uuid, Rc<RefCell<Damage>>> = HashMap::new();

        for damage in raw_damages {
            let damage = Rc::new(RefCell::new(Self::from_raw(damage, players, begin)?));
            // TODO replace with map
            let prev_damage = match latest_damage_per_damagee.get(&damage.borrow().damagee.uuid) {
                Some(prev_damage) => Some(Rc::clone(prev_damage)),
                None => None
            };

            // If the previously recorded damage is the same (same type, same damager if any, same
            // weapon), we merge them.
            let should_merge = match prev_damage {
                Some(ref prev_damage) => match &prev_damage.borrow().cause {
                    DamageCause::Player(prev_cause) => match &damage.borrow().cause {
                        DamageCause::Player(cause) => {
                            prev_cause.player.uuid == cause.player.uuid
                            && prev_cause.weapon == cause.weapon
                            // If the previous damage is lethal, we don't group them.
                            && !prev_damage.borrow().lethal
                        },
                        _ => false
                    },

                    DamageCause::Entity(prev_cause) => match &damage.borrow().cause {
                        DamageCause::Entity(cause) => {
                            prev_cause.entity == cause.entity
                            && prev_cause.weapon == cause.weapon
                            && !prev_damage.borrow().lethal
                        },
                        _ => false
                    },

                    _ => {
                        eprintln!("Testing merge: {:?} vs {:?} (prev lethal: {:?}) - {:?}", prev_damage.borrow().cause, damage.borrow().cause, prev_damage.borrow().lethal, prev_damage.borrow().cause == damage.borrow().cause && !prev_damage.borrow().lethal);
                        prev_damage.borrow().cause == damage.borrow().cause && !prev_damage.borrow().lethal
                    }
                },

                None => false
            };

            latest_damage_per_damagee.insert((&damage.borrow().damagee.uuid).clone(), Rc::clone(&damage));

            if should_merge {
                match &prev_damage {
                    Some(ref prev_damage) => {
                        let mut prev_damage = prev_damage.borrow_mut();
                        eprintln!("-- Merging damages --");
                        eprintln!("  Existing: {:?}", prev_damage);
                        let old_damage = prev_damage.damage;

                        prev_damage.damage += damage.borrow().damage;

                        // If the new damage is lethal, so is the previous one grouped with the new.
                        prev_damage.lethal = damage.borrow().lethal;

                        eprintln!("  Merging: {}+{} = {} - lethal: {} ({})", old_damage, damage.borrow().damage, prev_damage.damage, damage.borrow().lethal, prev_damage.lethal);
                    },
                    None => ()
                }
            } else {
                eprintln!("-- Inserting new damage -- {:?} - {}", damage.borrow(), damage.borrow().damage);
                damages.push(Rc::clone(&damage));
            }
        }

        Ok(damages.into_iter().map(|d| d.as_ref().clone().into_inner()).collect())
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
}
